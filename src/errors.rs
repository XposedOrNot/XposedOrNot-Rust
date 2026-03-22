//! Error types for the XposedOrNot client.

/// Errors that can occur when interacting with the XposedOrNot API.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Rate limit exceeded (HTTP 429). Contains the retry-after duration if available.
    #[error("rate limit exceeded: {message}")]
    RateLimit {
        /// Human-readable description of the rate limit error.
        message: String,
    },

    /// Resource not found (HTTP 404). The requested email or breach was not found.
    #[error("not found: {message}")]
    NotFound {
        /// Human-readable description of what was not found.
        message: String,
    },

    /// Authentication failure (HTTP 401/403). The API key is missing or invalid.
    #[error("authentication failed: {message}")]
    Authentication {
        /// Human-readable description of the authentication error.
        message: String,
    },

    /// Validation error. Input parameters failed client-side validation.
    #[error("validation error: {message}")]
    Validation {
        /// Human-readable description of the validation error.
        message: String,
    },

    /// Network or connection error from the underlying HTTP client.
    #[error("network error: {source}")]
    Network {
        /// The underlying reqwest error.
        #[from]
        source: reqwest::Error,
    },

    /// API returned an unexpected response or server error.
    #[error("API error (status {status_code}): {message}")]
    Api {
        /// The HTTP status code returned by the API.
        status_code: u16,
        /// Human-readable error message.
        message: String,
    },
}

/// A type alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;
