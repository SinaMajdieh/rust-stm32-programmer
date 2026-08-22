//! Target templates and build support for embedded projects.
//!
//! This crate provides project templates and build support for embedded
//! firmware projects. A target template can generate a complete project
//! directory, which can then be extended with additional source files and
//! compiled using the configured ARM GNU toolchain.
//!
//! The typical workflow is:
//!
//! 1. Select a target template such as [`stm32f103c8::Cmsis`].
//! 2. Generate a [`Project`] from the template.
//! 3. Add application source files with [`Project::add_source`].
//! 4. Compile the project with [`Project::compile`].
//!
//! The crate currently provides [`stm32f103c8`] templates and ARM GCC build
//! support.

mod arm_gcc;
mod project;

/// Project templates for supported microcontroller targets.
pub mod stm32f103c8;

pub use arm_gcc::{ArmGccConfig, BuildArtifacts, BuildError, BuildStage};
pub use project::Project;
