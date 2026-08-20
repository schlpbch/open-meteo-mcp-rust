//! Tool handler tests for Weather/Forecast - Phase 4
//! Comprehensive weather tool testing with all parameters, against a mocked API

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
async fn test_get_weather_success_minimal_params() {
    let mock_server = mock_weather_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_weather(48.1, 11.6, None, None, None, None)
        .await;

    assert!(
        result.is_ok(),
        "Weather request with minimal params should succeed"
    );
}

#[tokio::test]
async fn test_get_weather_validation_latitude_too_high() {
    let mock_server = mock_weather_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_weather(90.001, 11.6, None, None, None, None)
        .await;
    assert!(result.is_err(), "Latitude > 90 should be rejected");
}

#[tokio::test]
async fn test_get_weather_validation_longitude_too_high() {
    let mock_server = mock_weather_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_weather(48.1, 180.001, None, None, None, None)
        .await;
    assert!(result.is_err(), "Longitude > 180 should be rejected");
}

#[tokio::test]
async fn test_get_weather_boundary_latitude_max() {
    let mock_server = mock_weather_server().await;
    let service = mock_service(&mock_server);

    let result = service.get_weather(90.0, 0.0, None, None, None, None).await;
    assert!(result.is_ok(), "Latitude 90.0 should be valid");
}

#[tokio::test]
async fn test_get_weather_boundary_longitude_min() {
    let mock_server = mock_weather_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_weather(0.0, -180.0, None, None, None, None)
        .await;
    assert!(result.is_ok(), "Longitude -180.0 should be valid");
}

#[tokio::test]
async fn test_get_weather_null_island() {
    let mock_server = mock_weather_server().await;
    let service = mock_service(&mock_server);

    let result = service.get_weather(0.0, 0.0, None, None, None, None).await;
    assert!(result.is_ok(), "Null Island should be valid");
}

#[tokio::test]
async fn test_get_weather_forecast_days_valid_boundary() {
    let mock_server = mock_weather_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_weather(48.1, 11.6, None, None, Some(16), None)
        .await;
    assert!(result.is_ok(), "forecast_days 16 should be valid");
}

#[tokio::test]
async fn test_get_weather_forecast_days_invalid_zero() {
    let mock_server = mock_weather_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_weather(48.1, 11.6, None, None, Some(0), None)
        .await;
    assert!(result.is_err(), "forecast_days 0 should be invalid");
}

#[tokio::test]
async fn test_get_weather_forecast_days_invalid_too_high() {
    let mock_server = mock_weather_server().await;
    let service = mock_service(&mock_server);

    let result = service
        .get_weather(48.1, 11.6, None, None, Some(17), None)
        .await;
    assert!(result.is_err(), "forecast_days 17 should be invalid");
}

#[test]
fn test_weather_request_serialization() {
    let req = open_meteo_mcp::types::weather::WeatherRequest {
        latitude: 48.1,
        longitude: 11.6,
        forecast_days: Some(7),
        ..Default::default()
    };

    let json = serde_json::to_value(&req).expect("Valid JSON");
    assert_eq!(json["latitude"], 48.1);
    assert_eq!(json["forecast_days"], 7);
}
