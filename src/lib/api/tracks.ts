import { invoke } from "@tauri-apps/api/core";

export async function setTrackVolume(trackIndex: number, volume: number) {
  return invoke("set_track_volume", { trackIndex, volume });
}

export async function setTrackMute(trackIndex: number, mute: boolean) {
  return invoke("set_track_mute", { trackIndex, mute });
}

export async function setTrackSolo(trackIndex: number, solo: boolean) {
  return invoke("set_track_solo", { trackIndex, solo });
}

export async function setTrackPan(trackIndex: number, pan: number) {
  return invoke("set_track_pan", { trackIndex, pan });
}
