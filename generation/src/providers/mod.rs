//! Generation provider implementations.

mod ollama;
mod openai;

pub use ollama::{OllamaClientConfig, OllamaConfig, OllamaGenerationOptions, OllamaProvider};
pub use openai::{OpenAiClientConfig, OpenAiConfig, OpenAiGenerationOptions, OpenAiProvider};

pub mod duration_seconds {
    use std::time::Duration;

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(duration.as_secs())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let seconds = u64::deserialize(deserializer)?;
        Ok(Duration::from_secs(seconds))
    }
}
