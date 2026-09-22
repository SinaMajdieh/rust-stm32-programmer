use crate::CompileError;

/// An error produced while creating or building a firmware project.
///
/// `FirmwareError` combines errors from project file operations, target
/// selection, and the underlying firmware build process.
#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    /// An error occurred while accessing firmware project files.
    #[error("failed to access firmware project files")]
    Io(#[from] std::io::Error),

    /// An error occurred while building the firmware project.
    #[error("firmware project operation failed: {0}")]
    Compile(#[from] CompileError),
}
