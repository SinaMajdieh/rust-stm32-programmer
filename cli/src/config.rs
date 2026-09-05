use std::{fs, path::Path};

use backend::{GenerationConfigError, GeneratorConfig, ModelId};
use serde::Deserialize;

/// Application configuration for the firmware CLI.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// LLM configuration.
    pub llm: LlmConfig,
}

impl Config {
    /// Loads the application configuration from a TOML file.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, GenerationConfigError> {
        let contents = fs::read_to_string(path)?;
        Ok(toml::from_str(&contents)?)
    }
}

/// CLI-specific LLM configuration.
///
/// `GeneratorConfig` contains the generation backend configuration,
/// while these fields control which model and system prompt the CLI uses.
#[derive(Debug, Deserialize)]
pub struct LlmConfig {
    /// Model used when no model is specified on the command line.
    pub selected_model: ModelId,

    /// Path to the system prompt file.
    pub system_prompt_path: String,

    /// Generator configuration.
    #[serde(flatten)]
    pub generator: GeneratorConfig,
}

impl LlmConfig {
    /// Loads the configured system prompt.
    pub fn system_prompt(&self) -> Result<String, GenerationConfigError> {
        Ok(fs::read_to_string(&self.system_prompt_path)?)
    }
}
