pub mod actions;
pub mod hex;
pub mod report;

mod config;
mod error;
mod firmware;
mod generation;

pub use actions::*;
pub use config::{Config, GenerationConfig, OllamaConfig, OpenAiConfig};
pub use error::{
    ConfigError, Error, FirmwareError, GenerationError, OpenAIClientError, ProgrammingError, Result,
};
pub use firmware::{build_project, program, save_source};
pub use generation::{GenerationOutput, GenerationStatistics, Provider, generate_code};
pub use report::*;
