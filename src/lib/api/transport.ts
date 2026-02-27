import { invoke } from "@tauri-apps/api/core";

export async function play() {
  return invoke("play");
}

export async function stop() {
  return invoke("stop");
}

export async function toggleRecord() {
  return invoke("toggle_record");
}

export async function setTempo(bpm: number) {
  return invoke("set_tempo", { bpm });
}

export async function toggleLoop() {
  return invoke("toggle_loop");
}
