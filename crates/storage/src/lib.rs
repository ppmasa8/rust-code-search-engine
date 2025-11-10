use anyhow::Result;
use async_trait::async_trait;
use models::CodeDocument;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

#[async_trait]
pub trait Storage: Send + Sync {
    async fn put(&self, doc: CodeDocument) -> Result<()>;
    async fn all(&self) -> Result<Vec<CodeDocument>>;
}

#[derive(Clone, Default)]
pub struct InMemoryStorage {
    inner: Arc<RwLock<HashMap<String, CodeDocument>>>,
}

#[async_trait]
impl Storage for InMemoryStorage {
    async fn put(&self, doc: CodeDocument) -> Result<()> {
        self.inner.write().insert(doc.path.clone(), doc);
        Ok(())
    }

    async fn all(&self) -> Result<Vec<CodeDocument>> {
        Ok(self.inner.read().values().cloned().collect())
    }
}
