//! Streamable HTTP Transport Implementation
//!
//! HTTP transport for MCP protocol using rmcp's streamable-HTTP transport.
//! Provides:
//! - MCP protocol endpoint at `/sse` (POST, streamable HTTP)
//! - Health check endpoints (`/health`, `/ready`)
//! - CORS support for web clients
//! - Graceful shutdown handling

use crate::health::HealthChecker;
use crate::service::OpenMeteoService;
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::streamable_http_server::{StreamableHttpServerConfig, StreamableHttpService};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;

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
            "POST /sse": "MCP protocol streamable-HTTP endpoint (connect here)"
        },
        "documentation": "https://github.com/schlpbch/open-meteo-mcp-rust"
    });
    Json(info)
}

/// Create axum router with all endpoints
fn create_router(state: AppState) -> Router {
    let mcp_service = StreamableHttpService::new(
        {
            let service = state.service.clone();
            move || Ok((*service).clone())
        },
        Arc::new(LocalSessionManager::default()),
        StreamableHttpServerConfig::default(),
    );

    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
        .route("/sse/info", get(sse_info))
        .nest_service("/sse", mcp_service)
        .with_state(state)
        .layer(CorsLayer::permissive())
}

/// HTTP Server configuration and startup
pub async fn run_server(
    service: Arc<OpenMeteoService>,
    host: String,
    port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    // Port validation: u16 inherently prevents invalid values

    let health = Arc::new(HealthChecker::new(30));
    let state = AppState { service, health };

    let app = create_router(state);

    let addr: SocketAddr = format!("{host}:{port}").parse()?;
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

    let sigint = async { tokio::signal::ctrl_c().await };

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
