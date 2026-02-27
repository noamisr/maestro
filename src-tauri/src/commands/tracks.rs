use tauri::State;

use crate::osc::client::OscClient;
use crate::osc::messages::OscMessages;

#[tauri::command]
pub fn set_track_volume(
    osc: State<'_, OscClient>,
    track_index: i32,
    volume: f32,
) -> Result<(), String> {
    let (addr, args) = OscMessages::set_track_volume(track_index, volume);
    osc.send(addr, args).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_track_mute(
    osc: State<'_, OscClient>,
    track_index: i32,
    mute: bool,
) -> Result<(), String> {
    let (addr, args) = OscMessages::set_track_mute(track_index, mute);
    osc.send(addr, args).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_track_solo(
    osc: State<'_, OscClient>,
    track_index: i32,
    solo: bool,
) -> Result<(), String> {
    let (addr, args) = OscMessages::set_track_solo(track_index, solo);
    osc.send(addr, args).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_track_pan(
    osc: State<'_, OscClient>,
    track_index: i32,
    pan: f32,
) -> Result<(), String> {
    let (addr, args) = OscMessages::set_track_pan(track_index, pan);
    osc.send(addr, args).map_err(|e| e.to_string())
}
