use crate::{ModelProfile, Result, RuntimeError};
use async_trait::async_trait;
use futures_util::Stream;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

#[cfg(test)]
use futures_util::StreamExt;

/// Stream of response tokens from a provider
pub type ResponseStream = Pin<Box<dyn Stream<Item = Result<String>> + Send>>;

/// Configuration for a provider backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Provider type (e.g., "ollama", "llama-cpp")
    pub provider: String,

    /// Base URL/endpoint for the provider
    pub endpoint: String,

    /// Request timeout duration
    pub timeout_secs: u64,

    /// Maximum retries on transient failures
    pub max_retries: u32,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            provider: "ollama".to_string(),
            endpoint: "http://localhost:11434".to_string(),
            timeout_secs: 300,
            max_retries: 3,
        }
    }
}

/// Request to generate completions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionRequest {
    /// The input prompt
    pub prompt: String,

    /// Whether to stream the response
    pub stream: bool,

    /// Model profile to use
    pub model: ModelProfile,
}

impl CompletionRequest {
    /// Create a new completion request
    pub fn new(prompt: impl Into<String>, model: ModelProfile) -> Self {
        Self {
            prompt: prompt.into(),
            stream: true,
            model,
        }
    }

    /// Validate the request against the model's constraints
    pub fn validate(&self) -> Result<()> {
        self.model.validate()?;

        // Estimate token count (rough: ~4 chars per token)
        let estimated_tokens = (self.prompt.len() / 4) + self.model.max_tokens;
        if estimated_tokens > self.model.context_window {
            return Err(RuntimeError::ContextOverflow {
                model: self.model.name.clone(),
                tokens: estimated_tokens,
                max_tokens: self.model.context_window,
            });
        }

        Ok(())
    }
}

/// Response from a provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    /// The generated text
    pub text: String,

    /// Whether the generation is complete
    pub done: bool,

    /// Metadata about the generation
    pub metadata: ResponseMetadata,
}

/// Metadata about a completion response
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseMetadata {
    /// Estimated tokens in the prompt
    pub prompt_tokens: Option<usize>,

    /// Tokens generated
    pub completion_tokens: Option<usize>,

    /// Model that generated the response
    pub model: Option<String>,
}

/// Provider abstraction trait for pluggable LLM backends
#[async_trait]
pub trait Provider: Send + Sync {
    /// Get the provider name (e.g., "ollama", "llama-cpp")
    fn name(&self) -> &str;

    /// Check if a model is available
    async fn has_model(&self, model: &str) -> Result<bool>;

    /// Generate a completion - streaming response
    async fn generate_stream(&self, request: CompletionRequest) -> Result<ResponseStream>;

    /// Generate a completion - buffered response
    async fn generate(&self, request: CompletionRequest) -> Result<CompletionResponse>;
}

/// Local runtime managing provider interactions
pub struct LocalRuntime {
    config: ProviderConfig,
    provider: Box<dyn Provider>,
}

impl LocalRuntime {
    /// Create a new local runtime with a provider
    pub fn new(config: ProviderConfig, provider: Box<dyn Provider>) -> Result<Self> {
        Ok(Self { config, provider })
    }

    /// Get configuration
    pub fn config(&self) -> &ProviderConfig {
        &self.config
    }

    /// Get provider instance
    pub fn provider(&self) -> &dyn Provider {
        &*self.provider
    }

    /// Generate text with a streaming response
    pub async fn generate_stream(
        &self,
        prompt: impl Into<String>,
        model: ModelProfile,
    ) -> Result<ResponseStream> {
        model.validate()?;

        let request = CompletionRequest::new(prompt, model);
        request.validate()?;

        self.provider.generate_stream(request).await
    }

    /// Generate text with a buffered response
    pub async fn generate(
        &self,
        prompt: impl Into<String>,
        model: ModelProfile,
    ) -> Result<CompletionResponse> {
        model.validate()?;

        let request = CompletionRequest::new(prompt, model);
        request.validate()?;

        self.provider.generate(request).await
    }
}

// Mock provider for testing
#[cfg(test)]
pub struct MockProvider {
    models: std::collections::HashSet<String>,
    response: String,
    should_error: bool,
}

#[cfg(test)]
impl MockProvider {
    pub fn new(response: impl Into<String>) -> Self {
        let mut models = std::collections::HashSet::new();
        models.insert("test-model".to_string());

        Self {
            models,
            response: response.into(),
            should_error: false,
        }
    }

    pub fn with_models(mut self, models: Vec<String>) -> Self {
        self.models = models.into_iter().collect();
        self
    }

    pub fn with_error(mut self) -> Self {
        self.should_error = true;
        self
    }
}

#[cfg(test)]
#[async_trait]
impl Provider for MockProvider {
    fn name(&self) -> &str {
        "mock"
    }

    async fn has_model(&self, model: &str) -> Result<bool> {
        if self.should_error {
            return Err(RuntimeError::ConnectionError("mock error".to_string()));
        }
        Ok(self.models.contains(model))
    }

    async fn generate_stream(&self, _request: CompletionRequest) -> Result<ResponseStream> {
        if self.should_error {
            return Err(RuntimeError::ProviderError("mock error".to_string()));
        }

        let tokens: Vec<_> = self
            .response
            .split_whitespace()
            .map(|w| Ok(w.to_string() + " "))
            .collect();
        let stream = futures_util::stream::iter(tokens).boxed();
        Ok(stream)
    }

    async fn generate(&self, _request: CompletionRequest) -> Result<CompletionResponse> {
        if self.should_error {
            return Err(RuntimeError::ProviderError("mock error".to_string()));
        }

        Ok(CompletionResponse {
            text: self.response.clone(),
            done: true,
            metadata: ResponseMetadata::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_config_defaults() {
        let config = ProviderConfig::default();
        assert_eq!(config.provider, "ollama");
        assert_eq!(config.timeout_secs, 300);
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_completion_request_creation() {
        let model = ModelProfile::new("test", 4096, 1024);
        let request = CompletionRequest::new("hello", model);
        assert_eq!(request.prompt, "hello");
        assert!(request.stream);
    }

    #[test]
    fn test_completion_request_validation_valid() {
        let model = ModelProfile::new("test", 4096, 1024);
        let request = CompletionRequest::new("hello", model);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_completion_request_validation_context_overflow() {
        let model = ModelProfile::new("test", 100, 50);
        let long_prompt = "x".repeat(300); // Will exceed context when combined with max_tokens
        let request = CompletionRequest::new(long_prompt, model);
        assert!(request.validate().is_err());
    }

    #[tokio::test]
    async fn test_mock_provider_has_model() {
        let provider = MockProvider::new("test response");
        assert!(provider.has_model("test-model").await.unwrap());
        assert!(!provider.has_model("missing-model").await.unwrap());
    }

    #[tokio::test]
    async fn test_mock_provider_generate() {
        let provider = MockProvider::new("hello world");
        let model = ModelProfile::new("test", 4096, 1024);
        let response = provider
            .generate(CompletionRequest::new("test", model))
            .await
            .unwrap();
        assert_eq!(response.text, "hello world");
        assert!(response.done);
    }

    #[tokio::test]
    async fn test_local_runtime_creation() {
        let config = ProviderConfig::default();
        let provider = Box::new(MockProvider::new("test"));
        let runtime = LocalRuntime::new(config.clone(), provider).unwrap();
        assert_eq!(runtime.config().endpoint, config.endpoint);
    }

    #[tokio::test]
    async fn test_local_runtime_generate() {
        let config = ProviderConfig::default();
        let provider = Box::new(MockProvider::new("hello from runtime"));
        let runtime = LocalRuntime::new(config, provider).unwrap();

        let model = ModelProfile::new("test", 4096, 1024);
        let response = runtime.generate("test prompt", model).await.unwrap();
        assert_eq!(response.text, "hello from runtime");
    }

    #[tokio::test]
    async fn test_local_runtime_model_validation() {
        let config = ProviderConfig::default();
        let provider = Box::new(MockProvider::new("test"));
        let runtime = LocalRuntime::new(config, provider).unwrap();

        let invalid_model = ModelProfile::new("", 4096, 1024); // Empty name
        let result = runtime.generate("test", invalid_model).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mock_provider_error_handling() {
        let provider = MockProvider::new("test").with_error();
        let model = ModelProfile::new("test", 4096, 1024);
        let result = provider
            .generate(CompletionRequest::new("test", model))
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mock_provider_custom_models() {
        let provider = MockProvider::new("response").with_models(vec![
            "custom-model".to_string(),
            "another-model".to_string(),
        ]);
        assert!(provider.has_model("custom-model").await.unwrap());
        assert!(provider.has_model("another-model").await.unwrap());
        assert!(!provider.has_model("test-model").await.unwrap());
    }
}
