//! Open-Meteo MCP Server
//!
//! A Model Context Protocol (MCP) server providing weather, snow, air quality,
//! and location data via the Open-Meteo API.

pub mod client;
pub mod config;
pub mod error;
pub mod service;
pub mod tools;
pub mod types;

pub use client::OpenMeteoClient;
pub use config::Config;
pub use error::{Error, Result};
pub use service::OpenMeteoService;

/// Version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
