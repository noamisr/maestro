import { writable, derived } from "svelte/store";
import { listen } from "@tauri-apps/api/event";

export const isPlaying = writable(false);
export const isRecording = writable(false);
export const tempo = writable(120.0);
export const currentTime = writable(0.0);
export const loopEnabled = writable(false);
export const loopStart = writable(0.0);
export const loopLength = writable(4.0);

export const position = derived(
  [currentTime],
  ([$time]) => {
    const beatsPerBar = 4;
    const totalBeats = $time;
    const bar = Math.floor(totalBeats / beatsPerBar) + 1;
    const beat = Math.floor(totalBeats % beatsPerBar) + 1;
    const tick = Math.round((totalBeats % 1) * 100);
    return { bar, beat, tick };
  },
);

listen<{ is_playing: boolean }>("transport-state", (e) => {
  isPlaying.set(e.payload.is_playing);
});

listen<number>("tempo-changed", (e) => {
  tempo.set(e.payload);
});

listen<number>("song-time", (e) => {
  currentTime.set(e.payload);
});
