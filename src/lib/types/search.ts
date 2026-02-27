export interface SearchResultItem {
  id: string;
  filePath: string;
  fileName: string;
  distance: number;
  durationSeconds: number;
  metadata: Record<string, unknown>;
}

export interface SearchResponse {
  results: SearchResultItem[];
  query: string;
  total: number;
}
