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
#[path = "stm32f103c8_tests.rs"]
mod tests;
