# Maestro

A lightweight, keyboard-driven music editing interface built on top of [Zrythm](https://www.zrythm.org/) (or Ableton Live), with AI-powered sample search.

```
┌──────────────────────────────────────────────────────────┐
│                       TransportBar                        │
├────────────┬─────────────────────────────┬───────────────┤
│            │                             │               │
│ TrackList  │       center panel          │ Controls      │
│            │                             │ (custom MIDI  │
│            │                             │  sliders)     │
│            │                             │               │
├────────────┴─────────────────────────────┴───────────────┤
│                        StatusBar                          │
└──────────────────────────────────────────────────────────┘
```

## Requirements

### All engines
| Tool | Version | Install |
|------|---------|---------|
| Rust | ≥ 1.77.2 | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Node.js | ≥ 20 | https://nodejs.org |
| Tauri CLI | latest | `cargo install tauri-cli` |

### Zrythm engine (Linux default)
| Dependency | Install |
|-----------|---------|
| JACK Audio | `sudo apt install jackd2` **or** `sudo apt install pipewire-jack` |
| Zrythm ≥ 1.0 | https://www.zrythm.org/en/download.html |
| (optional) patchbay | `sudo apt install qjackctl` |

### Ableton engine (macOS/Windows)
| Dependency | Install |
|-----------|---------|
| Ableton Live 10+ | https://www.ableton.com |
| AbletonOSC | https://github.com/ideoforms/AbletonOSC |

## Installation

```bash
git clone https://github.com/your-org/maestro && cd maestro
npm install
```

## Running

Select an engine with the `MAESTRO_ENGINE` environment variable:

```bash
# Zrythm via JACK (Linux — requires JACK server + Zrythm)
MAESTRO_ENGINE=zrythm npm run tauri dev

# Ableton Live via AbletonOSC (macOS/Windows)
MAESTRO_ENGINE=ableton npm run tauri dev

# No-op mock — offline UI development, no DAW needed
MAESTRO_ENGINE=mock npm run tauri dev
```

`MAESTRO_ENGINE` defaults to `ableton` when unset.

---

## Zrythm setup

### 1. Start a JACK server

**PipeWire-JACK** (recommended on Ubuntu 22.04+):
```bash
# PipeWire automatically provides a JACK server — nothing extra needed.
# Verify it is running:
jack_lsp
```

**Classic jackd**:
```bash
jackd -d alsa -r 48000 -p 256 &
```

### 2. Launch Zrythm and Maestro

```bash
zrythm &
MAESTRO_ENGINE=zrythm npm run tauri dev
```

### 3. Connect MIDI ports

In your JACK patchbay (`qjackctl`, Carla, Catia, or command line):

```bash
jack_connect maestro:control_out Zrythm:MIDI_Input
```

Or open `qjackctl → Graph` and drag a cable from `maestro:control_out` to `Zrythm:MIDI_Input`.

### 4. MIDI-learn track parameters in Zrythm

In Zrythm, right-click any fader, pan knob, or button → **"MIDI Learn"**, then move the corresponding control in Maestro to bind it.

Default CC assignments:

| Parameter | CC  | MIDI Channel |
|-----------|-----|--------------|
| Volume    | 7   | track index  |
| Pan       | 10  | track index  |
| Mute      | 119 | track index  |
| Solo      | 118 | track index  |

---

## Custom MIDI controls

Expose any Zrythm parameter as a labeled slider in the **Controls** panel by
creating `~/.config/maestro/zrythm-map.toml`:

```toml
[[params]]
id      = "reverb_wet"
label   = "Reverb Wet"
cc      = 20        # CC number you MIDI-learned in Zrythm
channel = 0
min     = 0.0       # optional — slider minimum (default 0)
max     = 1.0       # optional — slider maximum (default 1)

[[params]]
id      = "limiter_threshold"
label   = "Limiter Threshold"
cc      = 21
channel = 0
min     = -20.0
max     = 0.0
```

Each slider maps its `[min, max]` range to MIDI CC values `[0, 127]`.
To use a different file, set `MAESTRO_MIDI_MAP=/path/to/map.toml`.

Sliders appear automatically after the next launch — no code changes needed.

---

## Ableton setup

1. Copy the `AbletonOSC` control surface to Ableton's MIDI Remote Scripts folder
   (see [AbletonOSC README](https://github.com/ideoforms/AbletonOSC#installation)).
2. In Ableton → **Preferences → Link/Tempo/MIDI**, enable the AbletonOSC surface.
   It listens on port **11000** by default.
3. Run:
   ```bash
   MAESTRO_ENGINE=ableton npm run tauri dev
   ```

---

## Development

```bash
# Install frontend deps
npm install

# Type-check frontend
npm run check

# Unit tests (no JACK/Zrythm/GUI needed)
cd src-tauri && cargo test

# Lint
cd src-tauri && cargo clippy

# Production build
npm run tauri build
```

### End-to-end test (requires JACK + Zrythm)

```bash
./scripts/e2e-zrythm.sh
```

The script verifies:
1. JACK server is reachable
2. `maestro:control_out` JACK MIDI port registers on startup
3. TOML param config is parsed correctly
4. MIDI CC values are emitted on the port
5. JACK transport state is queryable (Zrythm play/stop)

---

## Architecture

```
MAESTRO_ENGINE env var
        │
        ▼
┌───────────────────┐     IPC / events      ┌────────────────────────┐
│  Maestro UI        │ ◄────────────────────► │  Maestro Core (Tauri)  │
│  (Svelte + TS)     │                        │  EngineAdapter trait   │
└───────────────────┘                        └────────┬───────────────┘
                                                      │
                         ┌────────────────────────────┼──────────────────┐
                         │                            │                  │
                         ▼                            ▼                  ▼
               ┌─────────────────┐        ┌──────────────────┐  ┌──────────────┐
               │  ZrythmEngine   │        │  AbletonOscEngine│  │  MockEngine  │
               │                 │        │                  │  │              │
               │ JACK Transport  │        │  OSC (port 11000)│  │  no-op       │
               │ JACK MIDI CC    │        │                  │  │              │
               └────────┬────────┘        └──────────────────┘  └──────────────┘
                        │
               ┌────────┴────────┐
               │  Zrythm (DAW)   │
               │  JACK MIDI In   │
               └─────────────────┘
```

| Layer | Technology |
|-------|-----------|
| Frontend | Svelte 5, TypeScript, Tauri IPC |
| Backend | Rust, Tauri 2, `async-trait` |
| Zrythm bridge | `jack` crate — JACK Transport + JACK MIDI CC |
| Ableton bridge | `rosc` crate — AbletonOSC |
| Sample search | HTTP sidecar — vector DB (Qdrant/Milvus) |
