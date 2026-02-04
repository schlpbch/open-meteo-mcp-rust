//! Tool handler tests for Location Comparison
//! Phase 4: Comprehensive location comparison tool testing

use open_meteo_mcp::OpenMeteoService;

#[tokio::test]
async fn test_compare_locations_single_location_invalid() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let locations = vec![
        (48.1, 11.6),
    ];

    let result = service.compare_locations(
        locations,
        None, None, None
    ).await;

    assert!(result.is_err(), "Single location should be invalid");
}

#[tokio::test]
async fn test_compare_locations_empty_invalid() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let locations = vec![];

    let result = service.compare_locations(
        locations,
        None, None, None
    ).await;

    assert!(result.is_err(), "Empty locations should be invalid");
}

#[tokio::test]
async fn test_compare_locations_validation_latitude_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let locations = vec![
        (90.001, 11.6),
        (52.5, 13.4),
    ];

    let result = service.compare_locations(
        locations,
        None, None, None
    ).await;

    assert!(result.is_err(), "Invalid latitude should be rejected");
}

#[tokio::test]
async fn test_compare_locations_validation_longitude_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let locations = vec![
        (48.1, 180.001),
        (52.5, 13.4),
    ];

    let result = service.compare_locations(
        locations,
        None, None, None
    ).await;

    assert!(result.is_err(), "Invalid longitude should be rejected");
}

#[tokio::test]
async fn test_compare_locations_boundary_coordinates() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let locations = vec![
        (90.0, 180.0),
        (0.0, 0.0),
    ];

    let result = service.compare_locations(
        locations,
        None, None, None
    ).await;

    let _ = result;
}

#[tokio::test]
async fn test_compare_locations_with_forecast_days() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let locations = vec![
        (48.1, 11.6),
        (52.5, 13.4),
    ];

    let result = service.compare_locations(
        locations,
        None, None, Some(7)
    ).await;

    let _ = result;
}

#[tokio::test]
async fn test_compare_locations_forecast_days_too_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let locations = vec![
        (48.1, 11.6),
        (52.5, 13.4),
    ];

    let result = service.compare_locations(
        locations,
        None, None, Some(17)
    ).await;

    assert!(result.is_err(), "forecast_days 17 should be invalid");
}
