import logging
from pathlib import Path

import chromadb

from ..config import settings

logger = logging.getLogger("maestro-sidecar.db")


def get_chroma_client() -> chromadb.ClientAPI:
    """Create a persistent ChromaDB client."""
    persist_dir = Path(settings.chroma_persist_dir).expanduser()
    persist_dir.mkdir(parents=True, exist_ok=True)

    logger.info("Initializing ChromaDB at %s", persist_dir)

    client = chromadb.PersistentClient(path=str(persist_dir))
    return client
