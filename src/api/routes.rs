use super::repo_cache::RepoCache;
use crate::api::repo_state::{Commit, RepoState};
use axum::Router;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, header};
use axum::routing::post;
use hex;
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use std::env;
use std::sync::{Arc, RwLock};
use tokio;

type HmacSha256 = Hmac<Sha256>;

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
) -> Result<StatusCode, StatusCode> {
    headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| StatusCode::BAD_REQUEST)?
        .starts_with("application/json")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;
    headers
        .get("X-GitHub-Event")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| StatusCode::BAD_REQUEST)?
        .starts_with("push")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;

    let xhub_sig = headers
        .get("X-Hub-Signature-256")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| StatusCode::BAD_REQUEST)?
        .strip_prefix("sha256=")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;
    let xhub_sig = hex::decode(xhub_sig).map_err(|_| StatusCode::BAD_REQUEST)?;
    let webhook_secret = env::var("WEBHOOK_SECRET").expect("envvar WEBHOOK_SECRET");

    let mut mac = HmacSha256::new_from_slice(webhook_secret.as_bytes())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    mac.update(body.as_bytes());
    mac.verify_slice(&xhub_sig)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    r.set_event(body).await.map_err(|e| {
        println!("{:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(StatusCode::NO_CONTENT)
}
