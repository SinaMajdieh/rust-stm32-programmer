use std::path::PathBuf;

use firmware_targets::FirmwareError;
use generation::GenerationError;

/// Result type for operations spanning multiple core subsystems.
pub type Result<T> = std::result::Result<T, Error>;

/// The top-level error returned by the core library.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("Generation failed: {0}")]
    Generation(#[from] GenerationError),

    #[error("Firmware error: {0}")]
    Firmware(#[from] FirmwareError),

    #[error("Programming error: {0}")]
    Programming(#[from] ProgrammingError),
}

/// An error produced while programming firmware.
#[derive(Debug, thiserror::Error)]
pub enum ProgrammingError {
    #[error("Failed to program firmware `{firmware}`\n{source:#?}")]
    Program {
        firmware: PathBuf,
        #[source]
        source: firmware_targets::programmer::ProgramError,
    },
}

/// Errors produced by configuration operations.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
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
