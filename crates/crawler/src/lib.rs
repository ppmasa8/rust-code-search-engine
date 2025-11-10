use anyhow::Result;
use async_trait::async_trait;
use models::CodeDocument;

#[async_trait]
pub trait SourceCrawler: Send + Sync {
    async fn crawl(&self, root: &str) -> Result<Vec<CodeDocument>>;
}

pub struct LocalCrawler;

#[async_trait]
impl SourceCrawler for LocalCrawler {
    async fn crawl(&self, root: &str) -> Result<Vec<CodeDocument>> {
        // pretend to walk the filesystem and collect files
        let sample = CodeDocument {
            path: format!("{root}/lib.rs"),
            language: "rust".into(),
            contents: "fn main() {}".into(),
        };
        Ok(vec![sample])
    }
}
