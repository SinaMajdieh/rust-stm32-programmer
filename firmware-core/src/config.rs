use std::{fs, path::PathBuf};

use serde::Deserialize;

use crate::ConfigError;

const CONFIG_PATH: &str = "config.toml";

/// Application configuration loaded from disk.
#[derive(Debug)]
pub struct Config {
    pub ollama: OllamaConfig,
    pub openai: OpenAiConfig,
    pub generation: GenerationConfig,
    pub system_prompt: String,
}

/// Configuration for the Ollama backend.
#[derive(Debug, Deserialize)]
pub struct OllamaConfig {
    pub url: String,
    pub keep_alive: String,
    pub timeout_seconds: u64,
    pub context_length: u32,
}

/// Configuration for an OpenAI-compatible backend.
#[derive(Debug, Deserialize)]
pub struct OpenAiConfig {
    pub url: String,
    pub api_key_env: String,
}

/// Settings shared by generation backends.
#[derive(Debug, Deserialize)]
pub struct GenerationConfig {
    pub system_prompt_path: PathBuf,
    pub seed: u64,
    pub temperature: f32,
    pub max_output_tokens: u32,
}

impl Config {
    /// Loads the application configuration from `config.toml`.
    pub fn load() -> Result<Self, ConfigError> {
        let contents =
            fs::read_to_string(CONFIG_PATH).map_err(|source| ConfigError::ReadConfig {
                path: CONFIG_PATH.into(),
                source,
            })?;

        let raw: RawConfig =
            toml::from_str(&contents).map_err(|source| ConfigError::ParseConfig {
                path: CONFIG_PATH.into(),
                source,
            })?;

        let prompt_path = &raw.generation.system_prompt_path;
        let system_prompt =
            fs::read_to_string(prompt_path).map_err(|source| ConfigError::ReadSystemPrompt {
                path: prompt_path.clone(),
                source,
            })?;

        Ok(Self {
            ollama: raw.ollama,
            openai: raw.openai,
            generation: raw.generation,
            system_prompt,
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawConfig {
    ollama: OllamaConfig,
    openai: OpenAiConfig,
    generation: GenerationConfig,
}
