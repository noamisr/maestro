//! Zrythm engine adapter.
//!
//! ## How it works
//!
//! Zrythm v2 has no built-in OSC or scripting interface.  The recommended
//! programmatic control surface is the **JACK audio server**:
//!
//! * **Transport** (play/stop) — via the JACK Transport protocol.
//!   Maestro registers as the JACK *timebase master* so it can push BPM
//!   changes to every other JACK client including Zrythm.
//!
//! * **Track parameters** (volume, pan, mute, solo) — via **MIDI CC messages**
//!   sent over a dedicated JACK MIDI output port (`maestro:control_out`).
//!   Zrythm's "MIDI learn" feature maps each CC to the desired parameter:
//!
//!   | Parameter | CC  | Channel      |
//!   |-----------|-----|--------------|
//!   | Volume    |  7  | track index  |
//!   | Pan       | 10  | track index  |
//!   | Mute      | 119 | track index  |
//!   | Solo      | 118 | track index  |
//!
//! ## Setup
//!
//! 1. Start a JACK server (`jackd` or PipeWire-JACK).
//! 2. Set `MAESTRO_ENGINE=zrythm` and launch Maestro.
//! 3. In your JACK patchbay, connect `maestro:control_out` to Zrythm's MIDI
//!    input.
//! 4. In Zrythm, right-click each track parameter → "MIDI learn", then move
//!    the corresponding control in Maestro to bind it.
//!
//! References:
//! * <https://jackaudio.org/api/>
//! * <https://manual.zrythm.org/en/configuration/device-setup.html>

use std::collections::VecDeque;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use jack::{Client, ClientOptions, Control, MidiOut, Port, ProcessHandler, ProcessScope, RawMidi};
use tauri::{AppHandle, Emitter};

use crate::engine::{EngineAdapter, StateManager};

// ── MIDI CC assignments ────────────────────────────────────────────────────

const CC_VOLUME: u8 = 7;   // Standard MIDI volume (spec CC #7)
const CC_PAN: u8 = 10;     // Standard MIDI balance/pan (spec CC #10)
const CC_MUTE: u8 = 119;   // Custom — bind via Zrythm MIDI learn (≥64 = muted)
const CC_SOLO: u8 = 118;   // Custom — bind via Zrythm MIDI learn (≥64 = soloed)

/// Maximum MIDI channels == maximum directly-addressable tracks.
const MAX_MIDI_CH: i32 = 16;

// ── JACK process handler ───────────────────────────────────────────────────

struct ZrythmProcess {
    midi_out: Port<MidiOut>,
    /// Outbound MIDI CC queue.  `try_lock` is used in the real-time callback
    /// to avoid blocking; skips the cycle if the lock is contended.
    pending: Arc<Mutex<VecDeque<[u8; 3]>>>,
}

impl ProcessHandler for ZrythmProcess {
    fn process(&mut self, _client: &Client, ps: &ProcessScope) -> Control {
        let mut writer = self.midi_out.writer(ps);
        if let Ok(mut queue) = self.pending.try_lock() {
            for (t, msg) in queue.drain(..).enumerate() {
                let _ = writer.write(&RawMidi {
                    time: t as u32,
                    bytes: &msg,
                });
            }
        }
        Control::Continue
    }
}

// ── JACK timebase callback ─────────────────────────────────────────────────

/// Called each process cycle while Maestro is JACK timebase master.
/// Writes the current BPM into `pos` so Zrythm and other clients see it.
///
/// # Safety
/// `arg` must point to a live `AtomicU32` containing the desired BPM as bits.
unsafe extern "C" fn timebase_callback(
    _state: jack_sys::jack_transport_state_t,
    _nframes: jack_sys::jack_nframes_t,
    pos: *mut jack_sys::jack_position_t,
    _new_pos: ::std::os::raw::c_int,
    arg: *mut c_void,
) {
    let bpm_cell = &*(arg as *const AtomicU32);
    let bpm = f32::from_bits(bpm_cell.load(Ordering::Relaxed)) as f64;

    (*pos).valid = jack_sys::JackPositionBBT as jack_sys::jack_position_bits_t;
    (*pos).beats_per_minute = bpm;
    (*pos).beats_per_bar = 4.0;
    (*pos).beat_type = 4.0;
    (*pos).ticks_per_beat = 1920.0;
}

// ── Internal handle ────────────────────────────────────────────────────────

struct ZrythmHandle {
    /// Active JACK client; transport calls deref through to `jack::Client`.
    client: jack::AsyncClient<(), ZrythmProcess>,
    /// Shared outbound MIDI queue (same `Arc` as inside `ZrythmProcess`).
    pending_midi: Arc<Mutex<VecDeque<[u8; 3]>>>,
    /// Desired BPM, read atomically by the timebase callback.
    /// Allocated with `Box::leak` for a `'static` lifetime.
    bpm_cell: &'static AtomicU32,
}

// ── Public adapter ─────────────────────────────────────────────────────────

pub struct ZrythmEngine {
    handle: Mutex<Option<ZrythmHandle>>,
}

impl ZrythmEngine {
    pub fn new() -> Self {
        Self {
            handle: Mutex::new(None),
        }
    }

    fn with_handle<F, T>(&self, f: F) -> Result<T, String>
    where
        F: FnOnce(&ZrythmHandle) -> T,
    {
        let guard = self.handle.lock().map_err(|e| e.to_string())?;
        match guard.as_ref() {
            Some(h) => Ok(f(h)),
            None => Err(
                "Zrythm JACK connection not established. \
                 Ensure jackd/pipewire-jack is running and MAESTRO_ENGINE=zrythm."
                    .into(),
            ),
        }
    }

    fn queue_midi(&self, msg: [u8; 3]) -> Result<(), String> {
        self.with_handle(|h| h.pending_midi.lock().unwrap().push_back(msg))
    }

    fn check_track(track_index: i32) -> Result<u8, String> {
        if track_index < 0 || track_index >= MAX_MIDI_CH {
            Err(format!(
                "Track index {} out of MIDI channel range (0–{})",
                track_index,
                MAX_MIDI_CH - 1
            ))
        } else {
            Ok(track_index as u8)
        }
    }
}

impl EngineAdapter for ZrythmEngine {
    fn name(&self) -> &'static str {
        "Zrythm"
    }

    fn start(&self, app: AppHandle, _state_manager: StateManager) {
        log::info!("Zrythm: connecting to JACK server...");

        // ── Connect to JACK ────────────────────────────────────────────────
        let (client, _status) =
            match Client::new("maestro", ClientOptions::NO_START_SERVER) {
                Ok(c) => c,
                Err(e) => {
                    log::error!(
                        "Zrythm: failed to connect to JACK: {}. \
                         Ensure jackd or pipewire-jack is running.",
                        e
                    );
                    let _ = app.emit("engine-connection-changed", false);
                    return;
                }
            };

        log::info!(
            "Zrythm: connected to JACK (sample rate: {} Hz, buffer: {} frames)",
            client.sample_rate(),
            client.buffer_size(),
        );

        // ── Register JACK MIDI output port ─────────────────────────────────
        let pending: Arc<Mutex<VecDeque<[u8; 3]>>> = Arc::new(Mutex::new(VecDeque::new()));

        let midi_out = match client.register_port("control_out", MidiOut::default()) {
            Ok(p) => p,
            Err(e) => {
                log::error!("Zrythm: failed to register JACK MIDI port: {}", e);
                let _ = app.emit("engine-connection-changed", false);
                return;
            }
        };

        // ── Activate the JACK client ───────────────────────────────────────
        let active = match client.activate_async((), ZrythmProcess {
            midi_out,
            pending: Arc::clone(&pending),
        }) {
            Ok(a) => a,
            Err(e) => {
                log::error!("Zrythm: failed to activate JACK client: {}", e);
                let _ = app.emit("engine-connection-changed", false);
                return;
            }
        };

        // ── Register as JACK timebase master ──────────────────────────────
        // Leak an AtomicU32 so the callback (which has no Drop path in raw
        // JACK) can read the desired BPM safely for the process lifetime.
        let bpm_cell: &'static AtomicU32 =
            Box::leak(Box::new(AtomicU32::new(120.0_f32.to_bits())));

        // Safety:
        //   • `active.as_client().raw()` is valid while `active` lives (stored in handle).
        //   • `timebase_callback` has the correct `extern "C"` signature.
        //   • `bpm_cell` is `'static` and will never be freed.
        unsafe {
            let rc = jack_sys::jack_set_timebase_callback(
                active.as_client().raw(),
                0, // force (not conditional)
                Some(timebase_callback),
                bpm_cell as *const AtomicU32 as *mut c_void,
            );
            if rc != 0 {
                log::warn!(
                    "Zrythm: could not register as JACK timebase master (rc={}). \
                     Tempo changes will not propagate to Zrythm automatically.",
                    rc
                );
            } else {
                log::info!("Zrythm: registered as JACK timebase master.");
            }
        }

        *self.handle.lock().unwrap() = Some(ZrythmHandle {
            client: active,
            pending_midi: pending,
            bpm_cell,
        });

        log::info!(
            "Zrythm engine ready. \
             Connect 'maestro:control_out' → Zrythm's MIDI input in your patchbay, \
             then use Zrythm's MIDI learn to bind track parameters."
        );
        let _ = app.emit("engine-connection-changed", true);
    }

    // ── Transport ──────────────────────────────────────────────────────────

    fn play(&self) -> Result<(), String> {
        self.with_handle(|h| {
            h.client.as_client().transport().start().map_err(|e| e.to_string())
        })?
    }

    fn stop(&self) -> Result<(), String> {
        self.with_handle(|h| {
            h.client.as_client().transport().stop().map_err(|e| e.to_string())
        })?
    }

    fn toggle_record(&self) -> Result<(), String> {
        // JACK transport has no standardised record-arm message.
        // To support this: bind Zrythm's record button to a MIDI CC via
        // MIDI learn (e.g. CC #117 on channel 0) and send it here.
        Err(
            "Record toggle is not standardised in JACK transport. \
             Bind Zrythm's record button to a MIDI CC via MIDI learn."
                .into(),
        )
    }

    fn set_tempo(&self, bpm: f32) -> Result<(), String> {
        if !(20.0..=999.0).contains(&bpm) {
            return Err(format!("BPM {bpm} out of valid range (20–999)"));
        }
        self.with_handle(|h| h.bpm_cell.store(bpm.to_bits(), Ordering::Relaxed))?;
        log::debug!("Zrythm: desired tempo → {bpm} BPM (pushed via JACK timebase)");
        Ok(())
    }

    fn set_loop_enabled(&self, enabled: bool) -> Result<(), String> {
        // JACK transport has no standardised loop-enable message; Zrythm
        // manages its own loop region independently.
        log::debug!("Zrythm: set_loop_enabled({enabled}) — no JACK equivalent, ignored");
        Ok(())
    }

    // ── Tracks ─────────────────────────────────────────────────────────────

    fn set_track_volume(&self, track_index: i32, volume: f32) -> Result<(), String> {
        let ch = Self::check_track(track_index)?;
        let value = (volume.clamp(0.0, 1.0) * 127.0).round() as u8;
        self.queue_midi([0xB0 | ch, CC_VOLUME, value])
    }

    fn set_track_mute(&self, track_index: i32, mute: bool) -> Result<(), String> {
        let ch = Self::check_track(track_index)?;
        self.queue_midi([0xB0 | ch, CC_MUTE, if mute { 127 } else { 0 }])
    }

    fn set_track_solo(&self, track_index: i32, solo: bool) -> Result<(), String> {
        let ch = Self::check_track(track_index)?;
        self.queue_midi([0xB0 | ch, CC_SOLO, if solo { 127 } else { 0 }])
    }

    fn set_track_pan(&self, track_index: i32, pan: f32) -> Result<(), String> {
        let ch = Self::check_track(track_index)?;
        // Map −1.0..1.0 → 0..127  (centre = 64)
        let value = ((pan.clamp(-1.0, 1.0) + 1.0) / 2.0 * 127.0).round() as u8;
        self.queue_midi([0xB0 | ch, CC_PAN, value])
    }

    // ── Media ───────────────────────────────────────────────────────────────

    fn load_sample(
        &self,
        _track_index: i32,
        _scene_index: i32,
        _file_path: &str,
    ) -> Result<(), String> {
        Err(
            "Sample loading is not supported via JACK MIDI. \
             Drag audio files directly onto Zrythm's timeline."
                .into(),
        )
    }
}
