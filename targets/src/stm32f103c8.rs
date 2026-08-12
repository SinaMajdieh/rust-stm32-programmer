//! Project generation support for the STM32F103C8 target.
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use include_dir::{Dir, include_dir};

/// An STM32F103C8 project target.
#[derive(Debug, Default, Clone, Copy)]
pub struct Stm32F103C8;

/// A generated STM32 project stored on disk.
#[derive(Debug)]
pub struct Project {
    directory: PathBuf,
}

// Embed the project template so it is available at runtime.
static TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets/stm32f103c8");

impl Stm32F103C8 {
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
    pub fn generate(&self, output: impl Into<PathBuf>) -> Result<Project, io::Error> {
        let directory = output.into();

        if let Some(parent) = directory.parent()
            && !parent.exists()
        {
            fs::create_dir_all(parent)?;
        }

        fs::create_dir(&directory)?;
        TEMPLATE.extract(&directory)?;

        Ok(Project { directory })
    }
}

impl Project {
    /// Returns the root directory of the generated project.
    pub fn root(&self) -> &Path {
        self.directory.as_path()
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn generate_project_from_embedded_template() -> io::Result<()> {
        let test_dir = tempdir()?;
        let output = test_dir.path().join("generated").join("blink");

        let project = Stm32F103C8::new().generate(output.clone())?;

        assert_eq!(project.root(), output);
        assert!(project.root().is_dir());

        assert_template_was_extracted(&TEMPLATE, project.root());

        Ok(())
    }

    #[test]
    fn does_not_overwrite_an_existing_project() -> io::Result<()> {
        let test_dir = tempdir()?;
        let output = test_dir.path().join("overwrite");

        let target = Stm32F103C8::new();
        target.generate(output.clone())?;

        let error = match target.generate(output) {
            Ok(_) => panic!("Generation should not overwrite and existing project"),
            Err(err) => err,
        };

        assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);

        Ok(())
    }

    fn assert_template_was_extracted(template: &Dir<'_>, output: &Path) {
        for file in template.files() {
            let generated_file = output.join(file.path());

            assert!(
                generated_file.is_file(),
                "Expected generated file: {}",
                generated_file.display()
            );
        }

        for dir in template.dirs() {
            let generated_dir = output.join(dir.path());

            assert!(
                generated_dir.is_dir(),
                "Expected generated directory: {}",
                generated_dir.display()
            )
        }
    }
}
