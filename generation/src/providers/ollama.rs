//! Ollama generation provider.

use crate::providers::duration_seconds;
use ollama_client::{GenerateOptions, GenerateRequest as OllamaRequest, OllamaClient};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

use crate::{
    code::clean_generated_code,
    error::GenerationError,
    output::{GenerationOutput, GenerationStatistics},
    provider::GenerationProvider,
    request::GenerationRequest,
};

/// Configuration for the Ollama provider.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OllamaConfig {
    /// Configuration for the Ollama client.
    #[serde(default)]
    pub client: OllamaClientConfig,

    /// Configuration for text generation.
    #[serde(default)]
    pub generation: OllamaGenerationOptions,
}

/// Configuration for the Ollama client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaClientConfig {
    /// URL of the Ollama server.
    #[serde(default = "default_ollama_url")]
    pub url: String,

    /// How long a model should remain loaded after a request.
    #[serde(default)]
    pub keep_alive: Option<String>,

    /// Maximum duration of a generation request.
    #[serde(default = "default_request_timeout", with = "duration_seconds")]
    pub request_timeout: Duration,
}

impl Default for OllamaClientConfig {
    fn default() -> Self {
        Self {
            url: default_ollama_url(),
            keep_alive: None,
            request_timeout: default_request_timeout(),
        }
    }
}

/// Configuration for Ollama text generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaGenerationOptions {
    /// Sampling temperature.
    #[serde(default)]
    pub temperature: Option<f32>,

    /// Maximum number of tokens to generate.
    #[serde(default)]
    pub max_output_tokens: Option<u32>,

    /// Maximum context length.
    #[serde(default)]
    pub context_length: Option<u32>,

    /// Random seed used for generation.
    #[serde(default)]
    pub seed: Option<u64>,
}

impl Default for OllamaGenerationOptions {
    fn default() -> Self {
        Self {
            temperature: None,
            max_output_tokens: None,
            context_length: None,
            seed: None,
        }
    }
}

/// Runtime provider backed by Ollama.
pub struct OllamaProvider {
    client: OllamaClient,
    options: GenerateOptions,
    keep_alive: Option<String>,
    request_timeout: Duration,
}

impl OllamaProvider {
    /// Creates an Ollama provider from its configuration.
    pub fn new(config: OllamaConfig) -> Result<Self, GenerationError> {
        let client = OllamaClient::new(&config.client.url)?;
        let options = build_options(&config.generation);

        Ok(Self {
            client,
            options,
            keep_alive: config.client.keep_alive,
            request_timeout: config.client.request_timeout,
        })
    }
}

impl GenerationProvider for OllamaProvider {
    async fn generate(
        &self,
        request: GenerationRequest<'_>,
    ) -> Result<GenerationOutput, GenerationError> {
        let mut ollama_request = OllamaRequest::new(request.model, request.prompt)
            .with_thinking(false)
            .with_options(self.options.clone());

        if let Some(system_prompt) = request.system_prompt {
            ollama_request = ollama_request.with_system_prompt(system_prompt);
        }

        if let Some(keep_alive) = self.keep_alive.as_deref() {
            ollama_request = ollama_request.with_keep_alive(keep_alive);
        }

        let start = Instant::now();

        let response = self
            .client
            .generate(&ollama_request, self.request_timeout)
            .await?;

        let elapsed = start.elapsed();

        Ok(GenerationOutput {
            code: clean_generated_code(&response.response).to_owned(),
            statistics: GenerationStatistics {
                prompt_tokens: None,
                generated_tokens: response.generated_tokens,
                elapsed,
            },
        })
    }
}

/// Builds Ollama's generation options from application configuration.
fn build_options(config: &OllamaGenerationOptions) -> GenerateOptions {
    let mut options = GenerateOptions::new();

    if let Some(seed) = config.seed {
        options = options.with_seed(seed);
    }

    if let Some(temperature) = config.temperature {
        options = options.with_temperature(temperature);
    }

    if let Some(context_length) = config.context_length {
        options = options.with_context_length(context_length);
    }

    if let Some(max_output_tokens) = config.max_output_tokens {
        options = options.with_maximum_output_tokens(max_output_tokens);
    }

    options
}

fn default_ollama_url() -> String {
    "http://localhost:11434".to_owned()
}

const fn default_request_timeout() -> Duration {
    Duration::from_secs(120)
}
