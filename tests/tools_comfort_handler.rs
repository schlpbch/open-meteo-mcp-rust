//! Tool handler tests for Comfort Index
//! Phase 4: Comprehensive comfort index tool testing

use open_meteo_mcp::OpenMeteoService;

#[tokio::test]
async fn test_get_comfort_index_validation_latitude_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_comfort_index(
        90.001,
        11.6,
        None, None
    ).await;

    assert!(result.is_err(), "Invalid latitude should be rejected");
}

#[tokio::test]
async fn test_get_comfort_index_validation_latitude_low() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_comfort_index(
        -90.001,
        11.6,
        None, None
    ).await;

    assert!(result.is_err(), "Invalid latitude should be rejected");
}

#[tokio::test]
async fn test_get_comfort_index_validation_longitude_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_comfort_index(
        48.1,
        180.001,
        None, None
    ).await;

    assert!(result.is_err(), "Invalid longitude should be rejected");
}

#[tokio::test]
async fn test_get_comfort_index_validation_longitude_low() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_comfort_index(
        48.1,
        -180.001,
        None, None
    ).await;

    assert!(result.is_err(), "Invalid longitude should be rejected");
}

#[tokio::test]
async fn test_get_comfort_index_validation_forecast_days_zero() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_comfort_index(
        48.1,
        11.6,
        None, Some(0)
    ).await;

    assert!(result.is_err(), "forecast_days 0 should be invalid");
}

#[tokio::test]
async fn test_get_comfort_index_validation_forecast_days_too_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_comfort_index(
        48.1,
        11.6,
        None, Some(17)
    ).await;

    assert!(result.is_err(), "forecast_days 17 should be invalid");
}

#[tokio::test]
async fn test_get_comfort_index_boundary_latitude() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_comfort_index(
        90.0,
        11.6,
        None, Some(5)
    ).await;

    let _ = result;
}

#[tokio::test]
async fn test_get_comfort_index_boundary_longitude() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_comfort_index(
        48.1,
        180.0,
        None, Some(5)
    ).await;

    let _ = result;
}

#[tokio::test]
async fn test_get_comfort_index_boundary_forecast_days() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_comfort_index(
        48.1,
        11.6,
        None, Some(16)
    ).await;

    let _ = result;
}
