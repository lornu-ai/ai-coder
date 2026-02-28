use serde::{Deserialize, Serialize};

/// Configuration profile for a language model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelProfile {
    /// Model identifier (e.g., "qwen2.5-coder:7b")
    pub name: String,

    /// Context window size in tokens
    pub context_window: usize,

    /// Maximum tokens to generate
    pub max_tokens: usize,

    /// Sampling temperature (0.0 = deterministic, 1.0+ = creative)
    pub temperature: f32,

    /// Top-p nucleus sampling parameter
    pub top_p: f32,

    /// Top-k sampling parameter
    pub top_k: i32,

    /// Number of tokens to keep from chat history
    pub num_keep: i32,
}

impl ModelProfile {
    /// Create a new model profile
    pub fn new(name: impl Into<String>, context_window: usize, max_tokens: usize) -> Self {
        Self {
            name: name.into(),
            context_window,
            max_tokens,
            temperature: 0.7,
            top_p: 0.9,
            top_k: 40,
            num_keep: 4,
        }
    }

    /// Set temperature for sampling behavior
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature.clamp(0.0, 2.0);
        self
    }

    /// Set top_p for nucleus sampling
    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = top_p.clamp(0.0, 1.0);
        self
    }

    /// Validate that the profile is sensible
    pub fn validate(&self) -> crate::Result<()> {
        if self.name.is_empty() {
            return Err(crate::RuntimeError::InvalidConfig(
                "Model name cannot be empty".to_string(),
            ));
        }
        if self.context_window == 0 {
            return Err(crate::RuntimeError::InvalidConfig(
                "Context window must be > 0".to_string(),
            ));
        }
        if self.max_tokens == 0 {
            return Err(crate::RuntimeError::InvalidConfig(
                "Max tokens must be > 0".to_string(),
            ));
        }
        if self.max_tokens > self.context_window {
            return Err(crate::RuntimeError::InvalidConfig(
                "Max tokens cannot exceed context window".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_profile_creation() {
        let profile = ModelProfile::new("qwen2.5-coder:7b", 32768, 2048);
        assert_eq!(profile.name, "qwen2.5-coder:7b");
        assert_eq!(profile.context_window, 32768);
        assert_eq!(profile.max_tokens, 2048);
    }

    #[test]
    fn test_model_profile_defaults() {
        let profile = ModelProfile::new("test", 4096, 1024);
        assert_eq!(profile.temperature, 0.7);
        assert_eq!(profile.top_p, 0.9);
    }

    #[test]
    fn test_model_profile_with_temperature() {
        let profile = ModelProfile::new("test", 4096, 1024).with_temperature(1.5);
        assert_eq!(profile.temperature, 1.5);
    }

    #[test]
    fn test_model_profile_temperature_clamping() {
        let profile = ModelProfile::new("test", 4096, 1024).with_temperature(5.0);
        assert_eq!(profile.temperature, 2.0); // Clamped to max
    }

    #[test]
    fn test_model_profile_with_top_p() {
        let profile = ModelProfile::new("test", 4096, 1024).with_top_p(0.5);
        assert_eq!(profile.top_p, 0.5);
    }

    #[test]
    fn test_model_profile_top_p_clamping() {
        let profile = ModelProfile::new("test", 4096, 1024).with_top_p(1.5);
        assert_eq!(profile.top_p, 1.0); // Clamped to max
    }

    #[test]
    fn test_model_profile_validation_empty_name() {
        let profile = ModelProfile::new("", 4096, 1024);
        assert!(profile.validate().is_err());
    }

    #[test]
    fn test_model_profile_validation_zero_context() {
        let profile = ModelProfile::new("test", 0, 1024);
        assert!(profile.validate().is_err());
    }

    #[test]
    fn test_model_profile_validation_zero_max_tokens() {
        let profile = ModelProfile::new("test", 4096, 0);
        assert!(profile.validate().is_err());
    }

    #[test]
    fn test_model_profile_validation_max_exceeds_context() {
        let profile = ModelProfile::new("test", 1024, 2048);
        assert!(profile.validate().is_err());
    }

    #[test]
    fn test_model_profile_validation_valid() {
        let profile = ModelProfile::new("test", 4096, 1024);
        assert!(profile.validate().is_ok());
    }

    #[test]
    fn test_model_profile_builder_chain() {
        let profile = ModelProfile::new("test", 4096, 1024)
            .with_temperature(0.5)
            .with_top_p(0.8);
        assert_eq!(profile.temperature, 0.5);
        assert_eq!(profile.top_p, 0.8);
    }
}
