use std::path::PathBuf;

use generation::GenerationError;

/// Result type for operations spanning multiple core subsystems.
pub type Result<T> = std::result::Result<T, Error>;

/// The top-level error returned by the core library.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("generation error: {0}")]
    Generation(#[from] GenerationError),

    #[error("firmware error: {0}")]
    Firmware(#[from] FirmwareError),

    #[error("programming error: {0}")]
    Programming(#[from] ProgrammingError),
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
