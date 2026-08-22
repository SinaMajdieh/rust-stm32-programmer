//! Project templates for the STM32F103C8 microcontroller.
//!
//! This module provides preconfigured project templates for building
//! STM32F103C8 firmware with different levels of the STM32 software stack.
//!
//! [`Cmsis`] provides a project based on the ARM CMSIS headers and ST's
//! CMSIS device definitions, while [`Ll`] provides a lighter-weight template
//! based on the STM32 Low-Layer approach.
//!
//! [`ProjectTemplate`] defines the common interface used by these templates
//! to provide their embedded project files, built-in sources, and ARM GCC
//! configuration.

mod cmsis;
mod ll;
mod template;

pub use cmsis::Cmsis;
pub use ll::Ll;
pub use template::ProjectTemplate;
