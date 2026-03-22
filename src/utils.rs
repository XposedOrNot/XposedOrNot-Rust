//! Utility functions for email validation and password hashing.

use percent_encoding::{AsciiSet, CONTROLS};
use sha3::{Digest, Keccak512};

use crate::errors::Error;

/// Characters that need encoding in URL path segments.
/// Encodes everything except unreserved chars (RFC 3986) and sub-delimiters.
pub const PATH_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'`')
    .add(b'{')
    .add(b'}')
    .add(b'/')
    .add(b'@')
    .add(b'[')
    .add(b']')
    .add(b'%');

/// Characters that need encoding in URL query values.
pub const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'&')
    .add(b'=')
    .add(b'+')
    .add(b'%');

/// Validates that the given string looks like a valid email address.
///
/// This performs a basic structural check (contains exactly one `@` with
/// non-empty local and domain parts, and the domain contains a `.`).
///
/// # Errors
///
/// Returns [`Error::Validation`] if the email is invalid.
///
/// # Examples
///
/// ```
/// use xposedornot::utils::validate_email;
///
/// assert!(validate_email("user@example.com").is_ok());
/// assert!(validate_email("not-an-email").is_err());
/// ```
pub fn validate_email(email: &str) -> Result<(), Error> {
    let trimmed = email.trim();
    if trimmed.is_empty() {
        return Err(Error::Validation {
            message: "email must not be empty".to_string(),
        });
    }

    let parts: Vec<&str> = trimmed.splitn(2, '@').collect();
    if parts.len() != 2 {
        return Err(Error::Validation {
            message: format!("invalid email address: {trimmed}"),
        });
    }

    let (local, domain) = (parts[0], parts[1]);
    if local.is_empty() || domain.is_empty() {
        return Err(Error::Validation {
            message: format!("invalid email address: {trimmed}"),
        });
    }

    if !domain.contains('.') {
        return Err(Error::Validation {
            message: format!("invalid email domain: {domain}"),
        });
    }

    Ok(())
}

/// Hashes a password with original Keccak-512 and returns the first 10
/// hex characters of the digest.
///
/// The XposedOrNot password API uses an anonymized k-anonymity approach:
/// only a short prefix of the hash is sent to the server, which responds
/// with matching entries so the client can check locally.
///
/// **Important:** This uses the *original* Keccak-512 algorithm, not the
/// FIPS 202 SHA3-512 variant.
///
/// # Examples
///
/// ```
/// use xposedornot::utils::keccak_hash_prefix;
///
/// let prefix = keccak_hash_prefix("password123");
/// assert_eq!(prefix.len(), 10);
/// ```
pub fn keccak_hash_prefix(password: &str) -> String {
    let mut hasher = Keccak512::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    let hex = format!("{result:x}");
    hex[..10].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email_valid() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("a@b.c").is_ok());
        assert!(validate_email("user+tag@example.co.uk").is_ok());
    }

    #[test]
    fn test_validate_email_invalid() {
        assert!(validate_email("").is_err());
        assert!(validate_email("noatsign").is_err());
        assert!(validate_email("@domain.com").is_err());
        assert!(validate_email("user@").is_err());
        assert!(validate_email("user@nodot").is_err());
    }

    #[test]
    fn test_keccak_hash_prefix_length() {
        let prefix = keccak_hash_prefix("test");
        assert_eq!(prefix.len(), 10);
    }

    #[test]
    fn test_keccak_hash_prefix_deterministic() {
        let a = keccak_hash_prefix("hello");
        let b = keccak_hash_prefix("hello");
        assert_eq!(a, b);
    }

    #[test]
    fn test_keccak_hash_prefix_different_inputs() {
        let a = keccak_hash_prefix("password1");
        let b = keccak_hash_prefix("password2");
        assert_ne!(a, b);
    }
}
