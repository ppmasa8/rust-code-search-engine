use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchItem {
    pub path: String,
    pub score: f32,
    pub snippet: String,
}
