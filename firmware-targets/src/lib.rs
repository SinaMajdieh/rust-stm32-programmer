//! Target templates and build support for embedded projects.

mod arm_gcc;
mod project;

pub mod stm32f103c8;

pub use arm_gcc::{ArmGccConfig, BuildArtifacts, BuildError, BuildStage};
pub use project::Project;
