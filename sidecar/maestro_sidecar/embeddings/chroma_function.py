"""Custom ChromaDB embedding function using CLAP for text-to-audio queries."""

from typing import List

from .clap_embedder import CLAPEmbedder


class CLAPTextEmbeddingFunction:
    """ChromaDB-compatible embedding function that uses CLAP for text queries.

    This is used when querying ChromaDB with text descriptions.
    Audio embeddings are pre-computed and stored directly.
    """

    def __init__(self, embedder: CLAPEmbedder):
        self._embedder = embedder

    def __call__(self, input: List[str]) -> List[List[float]]:
        return self._embedder.embed_text(input).tolist()
