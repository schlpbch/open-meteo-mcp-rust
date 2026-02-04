//! Tool handler tests for Swiss Location Search
//! Phase 4: Comprehensive Swiss location search tool testing

use open_meteo_mcp::OpenMeteoService;

#[tokio::test]
async fn test_search_location_swiss_success() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.search_location_swiss(
        "Zurich".to_string(),
        None
    ).await;

    assert!(result.is_ok(), "Swiss location search should succeed");
}

#[tokio::test]
async fn test_search_location_swiss_with_count() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.search_location_swiss(
        "Bern".to_string(),
        Some(5)
    ).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_search_location_swiss_empty_name() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.search_location_swiss(
        "".to_string(),
        None
    ).await;

    assert!(result.is_err(), "Empty location name should be rejected");
}

#[tokio::test]
async fn test_search_location_swiss_count_zero() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.search_location_swiss(
        "Geneva".to_string(),
        Some(0)
    ).await;

    assert!(result.is_err(), "count 0 should be invalid");
}

#[tokio::test]
async fn test_search_location_swiss_count_too_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.search_location_swiss(
        "Lausanne".to_string(),
        Some(101)
    ).await;

    assert!(result.is_err(), "count > 100 should be invalid");
}

#[tokio::test]
async fn test_search_location_swiss_count_valid_boundaries() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    for count in [1, 50, 100].iter() {
        let result = service.search_location_swiss(
            "Basel".to_string(),
            Some(*count)
        ).await;

        assert!(result.is_ok(), "count {} should be valid", count);
    }
}

#[tokio::test]
async fn test_search_location_swiss_various_cities() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    for city in &["Lugano", "Sion", "Thun", "Interlaken"] {
        let result = service.search_location_swiss(
            city.to_string(),
            Some(5)
        ).await;

        assert!(result.is_ok(), "Swiss city {} should be searchable", city);
    }
}
