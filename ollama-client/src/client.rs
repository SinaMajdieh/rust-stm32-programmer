use reqwest::{Client, Response, Url};
use serde::Deserialize;

use crate::{
    Error,
    error::Result,
    generation::{GenerateBody, GenerateRequest, Generation},
};

async fn ensure_success(response: Response) -> Result<Response> {
    let status = response.status();

    if status.is_success() {
        return Ok(response);
    }

    let body = response
        .text()
        .await
        .unwrap_or_else(|error| format!("Failed to read response body: {error}"));

    Err(Error::OllamaStatus { status, body })
}

/// An asynchronous client for communicating with an Ollama server.
///
/// Cloning this type is inexpensive because Reqwest shares its internal
/// connection pool between clones.
#[derive(Debug, Clone)]
pub struct OllamaClient {
    http: Client,
    base_url: Url,
}

impl OllamaClient {
    /// Creates a client connected to the given Ollama server.
    ///
    /// The base URL should identify the server root, for example:
    ///
    /// `http://localhost:11434`
    ///
    /// # Errors
    ///
    /// Returns an error if the URL is invalid or the HTTP client cannot be
    /// constructed.
    pub fn new(base_url: &str) -> Result<Self> {
        Ok(Self {
            http: Client::builder().build()?,
            base_url: Url::parse(base_url)?,
        })
    }

    /// Retrieves the running Ollama server version.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails, Ollama returns an unsuccessful
    /// status code, or the response cannot be deserialized.
    pub async fn version(&self) -> Result<Version> {
        let url = self.base_url.join("/api/version")?;

        let response = self.http.get(url).send().await?;
        let response = ensure_success(response).await?;
        let version = response.json::<Version>().await?;

        Ok(version)
    }

    /// Generates a complete, non-streaming response.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails, Ollama returns an unsuccessful
    /// status code, or the response cannot be deserialized.
    pub async fn generate(&self, request: &GenerateRequest) -> Result<Generation> {
        let url = self.base_url.join("/api/generate")?;
        let body = GenerateBody::from(request);

        let response = self.http.post(url).json(&body).send().await?;
        let response = ensure_success(response).await?;
        let generation = response.json::<Generation>().await?;

        Ok(generation)
    }
}

/// Version information returned by Ollama.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Version {
    version: String,
}

impl Version {
    /// Returns the Ollama version string.
    pub fn as_str(&self) -> &str {
        &self.version
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;

    #[test]
    fn rejects_an_invalid_base_url() {
        let result = OllamaClient::new("not a valid URL");

        assert!(matches!(result, Err(Error::Url(_))));
    }
}
