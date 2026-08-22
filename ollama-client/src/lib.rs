//! A Rust client for interacting with the Ollama API.
//!
//! This crate provides types and utilities for communicating with an Ollama
//! server, generating responses, and working with locally available models.

mod client;
mod error;
mod generation;
mod model;
mod version;

pub use client::OllamaClient;

pub use error::{Error, Result};

pub use generation::{GenerateOptions, GenerateRequest, Generation};
pub use model::InstalledModel;
pub use model::LoadedModel;
pub use model::ModelDetails;
pub use model::ModelMetadata;
pub use version::Version;
