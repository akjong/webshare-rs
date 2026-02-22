//! Core HTTP client for the Webshare API.
//!
//! [`WebshareClient`] is the central struct that all API methods are called on.
//! It wraps an [`hpx::Client`], stores the base URL and an optional auth token,
//! and exposes internal helpers for building and sending requests.
//!
//! # Quick start
//!
//! ```no_run
//! use webshare_rs::client::WebshareClient;
//!
//! // Shorthand — uses default base URL and 30-second timeout.
//! let client = WebshareClient::new("my-api-token");
//!
//! // Full control via the builder.
//! let client = WebshareClient::builder()
//!     .token("my-api-token")
//!     .base_url("https://custom.proxy.endpoint")
//!     .timeout(std::time::Duration::from_secs(60))
//!     .build()
//!     .expect("valid client configuration");
//! ```

use std::{fmt, time::Duration};

use serde::de::DeserializeOwned;

use crate::error::{Result, WebshareError};

/// Default Webshare API base URL.
const DEFAULT_BASE_URL: &str = "https://proxy.webshare.io";

/// Default HTTP request timeout.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

/// A builder for constructing a [`WebshareClient`] with custom settings.
///
/// Obtain one via [`WebshareClient::builder()`].
///
/// # Example
///
/// ```no_run
/// use webshare_rs::client::WebshareClient;
///
/// let client = WebshareClient::builder()
///     .token("my-api-token")
///     .base_url("https://custom.proxy.endpoint")
///     .timeout(std::time::Duration::from_secs(60))
///     .build()
///     .expect("valid client configuration");
/// ```
pub struct WebshareClientBuilder {
    token: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
}

impl WebshareClientBuilder {
    /// Create a new builder with all fields set to `None` (defaults will be
    /// applied at build time).
    pub fn new() -> Self {
        Self {
            token: None,
            base_url: None,
            timeout: None,
        }
    }

    /// Set the API authentication token.
    ///
    /// When set, all *authenticated* requests include the header
    /// `Authorization: Token <TOKEN>`.
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Override the default base URL (`https://proxy.webshare.io`).
    ///
    /// The provided URL **must not** have a trailing slash.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Override the default request timeout (30 seconds).
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Consume the builder and create a [`WebshareClient`].
    ///
    /// # Errors
    ///
    /// Returns [`WebshareError::Config`] if the underlying HTTP client cannot
    /// be constructed (e.g. TLS back-end unavailable).
    pub fn build(self) -> Result<WebshareClient> {
        let timeout = self.timeout.unwrap_or(DEFAULT_TIMEOUT);

        let http = hpx::Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|e| WebshareError::Config(format!("failed to build HTTP client: {e}")))?;

        let base_url = self.base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_owned());

        Ok(WebshareClient {
            http,
            base_url,
            token: self.token,
        })
    }
}

impl Default for WebshareClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Client
// ---------------------------------------------------------------------------

/// The Webshare API client.
///
/// All API operations are implemented as methods on this struct.  Use
/// [`WebshareClient::new`] for a quick one-liner or [`WebshareClient::builder`]
/// for full customisation.
pub struct WebshareClient {
    /// The underlying HTTP client.
    http: hpx::Client,
    /// Base URL for all API requests (no trailing slash).
    base_url: String,
    /// Optional bearer-style API token.
    token: Option<String>,
}

impl WebshareClient {
    /// Create a client with the given API token, using the default base URL and
    /// a 30-second timeout.
    ///
    /// This is equivalent to:
    ///
    /// ```no_run
    /// # use webshare_rs::client::WebshareClient;
    /// WebshareClient::builder()
    ///     .token("my-api-token")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn new(token: impl Into<String>) -> Self {
        // `build()` only fails when the TLS back-end is missing which should
        // not happen with the default TLS feature enabled.
        Self::builder()
            .token(token)
            .build()
            .expect("default client configuration should always succeed")
    }

    /// Return a [`WebshareClientBuilder`] for constructing a client with
    /// custom settings.
    pub fn builder() -> WebshareClientBuilder {
        WebshareClientBuilder::new()
    }

    // -- Accessors ----------------------------------------------------------

    /// Return the configured base URL.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Return `true` if a token has been configured.
    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // -- Authenticated request builders (pub(crate)) ------------------------

    /// Build an authenticated request for the given HTTP `method` and `path`.
    ///
    /// The full URL is constructed as `{base_url}{path}`.  If a token is set
    /// the `Authorization: Token <TOKEN>` header is added automatically.
    pub(crate) fn request(&self, method: hpx::Method, path: &str) -> hpx::RequestBuilder {
        let url = format!("{}{}", self.base_url, path);
        let mut builder = self.http.request(method, &url);
        if let Some(ref token) = self.token {
            builder = builder.header("Authorization", format!("Token {token}"));
        }
        builder
    }

    /// Shorthand for an authenticated `GET` request.
    pub(crate) fn get(&self, path: &str) -> hpx::RequestBuilder {
        self.request(hpx::Method::GET, path)
    }

    /// Shorthand for an authenticated `POST` request.
    pub(crate) fn post(&self, path: &str) -> hpx::RequestBuilder {
        self.request(hpx::Method::POST, path)
    }

    /// Shorthand for an authenticated `PATCH` request.
    pub(crate) fn patch(&self, path: &str) -> hpx::RequestBuilder {
        self.request(hpx::Method::PATCH, path)
    }

    /// Shorthand for an authenticated `DELETE` request.
    pub(crate) fn delete(&self, path: &str) -> hpx::RequestBuilder {
        self.request(hpx::Method::DELETE, path)
    }

    // -- Unauthenticated request builders (pub(crate)) ----------------------

    /// Build an **unauthenticated** request for the given HTTP `method` and
    /// `path`.
    ///
    /// No `Authorization` header is added regardless of whether a token is
    /// configured.  This is required by a handful of public endpoints (e.g.
    /// registration and activation).
    pub(crate) fn request_unauthed(&self, method: hpx::Method, path: &str) -> hpx::RequestBuilder {
        let url = format!("{}{}", self.base_url, path);
        self.http.request(method, &url)
    }

    /// Shorthand for an unauthenticated `GET` request.
    pub(crate) fn get_unauthed(&self, path: &str) -> hpx::RequestBuilder {
        self.request_unauthed(hpx::Method::GET, path)
    }

    /// Shorthand for an unauthenticated `POST` request.
    pub(crate) fn post_unauthed(&self, path: &str) -> hpx::RequestBuilder {
        self.request_unauthed(hpx::Method::POST, path)
    }

    // -- Response helpers (pub(crate)) --------------------------------------

    /// Send a request and deserialize the JSON response body into `T`.
    ///
    /// Non-success status codes are mapped to [`WebshareError::Api`] with the
    /// raw body text and, when possible, a parsed JSON `details` object.
    pub(crate) async fn send_json<T: DeserializeOwned>(
        &self,
        request: hpx::RequestBuilder,
    ) -> Result<T> {
        let response = request.send().await.map_err(WebshareError::Transport)?;
        let status = response.status();

        if status.is_success() {
            response.json::<T>().await.map_err(WebshareError::Transport)
        } else {
            let body = response.text().await.unwrap_or_default();
            let details = serde_json::from_str(&body).ok();
            Err(WebshareError::Api {
                status: status.as_u16(),
                message: body,
                details,
            })
        }
    }

    /// Send a request and expect an empty (204 No Content) response.
    ///
    /// Any non-success status code is mapped to [`WebshareError::Api`].
    pub(crate) async fn send_no_content(&self, request: hpx::RequestBuilder) -> Result<()> {
        let response = request.send().await.map_err(WebshareError::Transport)?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let body = response.text().await.unwrap_or_default();
            let details = serde_json::from_str(&body).ok();
            Err(WebshareError::Api {
                status: status.as_u16(),
                message: body,
                details,
            })
        }
    }

    /// Send a request and return the response body as a plain `String`.
    ///
    /// Non-success status codes are mapped to [`WebshareError::Api`].
    pub(crate) async fn send_text(&self, request: hpx::RequestBuilder) -> Result<String> {
        let response = request.send().await.map_err(WebshareError::Transport)?;
        let status = response.status();

        if status.is_success() {
            response.text().await.map_err(WebshareError::Transport)
        } else {
            let body = response.text().await.unwrap_or_default();
            let details = serde_json::from_str(&body).ok();
            Err(WebshareError::Api {
                status: status.as_u16(),
                message: body,
                details,
            })
        }
    }

    /// Send a request and return the raw response bytes.
    ///
    /// Non-success status codes are mapped to [`WebshareError::Api`].
    pub(crate) async fn send_bytes(&self, request: hpx::RequestBuilder) -> Result<Vec<u8>> {
        let response = request.send().await.map_err(WebshareError::Transport)?;
        let status = response.status();

        if status.is_success() {
            response
                .bytes()
                .await
                .map(|b| b.to_vec())
                .map_err(WebshareError::Transport)
        } else {
            let body = response.text().await.unwrap_or_default();
            let details = serde_json::from_str(&body).ok();
            Err(WebshareError::Api {
                status: status.as_u16(),
                message: body,
                details,
            })
        }
    }
}

// ---------------------------------------------------------------------------
// Debug — redact the token
// ---------------------------------------------------------------------------

impl fmt::Debug for WebshareClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebshareClient")
            .field("base_url", &self.base_url)
            .field("token", &self.token.as_ref().map(|_| "[REDACTED]"))
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_client_with_token() {
        let client = WebshareClient::new("test-token");
        assert!(client.has_token());
        assert_eq!(client.base_url(), DEFAULT_BASE_URL);
    }

    #[test]
    fn builder_creates_client_with_defaults() {
        let client = WebshareClient::builder()
            .token("x")
            .build()
            .expect("should build");

        assert!(client.has_token());
        assert_eq!(client.base_url(), DEFAULT_BASE_URL);
    }

    #[test]
    fn builder_custom_base_url() {
        let client = WebshareClient::builder()
            .token("x")
            .base_url("https://example.com")
            .build()
            .expect("should build");

        assert_eq!(client.base_url(), "https://example.com");
    }

    #[test]
    fn builder_without_token() {
        let client = WebshareClient::builder().build().expect("should build");

        assert!(!client.has_token());
    }

    #[test]
    fn debug_redacts_token() {
        let client = WebshareClient::new("super-secret");
        let debug_output = format!("{:?}", client);

        assert!(
            debug_output.contains("[REDACTED]"),
            "token should be redacted in debug output"
        );
        assert!(
            !debug_output.contains("super-secret"),
            "actual token must not appear in debug output"
        );
    }

    #[test]
    fn debug_shows_none_when_no_token() {
        let client = WebshareClient::builder().build().expect("should build");
        let debug_output = format!("{:?}", client);

        assert!(
            debug_output.contains("None"),
            "debug output should show None when no token is set"
        );
    }
}
