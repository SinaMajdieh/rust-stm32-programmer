//! Provider-agnostic LLM generation for source-code generation workflows.
//!
//! This crate provides the types and runtime coordination needed to generate
//! source code through supported LLM providers. It owns generation
//! configuration, model identity, requests, outputs, provider abstractions,
//! and the high-level [`LlmGenerator`] coordinator.
//!
//! The generation flow is:
//!
//! 1. A [`GenerationRequest`] identifies a model and provides the prompts.
//! 2. [`LlmGenerator`] resolves the model to its configured [`Provider`].
//! 3. The corresponding provider performs the provider-specific API request.
//! 4. The provider normalizes the response into a [`GenerationOutput`].
//!
//! Application-specific concerns such as CLI model selection and system-prompt
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
pub use error::{GenerationError, OpenAIClientError};
pub use generator::LlmGenerator;
pub use model::{Model, ModelId, Provider};
pub use output::{GenerationOutput, GenerationStatistics};
pub(crate) use provider::GenerationProvider;
pub use providers::{
    OllamaClientConfig, OllamaConfig, OllamaGenerationOptions, OllamaProvider, OpenAiClientConfig,
    OpenAiConfig, OpenAiGenerationOptions, OpenAiProvider,
};
pub use request::GenerationRequest;
