#![allow(dead_code)]

use super::errors::{GitHubError, Result};
use super::models::{Commit, FileContent, PullRequest, PullRequestReview};
use base64::Engine;
use reqwest::Client;
use serde_json::json;
use std::env;

/// GitHub API client
pub struct GitHubClient {
    http_client: Client,
    token: String,
    base_url: String,
}

impl GitHubClient {
    /// Create a new GitHub client
    pub fn new(token: Option<String>) -> Result<Self> {
        let token = token
            .or_else(|| env::var("GITHUB_TOKEN").ok())
            .ok_or(GitHubError::AuthenticationError)?;

        Ok(Self {
            http_client: Client::new(),
            token,
            base_url: "https://api.github.com".to_string(),
        })
    }

    /// Get a pull request
    pub async fn get_pull_request(
        &self,
        owner: &str,
        repo: &str,
        pr_number: u32,
    ) -> Result<PullRequest> {
        let url = format!(
            "{}/repos/{}/{}/pulls/{}",
            self.base_url, owner, repo, pr_number
        );
        self.get(&url).await
    }

    /// Get file content from repository
    pub async fn get_file_content(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        branch: &str,
    ) -> Result<String> {
        let url = format!(
            "{}/repos/{}/{}/contents/{}?ref={}",
            self.base_url, owner, repo, path, branch
        );

        let file: FileContent = self.get(&url).await?;

        // GitHub returns base64-encoded content
        file.content
            .and_then(|encoded| {
                base64::engine::general_purpose::STANDARD
                    .decode(&encoded)
                    .ok()
                    .and_then(|bytes| String::from_utf8(bytes).ok())
            })
            .ok_or_else(|| GitHubError::ParseError("Could not decode file content".to_string()))
    }

    /// Post a review on a pull request
    pub async fn post_pr_review(
        &self,
        owner: &str,
        repo: &str,
        pr_number: u32,
        review: PullRequestReview,
    ) -> Result<()> {
        let url = format!(
            "{}/repos/{}/{}/pulls/{}/reviews",
            self.base_url, owner, repo, pr_number
        );

        let body = json!({
            "body": review.body,
            "event": review.event,
        });

        let _: serde_json::Value = self.post(&url, body).await?;
        Ok(())
    }

    /// Get a repository's README
    pub async fn get_readme(&self, owner: &str, repo: &str) -> Result<String> {
        match self
            .get_file_content(owner, repo, "README.md", "main")
            .await
        {
            Ok(content) => Ok(content),
            Err(_) => {
                self.get_file_content(owner, repo, "README.md", "master")
                    .await
            }
        }
    }

    /// Create a commit
    pub async fn create_commit(
        &self,
        owner: &str,
        repo: &str,
        message: &str,
        tree: &str,
        parents: Vec<String>,
    ) -> Result<String> {
        let url = format!("{}/repos/{}/{}/git/commits", self.base_url, owner, repo);

        let body = json!({
            "message": message,
            "tree": tree,
            "parents": parents,
        });

        let response: Commit = self.post(&url, body).await?;
        Ok(response.sha)
    }

    /// Generic GET request
    async fn get<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T> {
        let response = self
            .http_client
            .get(url)
            .header("Authorization", format!("token {}", self.token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "ai-coder")
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Generic POST request
    async fn post<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        body: serde_json::Value,
    ) -> Result<T> {
        let response = self
            .http_client
            .post(url)
            .header("Authorization", format!("token {}", self.token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "ai-coder")
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Handle HTTP response
    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();

        match status.as_u16() {
            200..=299 => {
                let text = response.text().await?;
                serde_json::from_str(&text).map_err(GitHubError::from)
            }
            401 => Err(GitHubError::AuthenticationError),
            404 => Err(GitHubError::NotFound("Resource not found".to_string())),
            403 => {
                if let Ok(text) = response.text().await {
                    if text.contains("API rate limit exceeded") {
                        return Err(GitHubError::RateLimited { reset_at: None });
                    }
                }
                Err(GitHubError::ApiError {
                    status: 403,
                    message: "Forbidden".to_string(),
                })
            }
            code => {
                let message = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(GitHubError::ApiError {
                    status: code,
                    message,
                })
            }
        }
    }
}
