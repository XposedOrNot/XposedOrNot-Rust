//! Password exposure check endpoint.

use crate::client::Client;
use crate::errors::{Error, Result};
use crate::models::PasswordCheckResponse;
use crate::utils::keccak_hash_prefix;

impl Client {
    /// Checks if a password has been exposed in known data breaches.
    ///
    /// The password is hashed locally using Keccak-512 and only the first
    /// 10 hex characters of the digest are sent to the API (k-anonymity).
    /// The full password never leaves the client.
    ///
    /// # Errors
    ///
    /// - [`Error::Validation`] if the password is empty.
    /// - [`Error::NotFound`] if the password hash prefix is not found.
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
    /// let result = client.check_password("mysecretpassword").await?;
    /// println!("Exposure count: {}", result.search_pass_anon.count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn check_password(&self, password: &str) -> Result<PasswordCheckResponse> {
        if password.is_empty() {
            return Err(Error::Validation {
                message: "password must not be empty".to_string(),
            });
        }

        let hash_prefix = keccak_hash_prefix(password);
        let url = format!(
            "{}/v1/pass/anon/{}",
            self.config.password_base_url, hash_prefix
        );
        let response = self.get_with_retry(&url).await?;
        let body: PasswordCheckResponse = response.json().await?;
        Ok(body)
    }
}
