use std::time::Duration;

use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use url::ParseError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Ollama request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Invalid Ollama Url: {0}")]
    Url(#[from] ParseError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct OllamaClient {
    http: Client,
    base_url: Url,
}

impl OllamaClient {
    pub fn new(base_url: &str) -> Result<Self> {
        Ok(Self {
            http: Client::builder().build()?,
            base_url: Url::parse(base_url)?,
        })
    }

    pub async fn version(&self) -> Result<Version> {
        let url = self.base_url.join("/api/version")?;
        let version = self
            .http
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<Version>()
            .await?;

        Ok(version)
    }

    pub async fn generate(&self, request: &GenerateRequest) -> Result<Generation> {
        let url = self.base_url.join("/api/generate")?;
        let body = GenerateBody {
            model: request.model(),
            prompt: request.prompt(),
            stream: false,
        };

        let response = self
            .http
            .post(url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json::<Generation>()
            .await?;

        Ok(response)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Version {
    version: String,
}

impl Version {
    pub fn as_str(&self) -> &str {
        &self.version
    }
}

#[derive(Debug, Clone)]
pub struct GenerateRequest {
    model: String,
    prompt: String,
}

impl GenerateRequest {
    pub fn new(model: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            prompt: prompt.into(),
        }
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn prompt(&self) -> &str {
        &self.prompt
    }
}

#[derive(Serialize)]
struct GenerateBody<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Generation {
    response: String,

    #[serde(default)]
    thinking: String,

    done: bool,
    done_reason: Option<String>,

    total_duration: u64,
    load_duration: u64,
    prompt_eval_count: u64,
    prompt_eval_duration: u64,
    eval_count: u64,
    eval_duration: u64,
}

impl Generation {
    pub fn response(&self) -> &str {
        &self.response
    }

    pub fn thinking(&self) -> &str {
        &self.thinking
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn done_reason(&self) -> Option<&str> {
        self.done_reason.as_deref()
    }

    pub fn total_duration(&self) -> Duration {
        Duration::from_nanos(self.total_duration)
    }

    pub fn load_duration(&self) -> Duration {
        Duration::from_nanos(self.load_duration)
    }

    pub fn prompt_evaluation_duration(&self) -> Duration {
        Duration::from_nanos(self.prompt_eval_duration)
    }

    pub fn evaluation_duration(&self) -> Duration {
        Duration::from_nanos(self.eval_duration)
    }

    pub fn prompt_tokens(&self) -> u64 {
        self.prompt_eval_count
    }

    pub fn generated_tokens(&self) -> u64 {
        self.eval_count
    }

    pub fn tokens_per_second(&self) -> Option<f64> {
        let seconds = self.evaluation_duration().as_secs_f64();

        if seconds == 0.0 {
            return None;
        }

        Some(self.generated_tokens() as f64 / seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_an_invalid_base_url() {
        let result = OllamaClient::new("not a valid URL");
        assert!(matches!(result, Err(Error::Url(_))));
    }
}
