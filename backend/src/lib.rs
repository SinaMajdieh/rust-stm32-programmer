//! Core backend for firmware generation, project building, validation, and programming.
//!
//! This crate provides the application-facing operations that coordinate the
//! firmware workflow. Target-specific project generation and programming are
//! delegated to [`firmware-targets`], while LLM-based source generation is
//! delegated to [`generation`].
//!
//! The crate also provides Intel HEX validation and [`Report`] values for
//! reporting progress and status to higher-level applications.

pub mod actions;
pub mod hex;
pub mod project;
pub mod report;

mod error;
mod firmware;

pub use actions::*;
pub use error::{ConfigError, Error, ProgrammingError, Result};
pub use firmware::{build_project, program, save_source};
pub use generation::*;
pub use report::*;
