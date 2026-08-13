//! Project generation support for the STM32F103C8 target.
use include_dir::{Dir, include_dir};
pub use project::Project;
use std::{fs, io, path::PathBuf};

mod compile;
mod project;

/// An STM32F103C8 Template.
#[derive(Debug, Default, Clone, Copy)]
pub struct Stm32F103C8;

// Embed the project template so it is available at runtime.
pub(crate) static PROJECT_TEMPLATE: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/assets/stm32f103c8");

impl Stm32F103C8 {
    const BUILT_IN_SOURCES: &[&str] = &["src/startup_stm32f103xb.s", "src/system_stm32f1xx.c"];
    /// Generates a project by extracting the embedded template into `output`.
    ///
    /// The output directory must not already exist. The generated project
    /// remains on disk after the returned [`Project`] is dropped.
    ///
    /// # Errors
    ///
    /// Returns an error if the output directory already exists or if a
    /// directory or template file cannot be created.
    pub fn generate(output: impl Into<PathBuf>) -> io::Result<Project> {
        let directory = output.into();

        if let Some(parent) = directory.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::create_dir(&directory)?;
        PROJECT_TEMPLATE.extract(&directory)?;

        let sources = Self::BUILT_IN_SOURCES
            .iter()
            .map(|source| directory.join(source))
            .collect();

        Ok(Project::from_generated(directory, sources))
    }
}

#[cfg(test)]
mod tests {
    use include_dir::Dir;
    use std::{fs, io, path::Path};
    use tempfile::tempdir;

    use crate::stm32f103c8::{PROJECT_TEMPLATE, Stm32F103C8};

    #[test]
    fn generate_project_from_embedded_template() -> io::Result<()> {
        let test_dir = tempdir()?;
        let output = test_dir.path().join("generated").join("blink");

        let project = Stm32F103C8::generate(output.clone())?;

        assert_eq!(project.root(), output);
        assert!(project.root().is_dir());

        assert_template_was_extracted(&PROJECT_TEMPLATE, project.root());

        Ok(())
    }

    #[test]
    fn does_not_overwrite_an_existing_project() -> io::Result<()> {
        let test_dir = tempdir()?;
        let output = test_dir.path().join("overwrite");

        Stm32F103C8::generate(output.clone())?;

        let error = Stm32F103C8::generate(output)
            .expect_err("generation must not overwrite an existing project");

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
            let generated_directory = output.join(dir.path());

            assert!(
                generated_directory.is_dir(),
                "expected generated directory: {}",
                generated_directory.display()
            );

            assert_template_was_extracted(dir, output);
        }
    }
    #[test]
    fn adds_source_to_generated_project() -> io::Result<()> {
        let test_dir = tempdir()?;
        let output = test_dir.path().join("project");

        let mut project = Stm32F103C8::generate(output)?;
        project.add_source("main.c", "int main(void) { return 0; }")?;

        let main = project.root().join("src/main.c");

        assert!(main.is_file());
        assert_eq!(fs::read_to_string(&main)?, "int main(void) { return 0; }");
        assert!(project.sources().contains(&main));

        Ok(())
    }
}
