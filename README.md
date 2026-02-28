# Maestro

A lightweight, keyboard-driven music editing interface backed by [Ableton Live](https://www.ableton.com) via AbletonOSC, with AI-powered sample search.

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

### Ableton engine (default)
| Dependency | Install |
|-----------|---------|
| Ableton Live 10+ | https://www.ableton.com |
| AbletonOSC | https://github.com/ideoforms/AbletonOSC |

### Zrythm engine (opt-in, Linux only)
| Dependency | Install |
|-----------|---------|
| JACK Audio | `sudo apt install jackd2` **or** `sudo apt install pipewire-jack` |
| Zrythm ≥ 1.0 | https://www.zrythm.org/en/download.html |
| (optional) patchbay | `sudo apt install qjackctl` |

## Installation

```bash
git clone https://github.com/your-org/maestro && cd maestro
npm install
```

## Running

`MAESTRO_ENGINE` defaults to `ableton` when unset:

```bash
# Ableton Live via AbletonOSC (default)
npm run tauri dev

# Zrythm via JACK (opt-in, Linux)
MAESTRO_ENGINE=zrythm npm run tauri dev

# No-op mock — offline UI development, no DAW needed
MAESTRO_ENGINE=mock npm run tauri dev
```

---

## Ableton setup

1. Copy the `AbletonOSC` control surface to Ableton's MIDI Remote Scripts folder
   (see [AbletonOSC README](https://github.com/ideoforms/AbletonOSC#installation)).
2. In Ableton → **Preferences → Link/Tempo/MIDI**, enable the AbletonOSC surface.
   It listens on port **11000** by default.
3. Launch Maestro:
   ```bash
   npm run tauri dev
   ```

---

## Zrythm setup (opt-in)

### 1. Start a JACK server

**PipeWire-JACK** (recommended on Ubuntu 22.04+):
```bash
# PipeWire automatically provides a JACK server — nothing extra needed.
jack_lsp   # verify it is running
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

```bash
jack_connect maestro:control_out Zrythm:MIDI_Input
```

Or use `qjackctl → Graph` to drag a cable from `maestro:control_out` to `Zrythm:MIDI_Input`.

### 4. MIDI-learn track parameters

In Zrythm, right-click any fader, pan knob, or button → **"MIDI Learn"**, then move the corresponding control in Maestro to bind it.

Default CC assignments:

| Parameter | CC  | MIDI Channel |
|-----------|-----|--------------|
| Volume    | 7   | track index  |
| Pan       | 10  | track index  |
| Mute      | 119 | track index  |
| Solo      | 118 | track index  |

### Custom MIDI controls (Zrythm only)

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
```

Set `MAESTRO_MIDI_MAP=/path/to/map.toml` to use a different file path.

---

## Development

```bash
# Install frontend deps
npm install

# Type-check frontend
npm run check

# Unit tests (no DAW or GUI needed)
cd src-tauri && cargo test

# Lint
cd src-tauri && cargo clippy

# Production build
npm run tauri build
```

### End-to-end test (Zrythm, requires JACK)

```bash
./scripts/e2e-zrythm.sh
```

---

## Architecture

```
MAESTRO_ENGINE env var (default: ableton)
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
               ┌──────────────────┐      ┌─────────────────┐  ┌──────────────┐
               │ AbletonOscEngine │      │  ZrythmEngine   │  │  MockEngine  │
               │  (default)       │      │  (opt-in)       │  │  (testing)   │
               │                  │      │                  │  │              │
               │ OSC port 11000   │      │ JACK Transport  │  │  no-op       │
               └────────┬─────────┘      │ JACK MIDI CC    │  └──────────────┘
                        │                └────────┬────────┘
               ┌────────┴─────────┐      ┌────────┴────────┐
               │   Ableton Live   │      │  Zrythm (DAW)   │
               │   AbletonOSC     │      │  JACK MIDI In   │
               └──────────────────┘      └─────────────────┘
```

| Layer | Technology |
|-------|-----------|
| Frontend | Svelte 5, TypeScript, Tauri IPC |
| Backend | Rust, Tauri 2, `async-trait` |
| Ableton bridge | `rosc` crate — AbletonOSC (default) |
| Zrythm bridge | `jack` crate — JACK Transport + JACK MIDI CC |
| Sample search | HTTP sidecar — vector DB (Qdrant/Milvus) |
