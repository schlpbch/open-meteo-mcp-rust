//! Open-Meteo MCP Server
//!
//! A Model Context Protocol (MCP) server providing weather, snow, air quality,
//! and location data via the Open-Meteo API.

pub mod client;
pub mod config;
pub mod error;
pub mod health;
pub mod mcp;
pub mod prompts;
pub mod resources;
pub mod service;
pub mod tools;
pub mod transport;
pub mod types;

pub use client::OpenMeteoClient;
pub use config::Config;
pub use error::{CallToolResult, Error, McpError, Result, ToolContent};
pub use health::HealthChecker;
pub use service::OpenMeteoService;
pub use transport::TransportMode;

/// Version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
