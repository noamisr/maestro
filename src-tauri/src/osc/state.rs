use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct AbletonState {
    pub is_playing: bool,
    pub tempo: f64,
    pub current_time: f64,
    pub loop_enabled: bool,
    pub loop_start: f64,
    pub loop_length: f64,
    pub num_tracks: usize,
    pub num_scenes: usize,
    pub tracks: Vec<TrackState>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct TrackState {
    pub index: usize,
    pub name: String,
    pub volume: f64,
    pub panning: f64,
    pub mute: bool,
    pub solo: bool,
    pub arm: bool,
    pub color: u32,
    pub meter_level: f64,
    pub clips: Vec<ClipState>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ClipState {
    pub track_index: usize,
    pub scene_index: usize,
    pub name: String,
    pub color: u32,
    pub length: f64,
    pub is_playing: bool,
    pub is_triggered: bool,
}

#[derive(Clone)]
pub struct StateManager {
    state: Arc<RwLock<AbletonState>>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(AbletonState::default())),
        }
    }

    pub fn get(&self) -> AbletonState {
        self.state.read().unwrap().clone()
    }

    pub fn set_playing(&self, playing: bool) {
        self.state.write().unwrap().is_playing = playing;
    }

    pub fn set_tempo(&self, tempo: f64) {
        self.state.write().unwrap().tempo = tempo;
    }

    pub fn set_current_time(&self, time: f64) {
        self.state.write().unwrap().current_time = time;
    }

    pub fn set_tracks(&self, tracks: Vec<TrackState>) {
        let mut state = self.state.write().unwrap();
        state.num_tracks = tracks.len();
        state.tracks = tracks;
    }

    pub fn set_track_volume(&self, index: usize, volume: f64) {
        let mut state = self.state.write().unwrap();
        if let Some(track) = state.tracks.get_mut(index) {
            track.volume = volume;
        }
    }

    pub fn set_track_mute(&self, index: usize, mute: bool) {
        let mut state = self.state.write().unwrap();
        if let Some(track) = state.tracks.get_mut(index) {
            track.mute = mute;
        }
    }

    pub fn set_track_solo(&self, index: usize, solo: bool) {
        let mut state = self.state.write().unwrap();
        if let Some(track) = state.tracks.get_mut(index) {
            track.solo = solo;
        }
    }

    pub fn set_track_meter(&self, index: usize, level: f64) {
        let mut state = self.state.write().unwrap();
        if let Some(track) = state.tracks.get_mut(index) {
            track.meter_level = level;
        }
    }
}
