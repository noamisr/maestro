<script lang="ts">
  import { tracks, selectedTrackIndex } from "../../stores/tracks";
  import TrackRow from "./TrackRow.svelte";
</script>

<div class="track-list">
  <div class="track-list-header">
    <span>Tracks</span>
  </div>
  <div class="track-list-body">
    {#each $tracks as track (track.index)}
      <TrackRow
        {track}
        selected={$selectedTrackIndex === track.index}
        onselect={() => selectedTrackIndex.set(track.index)}
      />
    {:else}
      <div class="empty-state">No tracks</div>
    {/each}
  </div>
</div>

<style>
  .track-list {
    width: 240px;
    min-width: 200px;
    background: var(--bg-primary);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
  }

  .track-list-header {
    height: 32px;
    padding: 0 12px;
    display: flex;
    align-items: center;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--text-muted);
    border-bottom: 1px solid var(--border);
  }

  .track-list-body {
    flex: 1;
    overflow-y: auto;
  }

  .empty-state {
    padding: 24px 12px;
    text-align: center;
    color: var(--text-muted);
    font-size: 12px;
  }
</style>
