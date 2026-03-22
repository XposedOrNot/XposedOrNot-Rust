use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};
use xposedornot::Client;

#[tokio::test]
async fn get_breaches_returns_all() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/breaches"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "exposedBreaches": [{
                "breachID": "Adobe",
                "breachedDate": "2013-10-04",
                "domain": "adobe.com",
                "industry": "Technology",
                "exposedData": "emails,passwords",
                "exposedRecords": 153000000,
                "verified": true
            }]
        })))
        .mount(&server)
        .await;

    let client = Client::builder()
        .base_url(server.uri())
        .build()
        .unwrap();

    let result = client.get_breaches(None).await.unwrap();
    assert_eq!(result.exposed_breaches.len(), 1);
    assert_eq!(result.exposed_breaches[0].breach_id, "Adobe");
    assert_eq!(result.exposed_breaches[0].exposed_records, 153000000);
    assert!(result.exposed_breaches[0].verified);
}

#[tokio::test]
async fn get_breaches_with_domain_filter() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/breaches"))
        .and(query_param("domain", "adobe.com"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "exposedBreaches": [{
                "breachID": "Adobe",
                "breachedDate": "2013-10-04",
                "domain": "adobe.com",
                "industry": "Technology",
                "exposedData": "emails,passwords",
                "exposedRecords": 153000000,
                "verified": true
            }]
        })))
        .mount(&server)
        .await;

    let client = Client::builder()
        .base_url(server.uri())
        .build()
        .unwrap();

    let result = client.get_breaches(Some("adobe.com")).await.unwrap();
    assert_eq!(result.exposed_breaches.len(), 1);
    assert_eq!(result.exposed_breaches[0].domain, "adobe.com");
}

#[tokio::test]
async fn get_breaches_empty_result() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/breaches"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "exposedBreaches": []
        })))
        .mount(&server)
        .await;

    let client = Client::builder()
        .base_url(server.uri())
        .build()
        .unwrap();

    let result = client.get_breaches(None).await.unwrap();
    assert!(result.exposed_breaches.is_empty());
}
