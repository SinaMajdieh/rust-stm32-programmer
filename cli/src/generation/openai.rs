use std::time::Instant;

use anyhow::{Context, Result};
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
};

use crate::{config::Config, generation::unfence_code};

use super::provider::LlmProvider;

/// LLM provider backed by an OpenAI-compatible API.
pub struct OpenAiProvider<'a> {
    config: &'a Config,
    client: Client<OpenAIConfig>,
}

impl<'a> OpenAiProvider<'a> {
    /// Creates an OpenAI-compatible provider.
    pub fn new(config: &'a Config) -> Result<Self> {
        let api_key = std::env::var(&config.openai.api_key_env).with_context(|| {
            format!(
                "failed to read API key from environment \
                         variable `{}`",
                config.openai.api_key_env
            )
        })?;

        let openai_config = OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(&config.openai.url);

        let client = Client::with_config(openai_config);

        Ok(Self { config, client })
    }
}

impl LlmProvider for OpenAiProvider<'_> {
    async fn generate(&self, model: &str, prompt: &str) -> Result<String> {
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

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .context("OpenAI code generation failed")?;

        let elapsed = start.elapsed();

        let choice = response
            .choices
            .first()
            .context("OpenAI response contained no choices")?;

        let content = choice.message.content.as_deref().unwrap_or("");

        print_statistics(response.usage.as_ref(), elapsed);

        Ok(unfence_code(content).to_owned())
    }
}

/// Prints OpenAI generation statistics.
fn print_statistics(
    usage: Option<&async_openai::types::chat::CompletionUsage>,
    elapsed: std::time::Duration,
) {
    let Some(usage) = usage else {
        println!("Generation completed in {:.2}s.", elapsed.as_secs_f64());

        return;
    };

    let seconds = elapsed.as_secs_f64();

    let tokens_per_second = if seconds > 0.0 {
        usage.completion_tokens as f64 / seconds
    } else {
        0.0
    };

    println!("Prompt: {} tokens.", usage.prompt_tokens);

    println!("Generated: {} tokens.", usage.completion_tokens);

    println!("Total: {} tokens.", usage.total_tokens);

    println!(
        "Time: {:.2}s at {:.1} tokens/s.",
        seconds, tokens_per_second
    );
}
