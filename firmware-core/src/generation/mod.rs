mod ollama;
mod openai;
mod provider;

use crate::{Config, GenerationError};

pub use provider::{GenerationOutput, GenerationStatistics, LlmProvider, Provider};

/// Generates firmware source code using the selected provider.
pub async fn generate_code(
    config: &Config,
    provider: Provider,
    model: &str,
    prompt: &[String],
) -> Result<GenerationOutput, GenerationError> {
    let prompt = prompt.join(" ");

    if prompt.trim().is_empty() {
        return Err(GenerationError::EmptyPrompt);
    }

    match provider {
        Provider::Ollama => {
            ollama::OllamaProvider::new(config)
                .generate(model, &prompt)
                .await
        }

        Provider::OpenAi => {
            openai::OpenAiProvider::new(config)?
                .generate(model, &prompt)
                .await
        }
    }
}

/// Removes Markdown code fences from generated source code.
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
