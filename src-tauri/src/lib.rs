mod commands;
mod osc;
mod sidecar;

use tauri::Manager;

use osc::client::OscClient;
use osc::state::StateManager;
use sidecar::api::SidecarClient;

const OSC_TARGET_PORT: u16 = 11000;
const SIDECAR_PORT: u16 = 9400;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let osc_client = OscClient::new(OSC_TARGET_PORT).expect("Failed to create OSC client");
    let state_manager = StateManager::new();
    let sidecar_client = SidecarClient::new(SIDECAR_PORT);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .setup({
            let state_manager = state_manager.clone();
            move |app| {
                if cfg!(debug_assertions) {
                    app.handle().plugin(
                        tauri_plugin_log::Builder::default()
                            .level(log::LevelFilter::Info)
                            .build(),
                    )?;
                }

                // Start OSC listener (receives from AbletonOSC on port 11001)
                osc::listener::start_listener(app.handle().clone(), state_manager.clone());

                // Send initial test ping to AbletonOSC
                let osc = app.state::<OscClient>();
                let _ = osc.send(
                    osc::messages::OscMessages::test().0,
                    osc::messages::OscMessages::test().1,
                );

                // Start polling for sidecar readiness in background
                let handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    sidecar::manager::start_sidecar(handle).await;
                });

                // Subscribe to Ableton state updates
                let osc2 = app.state::<OscClient>();
                let _ = osc2.send(
                    osc::messages::OscMessages::start_listen_tempo().0,
                    osc::messages::OscMessages::start_listen_tempo().1,
                );
                let _ = osc2.send(
                    osc::messages::OscMessages::start_listen_is_playing().0,
                    osc::messages::OscMessages::start_listen_is_playing().1,
                );

                Ok(())
            }
        })
        .manage(osc_client)
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
