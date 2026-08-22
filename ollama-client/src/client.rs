use std::time::Duration;

use reqwest::{Client, RequestBuilder, Url};
use serde::{Deserialize, de::DeserializeOwned};

use crate::{Error, Result};

/// A client for communicating with an Ollama server.
#[derive(Debug, Clone)]
pub struct OllamaClient {
    http: Client,
    base_url: Url,
}

impl OllamaClient {
    /// Creates a client configured to communicate with the given Ollama server.
    ///
    /// `base_url` should point to the root URL of the Ollama server, such as
    /// `http://localhost:11434`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Url`] if `base_url` is not a valid URL.
    pub fn new(base_url: &str) -> Result<Self> {
        Ok(Self {
            http: Client::new(),
            base_url: Url::parse(base_url)?,
        })
    }

    pub(crate) fn get(&self, path: &str) -> Result<RequestBuilder> {
        let url = self.base_url.join(path)?;

        Ok(self.http.get(url))
    }

    pub(crate) fn post(&self, path: &str) -> Result<RequestBuilder> {
        let url = self.base_url.join(path)?;

        Ok(self.http.post(url))
    }

    pub(crate) async fn execute_json<T>(
        &self,
        request: RequestBuilder,
        timeout: Duration,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = request
            .timeout(timeout)
            .send()
            .await
            .map_err(|source| classify_http_error(source, timeout))?;

        let status = response.status();

        if !status.is_success() {
            let body = response
                .text()
                .await
                .map_err(|source| classify_http_error(source, timeout))?;

            return Err(Error::Api {
                status,
                message: api_error_message(body),
            });
        }

        response
            .json()
            .await
            .map_err(|source| classify_http_error(source, timeout))
    }
}

fn classify_http_error(source: reqwest::Error, timeout: Duration) -> Error {
    if source.is_timeout() {
        Error::Timeout { timeout, source }
    } else {
        Error::Http(source)
    }
}

fn api_error_message(body: String) -> String {
    serde_json::from_str::<ApiErrorBody>(&body)
        .map(|body| body.error)
        .unwrap_or(body)
}

#[derive(Debug, Deserialize)]
struct ApiErrorBody {
    error: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_an_invalid_base_url() {
        let result = OllamaClient::new("not a valid URL");

        assert!(matches!(result, Err(Error::Url(_))));
    }

    #[test]
    fn extracts_an_ollama_api_error_message() {
        let body = r#"{"error":"model not found"}"#.to_owned();

        let message = api_error_message(body);

        assert_eq!(message, "model not found");
    }

    #[test]
    fn preserves_a_non_json_error_message() {
        let message = api_error_message("server unavailable".to_owned());

        assert_eq!(message, "server unavailable");
    }
}
