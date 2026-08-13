//! ARM GCC build support.

mod builder;
mod compile;
mod config;
mod link;
mod objcopy;

pub use builder::{BuildArtifacts, BuildError, BuildStage};
pub use config::ArmGccConfig;

pub(crate) use builder::ArmGcc;

use std::{path::Path, process::Command};

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
