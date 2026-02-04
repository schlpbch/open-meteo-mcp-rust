//! Tool handler tests for Weather/Forecast
//! Phase 4: Comprehensive weather tool testing with all parameters

mod utils {
    pub use crate::utils::*;
}

use open_meteo_mcp::OpenMeteoService;

#[tokio::test]
async fn test_get_weather_success_minimal_params() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_weather(
        utils::VALID_LATITUDE,
        utils::VALID_LONGITUDE,
        None, None, None, None
    ).await;
    
    assert!(result.is_ok(), "Weather request with minimal params should succeed");
    let call_result = result.unwrap();
    assert!(!call_result.is_error);
}

#[tokio::test]
async fn test_get_weather_success_with_all_params() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_weather(
        utils::VALID_LATITUDE,
        utils::VALID_LONGITUDE,
        Some("temperature_2m,precipitation".to_string()),
        Some("temperature_2m_max".to_string()),
        Some(7),
        Some("fahrenheit".to_string())
    ).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_weather_validation_latitude_too_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_weather(
        utils::INVALID_LATITUDE_HIGH,
        utils::VALID_LONGITUDE,
        None, None, None, None
    ).await;
    
    assert!(result.is_err(), "Latitude > 90 should be rejected");
}

#[tokio::test]
async fn test_get_weather_validation_latitude_too_low() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_weather(
        utils::INVALID_LATITUDE_LOW,
        utils::VALID_LONGITUDE,
        None, None, None, None
    ).await;
    
    assert!(result.is_err(), "Latitude < -90 should be rejected");
}

#[tokio::test]
async fn test_get_weather_validation_longitude_too_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_weather(
        utils::VALID_LATITUDE,
        utils::INVALID_LONGITUDE_HIGH,
        None, None, None, None
    ).await;
    
    assert!(result.is_err(), "Longitude > 180 should be rejected");
}

#[tokio::test]
async fn test_get_weather_validation_longitude_too_low() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_weather(
        utils::VALID_LATITUDE,
        utils::INVALID_LONGITUDE_LOW,
        None, None, None, None
    ).await;
    
    assert!(result.is_err(), "Longitude < -180 should be rejected");
}

#[tokio::test]
async fn test_get_weather_boundary_coordinates() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    // Test all four corners
    let corners = vec![
        (utils::LATITUDE_MAX, utils::LONGITUDE_MAX),
        (utils::LATITUDE_MAX, utils::LONGITUDE_MIN),
        (utils::LATITUDE_MIN, utils::LONGITUDE_MAX),
        (utils::LATITUDE_MIN, utils::LONGITUDE_MIN),
    ];
    
    for (lat, lon) in corners {
        let result = service.get_weather(lat, lon, None, None, None, None).await;
        assert!(result.is_ok(), "Corner ({}, {}) should be valid", lat, lon);
    }
}

#[tokio::test]
async fn test_get_weather_null_island() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_weather(
        utils::NULL_ISLAND_LAT,
        utils::NULL_ISLAND_LON,
        None, None, None, None
    ).await;
    
    assert!(result.is_ok(), "Null Island (0,0) should be valid");
}

#[tokio::test]
async fn test_get_weather_forecast_days_invalid_zero() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_weather(
        utils::VALID_LATITUDE,
        utils::VALID_LONGITUDE,
        None, None, Some(0), None
    ).await;
    
    assert!(result.is_err(), "forecast_days 0 should be invalid");
}

#[tokio::test]
async fn test_get_weather_forecast_days_invalid_too_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_weather(
        utils::VALID_LATITUDE,
        utils::VALID_LONGITUDE,
        None, None, Some(17), None
    ).await;
    
    assert!(result.is_err(), "forecast_days 17 should be invalid (max is 16)");
}

#[tokio::test]
async fn test_get_weather_forecast_days_valid_boundaries() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    // Test min and max valid values
    for days in [1, 7, 16].iter() {
        let result = service.get_weather(
            utils::VALID_LATITUDE,
            utils::VALID_LONGITUDE,
            None, None, Some(*days), None
        ).await;
        
        assert!(result.is_ok(), "forecast_days {} should be valid", days);
    }
}

#[test]
fn test_weather_request_json_serialization() {
    let req = open_meteo_mcp::types::weather::WeatherRequest {
        latitude: utils::VALID_LATITUDE,
        longitude: utils::VALID_LONGITUDE,
        hourly: Some("temperature_2m".to_string()),
        daily: Some("precipitation_sum".to_string()),
        forecast_days: Some(7),
        ..Default::default()
    };
    
    let json = serde_json::to_value(&req).expect("Valid JSON");
    assert_eq!(json["latitude"], utils::VALID_LATITUDE);
    assert_eq!(json["longitude"], utils::VALID_LONGITUDE);
    assert_eq!(json["forecast_days"], 7);
}
