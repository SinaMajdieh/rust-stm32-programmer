use std::{fmt, time::Duration};

use serde::Deserialize;

use crate::{OllamaClient, Result};

const VERSION_ENDPOINT: &str = "/api/version";

/// The version of an Ollama server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version(String);

impl Version {
    /// Returns the version as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Version {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl OllamaClient {
    /// Retrieves the version of the connected Ollama server.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Timeout`](crate::Error::Timeout) if the request does not
    /// complete within `timeout`, [`Error::Http`](crate::Error::Http) if the
    /// request fails for another HTTP-related reason, or [`Error::Api`](crate::Error::Api)
    /// if Ollama returns an unsuccessful HTTP status.
    pub async fn version(&self, timeout: Duration) -> Result<Version> {
        let request = self.get(VERSION_ENDPOINT)?;

        let response: VersionResponse = self.execute_json(request, timeout).await?;

        Ok(Version(response.version))
    }
}

#[derive(Debug, Deserialize)]
struct VersionResponse {
    version: String,
}
