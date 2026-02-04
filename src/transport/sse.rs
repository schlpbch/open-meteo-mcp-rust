//! SSE (Server-Sent Events) Transport Implementation
//!
//! HTTP transport for MCP protocol using Server-Sent Events for streaming responses.
//! Provides:
//! - SSE endpoint at `/sse` for MCP protocol messages
//! - Health check endpoints (`/health`, `/ready`)
//! - CORS support for web clients
//! - Graceful shutdown handling

use crate::health::HealthChecker;
use crate::service::OpenMeteoService;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router, Json,
};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{debug, error, info};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

/// Application state shared across handlers
#[derive(Clone)]
struct AppState {
    service: Arc<OpenMeteoService>,
    health: Arc<HealthChecker>,
}

/// Handler for health check (liveness probe)
async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    let response = state.health.liveness_response();
    (StatusCode::OK, Json(response))
}

/// Handler for readiness check
async fn readiness_check(State(state): State<AppState>) -> impl IntoResponse {
    let response = state.health.readiness_response(&state.service).await;
    let status = if response.status == "healthy" {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    (status, Json(response))
}

/// Simple SSE test endpoint (returns server info)
async fn sse_info(State(state): State<AppState>) -> impl IntoResponse {
    let info = json!({
        "type": "info",
        "version": crate::VERSION,
        "uptime_secs": state.health.get_uptime_secs(),
        "transport": "sse",
    });
    (StatusCode::OK, Json(info))
}

/// Root endpoint with API documentation
async fn root() -> impl IntoResponse {
    let info = json!({
        "name": "Open-Meteo MCP Server",
        "version": crate::VERSION,
        "description": "Weather, climate, and location data via MCP protocol",
        "endpoints": {
            "GET /": "This endpoint",
            "GET /health": "Liveness probe",
            "GET /ready": "Readiness probe",
            "GET /sse/info": "Server information",
            "GET /sse": "MCP protocol SSE endpoint (connect here)"
        },
        "documentation": "https://github.com/schlpbch/open-meteo-mcp-rust"
    });
    Json(info)
}

/// Create axum router with all endpoints
fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
        .route("/sse/info", get(sse_info))
        .route("/sse", get(sse_endpoint))
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .into_inner(),
        )
}

/// SSE endpoint for MCP protocol messages
///
/// Clients connect via GET /sse and receive server-sent events.
/// Currently returns server info; future version will handle bidirectional MCP protocol.
async fn sse_endpoint(
    State(_state): State<AppState>,
) -> impl IntoResponse {
    use futures::stream::{self, StreamExt};
    use axum::response::sse::{Event, Sse};

    debug!("SSE client connected");

    let stream = stream::once(async {
        // Send server capabilities
        let message = json!({
            "type": "initialize",
            "version": "2.0",
            "capabilities": {
                "tools": {
                    "listChanged": true
                },
                "resources": {
                    "listChanged": true
                },
                "prompts": {
                    "listChanged": true
                }
            },
            "serverInfo": {
                "name": "open-meteo-mcp",
                "version": crate::VERSION
            }
        });

        match Event::default().json_data(message) {
            Ok(event) => Ok(event),
            Err(e) => {
                error!("Failed to create SSE event: {}", e);
                Err("Event serialization failed")
            }
        }
    })
    .chain(stream::once(async {
        // Send available tools list
        let message = json!({
            "type": "tools/list",
            "tools": [
                {
                    "name": "get_weather",
                    "description": "Get weather forecast with temperature and precipitation"
                },
                {
                    "name": "get_snow_conditions",
                    "description": "Get snow conditions and snowfall data"
                },
                {
                    "name": "get_air_quality",
                    "description": "Get air quality index and pollutant data"
                },
                {
                    "name": "search_location",
                    "description": "Search for locations by name"
                },
                {
                    "name": "search_location_swiss",
                    "description": "Search for Swiss locations"
                },
                {
                    "name": "get_weather_alerts",
                    "description": "Get weather alerts based on thresholds"
                },
                {
                    "name": "get_astronomy",
                    "description": "Get astronomy data (sunrise, sunset, moon phase)"
                },
                {
                    "name": "get_marine_conditions",
                    "description": "Get marine and wave conditions"
                },
                {
                    "name": "get_comfort_index",
                    "description": "Get outdoor activity comfort index"
                },
                {
                    "name": "compare_locations",
                    "description": "Compare weather across multiple locations"
                },
                {
                    "name": "get_historical_weather",
                    "description": "Get historical weather data"
                }
            ]
        });

        match Event::default().json_data(message) {
            Ok(event) => Ok(event),
            Err(e) => {
                error!("Failed to create SSE event: {}", e);
                Err("Event serialization failed")
            }
        }
    }))
    .boxed();

    Sse::new(stream)
}

/// HTTP Server configuration and startup
pub async fn run_server(
    service: Arc<OpenMeteoService>,
    host: String,
    port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    // Port validation: u16 inherently prevents invalid values

    let health = Arc::new(HealthChecker::new(30));
    let state = AppState {
        service,
        health,
    };

    let app = create_router(state);

    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    let listener = TcpListener::bind(&addr).await?;

    info!("SSE transport listening on {}", addr);

    // Run server until SIGTERM
    let server = axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal());

    server.await?;
    info!("SSE server shut down gracefully");
    Ok(())
}

/// Graceful shutdown signal handler
async fn shutdown_signal() {
    let sigterm = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await
    };

    let sigint = async {
        tokio::signal::ctrl_c().await
    };

    tokio::select! {
        _ = sigterm => {
            info!("Received SIGTERM, shutting down");
        }
        _ = sigint => {
            info!("Received SIGINT, shutting down");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_clone() {
        // Verify state can be cloned for axum
        let service = Arc::new(OpenMeteoService::new(crate::Config::default()).unwrap());
        let health = Arc::new(HealthChecker::new(30));
        let state = AppState { service, health };
        let _cloned = state.clone();
    }
}
