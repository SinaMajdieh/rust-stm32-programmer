use std::{
    fs,
    path::{Path, PathBuf},
};

use firmware_targets::{
    BuildArtifacts, FirmwareError, Target, TemplateKind, programmer::ProgramResult,
};

use crate::ProgrammingError;

/// Saves generated C source code as `main.c` in the project directory.
///
/// The project directory is created if it does not already exist. If
/// `main.c` already exists, its contents are replaced.
pub fn save_source(project: impl AsRef<Path>, code: &str) -> Result<(), FirmwareError> {
    let directory = project.as_ref();

    fs::create_dir_all(directory)?;

    fs::write(directory.join("main.c"), code)?;

    Ok(())
}

/// Builds a generated firmware project.
///
/// The function reads `main.c` from the project directory, removes the
/// existing directory, generates a fresh project from `template`, adds the
/// generated source, and compiles the project.
///
/// # Errors
///
/// Returns [`FirmwareError`] if the source cannot be read, the existing
/// project cannot be removed, the project cannot be generated, the source
/// cannot be added, or compilation fails.
pub fn build_project(
    target: impl Target,
    template: TemplateKind,
    project: impl AsRef<Path>,
) -> Result<BuildArtifacts, FirmwareError> {
    let directory = project.as_ref();
    let source_path = directory.join("main.c");

    let code = fs::read_to_string(&source_path)?;

    fs::remove_dir_all(directory)?;

    let mut project = target.generate_project(template, directory)?;

    project.add_source("main.c", &code)?;

    Ok(project.compile()?)
}

/// Programs a firmware binary using the target's configured programmer.
///
/// # Errors
///
/// Returns [`ProgrammingError`] if programming fails. The error includes the
/// firmware path and the underlying target programming error.
pub fn program(
    target: impl Target,
    firmware: impl AsRef<Path>,
) -> Result<ProgramResult, ProgrammingError> {
    let firmware = firmware.as_ref();

    target
        .program(firmware)
        .map_err(|source| ProgrammingError::Program {
            firmware: PathBuf::from(firmware),
            source,
        })
}
