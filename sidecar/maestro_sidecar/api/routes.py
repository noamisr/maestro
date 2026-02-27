import logging
from fastapi import APIRouter, Request, HTTPException

from .schemas import (
    HealthResponse,
    TextSearchRequest,
    SimilaritySearchRequest,
    SearchResponse,
    IndexRequest,
    DirectoryIndexRequest,
    IndexResponse,
)

logger = logging.getLogger("maestro-sidecar.api")

router = APIRouter()


def _get_embedder(request: Request):
    """Lazy-load the CLAP embedder on first use."""
    if request.app.state.embedder is None:
        logger.info("Loading CLAP model (first use)... this may take a moment")
        from ..embeddings.clap_embedder import CLAPEmbedder

        request.app.state.embedder = CLAPEmbedder()
        logger.info("CLAP model loaded")
    return request.app.state.embedder


@router.get("/health")
async def health(request: Request) -> HealthResponse:
    try:
        samples = request.app.state.samples_collection
        clips = request.app.state.clips_collection
        return HealthResponse(
            status="ok",
            collections={
                "samples": samples.count(),
                "project_clips": clips.count(),
            },
        )
    except Exception:
        return HealthResponse(status="ok")


@router.post("/search/text")
async def search_by_text(req: TextSearchRequest, request: Request) -> SearchResponse:
    try:
        embedder = _get_embedder(request)
        collection = request.app.state.samples_collection
        if req.collection == "project_clips":
            collection = request.app.state.clips_collection

        # Generate text embedding via CLAP
        text_embedding = embedder.embed_text([req.query])

        results = collection.query(
            query_embeddings=text_embedding.tolist(),
            n_results=req.n_results,
            where=req.filters or None,
        )
        return SearchResponse.from_chroma_results(results, query=req.query)
    except Exception as e:
        logger.error("Text search failed: %s", e, exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/search/similar")
async def search_by_similarity(
    req: SimilaritySearchRequest, request: Request
) -> SearchResponse:
    try:
        embedder = _get_embedder(request)
        collection = request.app.state.samples_collection
        if req.collection == "project_clips":
            collection = request.app.state.clips_collection

        # Generate audio embedding for the reference file
        audio_embedding = embedder.embed_audio_files([req.reference_file_path])

        results = collection.query(
            query_embeddings=audio_embedding.tolist(),
            n_results=req.n_results,
            where=req.filters or None,
        )
        return SearchResponse.from_chroma_results(
            results, query=f"similar:{req.reference_file_path}"
        )
    except Exception as e:
        logger.error("Similarity search failed: %s", e, exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/index")
async def index_files(req: IndexRequest, request: Request) -> IndexResponse:
    try:
        embedder = _get_embedder(request)
        collection = request.app.state.samples_collection

        from ..db.indexer import AudioIndexer

        indexer = AudioIndexer(embedder, collection)
        result = indexer.index_files(req.file_paths)
        return IndexResponse(**result)
    except Exception as e:
        logger.error("Index files failed: %s", e, exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/index/directory")
async def index_directory(
    req: DirectoryIndexRequest, request: Request
) -> IndexResponse:
    try:
        embedder = _get_embedder(request)
        collection = request.app.state.samples_collection

        from ..db.indexer import AudioIndexer

        indexer = AudioIndexer(embedder, collection)
        files = indexer.scan_directory(req.directory)
        logger.info("Found %d audio files in %s", len(files), req.directory)
        result = indexer.index_files(files)
        return IndexResponse(**result)
    except Exception as e:
        logger.error("Index directory failed: %s", e, exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))
