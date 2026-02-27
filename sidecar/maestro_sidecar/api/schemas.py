from pydantic import BaseModel
from typing import Optional, Dict, List, Any


class HealthResponse(BaseModel):
    status: str
    collections: Optional[Dict[str, int]] = None


class TextSearchRequest(BaseModel):
    query: str
    n_results: int = 10
    filters: Optional[Dict[str, Any]] = None
    collection: str = "samples"


class SimilaritySearchRequest(BaseModel):
    reference_file_path: str
    n_results: int = 10
    filters: Optional[Dict[str, Any]] = None
    collection: str = "samples"


class SearchResultItem(BaseModel):
    id: str
    file_path: str
    file_name: str
    distance: float
    duration_seconds: float
    metadata: Dict[str, Any]


class SearchResponse(BaseModel):
    results: List[SearchResultItem]
    query: str
    total: int

    @classmethod
    def from_chroma_results(cls, results: dict, query: str = "") -> "SearchResponse":
        items = []
        if results["ids"] and results["ids"][0]:
            for i in range(len(results["ids"][0])):
                meta = results["metadatas"][0][i]
                items.append(
                    SearchResultItem(
                        id=results["ids"][0][i],
                        file_path=meta.get("file_path", ""),
                        file_name=meta.get("file_name", ""),
                        distance=results["distances"][0][i] if results.get("distances") else 0.0,
                        duration_seconds=meta.get("duration_seconds", 0.0),
                        metadata=meta,
                    )
                )
        return cls(results=items, query=query, total=len(items))


class IndexRequest(BaseModel):
    file_paths: List[str]


class DirectoryIndexRequest(BaseModel):
    directory: str


class IndexResponse(BaseModel):
    total: int
    new: int
    indexed: int
