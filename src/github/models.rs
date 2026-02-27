use serde::{Deserialize, Serialize};

/// GitHub Pull Request
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PullRequest {
    pub number: u32,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub head: GitRef,
    pub base: GitRef,
    pub user: User,
    pub created_at: String,
    pub updated_at: String,
    pub html_url: String,
}

/// Git reference (branch/commit)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitRef {
    pub sha: String,
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub repo: Option<Repository>,
}

/// Repository information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
    pub owner: User,
}

/// GitHub User
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
}

/// File content from repository
#[derive(Debug, Clone, Deserialize)]
pub struct FileContent {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: u32,
    pub content: Option<String>,
    #[serde(rename = "type")]
    pub content_type: String,
}

/// Pull Request Review
#[derive(Debug, Clone, Serialize)]
pub struct PullRequestReview {
    pub body: String,
    pub event: ReviewEvent,
}

/// Review event type
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReviewEvent {
    Approve,
    RequestChanges,
    Comment,
}

/// GitHub Issue
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Issue {
    pub number: u32,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub html_url: String,
}

/// Commit information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Commit {
    pub sha: String,
    pub message: String,
    pub author: Option<CommitAuthor>,
    pub url: String,
}

/// Commit author
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommitAuthor {
    pub name: String,
    pub email: String,
    pub date: String,
}
