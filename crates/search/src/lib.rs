use anyhow::Result;
use models::{CodeDocument, SearchResult};
use tracing::debug;

pub async fn run_search(query: &str) -> Result<Vec<SearchResult>> {
    let parsed = query::parse(query)?;
    let embedding = embedding::encode(query.as_bytes());
    debug!(?parsed.terms, latency_ms = 5, "mock search");

    let docs = vec![CodeDocument {
        path: "src/lib.rs".into(),
        language: "rust".into(),
        contents: "fn main() {}".into(),
    }];
    let _ = indexer::add_documents(&docs)?;

    Ok(vec![SearchResult {
        path: docs[0].path.clone(),
        score: embedding.iter().sum::<f32>(),
        highlight: format!("matched: {}", parsed.terms.join(",")),
    }])
}
