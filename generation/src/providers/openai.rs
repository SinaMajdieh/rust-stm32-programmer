//! OpenAI-compatible generation provider.

use crate::providers::duration_seconds;
use std::time::{Duration, Instant};

use async_openai::{
    Client,
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CompletionUsage, CreateChatCompletionRequestArgs,
    },
};
use serde::{Deserialize, Serialize};

use crate::{
    code::clean_generated_code,
    error::{GenerationError, OpenAIClientError},
    output::{GenerationOutput, GenerationStatistics},
    provider::GenerationProvider,
    request::GenerationRequest,
};

/// Configuration for an OpenAI-compatible provider.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OpenAiConfig {
    /// Configuration for the OpenAI client.
    #[serde(default)]
    pub client: OpenAiClientConfig,

    /// Configuration for text generation.
    #[serde(default)]
    pub generation: OpenAiGenerationOptions,
}

/// Configuration for the OpenAI-compatible client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiClientConfig {
    /// Base URL of the OpenAI-compatible API.
    #[serde(default = "default_openai_url")]
    pub url: String,

    /// Environment variable containing the API key.
    #[serde(default = "default_api_key_env")]
    pub api_key_env: Option<String>,

    /// Maximum duration of a generation request.
    #[serde(default = "default_request_timeout", with = "duration_seconds")]
    pub request_timeout: Duration,
}

impl Default for OpenAiClientConfig {
    fn default() -> Self {
        Self {
            url: default_openai_url(),
            api_key_env: default_api_key_env(),
            request_timeout: default_request_timeout(),
        }
    }
}

/// Options for OpenAI-compatible text generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiGenerationOptions {
    /// Sampling temperature.
    #[serde(default = "default_temperature")]
    pub temperature: Option<f32>,

    /// Nucleus sampling probability.
    #[serde(default = "default_top_p")]
    pub top_p: Option<f32>,

    /// Maximum number of tokens to generate.
    #[serde(default = "default_max_output_tokens")]
    pub max_output_tokens: Option<u32>,

    /// Random seed used for generation.
    #[serde(default = "default_seed")]
    pub seed: Option<i64>,

    /// Whether responses should be streamed.
    #[serde(default = "default_stream")]
    pub stream: bool,
}

impl Default for OpenAiGenerationOptions {
    fn default() -> Self {
        Self {
            temperature: default_temperature(),
            top_p: default_top_p(),
            max_output_tokens: default_max_output_tokens(),
            seed: default_seed(),
            stream: default_stream(),
        }
    }
}

/// Runtime provider backed by an OpenAI-compatible API.
pub struct OpenAiProvider {
    client: Client<OpenAIConfig>,
    options: OpenAiGenerationOptions,
    request_timeout: Duration,
}

impl OpenAiProvider {
    /// Creates an OpenAI-compatible provider from its configuration.
    pub fn new(config: OpenAiConfig) -> Result<Self, GenerationError> {
        let api_key_env = config.client.api_key_env.as_deref().ok_or_else(|| {
            OpenAIClientError::ApiKey("no API key environment variable configured".to_owned())
        })?;

        let api_key = std::env::var(api_key_env)
            .map_err(|_| OpenAIClientError::ApiKey(api_key_env.to_owned()))?;

        let client_config = OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(config.client.url);

        Ok(Self {
            client: Client::with_config(client_config),
            options: config.generation,
            request_timeout: config.client.request_timeout,
        })
    }

    /// Builds the chat messages for a generation request.
    fn build_messages(
        request: &GenerationRequest<'_>,
    ) -> Result<Vec<ChatCompletionRequestMessage>, OpenAIClientError> {
        let mut messages = Vec::with_capacity(2);

        if let Some(system_prompt) = request.system_prompt {
            messages.push(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_prompt)
                    .build()?
                    .into(),
            );
        }

        messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(request.prompt)
                .build()?
                .into(),
        );

        Ok(messages)
    }
}

impl GenerationProvider for OpenAiProvider {
    async fn generate(
        &self,
        request: GenerationRequest<'_>,
    ) -> Result<GenerationOutput, GenerationError> {
        let messages = Self::build_messages(&request)?;

        let mut builder = CreateChatCompletionRequestArgs::default();

        builder.model(request.model).messages(messages);

        if let Some(temperature) = self.options.temperature {
            builder.temperature(temperature);
        }

        if let Some(top_p) = self.options.top_p {
            builder.top_p(top_p);
        }

        if let Some(max_output_tokens) = self.options.max_output_tokens {
            builder.max_completion_tokens(max_output_tokens);
        }

        if let Some(seed) = self.options.seed {
            builder.seed(seed);
        }

        let request = builder.build().map_err(OpenAIClientError::Client)?;

        let start = Instant::now();

        let response =
            tokio::time::timeout(self.request_timeout, self.client.chat().create(request))
                .await
                .map_err(|_| OpenAIClientError::Timeout(self.request_timeout))?
                .map_err(OpenAIClientError::Client)?;

        let elapsed = start.elapsed();

        let choice = response
            .choices
            .first()
            .ok_or(OpenAIClientError::NoChoices)?;

        let code = clean_generated_code(choice.message.content.as_deref().unwrap_or_default());

        Ok(GenerationOutput {
            code: code.to_owned(),
            statistics: statistics(response.usage.as_ref(), elapsed),
        })
    }
}

fn statistics(usage: Option<&CompletionUsage>, elapsed: Duration) -> GenerationStatistics {
    GenerationStatistics {
        prompt_tokens: usage.map(|usage| usage.prompt_tokens as u64),
        generated_tokens: usage.map_or(0, |usage| usage.completion_tokens as u64),
        elapsed,
    }
}

fn default_openai_url() -> String {
    "https://api.openai.com/v1".to_owned()
}

fn default_api_key_env() -> Option<String> {
    Some("OPENAI_API_KEY".to_owned())
}

fn default_temperature() -> Option<f32> {
    Some(0.1)
}

fn default_top_p() -> Option<f32> {
    Some(1.0)
}

fn default_max_output_tokens() -> Option<u32> {
    Some(5000)
}

fn default_seed() -> Option<i64> {
    Some(42)
}

fn default_request_timeout() -> Duration {
    Duration::from_secs(120)
}

const fn default_stream() -> bool {
    false
}
