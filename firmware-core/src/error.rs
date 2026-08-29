use std::path::PathBuf;

/// Result type for operations spanning multiple core subsystems.
pub type Result<T> = std::result::Result<T, Error>;

/// The top-level error returned by the core library.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("generation error: {0}")]
    Generation(#[from] GenerationError),

    #[error("firmware error: {0}")]
    Firmware(#[from] FirmwareError),

    #[error("programming error: {0}")]
    Programming(#[from] ProgrammingError),
}

/// An error produced while loading application configuration.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("failed to read configuration file `{path}`")]
    ReadConfig {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to parse configuration file `{path}`")]
    ParseConfig {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },

    #[error("failed to read system prompt `{path}`")]
    ReadSystemPrompt {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

/// An error produced while generating firmware source code.
#[derive(Debug, thiserror::Error)]
pub enum GenerationError {
    #[error("prompt cannot be empty")]
    EmptyPrompt,

    #[error("Ollama generation failed: {0}")]
    Ollama(#[from] ollama_client::Error),

    #[error("OpenAI generation failed: {0}")]
    OpenAI(#[from] OpenAIClientError),
}

/// An error produced by the OpenAI-compatible backend.
#[derive(Debug, thiserror::Error)]
pub enum OpenAIClientError {
    #[error("failed to read OpenAI API key from environment variable `{0}`")]
    ApiKey(String),

    #[error("OpenAI response contained no choices")]
    NoChoices,

    #[error(transparent)]
    Client(#[from] async_openai::error::OpenAIError),
}

/// An error produced while creating or building firmware.
#[derive(Debug, thiserror::Error)]
pub enum FirmwareError {
    #[error("failed to access firmware project files")]
    Io(#[from] std::io::Error),

    #[error("firmware project operation failed: {0}")]
    Build(#[from] firmware_targets::BuildError),
}

/// An error produced while programming firmware.
#[derive(Debug, thiserror::Error)]
pub enum ProgrammingError {
    #[error("failed to program firmware `{firmware}`")]
    Program {
        firmware: PathBuf,
        #[source]
        source: firmware_targets::programmer::ProgramError,
    },
}
