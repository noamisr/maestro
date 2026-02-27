use std::sync::Arc;
use tauri::State;

use crate::engine::EngineAdapter;

#[tauri::command]
pub fn set_track_volume(
    engine: State<'_, Arc<dyn EngineAdapter>>,
    track_index: i32,
    volume: f32,
) -> Result<(), String> {
    engine.set_track_volume(track_index, volume)
}

#[tauri::command]
pub fn set_track_mute(
    engine: State<'_, Arc<dyn EngineAdapter>>,
    track_index: i32,
    mute: bool,
) -> Result<(), String> {
    engine.set_track_mute(track_index, mute)
}

#[tauri::command]
pub fn set_track_solo(
    engine: State<'_, Arc<dyn EngineAdapter>>,
    track_index: i32,
    solo: bool,
) -> Result<(), String> {
    engine.set_track_solo(track_index, solo)
}

#[tauri::command]
pub fn set_track_pan(
    engine: State<'_, Arc<dyn EngineAdapter>>,
    track_index: i32,
    pan: f32,
) -> Result<(), String> {
    engine.set_track_pan(track_index, pan)
}
