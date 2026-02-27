/**
 * Generic engine state types shared across all audio engine backends.
 *
 * These replace the Ableton-specific types in `ableton.ts` and are used
 * throughout the frontend for transport, track, and clip state.
 */

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

export interface EngineFullState {
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
