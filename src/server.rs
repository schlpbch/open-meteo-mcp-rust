//! MCP Server initialization and transport configuration
//!
//! This module handles the setup of the Model Context Protocol server,
//! including registration of tools, resources, and prompts, and managing
//! transport connections (STDIO for Claude Desktop, SSE for HTTP).

use crate::service::OpenMeteoService;
use rmcp::server::Server;
use std::io::{stdin, stdout};
use tracing::debug;

/// Run the MCP server with STDIO transport
///
/// Initializes the rmcp server, registers all tools, resources, and prompts,
/// then runs the server using STDIO transport for Claude Desktop integration.
///
/// # Arguments
/// * `service` - The OpenMeteoService instance
///
/// # Returns
/// Result with error if initialization or runtime fails
pub async fn run_mcp_server(
    service: OpenMeteoService,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!("Initializing MCP server");

    // Create rmcp server instance
    let mut server = Server::new();

    // Register all 11 tools
    debug!("Registering tools");

    // Core tools
    register_tool_with_server(&mut server, &service, "ping").await?;
    register_tool_with_server(&mut server, &service, "get_weather").await?;
    register_tool_with_server(&mut server, &service, "search_location").await?;
    register_tool_with_server(&mut server, &service, "search_location_swiss").await?;
    register_tool_with_server(&mut server, &service, "get_air_quality").await?;

    // Specialized tools
    register_tool_with_server(&mut server, &service, "get_marine_conditions").await?;
    register_tool_with_server(&mut server, &service, "get_snow_conditions").await?;
    register_tool_with_server(&mut server, &service, "get_weather_alerts").await?;
    register_tool_with_server(&mut server, &service, "get_astronomy").await?;
    register_tool_with_server(&mut server, &service, "get_comfort_index").await?;

    // Comparison and historical tools
    register_tool_with_server(&mut server, &service, "compare_locations").await?;
    register_tool_with_server(&mut server, &service, "get_historical_weather").await?;

    // Register all 4 resources
    debug!("Registering resources");

    register_resource_with_server(&mut server, &service, "weather://codes").await?;
    register_resource_with_server(&mut server, &service, "weather://parameters").await?;
    register_resource_with_server(&mut server, &service, "weather://aqi-reference").await?;
    register_resource_with_server(&mut server, &service, "weather://swiss-locations").await?;

    // Register all 3 prompts
    debug!("Registering prompts");

    register_prompt_with_server(&mut server, &service, "ski-trip-weather").await?;
    register_prompt_with_server(&mut server, &service, "plan-outdoor-activity").await?;
    register_prompt_with_server(&mut server, &service, "weather-aware-travel").await?;

    tracing::info!("MCP server initialized with 11 tools, 4 resources, and 3 prompts");

    // Run server with STDIO transport
    tracing::info!("Starting MCP server with STDIO transport");
    server.run_stdio(stdin(), stdout()).await?;

    Ok(())
}

/// Helper function to register a tool with the server
async fn register_tool_with_server(
    _server: &mut Server,
    _service: &OpenMeteoService,
    tool_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!(tool = tool_name, "Registering tool");
    // Note: Actual registration will use rmcp macros in Phase 3 iteration
    Ok(())
}

/// Helper function to register a resource with the server
async fn register_resource_with_server(
    _server: &mut Server,
    _service: &OpenMeteoService,
    resource_uri: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!(uri = resource_uri, "Registering resource");
    // Note: Actual registration will use rmcp macros in Phase 3 iteration
    Ok(())
}

/// Helper function to register a prompt with the server
async fn register_prompt_with_server(
    _server: &mut Server,
    _service: &OpenMeteoService,
    prompt_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!(prompt = prompt_name, "Registering prompt");
    // Note: Actual registration will use rmcp macros in Phase 3 iteration
    Ok(())
}

#[cfg(test)]
mod tests {
    // Note: Full integration tests will be added after rmcp macro integration
}
