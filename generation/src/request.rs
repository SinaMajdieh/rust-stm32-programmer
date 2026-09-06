//! Generation request types.
//!
//! Requests contain only the model identifier and textual generation inputs.
//! Provider configuration and provider-specific options are handled by the
//! corresponding provider implementations.

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

    /// Optional system prompt supplied to the model.
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

    /// Validates the request before it is sent to a provider.
    ///
    /// A request is valid only when both the model identifier and user prompt
    /// contain non-whitespace characters.
    ///
    /// # Errors
    ///
    /// Returns [`GenerationError::EmptyModel`] if the model identifier is
    /// empty or contains only whitespace.
    ///
    /// Returns [`GenerationError::EmptyPrompt`] if the prompt is empty or
    /// contains only whitespace.
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
