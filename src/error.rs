//! Error types for Open-Meteo MCP

use thiserror::Error;

/// Result type alias using our Error type
pub type Result<T> = std::result::Result<T, Error>;

/// Unified error type for the application
#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP client error: {0}")]
    HttpClient(#[from] reqwest::Error),

    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("MCP error: {0}")]
    Mcp(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Invalid coordinates: latitude {lat}, longitude {lon}")]
    InvalidCoordinates { lat: f64, lon: f64 },

    #[error("Rate limited: retry after {seconds} seconds")]
    RateLimit { seconds: u64 },

    #[error("Timeout after {0} seconds")]
    Timeout(u64),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Convenience function to validate coordinates
pub fn validate_coordinates(latitude: f64, longitude: f64) -> Result<()> {
    if !(-90.0..=90.0).contains(&latitude) {
        return Err(Error::InvalidCoordinates {
            lat: latitude,
            lon: longitude,
        });
    }
    if !(-180.0..=180.0).contains(&longitude) {
        return Err(Error::InvalidCoordinates {
            lat: latitude,
            lon: longitude,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_coordinates() {
        assert!(validate_coordinates(48.1, 11.6).is_ok());
        assert!(validate_coordinates(0.0, 0.0).is_ok());
        assert!(validate_coordinates(90.0, 180.0).is_ok());
        assert!(validate_coordinates(-90.0, -180.0).is_ok());
    }

    #[test]
    fn test_validate_invalid_coordinates() {
        assert!(validate_coordinates(91.0, 0.0).is_err());
        assert!(validate_coordinates(-91.0, 0.0).is_err());
        assert!(validate_coordinates(0.0, 181.0).is_err());
        assert!(validate_coordinates(0.0, -181.0).is_err());
    }
}
