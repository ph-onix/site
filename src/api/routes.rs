use crate::api::repo_state::{Commit, RepoState};

use super::repo_cache::RepoCache;
use axum::Router;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::routing::post;
use std::sync::{Arc, RwLock};
use tokio;

pub struct SSRState {
    pub repos: Vec<RepoState>,
    pub commits: Vec<Commit>,
}

impl SSRState {
    pub fn new() -> Arc<RwLock<Self>> {
        let ssr_state = Self {
            repos: vec![],
            commits: vec![],
        };
        Arc::new(RwLock::new(ssr_state))
    }

    async fn refresh(mut cache_conn: RepoCache) -> Option<Self> {
        let mut conn = cache_conn.clone();
        match tokio::join!(cache_conn.commits(None, 7), conn.repos(None)) {
            (Ok(commits), Ok(repos)) => Some(Self { commits, repos }),
            _ => None,
        }
    }
}

pub async fn routes(ssr_state: Arc<RwLock<SSRState>>) -> Router<()> {
    let ssr_fresh = async |cache_conn: RepoCache, ssr_state: Arc<RwLock<SSRState>>| {
        if let Some(refresh) = SSRState::refresh(cache_conn).await {
            if let Ok(mut v) = ssr_state.write() {
                *v = refresh;
            }
        }
    };

    let mut cache_conn = RepoCache::new().await;
    let c = cache_conn.clone();
    let s = ssr_state.clone();
    ssr_fresh(cache_conn.clone(), ssr_state).await;
    cache_conn
        .repo_subscribe(None, move || {
            let s = s.clone();
            let c = c.clone();
            async move { ssr_fresh(c, s).await }
        })
        .await;

    Router::new()
        .route("/webhooks/github", post(repo_push_event))
        .with_state(cache_conn)
}

/// Ingest incoming repo state change and `git push` events into the cache.
async fn repo_push_event(
    headers: HeaderMap,
    State(mut r): State<RepoCache>,
    body: String,
) -> StatusCode {
    let content_type = headers
        .get("Content-Type")
        .map_or_default(|v| v.to_str().unwrap_or_default());
    if content_type != "application/json" {
        return StatusCode::NOT_ACCEPTABLE;
    }
    if let Err(e) = r.set_event(body).await {
        println!("{:?}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    StatusCode::NO_CONTENT
}
