mod wire;

use std::time::Duration;

use serde::Deserialize;
use serde_json::{Map, Value};

use crate::{OllamaClient, Result};
use wire::{ListLoadedModelsResponse, ListModelsResponse, ShowModelRequest};

const LIST_MODELS_ENDPOINT: &str = "/api/tags";
const LIST_LOADED_MODELS_ENDPOINT: &str = "/api/ps";
const SHOW_MODEL_ENDPOINT: &str = "/api/show";

/// Details about the architecture and configuration of a model.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct ModelDetails {
    /// The parent model from which this model was derived, if any.
    #[serde(default)]
    pub parent_model: String,

    /// The model's file format.
    pub format: String,

    /// The model family.
    pub family: String,

    /// Additional model families associated with the model.
    #[serde(default)]
    pub families: Vec<String>,

    /// The approximate number of parameters in the model.
    pub parameter_size: String,

    /// The quantization level used by the model.
    pub quantization_level: String,
}

/// Information about a model installed on an Ollama server.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct InstalledModel {
    /// The model's name.
    pub name: String,

    /// The time at which the model was last modified.
    pub modified_at: String,

    /// The model's size in bytes.
    #[serde(rename = "size")]
    pub size_bytes: u64,

    /// The model's content digest.
    pub digest: String,

    /// Details about the model's architecture and configuration.
    pub details: ModelDetails,
}

/// Information about a model currently loaded by an Ollama server.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub struct LoadedModel {
    /// The model's name.
    pub name: String,

    /// The model's size in bytes.
    #[serde(rename = "size")]
    pub size_bytes: u64,

    /// The model's content digest.
    pub digest: String,

    /// Details about the model's architecture and configuration.
    pub details: ModelDetails,

    /// The time at which the model is expected to be unloaded.
    pub expires_at: String,

    /// The amount of VRAM occupied by the model, in bytes.
    #[serde(rename = "size_vram")]
    pub vram_size_bytes: u64,

    /// The model's context length in tokens.
    pub context_length: u64,
}

/// Detailed metadata associated with an Ollama model.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ModelMetadata {
    /// The number or description of parameters reported by Ollama.
    #[serde(default)]
    pub parameters: String,

    /// The model's license information.
    #[serde(default)]
    pub license: String,

    /// The time at which the model was last modified.
    #[serde(default)]
    pub modified_at: String,

    /// Details about the model's architecture and configuration.
    pub details: ModelDetails,

    /// The prompt template associated with the model.
    #[serde(default)]
    pub template: String,

    /// Capabilities reported by the model.
    #[serde(default)]
    pub capabilities: Vec<String>,

    /// Additional model information returned by Ollama.
    ///
    /// The contents of this map depend on the model and may vary between
    /// models or Ollama versions.
    #[serde(default, rename = "model_info")]
    pub raw_model_info: Map<String, Value>,
}

impl OllamaClient {
    /// Lists the models installed on the Ollama server.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails, times out, Ollama returns an
    /// unsuccessful HTTP status, or the response cannot be deserialized.
    pub async fn list_models(&self, timeout: Duration) -> Result<Vec<InstalledModel>> {
        let request = self.get(LIST_MODELS_ENDPOINT)?;

        let response: ListModelsResponse = self.execute_json(request, timeout).await?;

        Ok(response.models)
    }

    /// Lists the models currently loaded by the Ollama server.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails, times out, Ollama returns an
    /// unsuccessful HTTP status, or the response cannot be deserialized.
    pub async fn loaded_models(&self, timeout: Duration) -> Result<Vec<LoadedModel>> {
        let request = self.get(LIST_LOADED_MODELS_ENDPOINT)?;

        let response: ListLoadedModelsResponse = self.execute_json(request, timeout).await?;

        Ok(response.models)
    }

    /// Retrieves detailed metadata for a model.
    ///
    /// `model` is the name of the model to inspect.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails, times out, Ollama returns an
    /// unsuccessful HTTP status, or the response cannot be deserialized.
    pub async fn model_metadata(&self, model: &str, timeout: Duration) -> Result<ModelMetadata> {
        let body = ShowModelRequest::new(model);

        let request = self.post(SHOW_MODEL_ENDPOINT)?.json(&body);

        self.execute_json(request, timeout).await
    }
}

#[cfg(test)]
mod tests;
