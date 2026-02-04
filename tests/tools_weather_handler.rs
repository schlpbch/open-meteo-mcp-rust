//! Tool handler tests for Weather/Forecast - Phase 4
//! Comprehensive weather tool testing with all parameters

mod utils {
    pub use crate::*;
}

use open_meteo_mcp::OpenMeteoService;

// ... (copy all 12 tests from tools/weather_handler_test.rs)

#[tokio::test]
async fn test_get_weather_success_minimal_params() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_weather(
        48.1, 11.6, None, None, None, None
    ).await;
    
    assert!(result.is_ok(), "Weather request with minimal params should succeed");
}

#[tokio::test]
async fn test_get_weather_validation_latitude_too_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_weather(90.001, 11.6, None, None, None, None).await;
    assert!(result.is_err(), "Latitude > 90 should be rejected");
}

#[tokio::test]
async fn test_get_weather_validation_longitude_too_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_weather(48.1, 180.001, None, None, None, None).await;
    assert!(result.is_err(), "Longitude > 180 should be rejected");
}

#[tokio::test]
async fn test_get_weather_boundary_latitude_max() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_weather(90.0, 0.0, None, None, None, None).await;
    assert!(result.is_ok(), "Latitude 90.0 should be valid");
}

#[tokio::test]
async fn test_get_weather_boundary_longitude_min() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_weather(0.0, -180.0, None, None, None, None).await;
    assert!(result.is_ok(), "Longitude -180.0 should be valid");
}

#[tokio::test]
async fn test_get_weather_null_island() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_weather(0.0, 0.0, None, None, None, None).await;
    assert!(result.is_ok(), "Null Island should be valid");
}

#[tokio::test]
async fn test_get_weather_forecast_days_valid_boundary() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_weather(48.1, 11.6, None, None, Some(16), None).await;
    assert!(result.is_ok(), "forecast_days 16 should be valid");
}

#[tokio::test]
async fn test_get_weather_forecast_days_invalid_zero() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_weather(48.1, 11.6, None, None, Some(0), None).await;
    assert!(result.is_err(), "forecast_days 0 should be invalid");
}

#[tokio::test]
async fn test_get_weather_forecast_days_invalid_too_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_weather(48.1, 11.6, None, None, Some(17), None).await;
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
