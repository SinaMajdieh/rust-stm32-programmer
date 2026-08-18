mod client;
mod error;
mod generation;

pub use client::{OllamaClient, Version};
pub use error::{Error, Result};
pub use generation::{
    GenerateOptions,
    GenerateRequest,
    Generation,
};