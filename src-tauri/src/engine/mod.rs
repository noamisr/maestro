pub mod mock;
pub mod state;
pub mod zrythm;

pub use state::{ClipState, EngineState, StateManager, TrackState};

use tauri::AppHandle;

/// Which audio engine backend to connect to.
/// Controlled via the `MAESTRO_ENGINE` environment variable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineKind {
    /// Ableton Live via the AbletonOSC control surface (default)
    AbletonOsc,
    /// Zrythm via its OSC/scripting interface (stub — not yet implemented)
    Zrythm,
    /// No-op adapter for offline use and testing
    Mock,
}

impl EngineKind {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "zrythm" => EngineKind::Zrythm,
            "mock" => EngineKind::Mock,
            _ => EngineKind::AbletonOsc,
        }
    }

    pub fn from_env() -> Self {
        std::env::var("MAESTRO_ENGINE")
            .map(|s| Self::from_str(&s))
            .unwrap_or(EngineKind::AbletonOsc)
    }
}

/// Abstraction over different audio engine backends.
///
/// Each implementation translates high-level commands into engine-specific
/// protocol calls (OSC for AbletonOSC, Guile scripting for Zrythm, no-op for Mock).
///
/// Implementations must be `Send + Sync` so they can be shared across Tauri's
/// async command handlers as managed state.
pub trait EngineAdapter: Send + Sync {
    /// Human-readable engine name displayed in the UI status bar.
    fn name(&self) -> &'static str;

    /// Start the engine: launch listeners, subscribe to live state updates.
    ///
    /// Called once during app setup after Tauri has initialized.
    /// Implementations should emit `"engine-connection-changed"` with `true`
    /// once the connection is established.
    fn start(&self, app: AppHandle, state_manager: StateManager);

    // ── Transport ───────────────────────────────────────────────────────────

    fn play(&self) -> Result<(), String>;
    fn stop(&self) -> Result<(), String>;
    fn toggle_record(&self) -> Result<(), String>;
    fn set_tempo(&self, bpm: f32) -> Result<(), String>;

    /// Set the loop region enabled/disabled.
    fn set_loop_enabled(&self, enabled: bool) -> Result<(), String>;

    // ── Tracks ──────────────────────────────────────────────────────────────

    fn set_track_volume(&self, track_index: i32, volume: f32) -> Result<(), String>;
    fn set_track_mute(&self, track_index: i32, mute: bool) -> Result<(), String>;
    fn set_track_solo(&self, track_index: i32, solo: bool) -> Result<(), String>;
    fn set_track_pan(&self, track_index: i32, pan: f32) -> Result<(), String>;

    // ── Media ───────────────────────────────────────────────────────────────

    /// Load an audio file into the given track/scene slot.
    fn load_sample(
        &self,
        track_index: i32,
        scene_index: i32,
        file_path: &str,
    ) -> Result<(), String>;
}
