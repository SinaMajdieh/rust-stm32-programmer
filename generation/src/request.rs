//! Generation request types.

use crate::GenerationError;

/// Input to a generation request.
///
/// The request borrows its model identifier and textual data, allowing callers
/// to reuse existing strings without additional allocation.
#[derive(Debug, Clone)]
pub struct GenerationRequest<'a> {
    /// Qualified identifier of the model to use.
    pub model: &'a str,

    /// User prompt supplied to the model.
    pub prompt: &'a str,

    /// Optional system prompt.
    pub system_prompt: Option<&'a str>,
}

impl<'a> GenerationRequest<'a> {
    /// Creates a new generation request.
    pub fn new(model: &'a str, prompt: &'a str, system_prompt: Option<&'a str>) -> Self {
        Self {
            model,
            prompt,
            system_prompt,
        }
    }

    pub(crate) fn validate(&self) -> Result<(), GenerationError> {
        if self.model.trim().is_empty() {
            return Err(GenerationError::EmptyModel);
        }

        if self.prompt.trim().is_empty() {
            return Err(GenerationError::EmptyPrompt);
        }

        Ok(())
    }
}
