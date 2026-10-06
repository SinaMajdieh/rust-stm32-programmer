//! Core backend for firmware generation, project building, validation, and programming.
//!
//! This crate provides the application-facing operations that coordinate the
//! firmware workflow. Target-specific project generation and programming are
//! delegated to [`firmware-targets`], while LLM-based source generation is
//! delegated to [`generation`].
//!
//! The crate also provides Intel HEX validation and [`Report`] values for
//! reporting progress and status to higher-level applications.

mod config;
mod hex;
pub mod project;
mod report;

mod error;

pub use config::Config;
pub use error::{ConfigError, Error, Result};
pub use generation::*;
pub use report::*;
