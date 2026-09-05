//! Errors produced by the generation subsystem.

use std::time::Duration;

use crate::model::Provider;

/// An error produced while generating source code.
#[derive(Debug, thiserror::Error)]
pub enum GenerationError {
    #[error("Configuration error: {0}")]
    Config(#[from] GenerationConfigError),

    #[error("Prompt cannot be empty")]
    EmptyPrompt,

    #[error("Model cannot be empty")]
    EmptyModel,

    #[error("Provider `{0:?}` is disabled")]
    ProviderDisabled(Provider),

    /// The requested model is not registered.
    #[error("Model '{model}' is not available")]
    ModelNotFound {
        /// Name of the requested model.
        model: String,
    },

    /// A model is registered with a provider that the generator does not
    /// currently support.
    #[error("Provider '{provider}' is not configured")]
    ProviderUnavailable {
        /// Provider associated with the model.
        provider: String,
    },

    #[error("Ollama generation failed: {0}")]
    Ollama(#[from] ollama_client::Error),

    #[error("OpenAI generation failed: {0}")]
    OpenAI(#[from] OpenAIClientError),
}

/// An error produced by the OpenAI-compatible backend.
#[derive(Debug, thiserror::Error)]
pub enum OpenAIClientError {
    #[error("Failed to read OpenAI API key from environment variable `{0}`")]
    ApiKey(String),

    #[error("Seed {0} is invalid")]
    InvalidSeed(u64),

    #[error("OpenAI response contained no choices")]
    NoChoices,

    #[error("OpenAI request timed out after {0:?}")]
    Timeout(Duration),

    #[error("OpemAI client failed {0:?}")]
    Client(#[from] async_openai::error::OpenAIError),
}

/// Errors produced by configuration operations.
#[derive(Debug, thiserror::Error)]
pub enum GenerationConfigError {
    /// An I/O error occurred while reading or writing configuration.
    #[error("Configuration I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The configuration file contained invalid TOML.
    #[error("Invalid configuration TOML: {0}")]
    TomlDeserialize(#[from] toml::de::Error),

    /// Configuration could not be serialized to TOML.
    #[error("Failed to serialize configuration: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
}
