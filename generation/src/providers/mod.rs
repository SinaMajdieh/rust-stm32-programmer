//! Generation provider implementations.
//!
//! This module contains the concrete providers supported by the generation
//! subsystem. Each provider owns its provider-specific client, configuration,
//! generation options, and translation between [`crate::GenerationRequest`]
//! and the provider API.
//!
//! Provider implementations expose a common interface through the crate-private
//! [`crate::GenerationProvider`] trait and normalize successful responses into
//! [`crate::GenerationOutput`].

mod ollama;
mod openai;

pub use ollama::{OllamaClientConfig, OllamaConfig, OllamaGenerationOptions, OllamaProvider};
pub use openai::{OpenAiClientConfig, OpenAiConfig, OpenAiGenerationOptions, OpenAiProvider};

/// Serde helpers for serializing [`Duration`](std::time::Duration) values as
/// whole seconds.
///
/// Configuration files represent durations as integer seconds rather than
/// Rust's `Duration` representation.
pub mod duration_seconds {
    use std::time::Duration;

    use serde::{Deserialize, Deserializer, Serializer};

    /// Serializes a duration as its whole-second component.
    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(duration.as_secs())
    }

    /// Deserializes a duration from a number of whole seconds.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let seconds = u64::deserialize(deserializer)?;
        Ok(Duration::from_secs(seconds))
    }
}
