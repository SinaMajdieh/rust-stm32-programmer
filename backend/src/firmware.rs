use std::{
    fs,
    path::{Path, PathBuf},
};

use firmware_targets::{
    BuildArtifacts, FirmwareError, Target, TemplateKind,
    programmer::{OpenOcd, ProgramResult},
    stm32f103c8::Stm32f103c8,
};

use crate::ProgrammingError;

/// Saves generated C source code as `main.c`.
pub fn save_source(project: impl AsRef<Path>, code: &str) -> Result<(), FirmwareError> {
    let directory = project.as_ref();

    fs::create_dir_all(directory)?;

    fs::write(directory.join("main.c"), code)?;

    Ok(())
}

/// Builds a generated firmware project.
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

/// Programs a firmware binary using OpenOCD.
pub fn program(firmware: impl AsRef<Path>) -> Result<ProgramResult, ProgrammingError> {
    let firmware = firmware.as_ref();
    let target = Stm32f103c8::<OpenOcd>::default();

    target
        .program(firmware)
        .map_err(|source| ProgrammingError::Program {
            firmware: PathBuf::from(firmware),
            source,
        })
}
