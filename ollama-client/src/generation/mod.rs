mod protocol;

use std::time::Duration;

use serde::Deserialize;

pub(crate) use protocol::GenerateBody;

/// Optional runtime settings applied to a generation request.
///
/// Unspecified options are omitted from the HTTP request, allowing Ollama to
/// use its model or server defaults.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GenerateOptions {
    temperature: Option<f32>,
    seed: Option<u64>,
    context_length: Option<u32>,
    maximum_output_tokens: Option<u32>,
}

impl GenerateOptions {
    /// Creates options with no explicit overrides.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the sampling temperature.
    #[must_use]
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Sets the random seed used for generation.
    #[must_use]
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Sets the context-window length.
    #[must_use]
    pub fn with_context_length(mut self, context_length: u32) -> Self {
        self.context_length = Some(context_length);
        self
    }

    /// Sets the maximum number of tokens Ollama may generate.
    #[must_use]
    pub fn with_maximum_output_tokens(mut self, maximum_output_tokens: u32) -> Self {
        self.maximum_output_tokens = Some(maximum_output_tokens);
        self
    }

    /// Returns the configured sampling temperature.
    pub fn temperature(&self) -> Option<f32> {
        self.temperature
    }

    /// Returns the configured random seed.
    pub fn seed(&self) -> Option<u64> {
        self.seed
    }

    /// Returns the configured context-window length.
    pub fn context_length(&self) -> Option<u32> {
        self.context_length
    }

    /// Returns the configured maximum output-token count.
    pub fn maximum_output_tokens(&self) -> Option<u32> {
        self.maximum_output_tokens
    }
}

/// A request to generate text with an Ollama model.
#[derive(Debug, Clone)]
pub struct GenerateRequest {
    model: String,
    prompt: String,
    options: GenerateOptions,
    system_prompt: Option<String>,
    thinking: Option<bool>,
    keep_alive: Option<String>,
}

impl GenerateRequest {
    /// Creates a request with no explicit generation-option overrides.
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

    /// Applies explicit generation options.
    #[must_use]
    pub fn with_options(mut self, options: GenerateOptions) -> Self {
        self.options = options;
        self
    }

    #[must_use]
    pub fn with_system_prompt(mut self, system_prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(system_prompt.into());
        self
    }

    #[must_use]
    pub fn with_thinking(mut self, thinking: bool) -> Self {
        self.thinking = Some(thinking);
        self
    }

    #[must_use]
    pub fn with_keep_alive(mut self, keep_alive: impl Into<String>) -> Self {
        self.keep_alive = Some(keep_alive.into());
        self
    }

    /// Returns the requested model name.
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Returns the generation prompt.
    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    /// Returns the request's generation options.
    pub fn options(&self) -> &GenerateOptions {
        &self.options
    }

    pub fn system_prompt(&self) -> Option<&str> {
        self.system_prompt.as_deref()
    }

    pub fn thinking(&self) -> Option<bool> {
        self.thinking
    }

    pub fn keep_alive(&self) -> Option<&str> {
        self.keep_alive.as_deref()
    }
}

/// A completed, non-streaming generation returned by Ollama.
#[derive(Debug, Clone, Deserialize)]
pub struct Generation {
    response: String,

    #[serde(default)]
    thinking: String,

    done: bool,
    done_reason: Option<String>,

    total_duration: u64,
    load_duration: u64,
    prompt_eval_count: u64,
    prompt_eval_duration: u64,
    eval_count: u64,
    eval_duration: u64,
}

impl Generation {
    /// Returns the generated response text.
    pub fn response(&self) -> &str {
        &self.response
    }

    /// Returns separate thinking output when supplied by the model.
    pub fn thinking(&self) -> &str {
        &self.thinking
    }

    /// Reports whether Ollama marked the generation as complete.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Returns Ollama's completion reason, when provided.
    pub fn done_reason(&self) -> Option<&str> {
        self.done_reason.as_deref()
    }

    /// Returns the complete server-reported generation duration.
    pub fn total_duration(&self) -> Duration {
        Duration::from_nanos(self.total_duration)
    }

    /// Returns the time Ollama spent loading the model.
    pub fn load_duration(&self) -> Duration {
        Duration::from_nanos(self.load_duration)
    }

    /// Returns the time spent evaluating prompt tokens.
    pub fn prompt_evaluation_duration(&self) -> Duration {
        Duration::from_nanos(self.prompt_eval_duration)
    }

    /// Returns the time spent generating output tokens.
    pub fn evaluation_duration(&self) -> Duration {
        Duration::from_nanos(self.eval_duration)
    }

    /// Returns the number of evaluated prompt tokens.
    pub fn prompt_tokens(&self) -> u64 {
        self.prompt_eval_count
    }

    /// Returns the number of generated output tokens.
    pub fn generated_tokens(&self) -> u64 {
        self.eval_count
    }

    /// Calculates output tokens generated per second.
    ///
    /// Returns `None` if Ollama reports zero evaluation time.
    pub fn tokens_per_second(&self) -> Option<f64> {
        let seconds = self.evaluation_duration().as_secs_f64();

        if seconds == 0.0 {
            return None;
        }

        Some(self.generated_tokens() as f64 / seconds)
    }
}

#[cfg(test)]
mod tests;
