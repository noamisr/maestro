mod commands;
mod engine;
mod osc;
mod sidecar;

use std::sync::Arc;

use tauri::Manager;

use engine::{EngineAdapter, EngineKind, StateManager};
use osc::{adapter::AbletonOscEngine, client::OscClient};
use sidecar::api::SidecarClient;

const ABLETON_OSC_TARGET_PORT: u16 = 11000;
const SIDECAR_PORT: u16 = 9400;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let engine_kind = EngineKind::from_env();
    let state_manager = StateManager::new();
    let sidecar_client = SidecarClient::new(SIDECAR_PORT);

    // Build the engine adapter selected by the MAESTRO_ENGINE env var.
    // Defaults to AbletonOsc when the variable is unset.
    let engine: Arc<dyn EngineAdapter> = match engine_kind {
        EngineKind::AbletonOsc => {
            let osc_client = OscClient::new(ABLETON_OSC_TARGET_PORT)
                .expect("Failed to create OSC client");
            Arc::new(AbletonOscEngine::new(osc_client))
        }
        EngineKind::Zrythm => Arc::new(engine::zrythm::ZrythmEngine::new()),
        EngineKind::Mock => Arc::new(engine::mock::MockEngine),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .setup({
            let state_manager = state_manager.clone();
            let engine = Arc::clone(&engine);
            move |app| {
                if cfg!(debug_assertions) {
                    app.handle().plugin(
                        tauri_plugin_log::Builder::default()
                            .level(log::LevelFilter::Info)
                            .build(),
                    )?;
                }

                log::info!("Starting Maestro with engine: {}", engine.name());

                // Start the engine: launch listeners and subscribe to state updates.
                engine.start(app.handle().clone(), state_manager.clone());

                // Start polling for sidecar readiness in background.
                let handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    sidecar::manager::start_sidecar(handle).await;
                });

                Ok(())
            }
        })
        .manage(engine)
        .manage(state_manager)
        .manage(sidecar_client)
        .invoke_handler(tauri::generate_handler![
            // Transport
            commands::transport::play,
            commands::transport::stop,
            commands::transport::toggle_record,
            commands::transport::set_tempo,
            commands::transport::toggle_loop,
            commands::transport::get_transport_state,
            // Tracks
            commands::tracks::set_track_volume,
            commands::tracks::set_track_mute,
            commands::tracks::set_track_solo,
            commands::tracks::set_track_pan,
            // Search
            commands::search::search_by_text,
            commands::search::search_by_similarity,
            commands::search::index_directory,
            commands::search::insert_sample,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Maestro");
}
