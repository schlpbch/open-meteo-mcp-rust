# CLAUDE.md

AI development guide for the Open-Meteo MCP Rust project.

## Project Overview

**Open Meteo MCP (Rust)** - High-performance Rust implementation of the weather and climate data MCP server providing weather, snow conditions, and air quality data via [Open-Meteo API](https://open-meteo.com/) with minimal binary footprint.

**Status**: v2.0.0 - Production Ready
**Completed**: February 2026
**Test Coverage**: 258 tests | 72% code coverage (feature-complete parity with Java v2.0.2)

## Key Technologies

- Rust 1.75+, Tokio async runtime
- Official `rmcp` 0.3+ MCP SDK
- `reqwest` HTTP client, `axum` web framework
- `serde`/`schemars` for JSON + schema generation
- GitHub Actions for CI/CD

## Quick Commands

```bash
# Build & Test
cargo build
cargo test --all-features
cargo llvm-cov  # Coverage report

# Run Application
cargo run -- --transport stdio
cargo run -- --transport sse --port 8888

# Development
cargo clippy --all-targets
cargo fmt

# Release Build
cargo build --release  # Binary: target/release/open-meteo-mcp (~8 MB)
```

## Architecture

**Two Transport Modes (Phase-based):**

- **STDIO** - `/dev/stdin` ↔ `/dev/stdout` for Claude Desktop (Phase 0-1)
- **SSE** - HTTP Server-Sent Events for web clients (Phase 4+)

**11 MCP Tools**: `get_weather`, `get_snow_conditions`, `get_air_quality`, `search_location`, `search_location_swiss`, `get_weather_alerts`, `get_comfort_index`, `get_astronomy`, `compare_locations`, `get_historical_weather`, `get_marine_conditions`

**4 Resources**: weather codes, parameters, AQI reference, Swiss locations

**3 Prompts**: ski-trip, outdoor-activity, travel-planning

## Package Structure

```
src/
├── main.rs              # CLI entry point, transport selection
├── lib.rs               # Library re-exports
├── config.rs            # Configuration (env vars)
├── error.rs             # Unified error types
│
├── client/              # HTTP clients for 5 Open-Meteo APIs
│   ├── mod.rs
│   ├── weather.rs
│   ├── geocoding.rs
│   ├── air_quality.rs
│   ├── marine.rs
│   └── archive.rs
│
├── tools/               # 11 MCP tool implementations
│   ├── mod.rs
│   ├── weather.rs
│   ├── snow.rs
│   ├── air_quality.rs
│   ├── location.rs
│   ├── alerts.rs
│   ├── comfort.rs
│   ├── astronomy.rs
│   ├── compare.rs
│   ├── historical.rs
│   └── marine.rs
│
├── resources/           # 4 MCP resources (static data)
│   ├── mod.rs
│   ├── weather_codes.rs
│   ├── parameters.rs
│   ├── aqi.rs
│   └── swiss_locations.rs
│
├── prompts/             # 3 MCP prompts
│   ├── mod.rs
│   ├── ski_trip.rs
│   ├── outdoor_activity.rs
│   └── travel_planning.rs
│
├── transport/           # Transport implementations
│   ├── mod.rs
│   ├── stdio.rs         # STDIO for Claude Desktop
│   └── sse.rs           # SSE for HTTP
│
└── types/               # Typed request/response DTOs
    ├── mod.rs
    ├── weather.rs
    ├── location.rs
    ├── air_quality.rs
    └── marine.rs
```

## Development Phases

| Phase | Deliverable | Status | Tests |
| --- | --- | --- | --- |
| 0 | Scaffolding, ping tool, CI | ✅ Complete | - |
| 1 | HTTP clients for 5 APIs | ✅ Complete | 78 (library) |
| 2 | 11 tool implementations | ✅ Complete | 91 (Phase 4) |
| 3 | Resources & prompts | ✅ Complete | - |
| 4 | STDIO + SSE transports | ✅ Complete | - |
| 5 | Testing & coverage (72%) | ✅ Complete | 89 (Phase 5) |
| 6 | Docker & CD pipeline | ✅ Complete | - |

**Total**: ~2-3 weeks completed | **258 total tests** | **72% coverage**

## Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md) - Complete system design with mermaid diagrams
- [spec/ADR_COMPENDIUM.md](spec/ADR_COMPENDIUM.md) - Architecture decision records (12 ADRs)
- [README.md](README.md) - User guide and quick start
- [docs/openapi-open-meteo.yaml](docs/openapi-open-meteo.yaml) - REST API spec (if SSE+HTTP)

## Development Guidelines

### Core Patterns

- **Rust Records/Structs** for all DTOs (immutable, `serde` + `schemars`)
- **Tokio async** for all I/O (no blocking calls on main thread)
- **`#[tool]` macro** from `rmcp` for tool definitions
- **`thiserror` + `anyhow`** for error handling
- **`tracing`** for structured JSON logging
- **snake_case** for tool names (`get_weather`, not `getWeather`)
- **≥72% test coverage** target (unit + integration tests)

### New Tool Example

```rust
use rmcp::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MyToolRequest {
    pub param1: String,
    pub param2: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MyToolResponse {
    pub result: String,
}

impl OpenMeteoService {
    #[tool(description = "Tool description with examples")]
    pub async fn my_tool(
        &self,
        #[tool_param(description = "Parameter 1")] param1: String,
        #[tool_param(description = "Parameter 2 (optional)")] param2: Option<u8>,
    ) -> Result<CallToolResult, McpError> {
        let req = MyToolRequest { param1, param2 };
        let resp = self.perform_operation(&req).await?;

        let json = serde_json::to_string_pretty(&resp)?;
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
}
```

### Testing Pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{Mock, MockServer};

    #[tokio::test]
    async fn should_do_something() {
        // Arrange
        let mock_server = MockServer::start().await;
        // ... mock setup ...

        // Act
        let result = my_function().await;

        // Assert
        assert!(result.is_ok());
    }
}
```

## Configuration

**Environment** (.env):

```bash
LOG_LEVEL=info
TRANSPORT=stdio      # or "sse"
PORT=8888            # if SSE transport
API_BASE_URL=https://api.open-meteo.com
TIMEOUT_SECS=30
```

**Cargo.toml Dependencies** (Phase 0):

```toml
[dependencies]
rmcp = { version = "0.3", features = ["server", "transport-io"] }
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json", "gzip"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
schemars = "1"
thiserror = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json"] }
```

## MCP Components (Feature Parity with Java v2.0.2)

### Tools (11 total) ✅ Complete

| Tool | Description | Tests |
| --- | --- | --- |
| `get_weather` | Weather forecast with temperature, precipitation, wind | ✅ |
| `get_snow_conditions` | Snow depth, snowfall, mountain weather | ✅ |
| `get_air_quality` | AQI, pollutants, UV index, pollen | ✅ |
| `search_location` | Geocoding - search locations by name | ✅ |
| `search_location_swiss` | Swiss-specific location search | ✅ |
| `get_weather_alerts` | Weather alerts based on thresholds | ✅ |
| `get_comfort_index` | Outdoor activity comfort score (0-100) | ✅ |
| `get_astronomy` | Sunrise, sunset, golden hour, moon phase | ✅ |
| `compare_locations` | Multi-location weather comparison | ✅ |
| `get_historical_weather` | Historical weather data (1940-present) | ✅ |
| `get_marine_conditions` | Wave/swell data for lakes and coasts | ✅ |

### Resources (4 total) ✅ Complete

| Resource | Purpose | Status |
| --- | --- | --- |
| `weather://codes` | WMO weather code reference | ✅ |
| `weather://parameters` | Available API parameters | ✅ |
| `weather://aqi-reference` | AQI scales and health recommendations | ✅ |
| `weather://swiss-locations` | Swiss cities, mountains, passes | ✅ |

### Prompts (3 total) ✅ Complete

| Prompt | Description | Status |
| --- | --- | --- |
| `ski-trip-weather` | Ski trip planning with snow conditions | ✅ |
| `plan-outdoor-activity` | Weather-aware activity planning | ✅ |
| `weather-aware-travel` | Travel planning with weather integration | ✅ |

## Endpoints (Phase 4+)

- **App**: http://localhost:8888 (SSE when enabled)
- **MCP**: /sse (Server-Sent Events transport)
- **Health**: /health (basic liveness probe)

## Architecture Decisions (ADRs)

See [spec/ADR_COMPENDIUM.md](spec/ADR_COMPENDIUM.md) for detailed rationale.

Key decisions:
- ✅ **ADR-001**: Tokio async runtime
- ✅ **ADR-003**: STDIO transport Phase 0, SSE Phase 4
- ✅ **ADR-004**: Official `rmcp` 0.3+ SDK
- ✅ **ADR-006**: GitHub Releases (v0.1), crates.io (v0.2+)
- ✅ **ADR-011**: Unit + Integration testing, 72% coverage target

## Troubleshooting

```bash
# Build issues
cargo clean
cargo build -vv  # Verbose output

# Test failures
cargo test -- --nocapture  # Show println! output
cargo test -- --test-threads=1  # Single-threaded (easier debugging)

# Coverage reports
cargo llvm-cov --html  # HTML report in target/llvm-cov/html/index.html
```

## Important Reminders

1. **Async-first** — No blocking calls on main thread (ADR-001)
2. **Type safety** — Leverage `serde` + `schemars` for validation (ADR-008)
3. **Error handling** — Use `thiserror` for domain errors (ADR-009)
4. **Logging** — Structured JSON via `tracing` (ADR-010)
5. **Testing** — Unit + integration, target 72% coverage (ADR-011)
6. **Dependencies** — Minimize, audit regularly (ADR-012)

## Quick Links

- **Rust Book**: https://doc.rust-lang.org/book/
- **Tokio Docs**: https://tokio.rs/
- **rmcp SDK**: https://github.com/modelcontextprotocol/rust-sdk
- **Open-Meteo API**: https://open-meteo.com/en/docs
- **MCP Protocol**: https://modelcontextprotocol.io/

## Comparison with Java Version

| Aspect | Java v2.0.2 | Rust v2.0.0 |
| --- | --- | --- |
| Binary Size | 50 MB + JVM | 26.4 MB (Docker) |
| Cold Start | 2-5s | <100ms |
| Memory (idle) | ~150 MB | ~50-100 MB |
| Dependencies | 100+ (Maven) | ~16 direct (Cargo) |
| Chat Endpoint | ✅ Included | ❌ Dropped (client-side) |
| Transport | REST + MCP + SSE | STDIO + SSE |
| Test Coverage | 72% (426 tests) | ✅ 72% (258 tests) |
| Architecture Docs | Basic | ✅ Comprehensive (mermaid diagrams) |

---

**v2.0.0 (Production Ready)**
- Feature-complete parity with Java v2.0.2
- 258 comprehensive tests across 3 phases
- Enhanced documentation with mermaid architecture diagrams
- Production-ready Docker image (26.4MB)

**Maintainer**: @schlpbch
**License**: Apache-2.0

