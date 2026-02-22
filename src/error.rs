//! Error types for the Webshare SDK.

use thiserror::Error;

/// The error type returned by all Webshare SDK operations.
#[derive(Debug, Error)]
pub enum WebshareError {
    /// An HTTP transport-level error from the underlying client.
    #[error("HTTP transport error: {0}")]
    Transport(#[from] hpx::Error),

    /// A JSON serialization or deserialization error.
    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// An error returned by the Webshare API.
    #[error("API error (HTTP {status}): {message}")]
    Api {
        /// The HTTP status code.
        status: u16,
        /// A human-readable error message.
        message: String,
        /// Optional structured details from the API response.
        details: Option<serde_json::Value>,
    },

    /// A client configuration error.
    #[error("Configuration error: {0}")]
    Config(String),
}

/// A convenience `Result` type that uses [`WebshareError`].
pub type Result<T> = std::result::Result<T, WebshareError>;
