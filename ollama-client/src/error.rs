use url::ParseError;

/// An error produced while configuring or communicating with Ollama.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An HTTP request or response operation failed.
    #[error("Ollama request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// The configured Ollama URL was invalid.
    #[error("invalid Ollama URL: {0}")]
    Url(#[from] ParseError),

    #[error("Ollama returned HTTP {status}: {body}")]
    OllamaStatus {
        status: reqwest::StatusCode,
        body: String,
    },
}

/// A result returned by this crate.
pub type Result<T> = std::result::Result<T, Error>;
