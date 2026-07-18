use crate::api::repo_state::{Commit, RepoState};

use super::repo_cache::RepoCache;
use axum::Router;
use axum::extract::{FromRef, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::post;
use std::sync::{Arc, RwLock};
use tokio;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct HandlerState {
    pub cache_conn: RepoCache,
    pub ssr_state: Arc<RwLock<SSRState>>,
    async_lock: Arc<Mutex<()>>,
}

pub struct SSRState {
    pub repos: Vec<RepoState>,
    pub commits: Vec<Commit>,
}

impl SSRState {
    pub async fn new(mut cache_conn: RepoCache) -> Option<Self> {
        let mut conn = cache_conn.clone();
        match tokio::join!(cache_conn.commits(None, 7), conn.repos(None)) {
            (Ok(commits), Ok(repos)) => Some(SSRState { commits, repos }),
            _ => None,
        }
    }
}

impl FromRef<HandlerState> for RepoCache {
    fn from_ref(handler_state: &HandlerState) -> Self {
        handler_state.cache_conn.clone()
    }
}

pub async fn routes(cache_conn: RepoCache, ssr_state: Arc<RwLock<SSRState>>) -> Router<()> {
    Router::new()
        .route("/webhooks/github", post(repo_push_event))
        .with_state(HandlerState {
            cache_conn,
            ssr_state,
            async_lock: Arc::new(Mutex::new(())),
        })
}

/// Update the redis cache and return the most recent RepoStates and Commits to
/// keep Arc<RwLock<Vec<Commits>>> and Arc<RwLock<Vec<RepoState>>> fresh for SSR.
async fn repo_push_event(
    headers: HeaderMap,
    State(mut state): State<HandlerState>,
    body: String,
) -> StatusCode {
    let _lock = state.async_lock.lock().await;
    let content_type = headers
        .get("Content-Type")
        .map_or_default(|v| v.to_str().unwrap_or_default());
    if content_type != "application/json" {
        return StatusCode::NOT_ACCEPTABLE;
    }
    if let Err(e) = state.cache_conn.set_event(body).await {
        println!("{:?}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    match SSRState::new(state.cache_conn).await {
        Some(refresh) => match state.ssr_state.write() {
            Ok(mut current) => {
                *current = refresh;
                StatusCode::NO_CONTENT
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        },
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
