import { writable } from "svelte/store";
import { listen } from "@tauri-apps/api/event";
import type { TrackState } from "../types/ableton";

export const tracks = writable<TrackState[]>([]);
export const selectedTrackIndex = writable<number | null>(null);

listen<TrackState[]>("tracks-updated", (event) => {
  tracks.set(event.payload);
});
