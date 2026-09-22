use std::{io, path::Path};

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

use crate::{
    Project,
    programmer::{ProgramResult, ProgrammingError},
    stm32f103c8::Stm32f103c8,
};

/// Identifies a supported microcontroller target.
///
/// This type is intended for configuration and serialization. Use
/// [`create_target`] to construct the concrete target implementation
/// corresponding to a target kind.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Default,
    Display,
    EnumIter,
    EnumString,
)]
#[strum(serialize_all = "UPPERCASE")]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum TargetKind {
    /// An STM32F103C8 target.
    #[default]
    Stm32f103c8,
}

/// Identifies the software stack used by a target project template.
///
/// The selected template determines the project files and build configuration
/// used when generating a [`Project`].
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Default,
    Display,
    EnumIter,
    EnumString,
)]
#[strum(serialize_all = "UPPERCASE")]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum TemplateKind {
    /// An STM32 HAL-based template.
    #[default]
    Hal,

    /// An STM32 Low-Layer template.
    Ll,

    /// An ARM CMSIS-based template.
    Cmsis,
}

/// A target capable of generating a project and programming firmware.
///
/// Implementations provide the target-specific logic required to generate
/// projects from supported [`TemplateKind`]s and program firmware onto the
/// target device.
pub trait Target {
    /// Generates a project using the specified template.
    ///
    /// The generated project is written to `path`.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if the project template cannot be materialized
    /// at the requested path.
    fn generate_project(
        &self,
        template: TemplateKind,
        path: impl AsRef<Path>,
    ) -> io::Result<Project>;

    /// Programs a firmware image onto the target using the target's default
    /// programming configuration.
    ///
    /// # Errors
    ///
    /// Returns a [`ProgramError`] if the firmware cannot be found, the
    /// programmer cannot be started, or programming fails.
    fn program(&self, firmware: impl AsRef<Path>) -> Result<ProgramResult, ProgrammingError>;
}

/// Creates the target implementation corresponding to `target_kind`.
///
/// This function maps the configuration-level [`TargetKind`] to its concrete
/// target implementation.
pub fn create_target(target_kind: &TargetKind) -> Option<impl Target> {
    match target_kind {
        TargetKind::Stm32f103c8 => Some(Stm32f103c8::new()),
    }
}
