//! Tool handler tests for Air Quality
//! Phase 4: Comprehensive air quality tool testing

mod utils {
    pub use crate::utils::*;
}

use open_meteo_mcp::OpenMeteoService;

#[tokio::test]
async fn test_get_air_quality_success_minimal() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_air_quality(
        utils::VALID_LATITUDE,
        utils::VALID_LONGITUDE,
        None, None, None
    ).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_air_quality_with_parameters() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_air_quality(
        utils::VALID_LATITUDE,
        utils::VALID_LONGITUDE,
        Some("pm10,pm2_5,aqi".to_string()),
        Some("aqi".to_string()),
        Some(3)
    ).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_air_quality_validation_latitude() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_air_quality(
        utils::INVALID_LATITUDE_HIGH,
        utils::VALID_LONGITUDE,
        None, None, None
    ).await;
    
    assert!(result.is_err(), "Invalid latitude should be rejected");
}

#[tokio::test]
async fn test_get_air_quality_validation_longitude() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_air_quality(
        utils::VALID_LATITUDE,
        utils::INVALID_LONGITUDE_HIGH,
        None, None, None
    ).await;
    
    assert!(result.is_err(), "Invalid longitude should be rejected");
}

#[tokio::test]
async fn test_get_air_quality_forecast_days_valid() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    // AQI has different max than weather (5 instead of 16)
    for days in [1, 3, 5].iter() {
        let result = service.get_air_quality(
            utils::VALID_LATITUDE,
            utils::VALID_LONGITUDE,
            None, None, Some(*days)
        ).await;
        
        assert!(result.is_ok(), "AQI forecast_days {} should be valid", days);
    }
}

#[tokio::test]
async fn test_get_air_quality_forecast_days_invalid_too_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_air_quality(
        utils::VALID_LATITUDE,
        utils::VALID_LONGITUDE,
        None, None, Some(6)
    ).await;
    
    assert!(result.is_err(), "AQI forecast_days > 5 should be invalid");
}

#[tokio::test]
async fn test_get_air_quality_boundary_coordinates() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_air_quality(
        utils::LATITUDE_MIN,
        utils::LONGITUDE_MIN,
        None, None, None
    ).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_air_quality_null_island() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");
    
    let result = service.get_air_quality(
        utils::NULL_ISLAND_LAT,
        utils::NULL_ISLAND_LON,
        None, None, None
    ).await;
    
    assert!(result.is_ok());
}

#[test]
fn test_air_quality_request_serialization() {
    let req = open_meteo_mcp::types::air_quality::AirQualityRequest {
        latitude: utils::VALID_LATITUDE,
        longitude: utils::VALID_LONGITUDE,
        hourly: Some("pm10,pm2_5".to_string()),
        daily: Some("aqi".to_string()),
        forecast_days: Some(3),
        ..Default::default()
    };
    
    let json = serde_json::to_value(&req).expect("Valid JSON");
    assert_eq!(json["latitude"], utils::VALID_LATITUDE);
    assert_eq!(json["forecast_days"], 3);
}

#[test]
fn test_air_quality_request_validation_forecast_days() {
    let req = open_meteo_mcp::types::air_quality::AirQualityRequest {
        latitude: utils::VALID_LATITUDE,
        longitude: utils::VALID_LONGITUDE,
        hourly: None,
        daily: None,
        forecast_days: Some(6),
        ..Default::default()
    };
    
    assert!(req.validate().is_err(), "AQI forecast_days 6 should be invalid");
}
