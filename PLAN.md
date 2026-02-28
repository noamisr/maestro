# Maestro - Music Editing Interface

## Overview

Maestro is a music editing interface that uses **Ableton Live** as its primary backend audio engine, paired with a local **vector database** for fast soundtrack searching, and a **skills system** for quick operations. A **Zrythm** backend is available as an opt-in alternative on Linux.

## Why Ableton Live

Ableton Live is the primary backend engine because:

- **Industry standard** — widely used in professional music production
- **AbletonOSC** — open-source control surface exposing the full Live API over OSC
- **Rich API** — transport, tracks, clips, devices, scenes, and more
- **Cross-platform** — macOS and Windows

### Zrythm (opt-in alternative, Linux)

Zrythm is available as an opt-in backend via `MAESTRO_ENGINE=zrythm`. It is controlled over JACK Transport (tempo/play/stop) and JACK MIDI CC (track parameters). See the README for setup instructions.

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
