use xposedornot::Client;

#[test]
fn builder_creates_client_with_defaults() {
    let client = Client::builder().build().unwrap();
    assert!(!client.has_api_key());
}

#[test]
fn builder_with_api_key() {
    let client = Client::builder()
        .api_key("test-key")
        .build()
        .unwrap();
    assert!(client.has_api_key());
}

#[test]
fn builder_with_custom_timeout() {
    let client = Client::builder()
        .timeout_secs(60)
        .build()
        .unwrap();
    assert!(!client.has_api_key());
}

#[test]
fn builder_with_custom_retries() {
    let client = Client::builder()
        .max_retries(5)
        .build()
        .unwrap();
    assert_eq!(client.config.max_retries, 5);
}

#[test]
fn builder_with_custom_base_url() {
    let client = Client::builder()
        .base_url("http://localhost:8080")
        .build()
        .unwrap();
    assert_eq!(client.config.base_url, "http://localhost:8080");
}

#[test]
fn builder_with_custom_header() {
    let client = Client::builder()
        .header("x-custom", "value")
        .build()
        .unwrap();
    assert_eq!(
        client.config.custom_headers.get("x-custom"),
        Some(&"value".to_string())
    );
}

#[test]
fn builder_chaining() {
    let client = Client::builder()
        .base_url("http://localhost:8080")
        .plus_base_url("http://localhost:8081")
        .password_base_url("http://localhost:8082")
        .timeout_secs(10)
        .max_retries(1)
        .api_key("key")
        .header("x-test", "yes")
        .build()
        .unwrap();

    assert!(client.has_api_key());
    assert_eq!(client.config.base_url, "http://localhost:8080");
    assert_eq!(client.config.plus_base_url, "http://localhost:8081");
    assert_eq!(client.config.password_base_url, "http://localhost:8082");
    assert_eq!(client.config.max_retries, 1);
}
