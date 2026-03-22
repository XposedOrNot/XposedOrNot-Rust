//! HTTP client with builder pattern, rate limiting, and retry logic.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use tokio::sync::Mutex;
use tokio::time::Instant;

use crate::errors::{Error, Result};

/// Default base URL for the free XposedOrNot API.
pub const DEFAULT_BASE_URL: &str = "https://api.xposedornot.com";

/// Base URL for the Plus (commercial) API.
pub const PLUS_BASE_URL: &str = "https://plus-api.xposedornot.com";

/// Base URL for the password exposure API.
pub const PASSWORD_BASE_URL: &str = "https://passwords.xposedornot.com/api";

/// Default request timeout in seconds.
const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Default maximum number of retries on 429 responses.
const DEFAULT_MAX_RETRIES: u32 = 3;

/// Minimum interval between requests for rate limiting (free API).
const RATE_LIMIT_INTERVAL: Duration = Duration::from_secs(1);

/// Internal state for client-side rate limiting.
#[derive(Debug)]
pub(crate) struct RateLimitState {
    last_request: Option<Instant>,
}

/// Configuration produced by [`ClientBuilder`].
#[derive(Clone)]
pub struct ClientConfig {
    /// Base URL for the free API.
    pub base_url: String,
    /// Base URL for the Plus API.
    pub plus_base_url: String,
    /// Base URL for the password API.
    pub password_base_url: String,
    /// Request timeout.
    pub timeout: Duration,
    /// Maximum retries on 429.
    pub max_retries: u32,
    /// Optional API key for the Plus API.
    pub api_key: Option<String>,
    /// Additional custom headers.
    pub custom_headers: HashMap<String, String>,
}

impl std::fmt::Debug for ClientConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientConfig")
            .field("base_url", &self.base_url)
            .field("plus_base_url", &self.plus_base_url)
            .field("password_base_url", &self.password_base_url)
            .field("timeout", &self.timeout)
            .field("max_retries", &self.max_retries)
            .field("api_key", &self.api_key.as_ref().map(|_| "[REDACTED]"))
            .field("custom_headers", &self.custom_headers)
            .finish()
    }
}

/// Builder for constructing a [`Client`] with custom configuration.
///
/// # Examples
///
/// ```
/// use xposedornot::Client;
///
/// let client = Client::builder()
///     .api_key("my-secret-key")
///     .timeout_secs(60)
///     .build()
///     .unwrap();
/// ```
#[derive(Clone)]
pub struct ClientBuilder {
    base_url: Option<String>,
    plus_base_url: Option<String>,
    password_base_url: Option<String>,
    timeout_secs: Option<u64>,
    max_retries: Option<u32>,
    api_key: Option<String>,
    custom_headers: HashMap<String, String>,
}

impl std::fmt::Debug for ClientBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientBuilder")
            .field("base_url", &self.base_url)
            .field("plus_base_url", &self.plus_base_url)
            .field("password_base_url", &self.password_base_url)
            .field("timeout_secs", &self.timeout_secs)
            .field("max_retries", &self.max_retries)
            .field("api_key", &self.api_key.as_ref().map(|_| "[REDACTED]"))
            .field("custom_headers", &self.custom_headers)
            .finish()
    }
}

impl ClientBuilder {
    /// Creates a new builder with default settings.
    fn new() -> Self {
        Self {
            base_url: None,
            plus_base_url: None,
            password_base_url: None,
            timeout_secs: None,
            max_retries: None,
            api_key: None,
            custom_headers: HashMap::new(),
        }
    }

    /// Sets the base URL for the free API.
    #[must_use]
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Sets the base URL for the Plus API.
    #[must_use]
    pub fn plus_base_url(mut self, url: impl Into<String>) -> Self {
        self.plus_base_url = Some(url.into());
        self
    }

    /// Sets the base URL for the password API.
    #[must_use]
    pub fn password_base_url(mut self, url: impl Into<String>) -> Self {
        self.password_base_url = Some(url.into());
        self
    }

    /// Sets the request timeout in seconds.
    #[must_use]
    pub fn timeout_secs(mut self, secs: u64) -> Self {
        self.timeout_secs = Some(secs);
        self
    }

    /// Sets the maximum number of retries on HTTP 429.
    #[must_use]
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = Some(retries);
        self
    }

    /// Sets the API key for the Plus API. When set, the client uses the
    /// Plus API endpoints and skips client-side rate limiting.
    #[must_use]
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Adds a custom header that will be sent with every request.
    #[must_use]
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_headers.insert(name.into(), value.into());
        self
    }

    /// Builds the [`Client`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::Validation`] if the configuration is invalid, or
    /// [`Error::Network`] if the underlying HTTP client cannot be created.
    pub fn build(self) -> Result<Client> {
        let config = ClientConfig {
            base_url: self
                .base_url
                .unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            plus_base_url: self
                .plus_base_url
                .unwrap_or_else(|| PLUS_BASE_URL.to_string()),
            password_base_url: self
                .password_base_url
                .unwrap_or_else(|| PASSWORD_BASE_URL.to_string()),
            timeout: Duration::from_secs(self.timeout_secs.unwrap_or(DEFAULT_TIMEOUT_SECS)),
            max_retries: self.max_retries.unwrap_or(DEFAULT_MAX_RETRIES),
            api_key: self.api_key,
            custom_headers: self.custom_headers,
        };

        let mut default_headers = HeaderMap::new();
        for (name, value) in &config.custom_headers {
            let header_name = HeaderName::try_from(name.as_str()).map_err(|e| {
                Error::Validation {
                    message: format!("invalid header name '{name}': {e}"),
                }
            })?;
            let header_value = HeaderValue::try_from(value.as_str()).map_err(|e| {
                Error::Validation {
                    message: format!("invalid header value for '{name}': {e}"),
                }
            })?;
            default_headers.insert(header_name, header_value);
        }

        let http_client = reqwest::Client::builder()
            .timeout(config.timeout)
            .default_headers(default_headers)
            .build()?;

        Ok(Client {
            http: http_client,
            config,
            rate_limit: Arc::new(Mutex::new(RateLimitState {
                last_request: None,
            })),
        })
    }
}

/// Async client for the XposedOrNot API.
///
/// Create instances via [`Client::builder()`].
///
/// # Examples
///
/// ```no_run
/// use xposedornot::Client;
///
/// # async fn example() -> Result<(), xposedornot::Error> {
/// let client = Client::builder().build()?;
/// let breaches = client.get_breaches(None).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) http: reqwest::Client,
    /// The client configuration.
    pub config: ClientConfig,
    pub(crate) rate_limit: Arc<Mutex<RateLimitState>>,
}

impl Client {
    /// Returns a new [`ClientBuilder`] for configuring the client.
    #[must_use]
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Returns `true` if the client has an API key configured (Plus API mode).
    pub fn has_api_key(&self) -> bool {
        self.config.api_key.is_some()
    }

    /// Enforces client-side rate limiting for the free API.
    ///
    /// If the client has an API key set, this is a no-op.
    pub(crate) async fn enforce_rate_limit(&self) {
        if self.has_api_key() {
            return;
        }

        let mut state = self.rate_limit.lock().await;
        if let Some(last) = state.last_request {
            let elapsed = last.elapsed();
            if elapsed < RATE_LIMIT_INTERVAL {
                let wait = RATE_LIMIT_INTERVAL - elapsed;
                drop(state);
                tokio::time::sleep(wait).await;
                let mut state = self.rate_limit.lock().await;
                state.last_request = Some(Instant::now());
            } else {
                state.last_request = Some(Instant::now());
            }
        } else {
            state.last_request = Some(Instant::now());
        }
    }

    /// Sends a GET request with retry logic for 429 responses.
    ///
    /// Applies exponential backoff: 1s, 2s, 4s for up to `max_retries` attempts.
    pub(crate) async fn get_with_retry(&self, url: &str) -> Result<reqwest::Response> {
        self.enforce_rate_limit().await;

        let mut last_error = None;

        for attempt in 0..=self.config.max_retries {
            let mut request = self.http.get(url);

            if let Some(ref api_key) = self.config.api_key {
                request = request.header("x-api-key", api_key);
            }

            let response = request.send().await?;
            let status = response.status();

            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                if attempt < self.config.max_retries {
                    let delay = Duration::from_secs(1 << attempt);
                    tokio::time::sleep(delay).await;
                    last_error = Some(Error::RateLimit {
                        message: "too many requests (429)".to_string(),
                    });
                    continue;
                }
                return Err(Error::RateLimit {
                    message: "too many requests after max retries".to_string(),
                });
            }

            if status == reqwest::StatusCode::NOT_FOUND {
                return Err(Error::NotFound {
                    message: format!("resource not found: {url}"),
                });
            }

            if status == reqwest::StatusCode::UNAUTHORIZED
                || status == reqwest::StatusCode::FORBIDDEN
            {
                return Err(Error::Authentication {
                    message: "invalid or missing API key".to_string(),
                });
            }

            if status.is_server_error() || status.is_client_error() {
                let body = response.text().await.unwrap_or_default();
                return Err(Error::Api {
                    status_code: status.as_u16(),
                    message: body,
                });
            }

            return Ok(response);
        }

        Err(last_error.unwrap_or(Error::Api {
            status_code: 429,
            message: "rate limited after retries".to_string(),
        }))
    }
}
