import logging
from contextlib import asynccontextmanager

import uvicorn
from fastapi import FastAPI

from .config import settings
from .api.routes import router
from .db.client import get_chroma_client
from .db.collections import create_collections

logger = logging.getLogger("maestro-sidecar")


@asynccontextmanager
async def lifespan(app: FastAPI):
    """Startup: init ChromaDB. CLAP model is loaded lazily on first search."""
    logger.info("Starting Maestro sidecar...")

    # Initialize ChromaDB
    chroma = get_chroma_client()
    samples_col, clips_col = create_collections(chroma)
    app.state.chroma = chroma
    app.state.samples_collection = samples_col
    app.state.clips_collection = clips_col
    app.state.embedder = None  # Lazy-loaded on first embed request

    logger.info("ChromaDB initialized at %s", settings.chroma_persist_dir)
    logger.info("Maestro sidecar ready on port %d", settings.port)

    yield

    logger.info("Shutting down Maestro sidecar")


app = FastAPI(
    title="Maestro Sidecar",
    version="0.1.0",
    lifespan=lifespan,
)
app.include_router(router)


def run():
    """Entry point for the sidecar."""
    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s [%(name)s] %(levelname)s: %(message)s",
    )
    uvicorn.run(
        app,
        host=settings.host,
        port=settings.port,
        log_level="info",
    )


if __name__ == "__main__":
    run()
