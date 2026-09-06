//! Errors produced by the generation subsystem.

use std::time::Duration;

use crate::model::Provider;

/// An error produced while generating source code.
///
/// This error represents failures encountered while validating a request,
/// resolving its model and provider, or communicating with the selected
/// generation backend.
#[derive(Debug, thiserror::Error)]
pub enum GenerationError {
    /// The generation prompt is empty or contains only whitespace.
    #[error("Prompt cannot be empty")]
    EmptyPrompt,

    /// The requested model identifier is empty or contains only whitespace.
    #[error("Model cannot be empty")]
    EmptyModel,

    /// The requested provider has been disabled.
    #[error("Provider `{0:?}` is disabled")]
    ProviderDisabled(Provider),

    /// The requested model is not registered with the generator.
    #[error("Model '{model}' is not available")]
    ModelNotFound {
        /// Name of the requested model.
        model: String,
    },

    /// The model is registered, but its provider is not configured.
    #[error("Provider '{provider}' is not configured")]
    ProviderUnavailable {
        /// Provider associated with the model.
        provider: String,
    },

    /// Generation failed while communicating with Ollama.
    #[error("Ollama generation failed: {0}")]
    Ollama(#[from] ollama_client::Error),

    /// Generation failed while communicating with the OpenAI-compatible
    /// backend.
    #[error("OpenAI generation failed: {0}")]
    OpenAI(#[from] OpenAIClientError),
}

/// An error produced by the OpenAI-compatible backend.
#[derive(Debug, thiserror::Error)]
pub enum OpenAIClientError {
    /// The OpenAI API key could not be read from the required environment
    /// variable.
    #[error("Failed to read OpenAI API key from environment variable `{0}`")]
    ApiKey(String),

    /// The configured seed value is not accepted by the client.
    #[error("Seed {0} is invalid")]
    InvalidSeed(u64),

    /// The API response did not contain any generated choices.
    #[error("OpenAI response contained no choices")]
    NoChoices,

    /// The request exceeded the configured timeout.
    #[error("OpenAI request timed out after {0:?}")]
    Timeout(Duration),

    /// The underlying OpenAI client returned an error.
    #[error("OpenAI client failed: {0:?}")]
    Client(#[from] async_openai::error::OpenAIError),
}
