use generation::GenerationOutput;
use serde::{Deserialize, Serialize};

use super::{revision::Revision, stage::Stage};

/// Input passed to the language model for source generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRequest {
    /// Model identifier.
    pub model: String,

    /// User prompt.
    pub prompt: String,

    /// Optional system prompt.
    pub system_prompt: Option<String>,
}

impl GenerationRequest {
    pub fn new(
        model: impl Into<String>,
        prompt: impl Into<String>,
        system_prompt: Option<impl Into<String>>,
    ) -> Self {
        Self {
            model: model.into(),
            prompt: prompt.into(),
            system_prompt: system_prompt.map(Into::into),
        }
    }
}

/// Generated firmware source code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generation {
    revision: Revision,
    request: GenerationRequest,
    result: GenerationOutput,
}

impl Generation {
    pub(crate) fn new(request: GenerationRequest, result: GenerationOutput) -> Self {
        Self {
            revision: Revision::new(),
            request,
            result,
        }
    }

    pub fn code(&self) -> &str {
        &self.result.code
    }

    pub fn request(&self) -> &GenerationRequest {
        &self.request
    }

    pub fn result(&self) -> &GenerationOutput {
        &self.result
    }
}

impl Stage for Generation {
    fn revision(&self) -> Revision {
        self.revision
    }

    fn source_revision(&self) -> Option<Revision> {
        None
    }
}
