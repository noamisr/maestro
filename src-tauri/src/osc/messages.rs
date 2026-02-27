use rosc::OscType;

/// Typed OSC message builders for the AbletonOSC protocol.
pub struct OscMessages;

impl OscMessages {
    // ── Transport ──────────────────────────────────────────

    pub fn play() -> (&'static str, Vec<OscType>) {
        ("/live/song/start_playing", vec![])
    }

    pub fn stop() -> (&'static str, Vec<OscType>) {
        ("/live/song/stop_playing", vec![])
    }

    pub fn continue_playing() -> (&'static str, Vec<OscType>) {
        ("/live/song/continue_playing", vec![])
    }

    pub fn toggle_record() -> (&'static str, Vec<OscType>) {
        ("/live/song/set/record_mode", vec![])
    }

    pub fn set_tempo(bpm: f32) -> (&'static str, Vec<OscType>) {
        ("/live/song/set/tempo", vec![OscType::Float(bpm)])
    }

    pub fn get_tempo() -> (&'static str, Vec<OscType>) {
        ("/live/song/get/tempo", vec![])
    }

    pub fn set_loop_enabled(enabled: bool) -> (&'static str, Vec<OscType>) {
        (
            "/live/song/set/loop",
            vec![OscType::Int(if enabled { 1 } else { 0 })],
        )
    }

    pub fn get_is_playing() -> (&'static str, Vec<OscType>) {
        ("/live/song/get/is_playing", vec![])
    }

    pub fn get_song_time() -> (&'static str, Vec<OscType>) {
        ("/live/song/get/current_song_time", vec![])
    }

    // ── Listeners (subscribe to live updates) ──────────────

    pub fn start_listen_tempo() -> (&'static str, Vec<OscType>) {
        ("/live/song/start_listen/tempo", vec![])
    }

    pub fn start_listen_is_playing() -> (&'static str, Vec<OscType>) {
        ("/live/song/start_listen/is_playing", vec![])
    }

    pub fn start_listen_beat() -> (&'static str, Vec<OscType>) {
        ("/live/song/start_listen/beat", vec![])
    }

    // ── Track ──────────────────────────────────────────────

    pub fn get_num_tracks() -> (&'static str, Vec<OscType>) {
        ("/live/song/get/num_tracks", vec![])
    }

    pub fn get_track_data(
        track_min: i32,
        track_max: i32,
    ) -> (&'static str, Vec<OscType>) {
        (
            "/live/song/get/track_data",
            vec![
                OscType::Int(track_min),
                OscType::Int(track_max),
                OscType::String("track.name".into()),
                OscType::String("track.volume".into()),
                OscType::String("track.panning".into()),
                OscType::String("track.mute".into()),
                OscType::String("track.solo".into()),
                OscType::String("track.arm".into()),
                OscType::String("track.color".into()),
            ],
        )
    }

    pub fn set_track_volume(track: i32, vol: f32) -> (&'static str, Vec<OscType>) {
        (
            "/live/track/set/volume",
            vec![OscType::Int(track), OscType::Float(vol)],
        )
    }

    pub fn set_track_mute(track: i32, mute: bool) -> (&'static str, Vec<OscType>) {
        (
            "/live/track/set/mute",
            vec![OscType::Int(track), OscType::Int(if mute { 1 } else { 0 })],
        )
    }

    pub fn set_track_solo(track: i32, solo: bool) -> (&'static str, Vec<OscType>) {
        (
            "/live/track/set/solo",
            vec![OscType::Int(track), OscType::Int(if solo { 1 } else { 0 })],
        )
    }

    pub fn set_track_pan(track: i32, pan: f32) -> (&'static str, Vec<OscType>) {
        (
            "/live/track/set/panning",
            vec![OscType::Int(track), OscType::Float(pan)],
        )
    }

    pub fn start_listen_track_volume(track: i32) -> (&'static str, Vec<OscType>) {
        (
            "/live/track/start_listen/volume",
            vec![OscType::Int(track)],
        )
    }

    pub fn start_listen_track_meter(track: i32) -> (&'static str, Vec<OscType>) {
        (
            "/live/track/start_listen/output_meter_level",
            vec![OscType::Int(track)],
        )
    }

    // ── Clip ───────────────────────────────────────────────

    pub fn fire_clip(track: i32, scene: i32) -> (&'static str, Vec<OscType>) {
        (
            "/live/clip/fire",
            vec![OscType::Int(track), OscType::Int(scene)],
        )
    }

    pub fn stop_clip(track: i32, scene: i32) -> (&'static str, Vec<OscType>) {
        (
            "/live/clip/stop",
            vec![OscType::Int(track), OscType::Int(scene)],
        )
    }

    pub fn delete_clip(track: i32, scene: i32) -> (&'static str, Vec<OscType>) {
        (
            "/live/clip_slot/delete_clip",
            vec![OscType::Int(track), OscType::Int(scene)],
        )
    }

    pub fn duplicate_clip(
        track: i32,
        scene: i32,
        target_track: i32,
        target_scene: i32,
    ) -> (&'static str, Vec<OscType>) {
        (
            "/live/clip_slot/duplicate_clip_to",
            vec![
                OscType::Int(track),
                OscType::Int(scene),
                OscType::Int(target_track),
                OscType::Int(target_scene),
            ],
        )
    }

    /// Custom AbletonOSC extension — requires forked AbletonOSC control script.
    pub fn load_sample(
        track: i32,
        scene: i32,
        file_path: &str,
    ) -> (&'static str, Vec<OscType>) {
        (
            "/live/clip_slot/load_sample",
            vec![
                OscType::Int(track),
                OscType::Int(scene),
                OscType::String(file_path.into()),
            ],
        )
    }

    // ── Test / Ping ────────────────────────────────────────

    pub fn test() -> (&'static str, Vec<OscType>) {
        ("/live/test", vec![])
    }
}
