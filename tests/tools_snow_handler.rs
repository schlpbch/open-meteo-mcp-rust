//! Tool handler tests for Snow Conditions
//! Phase 4: Comprehensive snow tool testing

use open_meteo_mcp::OpenMeteoService;

#[tokio::test]
async fn test_get_snow_conditions_success_minimal() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_snow_conditions(
        48.1,
        11.6,
        None, None, None
    ).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_snow_conditions_forecast_days_max() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_snow_conditions(
        48.1,
        11.6,
        None,
        None,
        Some(16)
    ).await;

    assert!(result.is_ok(), "forecast_days 16 should be valid");
}

#[tokio::test]
async fn test_get_snow_conditions_validation_latitude() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_snow_conditions(
        90.001,
        11.6,
        None, None, None
    ).await;

    assert!(result.is_err(), "Invalid latitude should be rejected");
}

#[tokio::test]
async fn test_get_snow_conditions_validation_longitude() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_snow_conditions(
        48.1,
        180.001,
        None, None, None
    ).await;

    assert!(result.is_err(), "Invalid longitude should be rejected");
}

#[tokio::test]
async fn test_get_snow_conditions_boundary_coordinates() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_snow_conditions(
        90.0,
        180.0,
        None, None, None
    ).await;

    assert!(result.is_ok(), "Boundary coordinates should be valid");
}

#[tokio::test]
async fn test_get_snow_conditions_forecast_days_invalid_zero() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_snow_conditions(
        48.1,
        11.6,
        None, None, Some(0)
    ).await;

    assert!(result.is_err(), "forecast_days 0 should be invalid");
}

#[tokio::test]
async fn test_get_snow_conditions_forecast_days_invalid_too_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_snow_conditions(
        48.1,
        11.6,
        None, None, Some(17)
    ).await;

    assert!(result.is_err(), "forecast_days > 16 should be invalid");
}

#[tokio::test]
async fn test_get_snow_conditions_forecast_days_valid() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_snow_conditions(
        48.1,
        11.6,
        None, None, Some(7)
    ).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_snow_conditions_null_island() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_snow_conditions(
        0.0,
        0.0,
        None, None, None
    ).await;

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
