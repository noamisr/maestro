# Maestro - Music Editing Interface

## Overview

Maestro is a music editing interface that uses **Zrythm** as its backend audio engine, paired with a local **vector database** for fast soundtrack searching, and a **skills system** for quick operations.

## Why Zrythm

Zrythm was chosen over Ableton (proprietary, expensive) as the backend engine because:

- **Free & open source** (AGPLv3) — no licensing costs
- **Most Ableton-like UI** of any free DAW — modern, cohesive single-window layout
- **Built-in scripting API** via GNU Guile (Scheme/ECMAScript) — programmatic control over tracks, MIDI notes, regions, ports, channels, plugins, and undo
- **Full DAW features** — audio recording, MIDI, LV2/VST plugins, automation
- **Cross-platform** — Linux, macOS, Windows
- **Active development** — C++23 / Qt / JUCE (v2 branch)

### Alternatives Considered

| Engine | Why Not |
|---|---|
| Ableton Live | Proprietary, $449-$749, no scripting API |
| LMMS | No audio recording, no external scripting API |
| Ardour | No Ableton-like UI, Lua scripting is extension-level only |
| Tone.js | Library only (no UI), browser-only |
| SuperCollider | No DAW UI, steep learning curve, GPL |

## Architecture

```
+------------------+     +------------------+     +------------------+
|                  |     |                  |     |                  |
|   Maestro UI     |<--->|  Maestro Core    |<--->|    Zrythm        |
|   (Frontend)     |     |  (Backend)       |     |  (Audio Engine)  |
|                  |     |                  |     |                  |
+------------------+     +--------+---------+     +------------------+
                                  |
                                  v
                         +------------------+
                         |                  |
                         |  Vector DB       |
                         |  (Fast Search)   |
                         |                  |
                         +------------------+
```

### Components

1. **Maestro UI (Frontend)**
   - Web-based interface for editing and managing music projects
   - Communicates with Maestro Core via API

2. **Maestro Core (Backend)**
   - Orchestrates communication between UI, Zrythm, and Vector DB
   - Hosts the skills system for quick operations
   - Manages project state and user sessions

3. **Zrythm (Audio Engine)**
   - Handles all audio processing, MIDI, synthesis, and plugin hosting
   - Controlled programmatically via Guile scripting API
   - API modules: tracks, MIDI notes, regions, ports, channels, plugins, undo

4. **Vector DB (Fast Search)**
   - Local vector database for fast semantic search over soundtracks and audio assets
   - Candidate engines: ChromaDB, Qdrant, or Milvus Lite
   - Indexes audio features (embeddings) for similarity search

## Skills System

"Skills" are predefined quick operations that execute via Zrythm's scripting API:

- **Track operations** — add, remove, duplicate, reorder tracks
- **MIDI editing** — quantize, transpose, velocity adjustments
- **Audio processing** — apply effects, normalize, trim
- **Arrangement** — copy/move regions, loop sections
- **Mixing** — adjust levels, pan, send routing
- **Export** — render to WAV/MP3, stem export

Each skill maps to one or more Guile API calls against Zrythm.

## Vector DB Integration

The vector DB enables fast search over the editing soundtrack:

- Audio files are processed into feature embeddings (tempo, key, energy, timbre)
- Embeddings are stored in the local vector DB
- Users can search by similarity ("find sounds like this"), by mood, or by musical attributes
- Results are returned in milliseconds for real-time workflow

## References

- [Zrythm Documentation](https://manual.zrythm.org/en/index.html)
- [Zrythm Scripting API](https://manual.zrythm.org/en/scripting/api/zrythm.html)
- [Zrythm GitHub](https://github.com/zrythm/zrythm)
