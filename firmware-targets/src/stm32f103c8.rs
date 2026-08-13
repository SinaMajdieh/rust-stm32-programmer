//! STM32F103C8 target template and build configuration.

use crate::{ArmGccConfig, Project};

use include_dir::{Dir, include_dir};

use std::{fs, io, path::PathBuf};

/// The STM32F103C8T6 target used by the Blue Pill board.
#[derive(Debug, Default, Clone, Copy)]
pub struct Stm32F103C8;

static PROJECT_TEMPLATE: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/stm32f103c8");

impl Stm32F103C8 {
    const BUILT_IN_SOURCES: &[&str] = &["src/startup_stm32f103xb.s", "src/system_stm32f1xx.c"];

    /// Generates an STM32F103C8 project at `output`.
    ///
    /// The output directory must not already exist.
    ///
    /// # Errors
    ///
    /// Returns an error if the project directory or an embedded template file
    /// cannot be created.
    pub fn generate(output: impl Into<PathBuf>) -> io::Result<Project> {
        let directory = output.into();

        if let Some(parent) = directory
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            fs::create_dir_all(parent)?;
        }

        fs::create_dir(&directory)?;
        PROJECT_TEMPLATE.extract(&directory)?;

        let sources = Self::BUILT_IN_SOURCES
            .iter()
            .map(|source| directory.join(source))
            .collect();

        Project::from_generated(directory, sources, Self::build_config())
    }

    fn build_config() -> ArmGccConfig {
        ArmGccConfig::new(
            "arm-none-eabi-gcc",
            "arm-none-eabi-objcopy",
            "cortex-m3",
            ["STM32F103xB"],
            ["CMSIS/Include", "CMSIS/Device/ST/STM32F1xx"],
            "STM32F103C8TX_FLASH.ld",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use include_dir::Dir;
    use std::{fs, io, path::Path};
    use tempfile::tempdir;

    #[test]
    fn generates_project_from_embedded_template() -> io::Result<()> {
        let test_dir = tempdir()?;
        let output = test_dir.path().join("generated/blink");

        let project = Stm32F103C8::generate(output.clone())?;

        assert_eq!(project.root(), output);
        assert!(project.root().is_dir());

        assert_template_was_extracted(&PROJECT_TEMPLATE, project.root());

        Ok(())
    }

    #[test]
    fn does_not_overwrite_existing_project() -> io::Result<()> {
        let test_dir = tempdir()?;
        let output = test_dir.path().join("overwrite");

        Stm32F103C8::generate(output.clone())?;

        let error = Stm32F103C8::generate(output)
            .expect_err("generation must not overwrite an existing project");

        assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);

        Ok(())
    }

    #[test]
    fn adds_source_to_generated_project() -> io::Result<()> {
        let test_dir = tempdir()?;
        let output = test_dir.path().join("project");

        let mut project = Stm32F103C8::generate(output)?;

        project.add_source("main.c", "int main(void) { return 0; }")?;

        let main = project.root().join("src/main.c");

        assert!(main.is_file());
        assert_eq!(fs::read_to_string(&main)?, "int main(void) { return 0; }",);
        assert!(project.sources().contains(&main));

        Ok(())
    }

    #[test]
    #[ignore = "requires the Arm GNU toolchain"]
    fn compiles_generated_project() -> Result<(), Box<dyn std::error::Error>> {
        let test_dir = tempdir()?;
        let output = test_dir.path().join("blink");

        let mut project = Stm32F103C8::generate(output)?;

        project.add_source(
            "main.c",
            r#"
            #include "stm32f1xx.h"

            int main(void)
            {
                while (1) {
                }
            }
            "#,
        )?;

        let artifacts = project.compile()?;

        assert!(artifacts.elf().is_file());
        assert!(artifacts.hex().is_file());
        assert!(artifacts.binary().is_file());
        assert!(artifacts.map().is_file());

        Ok(())
    }

    fn assert_template_was_extracted(template: &Dir<'_>, output: &Path) {
        for file in template.files() {
            let generated_file = output.join(file.path());

            assert!(
                generated_file.is_file(),
                "missing generated file: {}",
                generated_file.display(),
            );
        }

        for directory in template.dirs() {
            let generated_directory = output.join(directory.path());

            assert!(
                generated_directory.is_dir(),
                "missing generated directory: {}",
                generated_directory.display(),
            );

            assert_template_was_extracted(directory, output);
        }
    }
}
