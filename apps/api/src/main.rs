use anyhow::Result;
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use common::AppConfig;
use contracts::{SearchRequest, SearchResponse};
use search::run_search;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

#[derive(Clone)]
struct ApiState {
    cfg: Arc<AppConfig>,
}

#[tokio::main]
async fn main() -> Result<()> {
    common::init_tracing();
    let cfg = Arc::new(AppConfig::default());
    let router = Router::new()
        .route("/search", post(search_endpoint))
        .with_state(ApiState { cfg: cfg.clone() });

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    info!(addr = %listener.local_addr()?, "listening");
    axum::serve(listener, router).await?;
    Ok(())
}

async fn search_endpoint(
    State(_state): State<ApiState>,
    Json(request): Json<SearchRequest>,
) -> Result<Json<SearchResponse>, (StatusCode, String)> {
    let results = run_search(&request.query)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    let payload = SearchResponse {
        results: results
            .into_iter()
            .map(|item| contracts::SearchItem {
                path: item.path,
                score: item.score,
                snippet: item.highlight,
            })
            .collect(),
    };
    Ok(Json(payload))
}
