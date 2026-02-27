use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct SidecarClient {
    client: reqwest::Client,
    base_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResultItem {
    pub id: String,
    pub file_path: String,
    pub file_name: String,
    pub distance: f64,
    pub duration_seconds: f64,
    pub metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    pub results: Vec<SearchResultItem>,
    pub query: String,
    pub total: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexResponse {
    pub total: usize,
    pub new: usize,
    pub indexed: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthResponse {
    pub status: String,
}

impl SidecarClient {
    pub fn new(port: u16) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: format!("http://127.0.0.1:{}", port),
        }
    }

    pub async fn health(&self) -> Result<bool, reqwest::Error> {
        let resp = self
            .client
            .get(format!("{}/health", self.base_url))
            .send()
            .await?;
        Ok(resp.status().is_success())
    }

    pub async fn search_text(
        &self,
        query: &str,
        n_results: usize,
    ) -> Result<SearchResponse, reqwest::Error> {
        self.client
            .post(format!("{}/search/text", self.base_url))
            .json(&serde_json::json!({
                "query": query,
                "n_results": n_results,
            }))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn search_similar(
        &self,
        file_path: &str,
        n_results: usize,
    ) -> Result<SearchResponse, reqwest::Error> {
        self.client
            .post(format!("{}/search/similar", self.base_url))
            .json(&serde_json::json!({
                "reference_file_path": file_path,
                "n_results": n_results,
            }))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn index_files(
        &self,
        paths: Vec<String>,
    ) -> Result<IndexResponse, reqwest::Error> {
        self.client
            .post(format!("{}/index", self.base_url))
            .json(&serde_json::json!({ "file_paths": paths }))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn index_directory(
        &self,
        directory: &str,
    ) -> Result<IndexResponse, reqwest::Error> {
        self.client
            .post(format!("{}/index/directory", self.base_url))
            .json(&serde_json::json!({ "directory": directory }))
            .send()
            .await?
            .json()
            .await
    }
}
