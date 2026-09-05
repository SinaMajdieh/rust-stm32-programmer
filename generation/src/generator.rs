//! High-level LLM generation coordinator.

use std::collections::HashMap;

use crate::{
    GenerationError, GenerationOutput, GenerationProvider, GenerationRequest, GeneratorConfig,
    Model, ModelId, OllamaProvider, OpenAiProvider, Provider,
};

/// Coordinates generation across all configured LLM providers.
///
/// The generator owns provider instances and indexes configured models by
/// their fully qualified [`ModelId`].
pub struct LlmGenerator {
    ollama: OllamaProvider,
    openai: OpenAiProvider,
    available_models: HashMap<ModelId, Model>,
}

impl LlmGenerator {
    /// Creates a generator from application configuration.
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
    pub fn available_models(&self) -> impl Iterator<Item = &Model> {
        self.available_models.values()
    }

    /// Generates source code using the provider associated with the request's model.
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

    /// Looks up a model by its qualified ID.
    fn find_model(&self, id: &str) -> Result<&Model, GenerationError> {
        let id = ModelId::from(id);

        self.available_models
            .get(&id)
            .ok_or_else(|| GenerationError::ModelNotFound {
                model: id.to_string(),
            })
    }

    /// Returns a reference to the Ollama provider.
    pub fn ollama(&self) -> &OllamaProvider {
        &self.ollama
    }

    /// Returns a reference to the OpenAI provider.
    pub fn openai(&self) -> &OpenAiProvider {
        &self.openai
    }
}

fn index_models(models: Vec<Model>) -> HashMap<ModelId, Model> {
    models
        .into_iter()
        .map(|model| (model.id(), model))
        .collect()
}
