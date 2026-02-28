//! Local model runtime abstraction for pluggable LLM backends (Issue #4)
pub mod error;
pub mod model;
pub mod runtime;

pub use error::{Result, RuntimeError};
pub use model::ModelProfile;
pub use runtime::{LocalRuntime, Provider, ProviderConfig};
