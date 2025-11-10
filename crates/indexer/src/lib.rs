use anyhow::Result;
use models::CodeDocument;
use tracing::info;

#[cfg(feature = "with-tantivy")]
use tantivy::schema::{Schema, SchemaBuilder, TEXT};

#[cfg(not(feature = "with-tantivy"))]
#[derive(Debug, Clone)]
pub struct Schema {
    pub fields: Vec<&'static str>,
}

#[cfg(feature = "with-tantivy")]
pub fn schema() -> Schema {
    let mut builder = SchemaBuilder::default();
    builder.add_text_field("path", TEXT);
    builder.add_text_field("contents", TEXT);
    builder.add_text_field("language", TEXT);
    builder.build()
}

#[cfg(not(feature = "with-tantivy"))]
pub fn schema() -> Schema {
    Schema {
        fields: vec!["path", "contents", "language"],
    }
}

pub fn add_documents(docs: &[CodeDocument]) -> Result<usize> {
    #[cfg(feature = "with-tantivy")]
    {
        info!(count = docs.len(), "tantivy indexing batch");
    }
    #[cfg(not(feature = "with-tantivy"))]
    {
        info!(count = docs.len(), "mock indexing batch");
    }
    Ok(docs.len())
}
