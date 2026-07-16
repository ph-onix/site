use super::repo_cache::{CachingError, RepoCache};
use axum::Router;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::routing::post;

pub fn routes(state: RepoCache) -> Router<()> {
    Router::new()
        .route("/webhooks/github", post(repo_push_event))
        .with_state(state)
}

async fn repo_push_event(
    headers: HeaderMap,
    State(state): State<RepoCache>,
    body: String,
) -> StatusCode {
    match headers.get("Content-Type") {
        Some(h) => {
            if let Ok(v) = h.to_str()
                && v != "application/json"
            {
                return StatusCode::NOT_ACCEPTABLE;
            }
        }
        _ => return StatusCode::NOT_ACCEPTABLE,
    }
    match state.clone().set_event(body).await {
        Err(CachingError::Redis(e)) => {
            println!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
        _ => StatusCode::NO_CONTENT,
    }
}
