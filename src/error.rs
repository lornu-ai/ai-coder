use std::fmt;

/// Result type for runtime operations
pub type Result<T> = std::result::Result<T, RuntimeError>;

/// Errors that can occur during LLM runtime operations
#[derive(Debug, Clone)]
pub enum RuntimeError {
    /// Model was not found or not loaded in the provider
    ModelNotFound(String),
    /// Context window exceeded for the model
    ContextOverflow {
        model: String,
        tokens: usize,
        max_tokens: usize,
    },
    /// Request timeout
    Timeout { model: String, duration_secs: u64 },
    /// Connection error to the provider
    ConnectionError(String),
    /// Provider returned an error
    ProviderError(String),
    /// Invalid configuration
    InvalidConfig(String),
    /// I/O or serialization error
    IoError(String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::ModelNotFound(model) => write!(f, "Model not found: {}", model),
            RuntimeError::ContextOverflow {
                model,
                tokens,
                max_tokens,
            } => write!(
                f,
                "Context overflow for model {}: {} tokens > {} max",
                model, tokens, max_tokens
            ),
            RuntimeError::Timeout {
                model,
                duration_secs,
            } => write!(
                f,
                "Timeout for model {} after {} seconds",
                model, duration_secs
            ),
            RuntimeError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            RuntimeError::ProviderError(msg) => write!(f, "Provider error: {}", msg),
            RuntimeError::InvalidConfig(msg) => write!(f, "Invalid config: {}", msg),
            RuntimeError::IoError(msg) => write!(f, "I/O error: {}", msg),
        }
    }
}

impl std::error::Error for RuntimeError {}

impl From<std::io::Error> for RuntimeError {
    fn from(err: std::io::Error) -> Self {
        RuntimeError::IoError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_model_not_found() {
        let err = RuntimeError::ModelNotFound("gpt-4".to_string());
        assert_eq!(err.to_string(), "Model not found: gpt-4");
    }

    #[test]
    fn test_error_display_context_overflow() {
        let err = RuntimeError::ContextOverflow {
            model: "bert".to_string(),
            tokens: 2048,
            max_tokens: 1024,
        };
        assert!(err.to_string().contains("Context overflow"));
    }

    #[test]
    fn test_error_display_timeout() {
        let err = RuntimeError::Timeout {
            model: "llama".to_string(),
            duration_secs: 300,
        };
        assert!(err.to_string().contains("Timeout"));
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let runtime_err = RuntimeError::from(io_err);
        assert!(matches!(runtime_err, RuntimeError::IoError(_)));
    }
}
