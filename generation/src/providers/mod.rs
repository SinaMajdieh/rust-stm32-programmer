//! Generation provider implementations.

mod ollama;
mod openai;

pub use ollama::{OllamaClientConfig, OllamaConfig, OllamaGenerationOptions, OllamaProvider};
pub use openai::{OpenAiClientConfig, OpenAiConfig, OpenAiGenerationOptions, OpenAiProvider};
