<script lang="ts">
  import "./app.css";
  import TransportBar from "./lib/components/transport/TransportBar.svelte";
  import TrackList from "./lib/components/tracks/TrackList.svelte";
  import StatusBar from "./lib/components/layout/StatusBar.svelte";
  import { abletonConnected, sidecarConnected } from "./lib/stores/connection";
</script>

<div class="app-shell">
  <TransportBar />

  <div class="main-content">
    <TrackList />
    <div class="center-panel">
      <div class="placeholder">
        <p class="title">Maestro</p>
        <p class="subtitle">
          {#if !$abletonConnected}
            Waiting for Ableton Live connection...
          {:else if !$sidecarConnected}
            Connecting to search engine...
          {:else}
            Connected — ready to create
          {/if}
        </p>
      </div>
    </div>
  </div>

  <StatusBar />
</div>

<style>
  .app-shell {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .main-content {
    flex: 1;
    display: flex;
    overflow: hidden;
  }

  .center-panel {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-secondary);
  }

  .placeholder {
    text-align: center;
  }

  .placeholder .title {
    font-size: 32px;
    font-weight: 700;
    color: var(--accent);
    margin-bottom: 8px;
  }

  .placeholder .subtitle {
    font-size: 14px;
    color: var(--text-secondary);
  }
</style>
