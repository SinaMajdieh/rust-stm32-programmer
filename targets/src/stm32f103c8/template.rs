use crate::stm32f103c8::project::Project;
use include_dir::{Dir, include_dir};
use std::{fs, io, path::PathBuf};

/// An STM32F103C8 Template.
#[derive(Debug, Default, Clone, Copy)]
pub struct Template;

// Embed the project template so it is available at runtime.
pub(crate) static TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets/stm32f103c8");

impl Template {
    const BUILT_IN_SOURCES: &[&str] = &["src/startup_stm32f103xb.s", "src/system_stm32f1xx.c"];
    /// Creates an STM32F103C8 target.
    pub fn new() -> Self {
        Self
    }

    /// Generates a project by extracting the embedded template into `output`.
    ///
    /// The output directory must not already exist. The generated project
    /// remains on disk after the returned [`Project`] is dropped.
    ///
    /// # Errors
    ///
    /// Returns an error if the output directory already exists or if a
    /// directory or template file cannot be created.
    pub fn generate(&self, output: impl Into<PathBuf>) -> io::Result<Project> {
        let directory = output.into();

        if let Some(parent) = directory.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::create_dir(&directory)?;
        TEMPLATE.extract(&directory)?;

        let sources = Self::BUILT_IN_SOURCES
            .iter()
            .map(|source| directory.join(source))
            .collect();

        Ok(Project::from_generated(directory, sources))
    }
}
