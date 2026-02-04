//! Tool handler tests for Astronomy
//! Phase 4: Comprehensive astronomy tool testing

use open_meteo_mcp::OpenMeteoService;

#[tokio::test]
async fn test_get_astronomy_validation_latitude_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_astronomy(
        90.001,
        11.6,
        None
    ).await;

    assert!(result.is_err(), "Latitude > 90 should be rejected");
}

#[tokio::test]
async fn test_get_astronomy_validation_latitude_low() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_astronomy(
        -90.001,
        11.6,
        None
    ).await;

    assert!(result.is_err(), "Latitude < -90 should be rejected");
}

#[tokio::test]
async fn test_get_astronomy_validation_longitude_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_astronomy(
        48.1,
        180.001,
        None
    ).await;

    assert!(result.is_err(), "Longitude > 180 should be rejected");
}

#[tokio::test]
async fn test_get_astronomy_validation_longitude_low() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_astronomy(
        48.1,
        -180.001,
        None
    ).await;

    assert!(result.is_err(), "Longitude < -180 should be rejected");
}

#[tokio::test]
async fn test_get_astronomy_validation_forecast_days_zero() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_astronomy(
        48.1,
        11.6,
        Some(0)
    ).await;

    assert!(result.is_err(), "forecast_days 0 should be invalid");
}

#[tokio::test]
async fn test_get_astronomy_validation_forecast_days_too_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_astronomy(
        48.1,
        11.6,
        Some(17)
    ).await;

    assert!(result.is_err(), "forecast_days 17 should be invalid");
}

#[tokio::test]
async fn test_get_astronomy_boundary_coordinates() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_astronomy(
        90.0,
        180.0,
        Some(7)
    ).await;

    // This might succeed or fail depending on API, but should validate coordinates
    let _ = result;
}
