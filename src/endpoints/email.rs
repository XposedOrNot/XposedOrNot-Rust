//! Email check and breach analytics endpoints.

use percent_encoding::utf8_percent_encode;

use crate::client::Client;
use crate::errors::{Error, Result};
use crate::models::{
    BreachAnalyticsResponse, EmailCheckResult, FreeEmailCheckResponse, PlusEmailCheckResponse,
};
use crate::utils::{validate_email, PATH_ENCODE_SET, QUERY_ENCODE_SET};

impl Client {
    /// Checks if an email address has been involved in known data breaches.
    ///
    /// When the client has an API key configured, this calls the Plus API
    /// (`/v3/check-email/{email}?detailed=true`) which returns detailed breach
    /// records. Otherwise, it uses the free API (`/v1/check-email/{email}`)
    /// which returns a simple list of breach names.
    ///
    /// # Errors
    ///
    /// - [`Error::Validation`] if the email address is invalid.
    /// - [`Error::NotFound`] if no breaches are found for the email.
    /// - [`Error::Authentication`] if the API key is invalid (Plus API).
    /// - [`Error::Network`] on connection errors.
    /// - [`Error::RateLimit`] if rate limited after retries.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xposedornot::Client;
    ///
    /// # async fn example() -> Result<(), xposedornot::Error> {
    /// let client = Client::builder().build()?;
    /// let result = client.check_email("user@example.com").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn check_email(&self, email: &str) -> Result<EmailCheckResult> {
        validate_email(email)?;

        let encoded_email = utf8_percent_encode(email, PATH_ENCODE_SET).to_string();

        if self.has_api_key() {
            let url = format!(
                "{}/v3/check-email/{}?detailed=true",
                self.config.plus_base_url, encoded_email
            );
            let response = self.get_with_retry(&url).await?;
            let body: PlusEmailCheckResponse = response.json().await?;
            Ok(EmailCheckResult::Plus(body))
        } else {
            let url = format!("{}/v1/check-email/{}", self.config.base_url, encoded_email);
            match self.get_with_retry(&url).await {
                Ok(response) => {
                    let body: FreeEmailCheckResponse = response.json().await?;
                    Ok(EmailCheckResult::Free(body))
                }
                Err(Error::NotFound { .. }) => {
                    // 404 means email not found in any breaches — valid result
                    Ok(EmailCheckResult::Free(FreeEmailCheckResponse {
                        breaches: vec![],
                    }))
                }
                Err(e) => Err(e),
            }
        }
    }

    /// Retrieves breach analytics for an email address.
    ///
    /// Returns detailed analytics including breach details, summaries,
    /// metrics, and paste information.
    ///
    /// # Errors
    ///
    /// - [`Error::Validation`] if the email address is invalid.
    /// - [`Error::NotFound`] if no data is found for the email.
    /// - [`Error::Network`] on connection errors.
    /// - [`Error::RateLimit`] if rate limited after retries.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xposedornot::Client;
    ///
    /// # async fn example() -> Result<(), xposedornot::Error> {
    /// let client = Client::builder().build()?;
    /// let analytics = client.breach_analytics("user@example.com").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn breach_analytics(&self, email: &str) -> Result<BreachAnalyticsResponse> {
        validate_email(email)?;

        let encoded_email = utf8_percent_encode(email, QUERY_ENCODE_SET).to_string();
        let url = format!(
            "{}/v1/breach-analytics?email={}",
            self.config.base_url, encoded_email
        );
        let response = self.get_with_retry(&url).await?;
        let body: BreachAnalyticsResponse = response.json().await?;
        Ok(body)
    }
}
