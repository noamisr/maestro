<script lang="ts">
  import { isPlaying, isRecording, tempo, position, loopEnabled } from "../../stores/transport";
  import * as api from "../../api/transport";

  let tempoInput = $state("");
  let editingTempo = $state(false);

  function handleTempoFocus() {
    editingTempo = true;
    tempoInput = $tempo.toFixed(1);
  }

  function handleTempoBlur() {
    editingTempo = false;
    const val = parseFloat(tempoInput);
    if (!isNaN(val) && val >= 20 && val <= 999) {
      api.setTempo(val);
    }
  }

  function handleTempoKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      (e.target as HTMLInputElement).blur();
    }
  }

  function formatPosition(pos: { bar: number; beat: number; tick: number }) {
    return `${String(pos.bar).padStart(3, " ")}.${pos.beat}.${String(pos.tick).padStart(2, "0")}`;
  }
</script>

<div class="transport-bar">
  <div class="transport-controls">
    <button
      class="transport-btn"
      class:active={$isPlaying}
      onclick={() => ($isPlaying ? api.stop() : api.play())}
      title={$isPlaying ? "Stop" : "Play"}
    >
      {#if $isPlaying}
        <span class="icon stop">&#9632;</span>
      {:else}
        <span class="icon play">&#9654;</span>
      {/if}
    </button>

    <button
      class="transport-btn"
      class:active={$isRecording}
      onclick={() => api.toggleRecord()}
      title="Record"
    >
      <span class="icon record">&#9679;</span>
    </button>

    <button
      class="transport-btn"
      class:active={$loopEnabled}
      onclick={() => api.toggleLoop()}
      title="Loop"
    >
      <span class="icon loop">&#8634;</span>
    </button>
  </div>

  <div class="position-display">
    <span class="position-value">{formatPosition($position)}</span>
  </div>

  <div class="tempo-section">
    <span class="tempo-label">BPM</span>
    {#if editingTempo}
      <input
        class="tempo-input"
        type="text"
        bind:value={tempoInput}
        onblur={handleTempoBlur}
        onkeydown={handleTempoKeydown}
      />
    {:else}
      <button class="tempo-display" onfocus={handleTempoFocus} onclick={handleTempoFocus}>
        {$tempo.toFixed(1)}
      </button>
    {/if}
  </div>
</div>

<style>
  .transport-bar {
    height: var(--transport-height);
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    padding: 0 16px;
    gap: 24px;
    flex-shrink: 0;
  }

  .transport-controls {
    display: flex;
    gap: 4px;
  }

  .transport-btn {
    width: 36px;
    height: 36px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.15s;
  }

  .transport-btn:hover {
    background: var(--bg-elevated);
  }

  .transport-btn.active {
    background: var(--bg-surface);
  }

  .icon {
    font-size: 16px;
  }

  .icon.play {
    color: var(--success);
  }

  .icon.stop {
    color: var(--text-primary);
  }

  .icon.record {
    color: var(--accent);
  }

  .icon.loop {
    font-size: 18px;
  }

  .transport-btn.active .icon.loop {
    color: var(--accent);
  }

  .position-display {
    font-family: "JetBrains Mono", "SF Mono", "Fira Code", monospace;
    font-size: 20px;
    font-weight: 600;
    letter-spacing: 1px;
    color: var(--text-primary);
    min-width: 120px;
    text-align: center;
  }

  .tempo-section {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .tempo-label {
    font-size: 11px;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .tempo-display {
    font-family: "JetBrains Mono", "SF Mono", monospace;
    font-size: 14px;
    color: var(--text-primary);
    padding: 4px 8px;
    border-radius: 4px;
    min-width: 60px;
    text-align: center;
  }

  .tempo-display:hover {
    background: var(--bg-elevated);
  }

  .tempo-input {
    width: 60px;
    font-family: "JetBrains Mono", "SF Mono", monospace;
    font-size: 14px;
    text-align: center;
    padding: 4px 8px;
  }
</style>
