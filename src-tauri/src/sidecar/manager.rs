use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Emitter};

use super::api::SidecarClient;

static SIDECAR_READY: AtomicBool = AtomicBool::new(false);

const SIDECAR_PORT: u16 = 9400;

pub fn is_ready() -> bool {
    SIDECAR_READY.load(Ordering::SeqCst)
}

/// Spawn the Python sidecar process and wait until it responds to health checks.
pub async fn start_sidecar(app: AppHandle) {
    let client = SidecarClient::new(SIDECAR_PORT);

    // Poll until the sidecar is ready (it may be started externally in dev mode)
    for attempt in 1..=60 {
        match client.health().await {
            Ok(true) => {
                log::info!("Sidecar is ready (attempt {})", attempt);
                SIDECAR_READY.store(true, Ordering::SeqCst);
                let _ = app.emit("sidecar-connection-changed", true);
                return;
            }
            _ => {
                if attempt == 1 {
                    log::info!(
                        "Waiting for sidecar on port {}... (start it with: cd sidecar && python -m maestro_sidecar.main)",
                        SIDECAR_PORT
                    );
                }
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    }

    log::warn!("Sidecar did not become ready within 60 seconds. Search features will be unavailable.");
    let _ = app.emit("sidecar-connection-changed", false);
}
