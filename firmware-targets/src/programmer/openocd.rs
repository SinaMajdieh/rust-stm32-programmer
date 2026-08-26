//! OpenOCD-based firmware programming.
//!
//! This module translates [`ProgramRequest`] values into OpenOCD commands and
//! executes the configured OpenOCD binary.

use std::{path::PathBuf, process::Command, time::Instant};

use crate::programmer::{
    EraseMode, ProgramError, ProgramRequest, ProgramResult, Programmer, ResetMode,
};

/// Configuration for an OpenOCD programming backend.
///
/// The interface and target configuration files are passed to OpenOCD with
/// `-f` arguments. By default, the `openocd` executable is used.
#[derive(Debug, Clone)]
pub struct OpenOcdConfig {
    /// Path to the OpenOCD executable.
    pub executable: PathBuf,

    /// OpenOCD interface configuration file.
    pub interface: PathBuf,

    /// OpenOCD target configuration file.
    pub target: PathBuf,
}

impl OpenOcdConfig {
    /// Creates an OpenOCD configuration using the default `openocd`
    /// executable.
    ///
    /// `interface` and `target` identify the OpenOCD configuration files used
    /// to connect to and configure the target device.
    pub fn new(interface: impl Into<PathBuf>, target: impl Into<PathBuf>) -> Self {
        Self {
            executable: "openocd".into(),
            interface: interface.into(),
            target: target.into(),
        }
    }

    /// Sets the OpenOCD executable to use.
    ///
    /// This can be used when OpenOCD is not available through the system
    /// `PATH` or when a specific OpenOCD installation should be used.
    #[must_use]
    pub fn executable(mut self, executable: impl Into<PathBuf>) -> Self {
        self.executable = executable.into();
        self
    }
}

/// A firmware programmer backed by OpenOCD.
#[derive(Debug, Clone)]
pub struct OpenOcd {
    config: OpenOcdConfig,
}

impl OpenOcd {
    /// Creates an OpenOCD programmer with the specified configuration.
    pub fn new(config: OpenOcdConfig) -> Self {
        Self { config }
    }

    /// Translates a programming request into OpenOCD command strings.
    ///
    /// Commands are executed in the following order:
    ///
    /// 1. Initialize OpenOCD.
    /// 2. Initialize the target through a reset.
    /// 3. Write the firmware image, optionally erasing required sectors.
    /// 4. Verify the image when requested.
    /// 5. Reset the target to run when requested.
    /// 6. Shut down OpenOCD.
    fn build_commands(&self, request: &ProgramRequest) -> Vec<String> {
        let mut commands = vec!["init".to_owned(), "reset init".to_owned()];

        let image = request.firmware.display().to_string();

        let erase = match request.erase {
            EraseMode::None => "",
            EraseMode::Required => " erase",
        };

        commands.push(format!("flash write_image{} {}", erase, image));

        if request.verify {
            commands.push(format!("verify_image {}", image));
        }

        if request.reset == ResetMode::Run {
            commands.push("reset run".to_owned());
        }

        commands.push("shutdown".to_owned());

        commands
    }
}

impl Programmer for OpenOcd {
    fn program(&self, request: &ProgramRequest) -> Result<ProgramResult, ProgramError> {
        let firmware = &request.firmware;

        if !firmware.exists() {
            return Err(ProgramError::FirmwareNotFound(firmware.to_owned()));
        }

        let start = Instant::now();

        let mut command = Command::new(&self.config.executable);

        command
            .arg("-f")
            .arg(&self.config.interface)
            .arg("-f")
            .arg(&self.config.target);

        for command_string in self.build_commands(request) {
            command.arg("-c").arg(command_string);
        }

        let output = command
            .output()
            .map_err(|source| ProgramError::Spawn { source })?;

        if !output.status.success() {
            return Err(ProgramError::Failed {
                code: output.status.code(),
                stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            });
        }

        Ok(ProgramResult {
            firmware: firmware.to_owned(),
            verified: request.verify,
            elapsed: start.elapsed(),
        })
    }
}
