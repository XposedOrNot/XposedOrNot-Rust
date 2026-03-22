//! Breach listing endpoint.

use percent_encoding::utf8_percent_encode;

use crate::client::Client;
use crate::errors::Result;
use crate::models::BreachListResponse;
use crate::utils::QUERY_ENCODE_SET;

impl Client {
    /// Lists known data breaches, optionally filtered by domain.
    ///
    /// # Arguments
    ///
    /// * `domain` - Optional domain to filter breaches (e.g., `"example.com"`).
    ///
    /// # Errors
    ///
    /// - [`Error::Network`] on connection errors.
    /// - [`Error::RateLimit`] if rate limited after retries.
    /// - [`Error::Api`] on unexpected server responses.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xposedornot::Client;
    ///
    /// # async fn example() -> Result<(), xposedornot::Error> {
    /// let client = Client::builder().build()?;
    ///
    /// // Get all breaches
    /// let all = client.get_breaches(None).await?;
    ///
    /// // Filter by domain
    /// let filtered = client.get_breaches(Some("example.com")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_breaches(&self, domain: Option<&str>) -> Result<BreachListResponse> {
        let mut url = format!("{}/v1/breaches", self.config.base_url);

        if let Some(d) = domain {
            let encoded_domain = utf8_percent_encode(d, QUERY_ENCODE_SET).to_string();
            url.push_str(&format!("?domain={}", encoded_domain));
        }

        let response = self.get_with_retry(&url).await?;
        let body: BreachListResponse = response.json().await?;
        Ok(body)
    }
}
