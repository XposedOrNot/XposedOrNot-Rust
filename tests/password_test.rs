use wiremock::matchers::{method, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};
use xposedornot::Client;

#[tokio::test]
async fn check_password_returns_exposure_data() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path_regex(r"/v1/pass/anon/[a-f0-9]{10}"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "SearchPassAnon": {
                "anon": "ab1234567890abcdef",
                "char": "D:3;A:8;S:0;L:11",
                "count": "62703"
            }
        })))
        .mount(&server)
        .await;

    let client = Client::builder()
        .password_base_url(server.uri())
        .build()
        .unwrap();

    let result = client.check_password("password123").await.unwrap();
    assert_eq!(result.search_pass_anon.count, "62703");
    assert_eq!(result.search_pass_anon.char, "D:3;A:8;S:0;L:11");
}

#[tokio::test]
async fn check_password_empty_returns_validation_error() {
    let client = Client::builder().build().unwrap();
    let result = client.check_password("").await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("empty"));
}

#[tokio::test]
async fn check_password_not_found() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path_regex(r"/v1/pass/anon/[a-f0-9]{10}"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let client = Client::builder()
        .password_base_url(server.uri())
        .build()
        .unwrap();

    let result = client.check_password("verysecurepassword").await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), xposedornot::Error::NotFound { .. }));
}

#[tokio::test]
async fn check_password_retry_on_429() {
    let server = MockServer::start().await;

    // First request returns 429, second returns success
    Mock::given(method("GET"))
        .and(path_regex(r"/v1/pass/anon/[a-f0-9]{10}"))
        .respond_with(ResponseTemplate::new(429))
        .up_to_n_times(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path_regex(r"/v1/pass/anon/[a-f0-9]{10}"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "SearchPassAnon": {
                "anon": "ab1234567890",
                "char": "D:1;A:4;S:0;L:5",
                "count": "100"
            }
        })))
        .mount(&server)
        .await;

    let client = Client::builder()
        .password_base_url(server.uri())
        .max_retries(3)
        .build()
        .unwrap();

    let result = client.check_password("test").await.unwrap();
    assert_eq!(result.search_pass_anon.count, "100");
}
