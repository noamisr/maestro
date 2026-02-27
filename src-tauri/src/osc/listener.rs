use rosc::{OscPacket, OscType};
use std::net::UdpSocket;
use tauri::{AppHandle, Emitter};

use super::state::StateManager;

pub fn start_listener(app: AppHandle, state_manager: StateManager) {
    std::thread::spawn(move || {
        let socket = match UdpSocket::bind("0.0.0.0:11001") {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to bind OSC listener on port 11001: {}", e);
                return;
            }
        };

        log::info!("OSC listener started on port 11001");
        let mut buf = [0u8; 65535];

        loop {
            match socket.recv_from(&mut buf) {
                Ok((size, _addr)) => {
                    if let Ok((_remaining, packet)) = rosc::decoder::decode_udp(&buf[..size]) {
                        handle_packet(&app, &state_manager, &packet);
                    }
                }
                Err(e) => {
                    log::error!("OSC recv error: {}", e);
                }
            }
        }
    });
}

fn handle_packet(app: &AppHandle, state: &StateManager, packet: &OscPacket) {
    match packet {
        OscPacket::Message(msg) => {
            handle_message(app, state, &msg.addr, &msg.args);
        }
        OscPacket::Bundle(bundle) => {
            for p in &bundle.content {
                handle_packet(app, state, p);
            }
        }
    }
}

fn handle_message(app: &AppHandle, state: &StateManager, addr: &str, args: &[OscType]) {
    match addr {
        "/live/song/get/is_playing" => {
            if let Some(OscType::Int(val)) = args.first() {
                let playing = *val != 0;
                state.set_playing(playing);
                let _ = app.emit(
                    "transport-state",
                    serde_json::json!({ "is_playing": playing }),
                );
            }
        }
        "/live/song/get/tempo" => {
            if let Some(val) = args.first() {
                let tempo = match val {
                    OscType::Float(f) => *f as f64,
                    OscType::Double(d) => *d,
                    _ => return,
                };
                state.set_tempo(tempo);
                let _ = app.emit("tempo-changed", tempo);
            }
        }
        "/live/song/get/current_song_time" => {
            if let Some(val) = args.first() {
                let time = match val {
                    OscType::Float(f) => *f as f64,
                    OscType::Double(d) => *d,
                    _ => return,
                };
                state.set_current_time(time);
                let _ = app.emit("song-time", time);
            }
        }
        "/live/test" => {
            log::info!("AbletonOSC test response received");
            let _ = app.emit("ableton-connection-changed", true);
        }
        _ => {
            log::trace!("Unhandled OSC: {} {:?}", addr, args);
        }
    }
}
