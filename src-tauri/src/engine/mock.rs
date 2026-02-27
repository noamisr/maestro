use tauri::{AppHandle, Emitter};

use super::{EngineAdapter, StateManager};

/// No-op engine adapter for offline use and automated testing.
///
/// All command methods succeed immediately without contacting any external process.
/// Emits `engine-connection-changed: true` at startup so the UI shows as connected.
pub struct MockEngine;

impl EngineAdapter for MockEngine {
    fn name(&self) -> &'static str {
        "Mock"
    }

    fn start(&self, app: AppHandle, _state_manager: StateManager) {
        log::info!("Mock engine started — all commands are no-ops");
        let _ = app.emit("engine-connection-changed", true);
    }

    fn play(&self) -> Result<(), String> {
        log::debug!("Mock: play");
        Ok(())
    }

    fn stop(&self) -> Result<(), String> {
        log::debug!("Mock: stop");
        Ok(())
    }

    fn toggle_record(&self) -> Result<(), String> {
        log::debug!("Mock: toggle_record");
        Ok(())
    }

    fn set_tempo(&self, bpm: f32) -> Result<(), String> {
        log::debug!("Mock: set_tempo({})", bpm);
        Ok(())
    }

    fn set_loop_enabled(&self, enabled: bool) -> Result<(), String> {
        log::debug!("Mock: set_loop_enabled({})", enabled);
        Ok(())
    }

    fn set_track_volume(&self, track_index: i32, volume: f32) -> Result<(), String> {
        log::debug!("Mock: set_track_volume({}, {})", track_index, volume);
        Ok(())
    }

    fn set_track_mute(&self, track_index: i32, mute: bool) -> Result<(), String> {
        log::debug!("Mock: set_track_mute({}, {})", track_index, mute);
        Ok(())
    }

    fn set_track_solo(&self, track_index: i32, solo: bool) -> Result<(), String> {
        log::debug!("Mock: set_track_solo({}, {})", track_index, solo);
        Ok(())
    }

    fn set_track_pan(&self, track_index: i32, pan: f32) -> Result<(), String> {
        log::debug!("Mock: set_track_pan({}, {})", track_index, pan);
        Ok(())
    }

    fn load_sample(
        &self,
        track_index: i32,
        scene_index: i32,
        file_path: &str,
    ) -> Result<(), String> {
        log::debug!(
            "Mock: load_sample({}, {}, {})",
            track_index,
            scene_index,
            file_path
        );
        Ok(())
    }
}
