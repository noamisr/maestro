<script lang="ts">
  import type { TrackState } from "../../types/engine";
  import * as api from "../../api/tracks";

  let { track, selected = false, onselect }: {
    track: TrackState;
    selected: boolean;
    onselect: () => void;
  } = $props();

  function colorToHex(color: number): string {
    return `#${(color & 0xffffff).toString(16).padStart(6, "0")}`;
  }
</script>

<div
  class="track-row"
  class:selected
  onclick={onselect}
  role="button"
  tabindex="0"
  onkeydown={(e) => e.key === "Enter" && onselect()}
>
  <div class="color-bar" style:background={colorToHex(track.color)}></div>

  <div class="track-info">
    <span class="track-name">{track.name}</span>
  </div>

  <div class="track-controls">
    <button
      class="ctrl-btn"
      class:active={track.mute}
      onclick={(e: MouseEvent) => { e.stopPropagation(); api.setTrackMute(track.index, !track.mute); }}
      title="Mute"
    >M</button>

    <button
      class="ctrl-btn solo"
      class:active={track.solo}
      onclick={(e: MouseEvent) => { e.stopPropagation(); api.setTrackSolo(track.index, !track.solo); }}
      title="Solo"
    >S</button>
  </div>

  <div class="volume-display">
    <div class="volume-bar">
      <div class="volume-fill" style:width="{track.volume * 100}%"></div>
    </div>
  </div>
</div>

<style>
  .track-row {
    height: var(--track-height);
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 8px;
    border-bottom: 1px solid var(--border);
    cursor: pointer;
    transition: background 0.1s;
  }

  .track-row:hover {
    background: var(--bg-secondary);
  }

  .track-row.selected {
    background: var(--bg-surface);
  }

  .color-bar {
    width: 4px;
    height: 24px;
    border-radius: 2px;
    flex-shrink: 0;
  }

  .track-info {
    flex: 1;
    min-width: 0;
  }

  .track-name {
    font-size: 12px;
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    display: block;
  }

  .track-controls {
    display: flex;
    gap: 2px;
    flex-shrink: 0;
  }

  .ctrl-btn {
    width: 22px;
    height: 22px;
    font-size: 10px;
    font-weight: 700;
    border-radius: 3px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    transition: all 0.1s;
  }

  .ctrl-btn:hover {
    background: var(--bg-elevated);
    color: var(--text-primary);
  }

  .ctrl-btn.active {
    background: var(--accent);
    color: white;
  }

  .ctrl-btn.solo.active {
    background: var(--warning);
    color: var(--bg-primary);
  }

  .volume-display {
    width: 48px;
    flex-shrink: 0;
  }

  .volume-bar {
    height: 4px;
    background: var(--border);
    border-radius: 2px;
    overflow: hidden;
  }

  .volume-fill {
    height: 100%;
    background: var(--success);
    border-radius: 2px;
    transition: width 0.1s;
  }
</style>
