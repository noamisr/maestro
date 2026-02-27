import logging
from typing import Tuple

import chromadb

logger = logging.getLogger("maestro-sidecar.db")


def create_collections(
    client: chromadb.ClientAPI,
) -> Tuple[chromadb.Collection, chromadb.Collection]:
    """Create or get the two main collections.

    - samples: Local sample library (WAV, MP3, AIFF, etc.)
    - project_clips: Clips from Ableton projects

    Both store 512-dim CLAP audio embeddings with cosine distance.
    """
    samples = client.get_or_create_collection(
        name="samples",
        metadata={"hnsw:space": "cosine"},
    )
    logger.info("Collection 'samples': %d items", samples.count())

    project_clips = client.get_or_create_collection(
        name="project_clips",
        metadata={"hnsw:space": "cosine"},
    )
    logger.info("Collection 'project_clips': %d items", project_clips.count())

    return samples, project_clips
