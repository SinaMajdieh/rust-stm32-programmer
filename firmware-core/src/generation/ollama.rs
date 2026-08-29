use std::time::Instant;

use ollama_client::{GenerateOptions, GenerateRequest, OllamaClient};

use crate::{Config, GenerationError, GenerationOutput, GenerationStatistics};

use super::{LlmProvider, unfence_code};

/// LLM provider backed by a local Ollama server.
pub struct OllamaProvider<'a> {
    config: &'a Config,
}

impl<'a> OllamaProvider<'a> {
    /// Creates an Ollama provider using `config`.
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }
}

impl LlmProvider for OllamaProvider<'_> {
    async fn generate(
        &self,
        model: &str,
        prompt: &str,
    ) -> Result<GenerationOutput, GenerationError> {
        let client = OllamaClient::new(&self.config.ollama.url)?;

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

        let start = Instant::now();

        let generation = client
            .generate(
                &request,
                std::time::Duration::from_secs(self.config.ollama.timeout_seconds),
            )
            .await?;

        let statistics = GenerationStatistics {
            prompt_tokens: None,
            generated_tokens: generation.generated_tokens,
            elapsed: start.elapsed(),
        };

        Ok(GenerationOutput {
            code: unfence_code(&generation.response).to_owned(),
            statistics,
        })
    }
}
