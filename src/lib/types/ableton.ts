export interface TransportState {
  isPlaying: boolean;
  tempo: number;
  currentTime: number;
  loopEnabled: boolean;
  loopStart: number;
  loopLength: number;
}

export interface TrackState {
  index: number;
  name: string;
  volume: number;
  panning: number;
  mute: boolean;
  solo: boolean;
  arm: boolean;
  color: number;
  meterLevel: number;
  clips: ClipState[];
}

export interface ClipState {
  trackIndex: number;
  sceneIndex: number;
  name: string;
  color: number;
  length: number;
  isPlaying: boolean;
  isTriggered: boolean;
}

export interface AbletonFullState {
  isPlaying: boolean;
  tempo: number;
  currentTime: number;
  loopEnabled: boolean;
  loopStart: number;
  loopLength: number;
  numTracks: number;
  numScenes: number;
  tracks: TrackState[];
}
