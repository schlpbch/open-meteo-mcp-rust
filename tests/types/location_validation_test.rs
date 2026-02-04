//! Type validation tests for Location/Geocoding
//! Mirrors Java LocationSearchRequestTest patterns

mod utils {
    pub use crate::utils::*;
}

use open_meteo_mcp::types::location::LocationRequest;

#[test]
fn test_location_request_valid() {
    let req = LocationRequest {
        name: "Munich".to_string(),
        count: Some(10),
        language: None,
        format: None,
    };
    
    assert!(req.validate().is_ok());
}

#[test]
fn test_location_request_name_empty() {
    let req = LocationRequest {
        name: String::new(),
        count: Some(10),
        language: None,
        format: None,
    };
    
    assert!(req.validate().is_err(), "Empty name should be invalid");
}

#[test]
fn test_location_request_count_valid_range() {
    for count in 1..=100 {
        let req = LocationRequest {
            name: "Munich".to_string(),
            count: Some(count),
            language: None,
            format: None,
        };
        
        assert!(req.validate().is_ok(), "count {} should be valid", count);
    }
}

#[test]
fn test_location_request_count_invalid_zero() {
    let req = LocationRequest {
        name: "Munich".to_string(),
        count: Some(0),
        language: None,
        format: None,
    };
    
    assert!(req.validate().is_err(), "count 0 should be invalid");
}

#[test]
fn test_location_request_count_invalid_too_high() {
    let req = LocationRequest {
        name: "Munich".to_string(),
        count: Some(101),
        language: None,
        format: None,
    };
    
    assert!(req.validate().is_err(), "count 101 should be invalid");
}

#[test]
fn test_location_request_count_none_default() {
    let req = LocationRequest {
        name: "Munich".to_string(),
        count: None,
        language: None,
        format: None,
    };
    
    assert!(req.validate().is_ok(), "count None should use default");
}

#[test]
fn test_location_response_deserialization() {
    let fixture = utils::parse_fixture::<open_meteo_mcp::types::location::LocationResponse>(
        "location_response.json"
    );
    
    assert!(!fixture.results.is_empty());
    let first = &fixture.results[0];
    assert_eq!(first.name, "Munich");
    assert_eq!(first.country_code, "DE");
}

#[test]
fn test_location_response_empty_results() {
    let json = r#"{"results": [], "generationtime_ms": 5.0}"#;
    let response: Result<open_meteo_mcp::types::location::LocationResponse, _> =
        serde_json::from_str(json);
    assert!(response.is_ok());
}

#[test]
fn test_location_serialization() {
    let req = LocationRequest {
        name: "Berlin".to_string(),
        count: Some(5),
        language: Some("en".to_string()),
        format: None,
    };
    
    let json = serde_json::to_value(&req).expect("Valid JSON");
    assert_eq!(json["name"], "Berlin");
    assert_eq!(json["count"], 5);
    assert_eq!(json["language"], "en");
}
