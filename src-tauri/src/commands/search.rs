use std::sync::Arc;
use tauri::State;

use crate::engine::EngineAdapter;
use crate::sidecar::api::{IndexResponse, SearchResultItem, SidecarClient};

#[tauri::command]
pub async fn search_by_text(
    sidecar: State<'_, SidecarClient>,
    query: String,
    n_results: usize,
) -> Result<Vec<SearchResultItem>, String> {
    let resp = sidecar
        .search_text(&query, n_results)
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp.results)
}

#[tauri::command]
pub async fn search_by_similarity(
    sidecar: State<'_, SidecarClient>,
    file_path: String,
    n_results: usize,
) -> Result<Vec<SearchResultItem>, String> {
    let resp = sidecar
        .search_similar(&file_path, n_results)
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp.results)
}

#[tauri::command]
pub async fn index_directory(
    sidecar: State<'_, SidecarClient>,
    directory: String,
) -> Result<IndexResponse, String> {
    sidecar
        .index_directory(&directory)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn insert_sample(
    engine: State<'_, Arc<dyn EngineAdapter>>,
    file_path: String,
    track_index: i32,
    scene_index: i32,
) -> Result<(), String> {
    engine.load_sample(track_index, scene_index, &file_path)
}
