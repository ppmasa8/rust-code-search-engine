use anyhow::Result;
use thiserror::Error;

#[derive(Debug)]
pub struct ParsedQuery {
    pub terms: Vec<String>,
    pub filters: Vec<String>,
}

#[derive(Debug, Error)]
pub enum QueryError {
    #[error("query must not be empty")]
    Empty,
}

pub fn parse(query: &str) -> Result<ParsedQuery> {
    if query.trim().is_empty() {
        return Err(QueryError::Empty.into());
    }
    let terms = query
        .split_whitespace()
        .filter(|segment| !segment.contains(':'))
        .map(|s| s.to_lowercase())
        .collect();
    let filters = query
        .split_whitespace()
        .filter(|segment| segment.contains(':'))
        .map(|s| s.to_owned())
        .collect();
    Ok(ParsedQuery { terms, filters })
}
