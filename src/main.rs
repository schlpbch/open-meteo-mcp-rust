//! Open-Meteo MCP Server
//!
//! A Model Context Protocol (MCP) server providing weather, snow, air quality,
//! and location data via the Open-Meteo API.

mod server;

use clap::Parser;
use open_meteo_mcp::{Config, OpenMeteoService};
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "open-meteo-mcp")]
#[command(about = "Open-Meteo MCP Server - Weather and climate data via Open-Meteo API", long_about = None)]
#[command(version)]
struct Args {
    /// Transport mode: "stdio" (Claude Desktop) or "sse" (HTTP)
    #[arg(long, default_value = "stdio", env = "TRANSPORT")]
    transport: String,

    /// HTTP port (for SSE transport)
    #[arg(long, default_value = "8888", env = "PORT")]
    port: u16,

    /// Log level
    #[arg(long, env = "LOG_LEVEL")]
    log_level: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize logging
    let log_level = args.log_level.unwrap_or_else(|| "info".to_string());
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_env("LOG_LEVEL")
                .unwrap_or_else(|_| EnvFilter::new(&log_level))
        )
        .with_writer(std::io::stderr)
        .init();

    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        transport = %args.transport,
        "Starting Open-Meteo MCP Server"
    );

    // Load configuration
    let mut config = Config::from_env_or_default();
    config.port = args.port;
    config.transport = args.transport.clone();

    // Create service
    let service = OpenMeteoService::new(config)?;

    // Test ping tool
    match service.ping().await {
        Ok(ping_result) => {
            if !ping_result.is_error && !ping_result.content.is_empty() {
                tracing::info!("Ping tool validation successful");
            } else {
                tracing::error!("Ping tool returned error response");
                return Err("Ping tool validation failed".into());
            }
        }
        Err(e) => {
            tracing::error!(error = ?e, "Ping tool validation failed");
            return Err(format!("Service initialization failed: {}", e).into());
        }
    }

    // Wrap service in Arc for transport handlers
    let service = Arc::new(service);

    // Select transport mode
    match args.transport.as_str() {
        "stdio" => run_stdio(service).await?,
        "sse" => run_sse(service, args.port).await?,
        _ => {
            tracing::error!(transport = %args.transport, "Unknown transport mode");
            return Err(format!("Unknown transport: {}", args.transport).into());
        }
    }

    Ok(())
}

async fn run_stdio(service: Arc<OpenMeteoService>) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Using STDIO transport for Claude Desktop");

    // Run with STDIO transport
    open_meteo_mcp::transport::stdio::run_stdio_server(service).await?;

    tracing::info!("MCP server shutdown complete");

    Ok(())
}

async fn run_sse(
    service: Arc<OpenMeteoService>,
    port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let host = "127.0.0.1".to_string();
    tracing::info!(host = %host, port = port, "Using SSE transport");

    // Run with SSE transport
    open_meteo_mcp::transport::sse::run_server(service, host, port).await?;

    tracing::info!("SSE transport shutdown complete");

    Ok(())
}
