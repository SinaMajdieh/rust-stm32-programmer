//! Persistent configuration for LLM generation.
use serde::{Deserialize, Serialize};

use crate::{
    model::Model,
    providers::{OllamaConfig, OpenAiConfig},
};

/// Configuration consumed by [`crate::LlmGenerator`].
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeneratorConfig {
    /// Models available to the application.
    #[serde(default)]
    pub available_models: Vec<Model>,

    /// Ollama configuration.
    #[serde(default)]
    pub ollama: OllamaConfig,

    /// OpenAI configuration.
    #[serde(default)]
    pub openai: OpenAiConfig,
}
