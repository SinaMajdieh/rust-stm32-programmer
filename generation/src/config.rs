//! Persistent configuration for LLM generation.
//!
//! [`GeneratorConfig`] contains the models exposed to the application and the
//! configuration required by each supported generation provider.

use serde::{Deserialize, Serialize};

use crate::{
    model::Model,
    providers::{OllamaConfig, OpenAiConfig},
};

/// Configuration consumed by [`crate::LlmGenerator`].
///
/// The configuration describes which models are available and how the
/// corresponding providers should be initialized.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeneratorConfig {
    /// Models available to the application.
    #[serde(default)]
    pub available_models: Vec<Model>,

    /// Configuration for the Ollama provider.
    #[serde(default)]
    pub ollama: OllamaConfig,

    /// Configuration for the OpenAI provider.
    #[serde(default)]
    pub openai: OpenAiConfig,
}
