import { writable } from "svelte/store";
import { listen } from "@tauri-apps/api/event";

/** True once the active audio engine reports a successful connection. */
export const engineConnected = writable(false);
export const sidecarConnected = writable(false);

listen<boolean>("engine-connection-changed", (event) => {
  engineConnected.set(event.payload);
});

listen<boolean>("sidecar-connection-changed", (event) => {
  sidecarConnected.set(event.payload);
});
