use tauri::State;

use crate::osc::client::OscClient;
use crate::osc::messages::OscMessages;
use crate::osc::state::StateManager;

#[tauri::command]
pub fn play(osc: State<'_, OscClient>) -> Result<(), String> {
    let (addr, args) = OscMessages::play();
    osc.send(addr, args).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn stop(osc: State<'_, OscClient>) -> Result<(), String> {
    let (addr, args) = OscMessages::stop();
    osc.send(addr, args).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_record(osc: State<'_, OscClient>) -> Result<(), String> {
    let (addr, args) = OscMessages::toggle_record();
    osc.send(addr, args).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_tempo(osc: State<'_, OscClient>, bpm: f32) -> Result<(), String> {
    let (addr, args) = OscMessages::set_tempo(bpm);
    osc.send(addr, args).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_loop(osc: State<'_, OscClient>, state: State<'_, StateManager>) -> Result<(), String> {
    let current = state.get();
    let (addr, args) = OscMessages::set_loop_enabled(!current.loop_enabled);
    osc.send(addr, args).map_err(|e| e.to_string())
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
