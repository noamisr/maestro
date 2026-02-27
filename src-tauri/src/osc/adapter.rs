use tauri::AppHandle;

use crate::engine::{EngineAdapter, StateManager};
use crate::osc::{client::OscClient, listener, messages::OscMessages};

/// Ableton Live engine adapter using the AbletonOSC control surface.
///
/// Communicates with Ableton Live via the AbletonOSC MIDI Remote Script:
/// - **Send** commands on UDP port 11000
/// - **Receive** state updates on UDP port 11001
///
/// References:
/// - <https://github.com/ideoforms/AbletonOSC>
pub struct AbletonOscEngine {
    client: OscClient,
}

impl AbletonOscEngine {
    pub fn new(client: OscClient) -> Self {
        Self { client }
    }
}

impl EngineAdapter for AbletonOscEngine {
    fn name(&self) -> &'static str {
        "AbletonOSC"
    }

    fn start(&self, app: AppHandle, state_manager: StateManager) {
        // Start the OSC listener thread (receives from AbletonOSC on port 11001)
        listener::start_listener(app.clone(), state_manager);

        // Send a test ping — AbletonOSC responds with /live/test confirming connection
        let (addr, args) = OscMessages::test();
        if let Err(e) = self.client.send(addr, args) {
            log::warn!("Failed to send OSC test ping: {}", e);
        }

        // Subscribe to live state updates from Ableton
        let subscriptions = [
            OscMessages::start_listen_tempo(),
            OscMessages::start_listen_is_playing(),
        ];
        for (addr, args) in subscriptions {
            if let Err(e) = self.client.send(addr, args) {
                log::warn!("Failed to subscribe to Ableton update '{}': {}", addr, e);
            }
        }
    }

    // ── Transport ───────────────────────────────────────────────────────────

    fn play(&self) -> Result<(), String> {
        let (addr, args) = OscMessages::play();
        self.client.send(addr, args).map_err(|e| e.to_string())
    }

    fn stop(&self) -> Result<(), String> {
        let (addr, args) = OscMessages::stop();
        self.client.send(addr, args).map_err(|e| e.to_string())
    }

    fn toggle_record(&self) -> Result<(), String> {
        let (addr, args) = OscMessages::toggle_record();
        self.client.send(addr, args).map_err(|e| e.to_string())
    }

    fn set_tempo(&self, bpm: f32) -> Result<(), String> {
        let (addr, args) = OscMessages::set_tempo(bpm);
        self.client.send(addr, args).map_err(|e| e.to_string())
    }

    fn set_loop_enabled(&self, enabled: bool) -> Result<(), String> {
        let (addr, args) = OscMessages::set_loop_enabled(enabled);
        self.client.send(addr, args).map_err(|e| e.to_string())
    }

    // ── Tracks ──────────────────────────────────────────────────────────────

    fn set_track_volume(&self, track_index: i32, volume: f32) -> Result<(), String> {
        let (addr, args) = OscMessages::set_track_volume(track_index, volume);
        self.client.send(addr, args).map_err(|e| e.to_string())
    }

    fn set_track_mute(&self, track_index: i32, mute: bool) -> Result<(), String> {
        let (addr, args) = OscMessages::set_track_mute(track_index, mute);
        self.client.send(addr, args).map_err(|e| e.to_string())
    }

    fn set_track_solo(&self, track_index: i32, solo: bool) -> Result<(), String> {
        let (addr, args) = OscMessages::set_track_solo(track_index, solo);
        self.client.send(addr, args).map_err(|e| e.to_string())
    }

    fn set_track_pan(&self, track_index: i32, pan: f32) -> Result<(), String> {
        let (addr, args) = OscMessages::set_track_pan(track_index, pan);
        self.client.send(addr, args).map_err(|e| e.to_string())
    }

    // ── Media ───────────────────────────────────────────────────────────────

    fn load_sample(
        &self,
        track_index: i32,
        scene_index: i32,
        file_path: &str,
    ) -> Result<(), String> {
        let (addr, args) = OscMessages::load_sample(track_index, scene_index, file_path);
        self.client.send(addr, args).map_err(|e| e.to_string())
    }
}
