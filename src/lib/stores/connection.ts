import { writable } from "svelte/store";
import { listen } from "@tauri-apps/api/event";

export const abletonConnected = writable(false);
export const sidecarConnected = writable(false);

listen<boolean>("ableton-connection-changed", (event) => {
  abletonConnected.set(event.payload);
});

listen<boolean>("sidecar-connection-changed", (event) => {
  sidecarConnected.set(event.payload);
});
