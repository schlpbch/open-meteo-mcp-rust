//! Open-Meteo MCP Server
//!
//! A Model Context Protocol (MCP) server providing weather, snow, air quality,
//! and location data via the Open-Meteo API.

mod server;

use clap::Parser;
use open_meteo_mcp::{Config, OpenMeteoService};
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
        Ok(ping_response) => {
            tracing::info!(
                message = ping_response.message,
                timestamp = ping_response.timestamp,
                "Ping tool validation successful"
            );
        }
        Err(e) => {
            tracing::error!(error = ?e, "Ping tool validation failed");
            return Err(format!("Service initialization failed: {}", e).into());
        }
    }

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

async fn run_stdio(service: OpenMeteoService) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Using STDIO transport for Claude Desktop");

    // Phase 3: Run the MCP server with STDIO transport
    server::run_mcp_server(service).await?;

    tracing::info!("MCP server shutdown complete");

    Ok(())
}

async fn run_sse(
    _service: OpenMeteoService,
    port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!(port = port, "Using SSE transport");

    // TODO: Phase 4 - Implement axum server with SSE transport
    tracing::info!("Phase 0: SSE transport initialized (will be implemented in Phase 4)");

    // For now, keep the process alive
    tokio::signal::ctrl_c().await?;
    tracing::info!("Shutdown signal received");

    Ok(())
}
