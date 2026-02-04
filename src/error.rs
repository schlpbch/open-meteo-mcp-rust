//! Error types for Open-Meteo MCP

use thiserror::Error;

/// Result type alias using our Error type
pub type Result<T> = std::result::Result<T, Error>;

/// MCP Protocol error type (compatible with rmcp SDK)
#[derive(Error, Debug, Clone)]
pub enum McpError {
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Tool execution error: {0}")]
    ToolError(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    #[error("Timeout: {0}")]
    Timeout(String),
}

/// MCP Tool call result
#[derive(Debug, Clone)]
pub struct CallToolResult {
    pub content: Vec<ToolContent>,
    pub is_error: bool,
}

/// Tool response content
#[derive(Debug, Clone)]
pub enum ToolContent {
    Text(String),
    Json(serde_json::Value),
}

impl CallToolResult {
    /// Create a successful tool result
    pub fn success(content: Vec<ToolContent>) -> Self {
        Self {
            content,
            is_error: false,
        }
    }

    /// Create an error result
    pub fn error(message: String) -> Self {
        Self {
            content: vec![ToolContent::Text(message)],
            is_error: true,
        }
    }

    /// Convert to MCP protocol response
    pub fn text(content: String) -> ToolContent {
        ToolContent::Text(content)
    }

    pub fn json(value: serde_json::Value) -> ToolContent {
        ToolContent::Json(value)
    }
}

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

/// Convert from domain Error to MCP error
impl From<Error> for McpError {
    fn from(err: Error) -> Self {
        match err {
            Error::InvalidCoordinates { lat, lon } => {
                McpError::InvalidParameter(format!("Invalid coordinates: lat={}, lon={}", lat, lon))
            }
            Error::InvalidParameter(msg) => McpError::InvalidParameter(msg),
            Error::RateLimit { seconds } => {
                McpError::RateLimit(format!("Retry after {} seconds", seconds))
            }
            Error::Timeout(secs) => McpError::Timeout(format!("Timeout after {} seconds", secs)),
            Error::ApiError(msg) => McpError::ToolError(msg),
            Error::HttpClient(e) => McpError::InternalError(format!("HTTP error: {}", e)),
            Error::Serialization(e) => McpError::InternalError(format!("Serialization error: {}", e)),
            Error::Mcp(msg) => McpError::InternalError(msg),
            Error::Config(msg) => McpError::InternalError(format!("Config error: {}", msg)),
            Error::Io(e) => McpError::InternalError(format!("IO error: {}", e)),
            Error::Internal(msg) => McpError::InternalError(msg),
        }
    }
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
