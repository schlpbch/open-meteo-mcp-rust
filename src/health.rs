//! Health check service for MCP server
//!
//! Provides liveness and readiness checks for:
//! - Docker health checks
//! - Kubernetes probes (liveness & readiness)
//! - HTTP health endpoints
//!
//! Liveness: Basic process health check
//! Readiness: Full system readiness (API connectivity, service initialization)

use crate::service::OpenMeteoService;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Health status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    /// Service is healthy and ready
    Healthy,
    /// Service is running but degraded
    Degraded,
    /// Service is unhealthy
    Unhealthy,
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "healthy"),
            HealthStatus::Degraded => write!(f, "degraded"),
            HealthStatus::Unhealthy => write!(f, "unhealthy"),
        }
    }
}

/// Health check response
#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct HealthCheckResponse {
    /// Overall health status
    pub status: String,
    /// Human-readable message
    pub message: String,
    /// Unix timestamp of check
    pub timestamp: u64,
    /// Service version
    pub version: String,
    /// Uptime in seconds
    pub uptime_secs: u64,
}

/// Health check service with caching
pub struct HealthChecker {
    /// Cached readiness result
    is_ready: Arc<AtomicBool>,
    /// Last readiness check time (for TTL)
    last_check: Arc<tokio::sync::Mutex<SystemTime>>,
    /// Readiness check cache TTL in seconds
    readiness_ttl_secs: u64,
    /// Service startup time
    startup_time: SystemTime,
}

impl HealthChecker {
    /// Create new health checker
    pub fn new(readiness_ttl_secs: u64) -> Self {
        Self {
            is_ready: Arc::new(AtomicBool::new(false)),
            last_check: Arc::new(tokio::sync::Mutex::new(SystemTime::UNIX_EPOCH)),
            readiness_ttl_secs,
            startup_time: SystemTime::now(),
        }
    }

    /// Liveness check - process is running
    pub fn check_liveness(&self) -> HealthStatus {
        // Basic check: if we're running, we're alive
        HealthStatus::Healthy
    }

    /// Readiness check - system is ready to serve requests
    pub async fn check_readiness(&self, service: &OpenMeteoService) -> HealthStatus {
        let mut last_check = self.last_check.lock().await;
        let now = SystemTime::now();

        // Check if we have a cached result within TTL
        if let Ok(duration) = now.duration_since(*last_check) {
            if duration.as_secs() < self.readiness_ttl_secs {
                return if self.is_ready.load(Ordering::Relaxed) {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Degraded
                };
            }
        }

        // Perform actual readiness check
        let is_ready = self.perform_readiness_check(service).await;
        self.is_ready.store(is_ready, Ordering::Relaxed);
        *last_check = now;

        if is_ready {
            HealthStatus::Healthy
        } else {
            HealthStatus::Degraded
        }
    }

    /// Perform actual readiness check (test API connectivity)
    async fn perform_readiness_check(&self, service: &OpenMeteoService) -> bool {
        // Quick ping to verify Open-Meteo API is reachable
        // This is intentionally simple - just check we can reach the API
        match service.ping().await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Get service uptime in seconds
    pub fn get_uptime_secs(&self) -> u64 {
        self.startup_time
            .elapsed()
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }

    /// Create liveness response
    pub fn liveness_response(&self) -> HealthCheckResponse {
        HealthCheckResponse {
            status: "healthy".to_string(),
            message: "MCP server is running".to_string(),
            timestamp: current_unix_timestamp(),
            version: crate::VERSION.to_string(),
            uptime_secs: self.get_uptime_secs(),
        }
    }

    /// Create readiness response
    pub async fn readiness_response(
        &self,
        service: &OpenMeteoService,
    ) -> HealthCheckResponse {
        let status = self.check_readiness(service).await;
        HealthCheckResponse {
            status: status.to_string(),
            message: match status {
                HealthStatus::Healthy => "Service is ready to handle requests".to_string(),
                HealthStatus::Degraded => "Service is running but not ready (API unreachable)".to_string(),
                HealthStatus::Unhealthy => "Service is unhealthy".to_string(),
            },
            timestamp: current_unix_timestamp(),
            version: crate::VERSION.to_string(),
            uptime_secs: self.get_uptime_secs(),
        }
    }
}

/// Get current Unix timestamp
fn current_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_display() {
        assert_eq!(HealthStatus::Healthy.to_string(), "healthy");
        assert_eq!(HealthStatus::Degraded.to_string(), "degraded");
        assert_eq!(HealthStatus::Unhealthy.to_string(), "unhealthy");
    }

    #[test]
    fn test_health_checker_creation() {
        let checker = HealthChecker::new(30);
        assert_eq!(checker.check_liveness(), HealthStatus::Healthy);
        let _uptime = checker.get_uptime_secs();
    }

    #[test]
    fn test_uptime_tracking() {
        use std::time::Duration;
        let checker = HealthChecker::new(30);
        let uptime1 = checker.get_uptime_secs();
        std::thread::sleep(Duration::from_millis(100));
        let uptime2 = checker.get_uptime_secs();
        assert!(uptime2 >= uptime1);
    }

    #[test]
    fn test_liveness_response() {
        let checker = HealthChecker::new(30);
        let response = checker.liveness_response();
        assert_eq!(response.status, "healthy");
        assert!(response.timestamp > 0);
    }
}
