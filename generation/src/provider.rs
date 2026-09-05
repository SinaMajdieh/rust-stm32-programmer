//! Abstraction implemented by all generation providers.

use crate::{GenerationError, GenerationOutput, GenerationRequest};

/// Backend capable of generating source code from a request.
///
/// Providers translate the common request into provider-specific API calls
/// and normalize the response into [`GenerationOutput`].
pub(crate) trait GenerationProvider: Send + Sync {
    /// Generates source code for `request`.
    async fn generate(
        &self,
        request: GenerationRequest<'_>,
    ) -> Result<GenerationOutput, GenerationError>;
}
