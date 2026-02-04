//! MCP Transport Layer - Multiple Protocol Support
//!
//! Provides transport abstraction for MCP protocol:
//! - STDIO: For Claude Desktop integration via standard input/output
//! - SSE: HTTP Server-Sent Events for web-based MCP clients
//!
//! Phase 4: Multi-transport architecture enabling both local (Claude Desktop)
//! and cloud deployment (Docker, Kubernetes) scenarios.

pub mod sse;
pub mod stdio;

use std::fmt;

/// Transport protocol modes
#[derive(Debug, Clone)]
pub enum TransportMode {
    /// STDIO transport for Claude Desktop
    Stdio,
    /// HTTP Server-Sent Events transport for web clients
    Sse {
        /// Host to bind to (e.g., "127.0.0.1", "0.0.0.0")
        host: String,
        /// Port to listen on (1-65535)
        port: u16,
    },
}

impl fmt::Display for TransportMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransportMode::Stdio => write!(f, "stdio"),
            TransportMode::Sse { host, port } => {
                write!(f, "sse ({}:{})", host, port)
            }
        }
    }
}

/// Configuration for MCP transport
#[derive(Debug, Clone)]
pub struct TransportConfig {
    /// Which transport mode to use
    pub mode: TransportMode,
    /// Request/response timeout in seconds
    pub timeout_secs: u64,
    /// Health check interval in seconds
    pub health_check_interval_secs: u64,
    /// Graceful shutdown timeout in seconds
    pub shutdown_timeout_secs: u64,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            mode: TransportMode::Stdio,
            timeout_secs: 30,
            health_check_interval_secs: 30,
            shutdown_timeout_secs: 10,
        }
    }
}

impl TransportConfig {
    /// Create SSE transport config
    pub fn sse(host: String, port: u16) -> Self {
        Self {
            mode: TransportMode::Sse { host, port },
            ..Default::default()
        }
    }

    /// Create STDIO transport config
    pub fn stdio() -> Self {
        Self {
            mode: TransportMode::Stdio,
            ..Default::default()
        }
    }

    /// Validate configuration settings
    pub fn validate(&self) -> Result<(), String> {
        match &self.mode {
            TransportMode::Sse { host, port } => {
                if port == &0 || port > &65535 {
                    return Err(format!("Invalid port number: {} (must be 1-65535)", port));
                }
                if host.is_empty() {
                    return Err("Host cannot be empty".to_string());
                }
            }
            TransportMode::Stdio => {}
        }

        if self.timeout_secs == 0 || self.timeout_secs > 300 {
            return Err(format!(
                "Invalid timeout: {} seconds (must be 1-300)",
                self.timeout_secs
            ));
        }

        if self.shutdown_timeout_secs == 0 || self.shutdown_timeout_secs > 60 {
            return Err(format!(
                "Invalid shutdown timeout: {} seconds (must be 1-60)",
                self.shutdown_timeout_secs
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_mode_display() {
        assert_eq!(TransportMode::Stdio.to_string(), "stdio");
        assert_eq!(
            TransportMode::Sse {
                host: "127.0.0.1".to_string(),
                port: 8888
            }
            .to_string(),
            "sse (127.0.0.1:8888)"
        );
    }

    #[test]
    fn test_config_validation_valid() {
        let config = TransportConfig::sse("127.0.0.1".to_string(), 8888);
        assert!(config.validate().is_ok());

        let config = TransportConfig::stdio();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_empty_host() {
        let config = TransportConfig {
            mode: TransportMode::Sse {
                host: String::new(),
                port: 8888,
            },
            timeout_secs: 30,
            health_check_interval_secs: 30,
            shutdown_timeout_secs: 10,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_invalid_timeout() {
        let config = TransportConfig {
            mode: TransportMode::Stdio,
            timeout_secs: 400,
            health_check_interval_secs: 30,
            shutdown_timeout_secs: 10,
        };
        assert!(config.validate().is_err());
    }
}
