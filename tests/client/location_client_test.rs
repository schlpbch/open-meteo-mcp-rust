//! HTTP Client tests for Location/Geocoding API
//! Phase 3: Tests location client request/response handling

use open_meteo_mcp::{OpenMeteoClient, types::location::LocationRequest};
use std::sync::Arc;
use wiremock::{Mock, MockServer, ResponseTemplate, matchers};

#[tokio::test]
async fn test_location_client_search_success() {
    let mock_server = MockServer::start().await;
    let fixture = include_str!("../../tests/fixtures/location_response.json");
    
    Mock::given(matchers::method("GET"))
        .and(matchers::path("/v1/search"))
        .and(matchers::query_param("name", "Munich"))
        .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
        .mount(&mock_server)
        .await;
    
    let http_client = Arc::new(
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Valid HTTP client"),
    );
    
    let client = OpenMeteoClient::new(http_client);
    let req = LocationRequest {
        name: "Munich".to_string(),
        count: Some(10),
        language: None,
        format: None,
    };
    
    let result = client.search_location(&req).await;
    assert!(result.is_ok());
    
    let response = result.unwrap();
    assert!(!response.results.is_empty());
    assert_eq!(response.results[0].name, "Munich");
}

#[tokio::test]
async fn test_location_client_empty_name_validation() {
    let http_client = Arc::new(
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Valid HTTP client"),
    );
    
    let client = OpenMeteoClient::new(http_client);
    let req = LocationRequest {
        name: String::new(),
        count: Some(10),
        language: None,
        format: None,
    };
    
    let result = client.search_location(&req).await;
    assert!(result.is_err(), "Empty location name should fail");
}

#[tokio::test]
async fn test_location_client_no_results() {
    let mock_server = MockServer::start().await;
    let json = r#"{"results": [], "generationtime_ms": 5.0}"#;
    
    Mock::given(matchers::method("GET"))
        .and(matchers::path("/v1/search"))
        .respond_with(ResponseTemplate::new(200).set_body_string(json))
        .mount(&mock_server)
        .await;
    
    let http_client = Arc::new(
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Valid HTTP client"),
    );
    
    let client = OpenMeteoClient::new(http_client);
    let req = LocationRequest {
        name: "XYZ123NonExistent".to_string(),
        count: Some(1),
        language: None,
        format: None,
    };
    
    let result = client.search_location(&req).await;
    assert!(result.is_ok(), "No results should not error");
    assert!(result.unwrap().results.is_empty());
}

#[tokio::test]
async fn test_location_client_count_parameter() {
    let mock_server = MockServer::start().await;
    let fixture = include_str!("../../tests/fixtures/location_response.json");
    
    Mock::given(matchers::method("GET"))
        .and(matchers::path("/v1/search"))
        .and(matchers::query_param("count", "5"))
        .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
        .mount(&mock_server)
        .await;
    
    let http_client = Arc::new(
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Valid HTTP client"),
    );
    
    let client = OpenMeteoClient::new(http_client);
    let req = LocationRequest {
        name: "Munich".to_string(),
        count: Some(5),
        language: None,
        format: None,
    };
    
    let result = client.search_location(&req).await;
    assert!(result.is_ok());
}
