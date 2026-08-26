//! Firmware programming support.
//!
//! This module provides an abstraction over firmware programmers together with
//! a request and result type for programming operations.
//!
//! [`Programmer`] defines the interface implemented by concrete programming
//! backends. [`OpenOcd`] provides an implementation backed by the OpenOCD
//! command-line tool.
//!
//! A programming operation is configured with [`ProgramRequest`], which
//! controls flash erasure, image verification, and target reset behavior.
//! Successful operations return a [`ProgramResult`], while failures are
//! reported as [`ProgramError`].

use std::{io, path::PathBuf, time::Duration};

mod openocd;

pub use openocd::{OpenOcd, OpenOcdConfig};

/// Controls whether flash memory is explicitly erased before programming.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EraseMode {
    /// Do not explicitly erase flash before programming.
    None,

    /// Erase the flash sectors required by the firmware image.
    #[default]
    Required,
}

/// Controls the target reset behavior after programming.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResetMode {
    /// Do not reset the target after programming.
    None,

    /// Reset the target and begin execution.
    #[default]
    Run,
}

/// Describes a firmware programming operation.
///
/// A `ProgramRequest` specifies the firmware image to program and controls
/// whether flash should be erased, whether the written image should be
/// verified, and whether the target should be reset after programming.
///
/// Use [`ProgramRequest::new`] to create a request with the default options.
#[derive(Debug, Clone)]
pub struct ProgramRequest {
    /// Path to the firmware image to program.
    pub firmware: PathBuf,

    /// Controls flash erasure before programming.
    pub erase: EraseMode,

    /// Whether the programmed firmware should be verified.
    pub verify: bool,

    /// Controls target reset behavior after programming.
    pub reset: ResetMode,
}

impl ProgramRequest {
    /// Creates a programming request for the specified firmware image.
    ///
    /// The default configuration erases the sectors required by the firmware,
    /// verifies the programmed image, and resets the target to begin
    /// execution.
    pub fn new(firmware: impl Into<PathBuf>) -> Self {
        Self {
            firmware: firmware.into(),
            erase: EraseMode::default(),
            verify: true,
            reset: ResetMode::default(),
        }
    }

    /// Sets the flash erase behavior.
    #[must_use]
    pub fn erase(mut self, mode: EraseMode) -> Self {
        self.erase = mode;
        self
    }

    /// Enables or disables verification of the programmed firmware.
    #[must_use]
    pub fn verify(mut self, verify: bool) -> Self {
        self.verify = verify;
        self
    }

    /// Sets the target reset behavior after programming.
    #[must_use]
    pub fn reset(mut self, mode: ResetMode) -> Self {
        self.reset = mode;
        self
    }
}

/// The result of a successful firmware programming operation.
#[derive(Debug)]
pub struct ProgramResult {
    /// Path to the firmware image that was programmed.
    pub firmware: PathBuf,

    /// Whether firmware verification was requested and completed.
    pub verified: bool,

    /// Time elapsed while programming the firmware.
    pub elapsed: Duration,
}

/// A backend capable of programming firmware onto a target device.
///
/// Implementations are responsible for translating a [`ProgramRequest`] into
/// the commands required by their underlying programming tool.
pub trait Programmer {
    /// Programs the firmware described by `request`.
    ///
    /// # Errors
    ///
    /// Returns an implementation-specific [`ProgramError`] if the firmware
    /// cannot be found, the programmer cannot be started, or the programming
    /// operation fails.
    fn program(&self, request: &ProgramRequest) -> Result<ProgramResult>;
}

/// An error produced while programming firmware.
#[derive(Debug, thiserror::Error)]
pub enum ProgramError {
    /// The requested firmware image does not exist.
    #[error("firmware image does not exist: {}", .0.display())]
    FirmwareNotFound(PathBuf),

    /// The programming tool could not be started.
    #[error("failed to start programmer")]
    Spawn {
        #[source]
        source: io::Error,
    },

    /// The programming tool exited unsuccessfully.
    ///
    /// The captured standard output and standard error are retained to aid
    /// diagnosis of programming failures.
    #[error("programming failed with exit code {code:?}:\n{stdout}\n{stderr}")]
    Failed {
        /// The process exit code, if one was provided by the operating system.
        code: Option<i32>,

        /// Output written to standard output by the programmer.
        stdout: String,

        /// Diagnostics written to standard error by the programmer.
        stderr: String,
    },
}

/// The result type used by the programming API.
pub type Result<T> = std::result::Result<T, ProgramError>;
