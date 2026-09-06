//! Abstraction implemented by all generation providers.
//!
//! The provider abstraction separates provider-specific API clients from the
//! provider-independent generation workflow used by [`crate::LlmGenerator`].

use crate::{GenerationError, GenerationOutput, GenerationRequest};

/// Backend capable of generating source code from a request.
///
/// Providers translate the common [`GenerationRequest`] into provider-specific
/// API calls and normalize successful responses into [`GenerationOutput`].
pub(crate) trait GenerationProvider: Send + Sync {
    /// Generates source code for `request`.
    ///
    /// # Errors
    ///
    /// Returns a [`GenerationError`] if the provider request fails or the
    /// provider response cannot be converted into a [`GenerationOutput`].
    async fn generate(
        &self,
        request: GenerationRequest<'_>,
    ) -> Result<GenerationOutput, GenerationError>;
}
