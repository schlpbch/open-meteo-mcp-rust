//! Tool handler tests for Snow Conditions
//! Phase 4: Comprehensive snow tool testing, against a mocked API

mod common;

use common::mock_service;
use wiremock::{matchers, Mock, MockServer, ResponseTemplate};

const WEATHER_FIXTURE: &str = include_str!("fixtures/weather_response.json");

async fn mock_weather_server() -> MockServer {
    let mock_server = MockServer::start().await;
    Mock::given(matchers::method("GET"))
        .and(matchers::path("/forecast"))
        .respond_with(ResponseTemplate::new(200).set_body_string(WEATHER_FIXTURE))
        .mount(&mock_server)
        .await;
    mock_server
}

#[tokio::test]
async fn test_get_snow_conditions_success_minimal() {
    let mock_server = mock_weather_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_snow_conditions(48.1, 11.6, None, None, None)
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_snow_conditions_forecast_days_max() {
    let mock_server = mock_weather_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_snow_conditions(48.1, 11.6, None, None, Some(16))
        .await;

    assert!(result.is_ok(), "forecast_days 16 should be valid");
}

#[tokio::test]
async fn test_get_snow_conditions_validation_latitude() {
    let mock_server = mock_weather_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_snow_conditions(90.001, 11.6, None, None, None)
        .await;

    assert!(result.is_err(), "Invalid latitude should be rejected");
}

#[tokio::test]
async fn test_get_snow_conditions_validation_longitude() {
    let mock_server = mock_weather_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_snow_conditions(48.1, 180.001, None, None, None)
        .await;

    assert!(result.is_err(), "Invalid longitude should be rejected");
}

#[tokio::test]
async fn test_get_snow_conditions_boundary_coordinates() {
    let mock_server = mock_weather_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_snow_conditions(90.0, 180.0, None, None, None)
        .await;

    assert!(result.is_ok(), "Boundary coordinates should be valid");
}

#[tokio::test]
async fn test_get_snow_conditions_forecast_days_invalid_zero() {
    let mock_server = mock_weather_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_snow_conditions(48.1, 11.6, None, None, Some(0))
        .await;

    assert!(result.is_err(), "forecast_days 0 should be invalid");
}

#[tokio::test]
async fn test_get_snow_conditions_forecast_days_invalid_too_high() {
    let mock_server = mock_weather_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_snow_conditions(48.1, 11.6, None, None, Some(17))
        .await;

    assert!(result.is_err(), "forecast_days > 16 should be invalid");
}

#[tokio::test]
async fn test_get_snow_conditions_forecast_days_valid() {
    let mock_server = mock_weather_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_snow_conditions(48.1, 11.6, None, None, Some(7))
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_snow_conditions_null_island() {
    let mock_server = mock_weather_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_snow_conditions(0.0, 0.0, None, None, None)
        .await;

    assert!(result.is_ok());
}

#[test]
fn test_snow_request_json_serialization() {
    let req = open_meteo_mcp::types::snow::SnowRequest {
        latitude: 48.1,
        longitude: 11.6,
        hourly: Some("snow_depth".to_string()),
        daily: Some("snowfall_sum".to_string()),
        forecast_days: Some(7),
        ..Default::default()
    };

    let json = serde_json::to_value(&req).expect("Valid JSON");
    assert_eq!(json["latitude"], 48.1);
    assert_eq!(json["forecast_days"], 7);
}
