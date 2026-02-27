import datetime
import logging
import os
import uuid
from pathlib import Path
from typing import List, Optional, Callable

import chromadb

from ..embeddings.clap_embedder import CLAPEmbedder
from ..embeddings.audio_processor import (
    extract_metadata,
    prepare_for_embedding,
    SUPPORTED_EXTENSIONS,
)

logger = logging.getLogger("maestro-sidecar.indexer")


class AudioIndexer:
    """Batch indexing pipeline: scan → embed → store in ChromaDB."""

    def __init__(
        self,
        embedder: CLAPEmbedder,
        collection: chromadb.Collection,
        batch_size: int = 32,
    ):
        self.embedder = embedder
        self.collection = collection
        self.batch_size = batch_size

    def scan_directory(self, directory: str) -> List[str]:
        """Recursively find all supported audio files in a directory."""
        audio_files = []
        for root, _, files in os.walk(directory):
            for f in files:
                if Path(f).suffix.lower() in SUPPORTED_EXTENSIONS:
                    audio_files.append(os.path.join(root, f))
        audio_files.sort()
        return audio_files

    def _get_existing_paths(self, file_paths: List[str]) -> set:
        """Check which file paths are already indexed."""
        existing = set()
        try:
            # Query in chunks since ChromaDB has limits
            for i in range(0, len(file_paths), 500):
                chunk = file_paths[i : i + 500]
                results = self.collection.get(
                    where={"file_path": {"$in": chunk}},
                    include=[],
                )
                if results and results["ids"]:
                    # We need to get the metadatas to check file_path
                    full_results = self.collection.get(
                        ids=results["ids"],
                        include=["metadatas"],
                    )
                    for meta in full_results["metadatas"]:
                        if meta and "file_path" in meta:
                            existing.add(meta["file_path"])
        except Exception as e:
            logger.warning("Could not check existing entries: %s", e)
        return existing

    def index_files(
        self,
        file_paths: List[str],
        on_progress: Optional[Callable[[int, int], None]] = None,
    ) -> dict:
        """Index a list of audio files into ChromaDB.

        Returns dict with keys: total, new, indexed.
        """
        total = len(file_paths)
        if total == 0:
            return {"total": 0, "new": 0, "indexed": 0}

        # Filter out already-indexed files
        existing = self._get_existing_paths(file_paths)
        new_files = [f for f in file_paths if f not in existing]

        logger.info(
            "Indexing: %d total, %d already indexed, %d new",
            total,
            len(existing),
            len(new_files),
        )

        indexed_count = 0

        for i in range(0, len(new_files), self.batch_size):
            batch = new_files[i : i + self.batch_size]

            # Prepare audio files (resample if needed)
            prepared = []
            valid_batch = []
            for f in batch:
                try:
                    prepared_path = prepare_for_embedding(f)
                    prepared.append(prepared_path)
                    valid_batch.append(f)
                except Exception as e:
                    logger.warning("Skipping %s: %s", f, e)

            if not valid_batch:
                continue

            try:
                # Generate embeddings
                embeddings = self.embedder.embed_audio_files(prepared)

                # Extract metadata
                metadatas = []
                ids = []
                documents = []
                for f in valid_batch:
                    try:
                        meta = extract_metadata(f)
                        meta["indexed_at"] = datetime.datetime.utcnow().isoformat()
                        metadatas.append(meta)
                        ids.append(str(uuid.uuid5(uuid.NAMESPACE_URL, f)))
                        documents.append(Path(f).stem)
                    except Exception as e:
                        logger.warning("Failed to extract metadata for %s: %s", f, e)

                # Upsert into ChromaDB
                if ids:
                    self.collection.upsert(
                        ids=ids,
                        embeddings=embeddings[: len(ids)].tolist(),
                        metadatas=metadatas,
                        documents=documents,
                    )
                    indexed_count += len(ids)

            except Exception as e:
                logger.error("Batch embedding failed: %s", e, exc_info=True)

            if on_progress:
                on_progress(indexed_count, len(new_files))

            logger.info(
                "Indexed %d/%d files", indexed_count, len(new_files)
            )

        return {"total": total, "new": len(new_files), "indexed": indexed_count}
