use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

const CONFIG_PATH: &str = "config.toml";

/// Application configuration.
#[derive(Debug)]
pub struct Config {
    /// Configuration specific to Ollama.
    pub ollama: OllamaConfig,

    /// Configuration specific to OpenAI-compatible APIs.
    pub openai: OpenAiConfig,

    /// Settings shared by all LLM providers.
    pub generation: GenerationConfig,

    /// Loaded system prompt shared by all providers.
    pub system_prompt: String,
}

/// Configuration for the Ollama provider.
#[derive(Debug, Deserialize)]
pub struct OllamaConfig {
    /// Base URL of the Ollama server.
    pub url: String,

    /// Duration for which the model remains loaded.
    pub keep_alive: String,

    /// Maximum time allowed for a generation request.
    pub timeout_seconds: u64,

    /// Maximum context length for Ollama.
    pub context_length: u32,
}

/// Configuration for an OpenAI-compatible provider.
#[derive(Debug, Deserialize)]
pub struct OpenAiConfig {
    /// Base URL of the OpenAI-compatible API.
    pub url: String,

    /// Environment variable containing the API key.
    pub api_key_env: String,
}

/// Settings shared between LLM providers.
#[derive(Debug, Deserialize)]
pub struct GenerationConfig {
    /// Path to the system prompt.
    pub system_prompt_path: PathBuf,

    /// Seed used for deterministic generation.
    pub seed: u64,

    /// Sampling temperature.
    pub temperature: f32,

    /// Maximum number of generated tokens.
    pub max_output_tokens: u32,
}

impl Config {
    /// Loads the configuration from `config.toml`.
    ///
    /// The system prompt is loaded from the path specified by
    /// [`GenerationConfig::system_prompt_path`].
    pub fn load() -> Result<Self> {
        let contents = fs::read_to_string(CONFIG_PATH)
            .with_context(|| format!("failed to read {CONFIG_PATH}"))?;

        let raw: RawConfig = toml::from_str(&contents).context("failed to parse config.toml")?;

        let system_prompt =
            fs::read_to_string(&raw.generation.system_prompt_path).with_context(|| {
                format!(
                    "failed to read system prompt: {}",
                    raw.generation.system_prompt_path.display()
                )
            })?;

        Ok(Self {
            ollama: raw.ollama,
            openai: raw.openai,
            generation: raw.generation,
            system_prompt,
        })
    }
}

/// Deserialized representation of [`Config`].
#[derive(Debug, Deserialize)]
struct RawConfig {
    ollama: OllamaConfig,
    openai: OpenAiConfig,
    generation: GenerationConfig,
}
