use anyhow::Result;

/// Interface implemented by LLM generation providers.
///
/// Providers are responsible for translating the common generation request
/// into the API-specific request format and returning the generated source
/// code.
pub trait LlmProvider {
    /// Generates source code using `model` in response to `prompt`.
    async fn generate(&self, model: &str, prompt: &str) -> Result<String>;
}
