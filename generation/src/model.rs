//! Model identity and provider metadata.

use std::fmt;

use serde::{Deserialize, Serialize};

/// LLM backend supported by the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    /// Local Ollama server.
    #[default]
    Ollama,

    /// OpenAI-compatible API.
    OpenAi,
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ollama => f.write_str("ollama"),
            Self::OpenAi => f.write_str("openai"),
        }
    }
}

/// Fully qualified model identifier such as `ollama:qwen2.5:7b`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct ModelId(String);

impl ModelId {
    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&Model> for ModelId {
    fn from(model: &Model) -> Self {
        Self(format!("{}:{}", model.provider, model.name))
    }
}

impl From<&str> for ModelId {
    fn from(id: &str) -> Self {
        Self(id.to_owned())
    }
}

impl From<String> for ModelId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl fmt::Display for ModelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A model available to the application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    /// Provider-specific model name.
    pub name: String,

    /// Provider hosting the model.
    pub provider: Provider,
}

impl Model {
    /// Creates a new model descriptor.
    pub fn new(name: impl Into<String>, provider: Provider) -> Self {
        Self {
            name: name.into(),
            provider,
        }
    }

    /// Returns the model's fully qualified identifier.
    pub fn id(&self) -> ModelId {
        self.into()
    }
}
