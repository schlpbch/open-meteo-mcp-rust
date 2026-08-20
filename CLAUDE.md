# CLAUDE.md

AI development guide for the Open-Meteo MCP Rust project.

## Project Overview

**Open Meteo MCP (Rust)** - High-performance Rust implementation of the weather and climate data MCP server providing weather, snow conditions, and air quality data via [Open-Meteo API](https://open-meteo.com/) with minimal binary footprint.

**Status**: v2.0.0 - Production Ready
**Completed**: February 2026 · **rmcp SDK wired up**: August 2026
**Test Coverage**: 280 tests | 72% code coverage (feature-complete parity with Java v2.0.2)

## Key Technologies

- Rust 1.88+, Tokio async runtime
- Official `rmcp` 3.1 MCP SDK (real `ServerHandler`/`#[tool_router]` wiring in `src/mcp.rs`)
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

**Two Transport Modes, both served by the real `rmcp` SDK (`src/mcp.rs`):**

- **STDIO** - `/dev/stdin` ↔ `/dev/stdout` for Claude Desktop, via `rmcp::ServiceExt::serve` + `rmcp::transport::stdio`
- **Streamable HTTP** - `/sse` (POST, JSON-RPC) for web clients, via `rmcp::transport::streamable_http_server::StreamableHttpService` nested into the axum router

**11 MCP Tools**: `get_weather`, `get_snow_conditions`, `get_air_quality`, `search_location`, `search_location_swiss`, `get_weather_alerts`, `get_comfort_index`, `get_astronomy`, `compare_locations`, `get_historical_weather`, `get_marine_conditions`

**4 Resources**: weather codes, parameters, AQI reference, Swiss locations

**3 Prompts**: ski-trip, outdoor-activity, travel-planning

## Package Structure

```
src/
├── main.rs              # CLI entry point, transport selection
├── lib.rs               # Library re-exports
├── config.rs            # Configuration (env vars)
├── error.rs             # Unified error types (project-internal CallToolResult/McpError)
├── service.rs           # OpenMeteoService — core state + ToolRouter field
├── mcp.rs               # rmcp wiring: #[tool_router], ServerHandler, resources, prompts
│
├── client/              # HTTP clients for 5 Open-Meteo APIs
│   ├── mod.rs
│   ├── weather.rs
│   ├── geocoding.rs
│   ├── air_quality.rs
│   ├── marine.rs
│   └── archive.rs
│
├── tools/                    # 11 MCP tool implementations (business logic)
│   ├── mod.rs                # ping
│   ├── weather.rs
│   ├── snow.rs
│   ├── air_quality.rs
│   ├── location.rs
│   ├── location_swiss.rs
│   ├── alerts.rs
│   ├── comfort.rs
│   ├── astronomy.rs
│   ├── comparison.rs
│   ├── historical.rs
│   └── marine.rs
│
├── resources/            # 4 MCP resources (static JSON data, embedded via include_str!)
│   └── mod.rs
│
├── prompts/              # 3 MCP prompts
│   └── mod.rs
│
├── transport/            # Transport implementations
│   ├── mod.rs
│   ├── stdio.rs          # STDIO for Claude Desktop (rmcp::ServiceExt::serve)
│   └── sse.rs            # Streamable HTTP for web clients (rmcp StreamableHttpService)
│
└── types/                # Typed request/response DTOs
    ├── mod.rs
    ├── weather.rs
    ├── location.rs
    ├── air_quality.rs
    ├── marine.rs
    ├── snow.rs
    ├── alerts.rs
    ├── astronomy.rs
    ├── comfort.rs
    └── comparison.rs
```

## Development Phases

| Phase | Deliverable | Status | Tests |
| --- | --- | --- | --- |
| 0 | Scaffolding, ping tool, CI | ✅ Complete | - |
| 1 | HTTP clients for 5 APIs | ✅ Complete | 81 (library) |
| 2 | 11 tool implementations | ✅ Complete | 199 (integration) |
| 3 | Resources & prompts | ✅ Complete | - |
| 4 | STDIO + SSE transports | ✅ Complete | - |
| 5 | Testing & coverage (72%) | ✅ Complete | 280 (total) |
| 6 | Docker & CD pipeline | ✅ Complete | - |
| 7 | Wire up real `rmcp` 3.1 SDK (`src/mcp.rs`) | ✅ Complete | +2 (router/capabilities) |

**Total**: ~2-3 weeks completed | **280 total tests** | **72% coverage**

## Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md) - Complete system design with mermaid diagrams
- [spec/ADR_COMPENDIUM.md](spec/ADR_COMPENDIUM.md) - Architecture decision records (12 ADRs)
- [README.md](README.md) - User guide and quick start
- [docs/openapi-open-meteo.yaml](docs/openapi-open-meteo.yaml) - REST API spec (if SSE+HTTP)

## Development Guidelines

### Core Patterns

- **Rust Records/Structs** for all DTOs (immutable, `serde` + `schemars`)
- **Tokio async** for all I/O (no blocking calls on main thread)
- **Two-layer tool pattern**: business logic lives on `OpenMeteoService` in `src/tools/*.rs`
  (returns the project's own `Result<CallToolResult, McpError>` from `src/error.rs`); a thin
  `#[tool]`-annotated wrapper in `src/mcp.rs` adapts it to `rmcp`'s real types via
  `to_rmcp_result()`. This keeps the 280 existing tests (which call the business-logic
  methods directly) decoupled from the MCP SDK's wire types.
- **`thiserror` + `anyhow`** for error handling
- **`tracing`** for structured JSON logging
- **snake_case** for tool names (`get_weather`, not `getWeather`)
- **≥72% test coverage** target (unit + integration tests)

### New Tool Example

Business logic goes in `src/tools/my_tool.rs`, exactly like the existing 11 tools:

```rust
use crate::service::OpenMeteoService;
use crate::{CallToolResult, McpError, ToolContent};

impl OpenMeteoService {
    pub async fn my_tool(
        &self,
        param1: String,
        param2: Option<u8>,
    ) -> std::result::Result<CallToolResult, McpError> {
        let resp = self.perform_operation(param1, param2).await?;
        let json = serde_json::to_value(&resp)
            .map_err(|e| McpError::InternalError(e.to_string()))?;
        Ok(CallToolResult::success(vec![ToolContent::Json(json)]))
    }
}
```

Then wire it into MCP in `src/mcp.rs`: add a `Parameters<T>` struct next to the others,
and a `#[tool]` wrapper inside the `#[tool_router]` impl block on `OpenMeteoService`:

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MyToolParams {
    #[schemars(description = "Parameter 1")]
    pub param1: String,
    #[schemars(description = "Parameter 2 (optional)")]
    pub param2: Option<u8>,
}

// inside #[tool_router] impl OpenMeteoService { ... }
#[tool(name = "my_tool", description = "Tool description with examples")]
pub async fn tool_my_tool(
    &self,
    Parameters(p): Parameters<MyToolParams>,
) -> std::result::Result<rmcp::model::CallToolResult, ErrorData> {
    to_rmcp_result(self.my_tool(p.param1, p.param2).await)
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

**Cargo.toml Dependencies** (current):

```toml
[dependencies]
rmcp = { version = "3.1", features = ["server", "transport-io", "transport-streamable-http-server"] }
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.13", features = ["json", "gzip", "query"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
schemars = "1"
thiserror = "2"
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

## Endpoints (streamable HTTP transport)

- **App**: http://localhost:8888
- **MCP**: /sse (POST, JSON-RPC over `rmcp`'s streamable-HTTP transport — connect here)
- **Health**: /health (basic liveness probe), /ready (readiness probe)

## Architecture Decisions (ADRs)

See [spec/ADR_COMPENDIUM.md](spec/ADR_COMPENDIUM.md) for detailed rationale.

Key decisions:
- ✅ **ADR-001**: Tokio async runtime
- ✅ **ADR-003**: STDIO transport Phase 0, SSE Phase 4
- ✅ **ADR-004**: Official `rmcp` SDK (now on 3.1, actually wired into `ServerHandler`/`#[tool_router]` as of August 2026 — see `src/mcp.rs`)
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
| Transport | REST + MCP + SSE | STDIO + Streamable HTTP |
| Test Coverage | 72% (426 tests) | ✅ 72% (280 tests) |
| Architecture Docs | Basic | ✅ Comprehensive (mermaid diagrams) |

---

**v2.0.0 (Production Ready)**
- Feature-complete parity with Java v2.0.2
- 280 comprehensive tests across 3 phases
- Real `rmcp` 3.1 SDK wiring (`src/mcp.rs`) — STDIO and streamable HTTP both actually serve MCP requests
- Enhanced documentation with mermaid architecture diagrams
- Production-ready Docker image (26.4MB)

**Maintainer**: @schlpbch
**License**: Apache-2.0

