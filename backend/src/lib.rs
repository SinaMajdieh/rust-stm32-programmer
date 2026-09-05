//! Core backend for firmware generation, project building, validation, and programming.
//!
//! This crate coordinates the higher-level backend operations while delegating
//! target-specific project handling to [`firmware-targets`] and LLM integration
//! to [`firmware-generation`].

pub mod actions;
pub mod hex;
pub mod report;

mod error;
mod firmware;

pub use actions::*;
pub use error::{Error, FirmwareError, ProgrammingError, Result};
pub use firmware::{build_project, program, save_source};
pub use generation::*;
pub use report::*;
