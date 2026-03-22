use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};
use xposedornot::Client;

#[tokio::test]
async fn check_email_free_api_returns_breaches() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/check-email/test%40example.com"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "breaches": [["Adobe", "LinkedIn"]]
        })))
        .mount(&server)
        .await;

    let client = Client::builder()
        .base_url(server.uri())
        .build()
        .unwrap();

    let result = client.check_email("test@example.com").await.unwrap();
    match result {
        xposedornot::EmailCheckResult::Free(resp) => {
            assert_eq!(resp.breaches.len(), 1);
            assert_eq!(resp.breaches[0], vec!["Adobe", "LinkedIn"]);
        }
        _ => panic!("expected Free result"),
    }
}

#[tokio::test]
async fn check_email_plus_api_returns_detailed_breaches() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v3/check-email/test%40example.com"))
        .and(query_param("detailed", "true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "success",
            "email": "test@example.com",
            "breaches": [{
                "breach_id": "Adobe",
                "breached_date": "2013-10-04",
                "logo": "https://example.com/logo.png",
                "password_risk": "high",
                "searchable": "yes",
                "xposed_data": "emails,passwords",
                "xposed_records": 153000000,
                "xposure_desc": "Adobe breach",
                "domain": "adobe.com"
            }]
        })))
        .mount(&server)
        .await;

    let client = Client::builder()
        .plus_base_url(server.uri())
        .api_key("test-key")
        .build()
        .unwrap();

    let result = client.check_email("test@example.com").await.unwrap();
    match result {
        xposedornot::EmailCheckResult::Plus(resp) => {
            assert_eq!(resp.status, "success");
            assert_eq!(resp.email, "test@example.com");
            assert_eq!(resp.breaches.len(), 1);
            assert_eq!(resp.breaches[0].breach_id, "Adobe");
            assert_eq!(resp.breaches[0].xposed_records, 153000000);
        }
        _ => panic!("expected Plus result"),
    }
}

#[tokio::test]
async fn check_email_validates_email() {
    let client = Client::builder().build().unwrap();
    let result = client.check_email("not-valid").await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("invalid email"));
}

#[tokio::test]
async fn check_email_not_found() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/check-email/nobody%40example.com"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let client = Client::builder()
        .base_url(server.uri())
        .build()
        .unwrap();

    let result = client.check_email("nobody@example.com").await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, xposedornot::Error::NotFound { .. }));
}

#[tokio::test]
async fn breach_analytics_returns_data() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/breach-analytics"))
        .and(query_param("email", "test@example.com"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "ExposedBreaches": {
                "breaches_details": [{"breach": "Adobe"}]
            },
            "BreachesSummary": {"total": 1},
            "BreachMetrics": {"risk": "high"},
            "PastesSummary": {"count": 0},
            "ExposedPastes": []
        })))
        .mount(&server)
        .await;

    let client = Client::builder()
        .base_url(server.uri())
        .build()
        .unwrap();

    let result = client.breach_analytics("test@example.com").await.unwrap();
    assert_eq!(result.exposed_breaches.breaches_details.len(), 1);
    assert!(result.exposed_pastes.is_empty());
}
