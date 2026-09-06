use crate::BuildError;

/// An error produced while creating or building a firmware project.
///
/// `FirmwareError` combines errors from project file operations, target
/// selection, and the underlying firmware build process.
#[derive(Debug, thiserror::Error)]
pub enum FirmwareError {
    /// An error occurred while accessing firmware project files.
    #[error("failed to access firmware project files")]
    Io(#[from] std::io::Error),

    /// The requested target is not supported.
    #[error("target is not supported")]
    TargetNotSupported,

    /// An error occurred while building the firmware project.
    #[error("firmware project operation failed: {0}")]
    Build(#[from] BuildError),
}
