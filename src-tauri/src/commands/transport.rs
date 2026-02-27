use std::sync::Arc;
use tauri::State;

use crate::engine::{EngineAdapter, StateManager};

#[tauri::command]
pub fn play(engine: State<'_, Arc<dyn EngineAdapter>>) -> Result<(), String> {
    engine.play()
}

#[tauri::command]
pub fn stop(engine: State<'_, Arc<dyn EngineAdapter>>) -> Result<(), String> {
    engine.stop()
}

#[tauri::command]
pub fn toggle_record(engine: State<'_, Arc<dyn EngineAdapter>>) -> Result<(), String> {
    engine.toggle_record()
}

#[tauri::command]
pub fn set_tempo(engine: State<'_, Arc<dyn EngineAdapter>>, bpm: f32) -> Result<(), String> {
    engine.set_tempo(bpm)
}

#[tauri::command]
pub fn toggle_loop(
    engine: State<'_, Arc<dyn EngineAdapter>>,
    state: State<'_, StateManager>,
) -> Result<(), String> {
    let current = state.get();
    engine.set_loop_enabled(!current.loop_enabled)
}

#[tauri::command]
pub fn get_transport_state(state: State<'_, StateManager>) -> Result<serde_json::Value, String> {
    let s = state.get();
    Ok(serde_json::json!({
        "isPlaying": s.is_playing,
        "tempo": s.tempo,
        "currentTime": s.current_time,
        "loopEnabled": s.loop_enabled,
        "loopStart": s.loop_start,
        "loopLength": s.loop_length,
    }))
}
