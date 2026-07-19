use crate::api::repo_state::{Commit, Repo, RepoState};
use redis::{AsyncCommands, Msg, RedisError};
use serde::Deserialize;
use std::env;
use thiserror::Error;
use tokio::sync::{broadcast, mpsc::unbounded_channel};
// use uuid::Uuid;

#[derive(Debug, Error)]
pub enum CachingError {
    #[error(transparent)]
    Redis(#[from] RedisError),
    #[error(transparent)]
    BadJson(#[from] serde_json::Error),
    #[error("json schema does not match a defined event struct")]
    UnknownEvent,
}

/// Manages GitHub repository commit and documentation state in a Redis data store.
///
/// A single instance is provided to the Leptos router via [`provide_context`] at startup,
/// making it available to server functions through [`use_context`] during SSR and to
/// webhook handlers when GitHub pushes updates.
#[derive(Clone)]
pub struct RepoCache {
    client: redis::aio::MultiplexedConnection,
    broadcast: broadcast::Sender<String>,
    // pub id: Uuid,
}

#[derive(Deserialize)]
pub struct PushEvent {
    // #[serde(rename = "ref")]
    // git_ref: String,
    repository: Repo,
    commits: Vec<Commit>,
    head_commit: Option<Commit>,
}

impl RepoCache {
    /// Connect to the data store you want to cache github webhook info in.
    pub async fn new() -> Self {
        let (redis_pubs, mut rx) = unbounded_channel();
        let config = redis::AsyncConnectionConfig::new().set_push_sender(redis_pubs);
        let redis_url = env::var("REDIS_URL").expect("envvar REDIS_URL");
        let client = redis::Client::open(redis_url)
            .expect("failed to connect to redis")
            .get_multiplexed_async_connection_with_config(&config)
            .await
            .expect("failed to connect to redis");

        let (broadcast, _) = broadcast::channel(256);
        let broadcaster = broadcast.clone();
        tokio::spawn(async move {
            while let Some(push) = rx.recv().await {
                if let Some(msg) = Msg::from_push_info(push) {
                    match msg.get_channel() {
                        Ok(channel) => {
                            let _ = broadcaster.send(channel);
                        }
                        _ => continue,
                    }
                }
            }
        });
        Self { client, broadcast }
    }

    /// Get a subset of every cached RepoState.
    ///
    /// If no names are provided you will get the entire set.
    pub async fn repos(&mut self, ids: Option<Vec<u64>>) -> Result<Vec<RepoState>, CachingError> {
        match ids {
            Some(ids) => {
                let repo_states = self
                    .client
                    .hmget::<_, _, Vec<Option<String>>>("repos:states", ids)
                    .await?
                    .iter()
                    .filter_map(|opt| match opt {
                        Some(v) => serde_json::from_str(v).ok(),
                        _ => None,
                    })
                    .collect();
                Ok(repo_states)
            }
            None => {
                let repo_states = self
                    .client
                    .hvals::<_, Vec<String>>("repos:states")
                    .await?
                    .iter()
                    .filter_map(|s| serde_json::from_str(s).ok())
                    .collect();
                Ok(repo_states)
            }
        }
    }

    /// Get up to the cache limit of the most recent commits from a repo in chronological order.
    ///
    /// If no repo ids are provided I will give you a repo agnostic commits in chronological order.
    pub async fn commits(
        &mut self,
        ids: Option<Vec<u64>>,
        limit: isize,
    ) -> Result<Vec<Commit>, CachingError> {
        let limit_index = limit - 1;
        match ids {
            Some(v) => {
                let commits = v
                    .iter()
                    .fold(&mut redis::pipe(), |p, id| {
                        p.zrange(format!("commits:{id}"), 0, limit_index)
                    })
                    .query_async::<Vec<String>>(&mut self.client)
                    .await?
                    .iter()
                    .filter_map(|s| serde_json::from_str(&s).ok())
                    .collect();
                Ok(commits)
            }
            None => {
                let commits = self
                    .client
                    .zrange::<_, Vec<String>>("commits", 0, limit_index)
                    .await?
                    .iter()
                    .filter_map(|s| serde_json::from_str(&s).ok())
                    .collect();
                Ok(commits)
            }
        }
    }

    /// Update a repo state and the commit log in response to a webhook event.
    ///
    /// I will delete existing values in the cache.
    ///
    /// NOTE: My job is to parse and cache fields from a JSON string
    ///       with a schema I have been defined on; it's up to the caller to validate the source
    ///       before calling me.
    ///
    /// PLANNED: fan-out on changed or added markdown documentation BLOBs
    ///          then parse and cache the results for a dynamic route to render (auto doc pages).
    pub async fn set_event(&mut self, payload: String) -> Result<(), CachingError> {
        let cap: isize = 50;
        if let Ok(mut v) = serde_json::from_str::<PushEvent>(&payload) {
            let repo_id = v.repository.id;
            let repo_name = v.repository.name;
            let new_repo_state = RepoState {
                id: repo_id,
                language: v.repository.language,
                name: repo_name.clone(),
                description: v.repository.description,
                head_commit: v.head_commit,
            };
            let new_repo_state = serde_json::to_string(&new_repo_state)?;
            let score_member_pairs: Vec<(i64, String)> = v
                .commits
                .iter_mut()
                .filter_map(|c| {
                    c.repo_name = repo_name.clone();
                    if let Ok(json) = serde_json::to_string(c) {
                        return Some((c.timestamp.timestamp() * -1, json));
                    }
                    None
                })
                .collect();

            let repo_commit_log = format!("commits:{}", repo_id);
            let commit_log = "commits";
            let repo_channel = format!("repos:states:{}", repo_id);
            let cap_index = cap - 1;
            let _: () = redis::pipe()
                .atomic()
                .hset("repos:states", repo_id, &new_repo_state)
                .ignore()
                .zadd_multiple(&repo_commit_log, &score_member_pairs)
                .ignore()
                .zremrangebyrank(&repo_commit_log, cap_index, -1)
                .ignore()
                .zadd_multiple(&commit_log, &score_member_pairs)
                .ignore()
                .zremrangebyrank(&commit_log, cap_index, -1)
                .ignore()
                .publish(&repo_channel, 0)
                .query_async(&mut (*self).client)
                .await?;
        } else {
            return Err(CachingError::UnknownEvent);
        };
        Ok(())
    }

    /// On any repo state change made by another RepoCache instance
    /// `f` will be called as a background task.
    ///
    /// If provided an id I will watch that repo's channel otherwise I will watch
    /// every repo state change.
    pub async fn repo_subscribe<F, Fut>(&mut self, id: Option<u64>, mut f: F)
    where
        F: FnMut() -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        match id {
            Some(v) => {
                let channel = format!("repos:states:{}", v);
                self.client.subscribe(&channel).await
            }
            _ => {
                let channel = "repos:states:*".to_string();
                self.client.psubscribe(&channel).await
            }
        }
        .expect(&format!("failed to subsribe to repo id {:?}", id));

        let mut rx = self.broadcast.subscribe();
        tokio::spawn(async move {
            while let Ok(_) = rx.recv().await {
                f().await;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::repo_state::*;
    use chrono::*;

    #[test]
    fn parses_json_correctly() -> () {
        let x = r#"
            {
              "ref": "refs/heads/main",
              "before": "fac7aaec1ab758fac59ceb774c2ac595de550d10",
              "after": "b1c974978a2c744965bba2a4ff406a4e95c94c66",
              "repository": {
                "id": 1055909254,
                "node_id": "R_kgDOPu_lhg",
                "name": "nvim",
                "full_name": "ph-onix/nvim",
                "private": false,
                "owner": {},
                "html_url": "https://github.com/ph-onix/nvim",
                "description": null,
                "url": "https://api.github.com/repos/ph-onix/nvim",
                "commits_url": "https://api.github.com/repos/ph-onix/nvim/commits{/sha}",
                "created_at": 1757732043,
                "updated_at": "2026-07-05T06:37:30Z",
                "pushed_at": 1784068160,
                "open_issues_count": 0,
                "license": {},
                "allow_forking": true,
                "is_template": false,
                "language": "pussy",
                "web_commit_signoff_required": false
              },
              "pusher": {
                "name": "ph-onix",
                "email": "184308910+ph-onix@users.noreply.github.com"
              },
              "sender": {},
              "created": false,
              "deleted": false,
              "compare": "https://github.com/ph-onix/nvim/compare/fac7aaec1ab7...b1c974978a2c",
              "commits": [
                {
                  "id": "b1c974978a2c744965bba2a4ff406a4e95c94c66",
                  "tree_id": "d7aadc16e73813d3fb2c57e7f2f4e07302f7ef62",
                  "distinct": true,
                  "message": "chore",
                  "timestamp": "2026-07-14T17:29:14-05:00",
                  "url": "https://github.com/ph-onix/nvim/commit/b1c974978a2c744965bba2a4ff406a4e95c94c66",
                  "author": {
                    "name": "ph-onix",
                    "email": "pmiller0706@gmail.com",
                    "date": "2026-07-14T17:29:14-05:00",
                    "username": "ph-onix"
                  },
                  "committer": {},
                  "added": [],
                  "removed": [],
                  "modified": [
                    "README.md"
                  ]
                }
              ],
              "head_commit": {
                "id": "b1c974978a2c744965bba2a4ff406a4e95c94c66",
                "tree_id": "d7aadc16e73813d3fb2c57e7f2f4e07302f7ef62",
                "distinct": true,
                "message": "chore",
                "timestamp": "2026-07-14T17:29:14-05:00",
                "url": "https://github.com/ph-onix/nvim/commit/b1c974978a2c744965bba2a4ff406a4e95c94c66",
                "author": {
                  "name": "ph-onix",
                  "email": "pmiller0706@gmail.com",
                  "date": "2026-07-14T17:29:14-05:00",
                  "username": "ph-onix"
                },
                "committer": {},
                "added": [],
                "removed": [],
                "modified": [
                  "README.md"
                ]
              }
            }
        "#;
        let x = serde_json::from_str::<PushEvent>(&x).unwrap();
        let repo_state = RepoState {
            id: x.repository.id,
            name: x.repository.name,
            language: x.repository.language,
            description: x.repository.description,
            head_commit: x.head_commit,
        };
        let expect = r#"{
            "id": 1055909254,
            "name": "nvim",
            "language": "pussy",
            "description": null,
            "head_commit": {
                "id": "b1c974978a2c744965bba2a4ff406a4e95c94c66",
                "distinct": true,
                "message": "chore",
                "timestamp": "2026-07-14T17:29:14-05:00",
                "author": {
                  "username": "ph-onix",
                  "email": "pmiller0706@gmail.com"
                },
                "added": [],
                "removed": [],
                "modified": [
                  "README.md"
                ]
            }
        }"#;
        let expect = serde_json::from_str::<RepoState>(&expect).unwrap();
        let expect = serde_json::to_string(&expect).unwrap();
        let result = serde_json::to_string(&repo_state).unwrap();
        assert_eq!(result, expect);
    }

    #[test]
    fn can_parse_array_of_json_strings() {
        let _ = vec![
            RepoState {
                id: 1,
                name: "personal-site".into(),
                language: Some("Rust".into()),
                description: Some("My Leptos personal site".into()),
                head_commit: Some(Commit {
                    id: "a1b2c3d".into(),
                    repo_name: "".into(),
                    timestamp: Utc.with_ymd_and_hms(2026, 7, 16, 9, 30, 0).unwrap(),
                    author: Author {
                        username: "ph-onix".into(),
                        email: "pheonixmiller@industrialacqai.com".into(),
                    },
                    distinct: true,
                    message: "feat: complete build log UI".into(),
                    added: vec!["src/components/term".into()],
                    modified: vec!["src/app.rs".into(), "src/lib.rs".into()],
                    removed: vec!["src/repo_state.rs".into()],
                }),
            },
            RepoState {
                id: 2,
                name: "claw".into(),
                language: Some("Rust".into()),
                description: None,
                head_commit: Some(Commit {
                    id: "e4f5g6h".into(),
                    repo_name: "".into(),
                    timestamp: Utc::now(),
                    author: Author {
                        username: "ph-onix".into(),
                        email: "pheonixmiller@industrialacqai.com".into(),
                    },
                    distinct: true,
                    message: "chore: initial scaffol".into(),
                    added: vec!["Cargo.toml".into(), "src/main.rs".into()],
                    modified: vec![],
                    removed: vec![],
                }),
            },
            RepoState {
                id: 3,
                name: "dotfiles".into(),
                language: None,
                description: Some("Shell + editor co".into()),
                head_commit: None,
            },
        ]
        .iter()
        .for_each(|expect| {
            let v = serde_json::to_string(&expect).unwrap();
            let result: RepoState = serde_json::from_str(&v).unwrap();
            assert_eq!(result, *expect);
        });
    }
}
