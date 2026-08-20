//! Tool handler tests for Swiss Location Search
//! Phase 4: Comprehensive Swiss location search tool testing, against a mocked API

mod common;

use common::mock_service;
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
async fn test_search_location_swiss_success() {
    let mock_server = mock_geocode_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .search_location_swiss("Zurich".to_string(), None)
        .await;

    assert!(result.is_ok(), "Swiss location search should succeed");
}

#[tokio::test]
async fn test_search_location_swiss_with_count() {
    let mock_server = mock_geocode_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .search_location_swiss("Bern".to_string(), Some(5))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_search_location_swiss_empty_name() {
    let mock_server = mock_geocode_server().await;
    let service = mock_service(&mock_server);

    let result = service.search_location_swiss("".to_string(), None).await;

    assert!(result.is_err(), "Empty location name should be rejected");
}

#[tokio::test]
async fn test_search_location_swiss_count_zero() {
    let mock_server = mock_geocode_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .search_location_swiss("Geneva".to_string(), Some(0))
        .await;

    assert!(result.is_err(), "count 0 should be invalid");
}

#[tokio::test]
async fn test_search_location_swiss_count_too_high() {
    let mock_server = mock_geocode_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .search_location_swiss("Lausanne".to_string(), Some(101))
        .await;

    assert!(result.is_err(), "count > 100 should be invalid");
}

#[tokio::test]
async fn test_search_location_swiss_count_valid_boundaries() {
    let mock_server = mock_geocode_server().await;
    let service = mock_service(&mock_server);

    for count in [1, 50, 100].iter() {
        let result = service
            .search_location_swiss("Basel".to_string(), Some(*count))
            .await;

        assert!(result.is_ok(), "count {count} should be valid");
    }
}

#[tokio::test]
async fn test_search_location_swiss_various_cities() {
    let mock_server = mock_geocode_server().await;
    let service = mock_service(&mock_server);

    for city in &["Lugano", "Sion", "Thun", "Interlaken"] {
        let result = service
            .search_location_swiss(city.to_string(), Some(5))
            .await;

        assert!(result.is_ok(), "Swiss city {city} should be searchable");
    }
}
