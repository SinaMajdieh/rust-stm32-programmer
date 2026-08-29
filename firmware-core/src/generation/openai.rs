use std::time::{Duration, Instant};

use async_openai::{
    Client,
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CompletionUsage, CreateChatCompletionRequestArgs,
    },
};

use crate::{Config, GenerationError, GenerationOutput, GenerationStatistics, OpenAIClientError};

use super::{LlmProvider, unfence_code};

/// LLM provider backed by an OpenAI-compatible API.
pub struct OpenAiProvider<'a> {
    config: &'a Config,
    client: Client<OpenAIConfig>,
}

impl<'a> OpenAiProvider<'a> {
    /// Creates an OpenAI-compatible provider.
    pub fn new(config: &'a Config) -> Result<Self, GenerationError> {
        let api_key = std::env::var(&config.openai.api_key_env)
            .map_err(|_| OpenAIClientError::ApiKey(config.openai.api_key_env.clone()))?;

        let openai_config = OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(&config.openai.url);

        Ok(Self {
            config,
            client: Client::with_config(openai_config),
        })
    }

    async fn generate_inner(
        &self,
        model: &str,
        prompt: &str,
    ) -> Result<GenerationOutput, OpenAIClientError> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .temperature(self.config.generation.temperature)
            .max_completion_tokens(self.config.generation.max_output_tokens)
            .seed(self.config.generation.seed as i64)
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(self.config.system_prompt.as_str())
                    .build()?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(prompt)
                    .build()?
                    .into(),
            ])
            .build()?;

        let start = Instant::now();
        let response = self.client.chat().create(request).await?;

        let elapsed = start.elapsed();

        let choice = response
            .choices
            .first()
            .ok_or(OpenAIClientError::NoChoices)?;

        let code = unfence_code(choice.message.content.as_deref().unwrap_or(""));

        let statistics = statistics(response.usage.as_ref(), elapsed);

        Ok(GenerationOutput {
            code: code.to_owned(),
            statistics,
        })
    }
}

impl LlmProvider for OpenAiProvider<'_> {
    async fn generate(
        &self,
        model: &str,
        prompt: &str,
    ) -> Result<GenerationOutput, GenerationError> {
        self.generate_inner(model, prompt)
            .await
            .map_err(GenerationError::from)
    }
}

fn statistics(usage: Option<&CompletionUsage>, elapsed: Duration) -> GenerationStatistics {
    GenerationStatistics {
        prompt_tokens: usage.map(|usage| usage.prompt_tokens as u64),
        generated_tokens: usage.map_or(0, |usage| usage.completion_tokens as u64),
        elapsed,
    }
}
