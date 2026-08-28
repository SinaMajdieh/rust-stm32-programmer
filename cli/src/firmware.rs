use std::{fs, path::Path};

use anyhow::{Context, Result};
use firmware_targets::{
    BuildArtifacts,
    programmer::{OpenOcd, ProgramResult},
    stm32f103c8::{Hal, ProjectTemplate, Target},
};

/// Saves generated C source code as `main.c`.
pub fn save_source(project: &str, code: &str) -> Result<()> {
    let directory = Path::new(project);

    fs::create_dir_all(directory)
        .with_context(|| format!("failed to create project directory: {project}"))?;

    let path = directory.join("main.c");

    fs::write(&path, code).with_context(|| format!("failed to write {}", path.display()))?;

    Ok(())
}

/// Builds a generated firmware project.
pub fn build_project(project: &str) -> Result<BuildArtifacts> {
    let directory = Path::new(project);
    let source_path = directory.join("main.c");

    let code = fs::read_to_string(&source_path)
        .with_context(|| format!("failed to read {}", source_path.display()))?;

    fs::remove_dir_all(directory).with_context(|| {
        format!(
            "failed to remove existing project \
                 directory: {project}"
        )
    })?;

    let mut project = Hal::generate(project).context("failed to create firmware project")?;

    project
        .add_source("main.c", &code)
        .context("failed to add main.c to firmware project")?;

    project.compile().context("firmware build failed")
}

/// Programs a firmware binary using OpenOCD.
pub fn program(firmware: impl AsRef<Path>) -> Result<ProgramResult> {
    let target = Target::<OpenOcd>::default();

    target.program(firmware).context("programming failed")
}
