use std::time::Duration;

use anyhow::{Context, Result};
use ollama_client::{GenerateOptions, GenerateRequest, OllamaClient};

use crate::{config::Config, generation::unfence_code};

use super::provider::LlmProvider;

/// LLM provider backed by a local Ollama server.
pub struct OllamaProvider<'a> {
    config: &'a Config,
}

impl<'a> OllamaProvider<'a> {
    /// Creates an Ollama provider using the application configuration.
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }
}

impl LlmProvider for OllamaProvider<'_> {
    async fn generate(&self, model: &str, prompt: &str) -> Result<String> {
        let client =
            OllamaClient::new(&self.config.ollama.url).context("failed to create Ollama client")?;

        let options = GenerateOptions::new()
            .with_seed(self.config.generation.seed)
            .with_temperature(self.config.generation.temperature)
            .with_context_length(self.config.ollama.context_length)
            .with_maximum_output_tokens(self.config.generation.max_output_tokens);

        let request = GenerateRequest::new(model, prompt)
            .with_system_prompt(&self.config.system_prompt)
            .with_thinking(false)
            .with_keep_alive(&self.config.ollama.keep_alive)
            .with_options(options);

        let generation = client
            .generate(
                &request,
                Duration::from_secs(self.config.ollama.timeout_seconds),
            )
            .await
            .context("Ollama code generation failed")?;

        println!(
            "Generated {} tokens at {:.1} tokens/s.",
            generation.generated_tokens,
            generation.tokens_per_second().unwrap_or(0.0),
        );

        Ok(unfence_code(&generation.response).to_owned())
    }
}
