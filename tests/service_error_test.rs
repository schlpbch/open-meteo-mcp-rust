//! Error handling and type conversion tests for Open-Meteo MCP
//! Phase 5: Service layer error handling

use open_meteo_mcp::{Error, McpError, CallToolResult, ToolContent};

// Error Type Creation Tests

#[test]
fn test_error_invalid_parameter_creation() {
    let err = Error::InvalidParameter("latitude too high".to_string());
    assert_eq!(err.to_string(), "Invalid parameter: latitude too high");
}

#[test]
fn test_error_api_error_creation() {
    let err = Error::ApiError("Bad request from API".to_string());
    assert_eq!(err.to_string(), "API error: Bad request from API");
}

#[test]
fn test_error_rate_limit_creation() {
    let err = Error::RateLimit { seconds: 60 };
    assert!(err.to_string().contains("60"));
    assert!(err.to_string().contains("Rate"));
}

#[test]
fn test_error_timeout_creation() {
    let err = Error::Timeout(30);
    assert_eq!(err.to_string(), "Timeout after 30 seconds");
}

#[test]
fn test_error_coordinates_creation() {
    let err = Error::InvalidCoordinates {
        lat: 91.0,
        lon: 180.0,
    };
    assert!(err.to_string().contains("91"));
    assert!(err.to_string().contains("180"));
}

#[test]
fn test_error_config_creation() {
    let err = Error::Config("Missing API key".to_string());
    assert_eq!(err.to_string(), "Configuration error: Missing API key");
}

#[test]
fn test_error_mcp_creation() {
    let err = Error::Mcp("Protocol error".to_string());
    assert_eq!(err.to_string(), "MCP error: Protocol error");
}

#[test]
fn test_error_internal_creation() {
    let err = Error::Internal("Unexpected state".to_string());
    assert_eq!(err.to_string(), "Internal error: Unexpected state");
}

// Error Conversion Tests (Error to McpError)

#[test]
fn test_error_to_mcp_invalid_parameter() {
    let err = Error::InvalidParameter("bad param".to_string());
    let mcp_err: McpError = err.into();
    assert_eq!(
        mcp_err.to_string(),
        "Invalid parameter: bad param"
    );
}

#[test]
fn test_error_to_mcp_invalid_coordinates() {
    let err = Error::InvalidCoordinates { lat: 91.0, lon: 200.0 };
    let mcp_err: McpError = err.into();
    assert!(mcp_err.to_string().contains("Invalid parameter"));
    assert!(mcp_err.to_string().contains("91"));
}

#[test]
fn test_error_to_mcp_rate_limit() {
    let err = Error::RateLimit { seconds: 120 };
    let mcp_err: McpError = err.into();
    assert!(mcp_err.to_string().contains("Rate limit"));
    assert!(mcp_err.to_string().contains("120"));
}

#[test]
fn test_error_to_mcp_timeout() {
    let err = Error::Timeout(45);
    let mcp_err: McpError = err.into();
    assert!(mcp_err.to_string().contains("Timeout"));
    assert!(mcp_err.to_string().contains("45"));
}

#[test]
fn test_error_to_mcp_api_error() {
    let err = Error::ApiError("Server error".to_string());
    let mcp_err: McpError = err.into();
    assert_eq!(mcp_err.to_string(), "Tool execution error: Server error");
}

#[test]
fn test_error_to_mcp_mcp_error() {
    let err = Error::Mcp("Protocol issue".to_string());
    let mcp_err: McpError = err.into();
    assert!(mcp_err.to_string().contains("Internal error"));
}

#[test]
fn test_error_to_mcp_config_error() {
    let err = Error::Config("Invalid config".to_string());
    let mcp_err: McpError = err.into();
    assert!(mcp_err.to_string().contains("Internal error"));
    assert!(mcp_err.to_string().contains("Config"));
}

#[test]
fn test_error_to_mcp_internal_error() {
    let err = Error::Internal("Unexpected failure".to_string());
    let mcp_err: McpError = err.into();
    assert!(mcp_err.to_string().contains("Internal error"));
}

// CallToolResult Tests

#[test]
fn test_call_tool_result_success_creation() {
    let content = vec![ToolContent::Text("Success".to_string())];
    let result = CallToolResult::success(content.clone());
    assert!(!result.is_error);
    assert_eq!(result.content.len(), 1);
}

#[test]
fn test_call_tool_result_error_creation() {
    let result = CallToolResult::error("Something failed".to_string());
    assert!(result.is_error);
    assert_eq!(result.content.len(), 1);
}

#[test]
fn test_call_tool_result_json_content() {
    let json_value = serde_json::json!({"temp": 25.0});
    let content = vec![ToolContent::Json(json_value.clone())];
    let result = CallToolResult::success(content);
    assert!(!result.is_error);
}

#[test]
fn test_call_tool_result_multiple_contents() {
    let contents = vec![
        ToolContent::Text("Title".to_string()),
        ToolContent::Json(serde_json::json!({"data": "value"})),
        ToolContent::Text("Footer".to_string()),
    ];
    let result = CallToolResult::success(contents);
    assert_eq!(result.content.len(), 3);
    assert!(!result.is_error);
}

// ToolContent Helpers Tests

#[test]
fn test_tool_content_text_creation() {
    let content = CallToolResult::text("Hello".to_string());
    match content {
        ToolContent::Text(s) => assert_eq!(s, "Hello"),
        _ => panic!("Expected Text variant"),
    }
}

#[test]
fn test_tool_content_json_creation() {
    let json = serde_json::json!({"key": "value"});
    let content = CallToolResult::json(json.clone());
    match content {
        ToolContent::Json(j) => assert_eq!(j["key"], "value"),
        _ => panic!("Expected Json variant"),
    }
}

// Coordinate Validation Tests (as part of error module)

#[test]
fn test_validate_coordinates_valid_munich() {
    let result = open_meteo_mcp::error::validate_coordinates(48.1, 11.6);
    assert!(result.is_ok());
}

#[test]
fn test_validate_coordinates_valid_boundaries() {
    // Maximum valid values
    assert!(open_meteo_mcp::error::validate_coordinates(90.0, 180.0).is_ok());
    assert!(open_meteo_mcp::error::validate_coordinates(-90.0, -180.0).is_ok());
}

#[test]
fn test_validate_coordinates_null_island() {
    assert!(open_meteo_mcp::error::validate_coordinates(0.0, 0.0).is_ok());
}

#[test]
fn test_validate_coordinates_invalid_latitude_high() {
    let result = open_meteo_mcp::error::validate_coordinates(90.1, 0.0);
    assert!(result.is_err());
}

#[test]
fn test_validate_coordinates_invalid_latitude_low() {
    let result = open_meteo_mcp::error::validate_coordinates(-90.1, 0.0);
    assert!(result.is_err());
}

#[test]
fn test_validate_coordinates_invalid_longitude_high() {
    let result = open_meteo_mcp::error::validate_coordinates(0.0, 180.1);
    assert!(result.is_err());
}

#[test]
fn test_validate_coordinates_invalid_longitude_low() {
    let result = open_meteo_mcp::error::validate_coordinates(0.0, -180.1);
    assert!(result.is_err());
}

#[test]
fn test_validate_coordinates_both_invalid() {
    let result = open_meteo_mcp::error::validate_coordinates(100.0, 200.0);
    assert!(result.is_err());
}

// McpError Type Tests

#[test]
fn test_mcp_error_invalid_request() {
    let err = McpError::InvalidRequest("bad request".to_string());
    assert_eq!(err.to_string(), "Invalid request: bad request");
}

#[test]
fn test_mcp_error_invalid_parameter() {
    let err = McpError::InvalidParameter("bad param".to_string());
    assert_eq!(err.to_string(), "Invalid parameter: bad param");
}

#[test]
fn test_mcp_error_resource_not_found() {
    let err = McpError::ResourceNotFound("not found".to_string());
    assert_eq!(err.to_string(), "Resource not found: not found");
}

#[test]
fn test_mcp_error_internal_error() {
    let err = McpError::InternalError("something broke".to_string());
    assert_eq!(err.to_string(), "Internal error: something broke");
}

#[test]
fn test_mcp_error_tool_error() {
    let err = McpError::ToolError("tool failed".to_string());
    assert_eq!(err.to_string(), "Tool execution error: tool failed");
}

#[test]
fn test_mcp_error_rate_limit() {
    let err = McpError::RateLimit("too many requests".to_string());
    assert!(err.to_string().contains("Rate limit"));
}

#[test]
fn test_mcp_error_timeout() {
    let err = McpError::Timeout("took too long".to_string());
    assert!(err.to_string().contains("Timeout"));
}
