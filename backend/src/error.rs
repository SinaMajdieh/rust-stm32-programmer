use std::path::PathBuf;

use firmware_targets::FirmwareError;
use generation::GenerationError;

/// Result type for operations spanning multiple core subsystems.
pub type Result<T> = std::result::Result<T, Error>;

/// The top-level error returned by the core library.
///
/// `Error` combines failures from configuration, LLM generation, firmware
/// operations, and programming into a single error type suitable for
/// higher-level applications such as the CLI.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Configuration loading or serialization failed.
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    /// LLM source-code generation failed.
    #[error("Generation failed: {0}")]
    Generation(#[from] GenerationError),

    /// A firmware project operation failed.
    #[error("Firmware error: {0}")]
    Firmware(#[from] FirmwareError),

    /// Programming a firmware image failed.
    #[error("Programming error: {0}")]
    Programming(#[from] ProgrammingError),
}

/// An error produced while programming firmware.
///
/// The error retains the firmware path along with the underlying programming
/// error so callers can identify which firmware image failed to be programmed.
#[derive(Debug, thiserror::Error)]
pub enum ProgrammingError {
    /// The target programmer failed to program the specified firmware.
    #[error("Failed to program firmware `{firmware}`\n{source:#?}")]
    Program {
        /// Path to the firmware image that could not be programmed.
        firmware: PathBuf,

        /// Underlying programming error.
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
