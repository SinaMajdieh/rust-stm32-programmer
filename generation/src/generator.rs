//! High-level LLM generation coordinator.
//!
//! This module resolves configured model identifiers to providers and delegates
//! generation to the appropriate provider implementation.

use std::collections::HashMap;

use crate::{
    GenerationError, GenerationOutput, GenerationProvider, GenerationRequest, GeneratorConfig,
    Model, ModelId, OllamaProvider, OpenAiProvider, Provider,
};

/// Coordinates generation across all configured LLM providers.
///
/// The generator owns the configured provider instances and indexes available
/// models by their fully qualified [`ModelId`]. Generation requests are
/// validated, resolved to a model, and then delegated to that model's provider.
pub struct LlmGenerator {
    ollama: OllamaProvider,
    openai: OpenAiProvider,
    available_models: HashMap<ModelId, Model>,
}

impl LlmGenerator {
    /// Creates a generator from application configuration.
    ///
    /// Provider instances are initialized from the corresponding provider
    /// configuration, while the configured models are indexed for lookup.
    ///
    /// # Errors
    ///
    /// Returns a [`GenerationError`] if either provider cannot be initialized.
    pub fn from_config(config: GeneratorConfig) -> Result<Self, GenerationError> {
        let ollama = OllamaProvider::new(config.ollama)?;
        let openai = OpenAiProvider::new(config.openai)?;

        Ok(Self {
            ollama,
            openai,
            available_models: index_models(config.available_models),
        })
    }

    /// Creates a generator from already-initialized providers and models.
    ///
    /// This constructor is useful when provider initialization needs to be
    /// controlled by the caller or when providers are constructed
    /// independently of [`GeneratorConfig`].
    pub fn new(
        ollama: OllamaProvider,
        openai: OpenAiProvider,
        available_models: Vec<Model>,
    ) -> Self {
        Self {
            ollama,
            openai,
            available_models: index_models(available_models),
        }
    }

    /// Returns all models currently available to the generator.
    ///
    /// The returned iterator does not guarantee any particular ordering.
    pub fn available_models(&self) -> impl Iterator<Item = &Model> {
        self.available_models.values()
    }

    /// Generates source code using the provider associated with the request's model.
    ///
    /// The request is first validated and its model identifier is resolved
    /// against the configured models. The request is then delegated to the
    /// provider associated with the resolved model.
    ///
    /// # Errors
    ///
    /// Returns a [`GenerationError`] if the request is invalid, the requested
    /// model is unavailable, or the selected provider fails to generate the
    /// response.
    pub async fn generate(
        &self,
        request: GenerationRequest<'_>,
    ) -> Result<GenerationOutput, GenerationError> {
        request.validate()?;

        let model = self.find_model(request.model)?;

        let provider_request = GenerationRequest {
            model: &model.name,
            prompt: request.prompt,
            system_prompt: request.system_prompt,
        };

        match model.provider {
            Provider::Ollama => self.ollama.generate(provider_request).await,
            Provider::OpenAi => self.openai.generate(provider_request).await,
        }
    }

    /// Looks up a model by its fully qualified identifier.
    fn find_model(&self, id: &str) -> Result<&Model, GenerationError> {
        let id = ModelId::from(id);

        self.available_models
            .get(&id)
            .ok_or_else(|| GenerationError::ModelNotFound {
                model: id.to_string(),
            })
    }

    /// Returns a reference to the configured Ollama provider.
    pub fn ollama(&self) -> &OllamaProvider {
        &self.ollama
    }

    /// Returns a reference to the configured OpenAI provider.
    pub fn openai(&self) -> &OpenAiProvider {
        &self.openai
    }
}

/// Indexes models by their fully qualified identifiers.
fn index_models(models: Vec<Model>) -> HashMap<ModelId, Model> {
    models
        .into_iter()
        .map(|model| (model.id(), model))
        .collect()
}
