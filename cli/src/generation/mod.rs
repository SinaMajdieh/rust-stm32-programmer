mod ollama;
mod openai;
mod provider;

use anyhow::Result;

use crate::{cli::Provider, config::Config};

pub use provider::LlmProvider;

/// Generates firmware source code using the selected provider.
///
/// The provider-specific implementation is responsible for communicating
/// with the underlying API and reporting provider-specific generation
/// statistics.
pub async fn generate_code(
    config: &Config,
    provider: Provider,
    model: &str,
    prompt: &[String],
) -> Result<String> {
    let prompt = prompt.join(" ");

    if prompt.trim().is_empty() {
        anyhow::bail!("prompt cannot be empty");
    }

    match provider {
        Provider::Ollama => {
            let provider = ollama::OllamaProvider::new(config);
            provider.generate(model, &prompt).await
        }

        Provider::OpenAi => {
            let provider = openai::OpenAiProvider::new(config)?;
            provider.generate(model, &prompt).await
        }
    }
}

/// Removes Markdown code fences from an LLM response.
fn unfence_code(code: &str) -> &str {
    let code = code.trim();

    let Some(code) = code.strip_prefix("```") else {
        return code;
    };

    let code = code
        .strip_prefix("c\n")
        .or_else(|| code.strip_prefix("C\n"))
        .or_else(|| code.strip_prefix('\n'))
        .unwrap_or(code);

    code.strip_suffix("```").unwrap_or(code).trim()
}
