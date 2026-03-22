use xposedornot::utils::{keccak_hash_prefix, validate_email};

#[test]
fn validate_email_accepts_valid_addresses() {
    assert!(validate_email("user@example.com").is_ok());
    assert!(validate_email("test+tag@sub.domain.org").is_ok());
    assert!(validate_email("a@b.c").is_ok());
}

#[test]
fn validate_email_rejects_empty() {
    let err = validate_email("").unwrap_err();
    assert!(err.to_string().contains("empty"));
}

#[test]
fn validate_email_rejects_missing_at() {
    assert!(validate_email("nodomain").is_err());
}

#[test]
fn validate_email_rejects_missing_local() {
    assert!(validate_email("@example.com").is_err());
}

#[test]
fn validate_email_rejects_missing_domain() {
    assert!(validate_email("user@").is_err());
}

#[test]
fn validate_email_rejects_domain_without_dot() {
    assert!(validate_email("user@localhost").is_err());
}

#[test]
fn keccak_hash_prefix_returns_10_chars() {
    let prefix = keccak_hash_prefix("password");
    assert_eq!(prefix.len(), 10);
}

#[test]
fn keccak_hash_prefix_is_deterministic() {
    assert_eq!(keccak_hash_prefix("abc"), keccak_hash_prefix("abc"));
}

#[test]
fn keccak_hash_prefix_differs_for_different_inputs() {
    assert_ne!(keccak_hash_prefix("foo"), keccak_hash_prefix("bar"));
}

#[test]
fn keccak_hash_prefix_is_hex() {
    let prefix = keccak_hash_prefix("test123");
    assert!(prefix.chars().all(|c| c.is_ascii_hexdigit()));
}
