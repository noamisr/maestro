"""Text-to-audio search using CLAP embeddings."""

import logging
from typing import Dict, Any, List, Optional

import chromadb

from ..embeddings.clap_embedder import CLAPEmbedder

logger = logging.getLogger("maestro-sidecar.search")


def search_by_text(
    query: str,
    embedder: CLAPEmbedder,
    collection: chromadb.Collection,
    n_results: int = 10,
    filters: Optional[Dict[str, Any]] = None,
) -> dict:
    """Search for audio samples matching a text description.

    Uses CLAP to embed the text query and then queries ChromaDB
    for the nearest audio embeddings using cosine similarity.
    """
    # Generate text embedding
    text_embedding = embedder.embed_text([query])

    # Query ChromaDB
    results = collection.query(
        query_embeddings=text_embedding.tolist(),
        n_results=n_results,
        where=filters,
    )

    return results
