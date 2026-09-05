//! Provider-agnostic LLM generation for source-code generation workflows.
//!
//! This crate owns generation configuration, model identity, providers,
//! requests, outputs, and the runtime generation coordinator.
//!
//! Application-specific concerns such as CLI model selection or system-prompt
//! file locations remain outside this crate.

mod code;
mod config;
mod error;
mod generator;
mod model;
mod output;
mod provider;
mod providers;
mod request;

pub use config::GeneratorConfig;
pub use error::{GenerationConfigError, GenerationError, OpenAIClientError};
pub use generator::LlmGenerator;
pub use model::{Model, ModelId, Provider};
pub use output::{GenerationOutput, GenerationStatistics};
pub(crate) use provider::GenerationProvider;
pub use providers::{
    OllamaClientConfig, OllamaConfig, OllamaGenerationOptions, OllamaProvider, OpenAiClientConfig,
    OpenAiConfig, OpenAiGenerationOptions, OpenAiProvider,
};
pub use request::GenerationRequest;
