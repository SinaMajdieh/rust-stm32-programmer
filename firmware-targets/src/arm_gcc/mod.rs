//! ARM GNU toolchain support for firmware projects.
//!
//! This module implements the build pipeline used to turn firmware source
//! files into deployable images using the ARM GNU toolchain.
//!
//! The pipeline consists of four stages:
//!
//! 1. [`compile`] compiles C and assembly sources into object files.
//! 2. [`link`] links the object files into an ELF image and linker map.
//! 3. [`objcopy`] converts the ELF image into Intel HEX and raw binary images.
//!
//! [`ArmGccConfig`] describes the toolchain and target-specific build
//! configuration, while [`ArmGcc`] coordinates the individual build stages.
//!
//! The module keeps toolchain-specific wire-up and process execution internal;
//! callers interact primarily with [`ArmGccConfig`], [`BuildArtifacts`],
//! [`BuildError`], and [`BuildStage`].

mod builder;
mod compile;
mod config;
mod link;
mod objcopy;

pub use builder::{BuildArtifacts, BuildError, BuildStage};
pub use config::ArmGccConfig;

pub(crate) use builder::ArmGcc;

use std::{path::Path, process::Command};

/// Executes an external build-tool command and converts unsuccessful exits
/// into [`BuildError::CommandFailed`].
///
/// Standard error is preferred as the diagnostic source. If the command does
/// not write anything to standard error, standard output is used instead.
/// This accommodates tools that report diagnostics through either stream.
fn run(command: &mut Command, stage: BuildStage, path: &Path) -> Result<(), BuildError> {
    let output = command.output()?;

    if output.status.success() {
        return Ok(());
    }

    let diagnostics = if output.stderr.is_empty() {
        String::from_utf8_lossy(&output.stdout).into_owned()
    } else {
        String::from_utf8_lossy(&output.stderr).into_owned()
    };

    Err(BuildError::CommandFailed {
        stage,
        path: path.to_path_buf(),
        status: output.status,
        diagnostics,
    })
}

#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;
