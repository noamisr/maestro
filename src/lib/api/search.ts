import { invoke } from "@tauri-apps/api/core";
import type { SearchResultItem } from "../types/search";

export async function searchByText(
  query: string,
  nResults: number = 10,
): Promise<SearchResultItem[]> {
  return invoke("search_by_text", { query, nResults });
}

export async function searchBySimilarity(
  filePath: string,
  nResults: number = 10,
): Promise<SearchResultItem[]> {
  return invoke("search_by_similarity", { filePath, nResults });
}

export async function insertSample(
  filePath: string,
  trackIndex: number,
  sceneIndex: number,
): Promise<void> {
  return invoke("insert_sample", { filePath, trackIndex, sceneIndex });
}

export async function indexDirectory(
  directory: string,
): Promise<{ total: number; indexed: number }> {
  return invoke("index_directory", { directory });
}
