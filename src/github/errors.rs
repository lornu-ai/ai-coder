use std::fmt;

pub type Result<T> = std::result::Result<T, GitHubError>;

/// GitHub API errors
#[derive(Debug)]
pub enum GitHubError {
    /// Network request failed
    RequestError(String),
    /// Invalid authentication
    AuthenticationError,
    /// Resource not found (404)
    NotFound(String),
    /// Rate limited by GitHub API
    RateLimited { reset_at: Option<u64> },
    /// Invalid input
    InvalidInput(String),
    /// JSON parsing error
    ParseError(String),
    /// Other GitHub API error
    ApiError { status: u16, message: String },
}

impl fmt::Display for GitHubError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitHubError::RequestError(msg) => write!(f, "Request failed: {}", msg),
            GitHubError::AuthenticationError => write!(f, "Authentication failed: invalid token"),
            GitHubError::NotFound(resource) => write!(f, "Not found: {}", resource),
            GitHubError::RateLimited { reset_at } => {
                if let Some(reset) = reset_at {
                    write!(f, "Rate limited. Reset at: {}", reset)
                } else {
                    write!(f, "Rate limited by GitHub API")
                }
            }
            GitHubError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            GitHubError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            GitHubError::ApiError { status, message } => {
                write!(f, "GitHub API error ({}): {}", status, message)
            }
        }
    }
}

impl std::error::Error for GitHubError {}

impl From<reqwest::Error> for GitHubError {
    fn from(err: reqwest::Error) -> Self {
        GitHubError::RequestError(err.to_string())
    }
}

impl From<serde_json::Error> for GitHubError {
    fn from(err: serde_json::Error) -> Self {
        GitHubError::ParseError(err.to_string())
    }
}
