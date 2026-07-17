use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RepoState {
    pub id: u64,
    pub name: String,
    pub language: Option<String>,
    pub description: Option<String>,
    pub head_commit: Option<Commit>,
}

#[derive(Deserialize)]
pub struct Repo {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub language: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Commit {
    pub id: String,
    #[serde(default)]
    pub repo_name: String,
    pub timestamp: DateTime<Utc>,
    pub author: Author,
    pub distinct: bool,
    pub message: String,
    pub added: Vec<String>,
    pub modified: Vec<String>,
    pub removed: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Author {
    pub username: String,
    pub email: String,
}
