mod wire;

use std::time::Duration;

use crate::{OllamaClient, Result};
use wire::{GenerateRequestBody, GenerateResponseBody};

const GENERATE_ENDPOINT: &str = "/api/generate";

/// Options that control text generation.
#[derive(Debug, Clone, Default, PartialEq)]
#[non_exhaustive]
pub struct GenerateOptions {
    /// Controls the randomness of the generated output.
    pub temperature: Option<f32>,

    /// Sets the random seed used during generation.
    pub seed: Option<u64>,

    /// Sets the maximum context length in tokens.
    pub context_length: Option<u32>,

    /// Sets the maximum number of tokens to generate.
    pub maximum_output_tokens: Option<u32>,
}

impl GenerateOptions {
    /// Creates a set of generation options with all options unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the temperature used during generation.
    ///
    /// Higher values produce more varied output, while lower values make the
    /// output more deterministic.
    #[must_use]
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Sets the random seed used during generation.
    ///
    /// Using the same seed can make generation reproducible when the other
    /// generation parameters and model state are unchanged.
    #[must_use]
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Sets the maximum context length in tokens.
    #[must_use]
    pub fn with_context_length(mut self, context_length: u32) -> Self {
        self.context_length = Some(context_length);
        self
    }

    /// Sets the maximum number of tokens to generate.
    #[must_use]
    pub fn with_maximum_output_tokens(mut self, maximum_output_tokens: u32) -> Self {
        self.maximum_output_tokens = Some(maximum_output_tokens);

        self
    }
}

/// A request to generate a response from an Ollama model.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GenerateRequest {
    /// The name of the model to use for generation.
    pub model: String,

    /// The prompt to provide to the model.
    pub prompt: String,

    /// Options controlling how the response is generated.
    pub options: GenerateOptions,

    /// An optional system prompt that provides instructions or context to the
    /// model.
    pub system_prompt: Option<String>,

    /// Whether the model should include its thinking process in the response.
    pub thinking: Option<bool>,

    /// How long the model should remain loaded after the request completes.
    ///
    /// The value is passed directly to Ollama as its `keep_alive` setting.
    pub keep_alive: Option<String>,
}

impl GenerateRequest {
    /// Creates a generation request for the given model and prompt.
    ///
    /// All optional generation settings are left unset.
    pub fn new(model: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            prompt: prompt.into(),
            options: GenerateOptions::default(),
            system_prompt: None,
            thinking: None,
            keep_alive: None,
        }
    }

    /// Sets the options used for generation.
    #[must_use]
    pub fn with_options(mut self, options: GenerateOptions) -> Self {
        self.options = options;
        self
    }

    /// Sets a system prompt for the model.
    #[must_use]
    pub fn with_system_prompt(mut self, system_prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(system_prompt.into());
        self
    }

    /// Enables or disables thinking for the generation request.
    #[must_use]
    pub fn with_thinking(mut self, thinking: bool) -> Self {
        self.thinking = Some(thinking);
        self
    }

    /// Sets how long Ollama should keep the model loaded after the request.
    ///
    /// The value is passed directly to Ollama's `keep_alive` option.
    #[must_use]
    pub fn with_keep_alive(mut self, keep_alive: impl Into<String>) -> Self {
        self.keep_alive = Some(keep_alive.into());
        self
    }
}

/// The result of a completed text generation request.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Generation {
    /// The generated response text.
    pub response: String,

    /// The model's thinking output, if provided.
    pub thinking: String,

    /// Whether Ollama has finished generating the response.
    pub done: bool,

    /// The reason Ollama reported for completing generation.
    pub done_reason: Option<String>,

    /// The total time spent processing the request.
    pub total_duration: Duration,

    /// The time spent loading the model.
    pub load_duration: Duration,

    /// The number of tokens in the input prompt.
    pub prompt_tokens: u64,

    /// The time spent evaluating the input prompt.
    pub prompt_evaluation_duration: Duration,

    /// The number of tokens generated in the response.
    pub generated_tokens: u64,

    /// The time spent generating the response tokens.
    pub evaluation_duration: Duration,
}

impl Generation {
    /// Returns the average generation rate in tokens per second.
    ///
    /// Returns `None` when the recorded evaluation duration is zero.
    pub fn tokens_per_second(&self) -> Option<f64> {
        let seconds = self.evaluation_duration.as_secs_f64();

        if seconds == 0.0 {
            return None;
        }

        Some(self.generated_tokens as f64 / seconds)
    }
}

impl From<GenerateResponseBody> for Generation {
    fn from(response: GenerateResponseBody) -> Self {
        Self {
            response: response.response,
            thinking: response.thinking,
            done: response.done,
            done_reason: response.done_reason,
            total_duration: Duration::from_nanos(response.total_duration),
            load_duration: Duration::from_nanos(response.load_duration),
            prompt_tokens: response.prompt_eval_count,
            prompt_evaluation_duration: Duration::from_nanos(response.prompt_eval_duration),
            generated_tokens: response.eval_count,
            evaluation_duration: Duration::from_nanos(response.eval_duration),
        }
    }
}

impl OllamaClient {
    /// Generates a response using an Ollama model.
    ///
    /// The request is sent as a non-streaming generation request and the
    /// complete response is returned once generation finishes.
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be sent, times out, Ollama
    /// returns an unsuccessful HTTP status, or the response cannot be
    /// deserialized.
    pub async fn generate(
        &self,
        request: &GenerateRequest,
        timeout: Duration,
    ) -> Result<Generation> {
        let body = GenerateRequestBody::from(request);

        let request = self.post(GENERATE_ENDPOINT)?.json(&body);

        let response: GenerateResponseBody = self.execute_json(request, timeout).await?;

        Ok(response.into())
    }
}

#[cfg(test)]
mod tests;
