use std::time::Duration;

use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid Ollama URL: {0}")]
    Url(#[from] url::ParseError),

    #[error("Ollama request failed: {0}")]
    Http(#[source] reqwest::Error),

    #[error("Ollama request timed out after {timeout:?}")]
    Timeout {
        timeout: Duration,

        #[source]
        source: reqwest::Error,
    },

    #[error("Ollama returned HTTP {status}: {message}")]
    Api { status: StatusCode, message: String },
}

impl Error {
    /// Returns `true` if the error was caused by a request timing out.
    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Timeout { .. })
    }

    /// Returns the HTTP status code returned by Ollama, if available.
    ///
    /// This returns `Some` only for [`Error::Api`] errors. Transport errors,
    /// URL parsing errors, and timeouts do not contain an HTTP response status.
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            Self::Api { status, .. } => Some(*status),
            _ => None,
        }
    }
}

/// A specialized [`Result`] type for Ollama API operations.
pub type Result<T> = std::result::Result<T, Error>;
