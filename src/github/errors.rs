use std::fmt;

#[derive(Debug)]
pub enum GitHubError {
    InvalidInput(String),
    AuthenticationError,
    NotFound(String),
    ApiError { status: u16, message: String },
    RequestError(String),
}

impl fmt::Display for GitHubError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitHubError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            GitHubError::AuthenticationError => write!(f, "Authentication failed"),
            GitHubError::NotFound(msg) => write!(f, "Not found: {}", msg),
            GitHubError::ApiError { status, message } => {
                write!(f, "GitHub API error ({}): {}", status, message)
            }
            GitHubError::RequestError(msg) => write!(f, "Request error: {}", msg),
        }
    }
}

impl std::error::Error for GitHubError {}

impl From<reqwest::Error> for GitHubError {
    fn from(err: reqwest::Error) -> Self {
        GitHubError::RequestError(err.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for GitHubError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        GitHubError::InvalidInput(format!("JWT error: {}", err))
    }
}

pub type Result<T> = std::result::Result<T, GitHubError>;
