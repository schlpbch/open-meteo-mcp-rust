//! Tool handler tests for Location/Geocoding
//! Phase 4: Comprehensive location search tool testing, against a mocked API

mod common;

use common::mock_service;
use open_meteo_mcp::types::location::GeocodeRequest;
use wiremock::{matchers, Mock, MockServer, ResponseTemplate};

const GEOCODE_FIXTURE: &str = include_str!("fixtures/geocode_response.json");

async fn mock_geocode_server() -> MockServer {
    let mock_server = MockServer::start().await;
    Mock::given(matchers::method("GET"))
        .and(matchers::path("/search"))
        .respond_with(ResponseTemplate::new(200).set_body_string(GEOCODE_FIXTURE))
        .mount(&mock_server)
        .await;
    mock_server
}

#[tokio::test]
async fn test_search_location_success() {
    let mock_server = mock_geocode_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .search_location("Munich".to_string(), None, None)
        .await;

    assert!(result.is_ok(), "Location search should succeed");
    let call_result = result.unwrap();
    assert!(!call_result.is_error);
}

#[tokio::test]
async fn test_search_location_with_count() {
    let mock_server = mock_geocode_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .search_location("Munich".to_string(), Some(5), None)
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_search_location_empty_name() {
    let mock_server = mock_geocode_server().await;
    let service = mock_service(&mock_server);

    let result = service.search_location("".to_string(), None, None).await;

    assert!(result.is_err(), "Empty location name should be rejected");
}

#[tokio::test]
async fn test_search_location_count_zero() {
    let mock_server = mock_geocode_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .search_location("Munich".to_string(), Some(0), None)
        .await;

    assert!(result.is_err(), "count 0 should be invalid");
}

#[tokio::test]
async fn test_search_location_count_too_high() {
    let mock_server = mock_geocode_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .search_location("Munich".to_string(), Some(101), None)
        .await;

    assert!(result.is_err(), "count > 100 should be invalid");
}

#[tokio::test]
async fn test_search_location_count_valid_boundaries() {
    let mock_server = mock_geocode_server().await;
    let service = mock_service(&mock_server);

    // Test min and max valid values
    for count in [1, 50, 100].iter() {
        let result = service
            .search_location("Munich".to_string(), Some(*count), None)
            .await;

        assert!(result.is_ok(), "count {count} should be valid");
    }
}

#[tokio::test]
async fn test_search_location_count_none_default() {
    let mock_server = mock_geocode_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .search_location("Munich".to_string(), None, None)
        .await;

    assert!(result.is_ok(), "count None should use default");
}

#[tokio::test]
async fn test_search_location_with_language() {
    let mock_server = mock_geocode_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .search_location("Munich".to_string(), Some(10), Some("en".to_string()))
        .await;

    assert!(result.is_ok());
}

#[test]
fn test_location_request_json_serialization() {
    let req = GeocodeRequest {
        name: "Berlin".to_string(),
        count: Some(5),
        language: Some("de".to_string()),
        format: None,
    };

    let json = serde_json::to_value(&req).expect("Valid JSON");
    assert_eq!(json["name"], "Berlin");
    assert_eq!(json["count"], 5);
    assert_eq!(json["language"], "de");
}

#[test]
fn test_location_request_validation_empty_name() {
    let req = GeocodeRequest {
        name: String::new(),
        count: Some(10),
        language: None,
        format: None,
    };

    assert!(req.validate().is_err(), "Empty name should fail validation");
}
