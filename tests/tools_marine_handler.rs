//! Tool handler tests for Marine Conditions
//! Phase 4: Comprehensive marine weather tool testing, against a mocked API

mod common;

use common::mock_service;
use wiremock::{matchers, Mock, MockServer, ResponseTemplate};

const MARINE_FIXTURE: &str = include_str!("fixtures/marine_response.json");

async fn mock_marine_server() -> MockServer {
    let mock_server = MockServer::start().await;
    Mock::given(matchers::method("GET"))
        .and(matchers::path("/marine"))
        .respond_with(ResponseTemplate::new(200).set_body_string(MARINE_FIXTURE))
        .mount(&mock_server)
        .await;
    mock_server
}

#[tokio::test]
async fn test_get_marine_conditions_success_minimal() {
    let mock_server = mock_marine_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_marine_conditions(48.1, 11.6, None, None, None)
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_marine_conditions_with_daily() {
    let mock_server = mock_marine_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_marine_conditions(
            48.1,
            11.6,
            None,
            Some("wave_height_max".to_string()),
            Some(5),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_marine_conditions_validation_latitude() {
    let mock_server = mock_marine_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_marine_conditions(90.001, 11.6, None, None, None)
        .await;

    assert!(result.is_err(), "Invalid latitude should be rejected");
}

#[tokio::test]
async fn test_get_marine_conditions_validation_longitude() {
    let mock_server = mock_marine_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_marine_conditions(48.1, 180.001, None, None, None)
        .await;

    assert!(result.is_err(), "Invalid longitude should be rejected");
}

#[tokio::test]
async fn test_get_marine_conditions_boundary_coordinates() {
    let mock_server = mock_marine_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_marine_conditions(90.0, 180.0, None, None, None)
        .await;

    assert!(result.is_ok(), "Boundary coordinates should be valid");
}

#[tokio::test]
async fn test_get_marine_conditions_forecast_days_valid_max() {
    let mock_server = mock_marine_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_marine_conditions(48.1, 11.6, None, None, Some(16))
        .await;

    assert!(result.is_ok(), "forecast_days 16 should be valid");
}

#[tokio::test]
async fn test_get_marine_conditions_forecast_days_invalid_zero() {
    let mock_server = mock_marine_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_marine_conditions(48.1, 11.6, None, None, Some(0))
        .await;

    assert!(result.is_err(), "forecast_days 0 should be invalid");
}

#[tokio::test]
async fn test_get_marine_conditions_null_island() {
    let mock_server = mock_marine_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_marine_conditions(0.0, 0.0, None, None, None)
        .await;

    assert!(result.is_ok());
}
