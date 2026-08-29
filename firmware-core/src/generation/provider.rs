use std::time::Duration;

use crate::GenerationError;

/// Available LLM generation backends.
#[derive(Debug, Clone, Copy)]
pub enum Provider {
    /// Use a local Ollama server.
    Ollama,

    /// Use an OpenAI-compatible API.
    OpenAi,
}

/// Result of an LLM generation request.
#[derive(Debug)]
pub struct GenerationOutput {
    pub code: String,
    pub statistics: GenerationStatistics,
}

/// Statistics collected during generation.
#[derive(Debug)]
pub struct GenerationStatistics {
    pub prompt_tokens: Option<u64>,
    pub generated_tokens: u64,
    pub elapsed: Duration,
}

impl GenerationStatistics {
    /// Returns the observed generation throughput.
    pub fn tokens_per_second(&self) -> f64 {
        let seconds = self.elapsed.as_secs_f64();

        if seconds > 0.0 {
            self.generated_tokens as f64 / seconds
        } else {
            0.0
        }
    }
}

/// Common interface implemented by all generation backends.
pub trait LlmProvider {
    /// Generates firmware source code using `model` and `prompt`.
    async fn generate(
        &self,
        model: &str,
        prompt: &str,
    ) -> Result<GenerationOutput, GenerationError>;
}
