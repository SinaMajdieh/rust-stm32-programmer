//! Persistent configuration for LLM generation.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{
    error::GenerationConfigError,
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

impl GeneratorConfig {
    /// Loads generator configuration from a TOML file.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, GenerationConfigError> {
        let contents = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&contents)?)
    }
}
