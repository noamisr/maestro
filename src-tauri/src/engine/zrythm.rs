use tauri::AppHandle;

use super::{EngineAdapter, StateManager};

/// Zrythm engine adapter (stub — not yet implemented).
///
/// Zrythm exposes a GNU Guile (Scheme) scripting API for programmatic control.
/// This adapter will translate `EngineAdapter` calls into Guile script invocations
/// once the integration is complete.
///
/// References:
/// - <https://manual.zrythm.org/en/scripting/api/zrythm.html>
/// - <https://github.com/zrythm/zrythm>
pub struct ZrythmEngine;

impl EngineAdapter for ZrythmEngine {
    fn name(&self) -> &'static str {
        "Zrythm"
    }

    fn start(&self, _app: AppHandle, _state_manager: StateManager) {
        log::warn!(
            "Zrythm engine adapter is not yet implemented. \
             Set MAESTRO_ENGINE=ableton_osc or MAESTRO_ENGINE=mock."
        );
    }

    fn play(&self) -> Result<(), String> {
        Err("Zrythm engine adapter not yet implemented".into())
    }

    fn stop(&self) -> Result<(), String> {
        Err("Zrythm engine adapter not yet implemented".into())
    }

    fn toggle_record(&self) -> Result<(), String> {
        Err("Zrythm engine adapter not yet implemented".into())
    }

    fn set_tempo(&self, _bpm: f32) -> Result<(), String> {
        Err("Zrythm engine adapter not yet implemented".into())
    }

    fn set_loop_enabled(&self, _enabled: bool) -> Result<(), String> {
        Err("Zrythm engine adapter not yet implemented".into())
    }

    fn set_track_volume(&self, _track_index: i32, _volume: f32) -> Result<(), String> {
        Err("Zrythm engine adapter not yet implemented".into())
    }

    fn set_track_mute(&self, _track_index: i32, _mute: bool) -> Result<(), String> {
        Err("Zrythm engine adapter not yet implemented".into())
    }

    fn set_track_solo(&self, _track_index: i32, _solo: bool) -> Result<(), String> {
        Err("Zrythm engine adapter not yet implemented".into())
    }

    fn set_track_pan(&self, _track_index: i32, _pan: f32) -> Result<(), String> {
        Err("Zrythm engine adapter not yet implemented".into())
    }

    fn load_sample(
        &self,
        _track_index: i32,
        _scene_index: i32,
        _file_path: &str,
    ) -> Result<(), String> {
        Err("Zrythm engine adapter not yet implemented".into())
    }
}
