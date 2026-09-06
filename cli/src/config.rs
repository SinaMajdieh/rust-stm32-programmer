//! Application configuration loaded by the firmware CLI.
//!
//! The configuration combines CLI-specific settings with the backend's
//! [`GeneratorConfig`]. It defines the default model, system prompt,
//! firmware target, and project template used when corresponding command-line
//! options are not provided.

use std::{fs, path::Path};

use backend::{ConfigError, GeneratorConfig, ModelId};
use firmware_targets::{TargetKind, TemplateKind};
use serde::Deserialize;

/// Application configuration for the firmware CLI.
///
/// This configuration provides defaults used by CLI commands. Individual
/// command-line options may override selected values such as the model,
/// target, or project template.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// LLM configuration.
    pub llm: LlmConfig,

    /// Firmware configuration.
    pub firmware: FirmwareConfig,
}

impl Config {
    /// Loads the application configuration from a TOML file.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] if the file cannot be read or its contents
    /// cannot be deserialized as [`Config`].
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let contents = fs::read_to_string(path)?;
        Ok(toml::from_str(&contents)?)
    }
}

/// CLI-specific LLM configuration.
///
/// [`GeneratorConfig`] contains the generation backend configuration,
/// while these fields control which model and system prompt the CLI uses.
#[derive(Debug, Deserialize)]
pub struct LlmConfig {
    /// Model used when no model is specified on the command line.
    pub selected_model: ModelId,

    /// Path to the system prompt file.
    pub system_prompt_path: String,

    /// Generator backend configuration.
    #[serde(flatten)]
    pub generator: GeneratorConfig,
}

impl LlmConfig {
    /// Loads the configured system prompt from disk.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] if the configured file cannot be read.
    pub fn system_prompt(&self) -> Result<String, ConfigError> {
        Ok(fs::read_to_string(&self.system_prompt_path)?)
    }
}

/// Firmware configuration used by the CLI.
///
/// The selected target and project template act as defaults for commands that
/// do not provide command-line overrides.
#[derive(Debug, Deserialize)]
pub struct FirmwareConfig {
    /// Currently selected target device.
    pub selected_target: TargetKind,

    /// Firmware generation configuration.
    pub generation: FirmwareGenerationConfig,
}

/// Firmware generation configuration.
#[derive(Debug, Deserialize)]
pub struct FirmwareGenerationConfig {
    /// Currently selected project template.
    pub selected_template: TemplateKind,
}
