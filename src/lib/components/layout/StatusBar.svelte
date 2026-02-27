<script lang="ts">
  import { abletonConnected, sidecarConnected } from "../../stores/connection";
</script>

<div class="status-bar">
  <div class="status-left">
    <span class="status-item">
      <span class="dot" class:connected={$abletonConnected}></span>
      Ableton
    </span>
    <span class="status-item">
      <span class="dot" class:connected={$sidecarConnected}></span>
      Search Engine
    </span>
  </div>
  <div class="status-right">
    <span class="status-text">Maestro v0.1.0</span>
  </div>
</div>

<style>
  .status-bar {
    height: var(--status-bar-height);
    background: var(--bg-primary);
    border-top: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 12px;
    font-size: 11px;
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .status-left {
    display: flex;
    gap: 16px;
  }

  .status-item {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--error);
    flex-shrink: 0;
  }

  .dot.connected {
    background: var(--success);
  }

  .status-right {
    display: flex;
    gap: 12px;
  }

  .status-text {
    color: var(--text-muted);
  }
</style>
