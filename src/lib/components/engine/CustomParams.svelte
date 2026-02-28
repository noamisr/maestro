<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { engineConnected } from "../../stores/connection";

  interface ParamDef {
    id: string;
    label: string;
    min: number;
    max: number;
  }

  let params: ParamDef[] = [];
  let values: Record<string, number> = {};

  async function loadParams() {
    try {
      const loaded = await invoke<ParamDef[]>("get_engine_params");
      for (const p of loaded) {
        if (!(p.id in values)) {
          values[p.id] = (p.min + p.max) / 2;
        }
      }
      params = loaded;
    } catch (e) {
      console.error("Failed to load engine params:", e);
    }
  }

  async function handleChange(id: string, raw: string) {
    const value = parseFloat(raw);
    values[id] = value;
    try {
      await invoke("set_engine_param", { id, value });
    } catch (e) {
      console.error("Failed to set engine param:", e);
    }
  }

  $: if ($engineConnected) loadParams();
</script>

{#if params.length > 0}
  <div class="custom-params">
    <div class="section-title">Controls</div>
    {#each params as param (param.id)}
      <div class="param-row">
        <div class="param-header">
          <label for={param.id}>{param.label}</label>
          <span class="value">{(values[param.id] ?? 0).toFixed(2)}</span>
        </div>
        <input
          id={param.id}
          type="range"
          min={param.min}
          max={param.max}
          step={(param.max - param.min) / 127}
          value={values[param.id] ?? (param.min + param.max) / 2}
          on:input={(e) => handleChange(param.id, e.currentTarget.value)}
        />
      </div>
    {/each}
  </div>
{/if}

<style>
  .custom-params {
    padding: 16px 12px;
    display: flex;
    flex-direction: column;
    gap: 14px;
    min-width: 200px;
    border-left: 1px solid var(--border);
    background: var(--bg-primary);
    overflow-y: auto;
  }

  .section-title {
    font-size: 10px;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--text-muted);
    margin-bottom: 2px;
  }

  .param-row {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .param-header {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
  }

  label {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .value {
    font-size: 11px;
    color: var(--text-muted);
  }

  input[type="range"] {
    width: 100%;
    accent-color: var(--accent);
    cursor: pointer;
  }
</style>
