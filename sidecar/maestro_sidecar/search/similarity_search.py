"""Audio-to-audio similarity search using CLAP embeddings."""

import logging
from typing import Dict, Any, Optional

import chromadb

from ..embeddings.clap_embedder import CLAPEmbedder
from ..embeddings.audio_processor import prepare_for_embedding

logger = logging.getLogger("maestro-sidecar.search")


def search_by_similarity(
    reference_file_path: str,
    embedder: CLAPEmbedder,
    collection: chromadb.Collection,
    n_results: int = 10,
    filters: Optional[Dict[str, Any]] = None,
) -> dict:
    """Search for audio samples similar to a reference audio file.

    Generates a CLAP embedding for the reference file and queries
    ChromaDB for the nearest audio embeddings.
    """
    # Prepare reference audio (resample if needed)
    prepared_path = prepare_for_embedding(reference_file_path)

    # Generate audio embedding
    audio_embedding = embedder.embed_audio_files([prepared_path])

    # Query ChromaDB
    results = collection.query(
        query_embeddings=audio_embedding.tolist(),
        n_results=n_results,
        where=filters,
    )

    return results
