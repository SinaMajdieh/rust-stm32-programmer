use std::{
    fs,
    path::{Path, PathBuf},
};

use firmware_targets::{
    BuildArtifacts,
    programmer::{OpenOcd, ProgramResult},
    stm32f103c8::{Hal, ProjectTemplate, Target},
};

use crate::{FirmwareError, ProgrammingError};

/// Saves generated C source code as `main.c`.
pub fn save_source(project: impl AsRef<Path>, code: &str) -> Result<(), FirmwareError> {
    let directory = project.as_ref();

    fs::create_dir_all(directory)?;

    fs::write(directory.join("main.c"), code)?;

    Ok(())
}

/// Builds a generated firmware project.
pub fn build_project(project: impl AsRef<Path>) -> Result<BuildArtifacts, FirmwareError> {
    let directory = project.as_ref();
    let source_path = directory.join("main.c");

    let code = fs::read_to_string(&source_path)?;

    fs::remove_dir_all(directory)?;

    let mut project = Hal::generate(directory)?;

    project.add_source("main.c", &code)?;

    Ok(project.compile()?)
}

/// Programs a firmware binary using OpenOCD.
pub fn program(firmware: impl AsRef<Path>) -> Result<ProgramResult, ProgrammingError> {
    let firmware = firmware.as_ref();
    let target = Target::<OpenOcd>::default();

    target
        .program(firmware)
        .map_err(|source| ProgrammingError::Program {
            firmware: PathBuf::from(firmware),
            source,
        })
}
