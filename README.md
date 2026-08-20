# Open-Meteo MCP (Rust)

[![Status: Production Ready](https://img.shields.io/badge/status-production%20ready-brightgreen?style=flat-square)](https://github.com/schlpbch/open-meteo-mcp-rust/releases/tag/v1.2.0)
[![Tests: 280](https://img.shields.io/badge/tests-280%20passing-brightgreen?style=flat-square)](tests/)
[![Coverage: 72%](https://img.shields.io/badge/coverage-72%25-brightgreen?style=flat-square)](ARCHITECTURE.md)
[![Rust: 1.88+](https://img.shields.io/badge/rust-1.88%2B-orange?style=flat-square)](https://www.rust-lang.org/)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue?style=flat-square)](LICENSE)
[![Docker: 26.4 MB](https://img.shields.io/badge/docker-26.4%20MB-blueviolet?style=flat-square)](Dockerfile)

A lightweight, high-performance Model Context Protocol (MCP) server providing weather, snow conditions, air quality, and location data via the [Open-Meteo API](https://open-meteo.com/), built on the official [`rmcp`](https://github.com/modelcontextprotocol/rust-sdk) 3.1 SDK.

**Rust port of [open-meteo-mcp-java](https://github.com/schlpbch/open-meteo-mcp-java)** — featuring a 26.4 MB Docker image with <100ms cold start and 280 comprehensive tests.

**Status**: ✅ v1.2.0 - Production Ready | 72% test coverage | Feature-complete parity with Java v2.0.2

---

## ✨ Highlights

- ⚡ **Sub-100ms Cold Start** — Lightning-fast initialization
- 📦 **26.4 MB Docker Image** — Lightweight distribution
- 🎯 **11 MCP Tools** — Complete weather & climate toolkit
- 🧪 **280 Comprehensive Tests** — Production-grade quality assurance
- 🔄 **STDIO + Streamable HTTP Transports** — Real `rmcp` 3.1 SDK wiring, flexible deployment
- 🦀 **Pure Rust** — Type-safe, memory-efficient, no GC pauses
- 📊 **Connection Pooling** — High-performance HTTP client with retry logic
- 🌍 **Open-Meteo API** — Free, reliable weather data source

---

## Quick Start

### Prerequisites

- Rust 1.88+ ([install](https://rustup.rs/))
- macOS / Linux / Windows

### Installation

```bash
# Clone repository
git clone https://github.com/schlpbch/open-meteo-mcp-rust.git
cd open-meteo-mcp-rust

# Build
cargo build --release

# Binary location
./target/release/open-meteo-mcp
```

### Usage

#### Claude Desktop

1. Copy the binary to a location in your PATH:
   ```bash
   cp target/release/open-meteo-mcp ~/.local/bin/
   ```

2. Add to Claude Desktop config (`~/.config/Claude/claude-desktop.json`):
   ```json
   {
     "mcpServers": {
       "open-meteo": {
         "command": "open-meteo-mcp",
         "args": ["--transport", "stdio"]
       }
     }
   }
   ```

3. Restart Claude Desktop

#### HTTP (Streamable HTTP Transport)

```bash
# Terminal 1: Start server
cargo run -- --transport sse --port 8888

# Terminal 2: Test
curl http://localhost:8888/health

# MCP protocol endpoint (POST, JSON-RPC over rmcp's streamable-HTTP transport)
curl -X POST http://localhost:8888/sse \
  -H "Content-Type: application/json" \
  -H "Accept: application/json, text/event-stream" \
  -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"test","version":"0.0.1"}}}'
```

## Features

### MCP Tools (11 total) ✅

- `get_weather` — Weather forecast with temperature, precipitation, wind
- `get_snow_conditions` — Snow depth, snowfall, mountain weather
- `get_air_quality` — AQI, pollutants, UV index, pollen
- `search_location` — Geocoding - search locations by name
- `search_location_swiss` — Swiss-specific location search
- `get_weather_alerts` — Weather alerts based on thresholds
- `get_comfort_index` — Outdoor activity comfort score (0-100)
- `get_astronomy` — Sunrise, sunset, golden hour, moon phase
- `compare_locations` — Multi-location weather comparison
- `get_historical_weather` — Historical weather data (1940-present)
- `get_marine_conditions` — Wave/swell data for lakes and coasts

### MCP Resources (4 total) ✅

- `weather://codes` — WMO weather code reference
- `weather://parameters` — Available API parameters
- `weather://aqi-reference` — AQI scales and health recommendations
- `weather://swiss-locations` — Swiss cities, mountains, passes

### MCP Prompts (3 total) ✅

- `ski-trip-weather` — Ski trip planning with snow conditions
- `plan-outdoor-activity` — Weather-aware activity planning
- `weather-aware-travel` — Travel planning with weather integration

## Configuration

Environment variables (see `.env.example`):

```bash
HOST=0.0.0.0           # HTTP host (default: 0.0.0.0)
PORT=8888              # HTTP port (default: 8888)
API_BASE_URL=...       # Open-Meteo API base (default: https://api.open-meteo.com)
TIMEOUT_SECS=30        # Request timeout (default: 30)
LOG_LEVEL=info         # Logging level (default: info)
TRANSPORT=stdio        # "stdio" or "sse" (default: stdio)
```

## Development

### Build & Test

```bash
# Build
cargo build --release

# Test (280 tests)
cargo test --all-features

# Format check
cargo fmt --check

# Lint
cargo clippy --all-targets

# Coverage report
cargo llvm-cov --html  # Report in target/llvm-cov/html/index.html
```

### Test Suite (280 tests)

- **Library Tests** (81): Configuration, error handling, types, `rmcp` tool router wiring
- **Integration Tests** (199): Tool handler validation/boundary testing, service layer, error conversion, configuration management

### Project Structure

```
src/
├── main.rs          # CLI entry point
├── lib.rs           # Library root
├── config.rs        # Configuration
├── error.rs         # Error types
├── service.rs       # Core service
├── mcp.rs           # rmcp SDK wiring (ServerHandler, tool router, resources, prompts)
├── tools/           # Tool implementations
├── client/          # HTTP clients (Phase 1)
├── resources/       # MCP resources (Phase 3)
├── prompts/         # MCP prompts (Phase 3)
└── transport/       # Transport implementations (STDIO + streamable HTTP)
```

## Architecture

See [spec/ADR_COMPENDIUM.md](spec/ADR_COMPENDIUM.md) for architecture decisions.

**Key Decisions:**
- **Async**: Tokio 1.x runtime
- **HTTP**: reqwest with connection pooling
- **MCP SDK**: Official `rmcp` 3.1
- **Transport**: STDIO, streamable HTTP (`/sse`) — both served via `rmcp`'s `ServiceExt`/`StreamableHttpService`
- **Testing**: Unit + Integration, 72% coverage target

## Development Roadmap

| Phase | Deliverable | Status |
| --- | --- | --- |
| 0 | Scaffolding, ping tool, CI | ✅ Complete |
| 1 | HTTP clients for 5 APIs | ✅ Complete |
| 2 | 11 tool implementations | ✅ Complete |
| 3 | Resources & prompts | ✅ Complete |
| 4 | STDIO + SSE transports | ✅ Complete |
| 5 | Testing & coverage (72%) | ✅ Complete (280 tests) |
| 6 | Docker & CD pipeline | ✅ Complete |

**Status**: All phases complete | Feature-complete parity with Java v2.0.2

## Comparison with Java Version

| Aspect | Java v2.0.2 | Rust v1.2.0 |
| --- | --- | --- |
| Binary Size | 50 MB + JVM | 26.4 MB (Docker) |
| Cold Start | 2-5s | <100ms |
| Memory (idle) | ~150 MB | 50-100 MB |
| Dependencies | 100+ (Maven) | ~16 direct (Cargo) |
| Transport | REST + MCP + Chat | STDIO + Streamable HTTP |
| Chat Endpoint | ✅ Included | ❌ Client-side only |
| Test Coverage | 72% (426 tests) | ✅ 72% (280 tests) |
| Architecture Docs | Basic | ✅ Enhanced (mermaid diagrams) |

## Documentation

- [CLAUDE.md](CLAUDE.md) — AI development guide
- [ARCHITECTURE.md](ARCHITECTURE.md) — System design with mermaid diagrams
- [spec/ADR_COMPENDIUM.md](spec/ADR_COMPENDIUM.md) — Architecture decision records (12 ADRs)
- [Open-Meteo API](https://open-meteo.com/en/docs) — Weather data source
- [MCP Protocol](https://modelcontextprotocol.io/) — Model Context Protocol specification

## License

Apache License 2.0 — See [LICENSE](LICENSE) file

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Support

- **Issues**: [GitHub Issues](https://github.com/schlpbch/open-meteo-mcp-rust/issues)
- **Discussions**: [GitHub Discussions](https://github.com/schlpbch/open-meteo-mcp-rust/discussions)

---

**Maintained by**: [@schlpbch](https://github.com/schlpbch)
**Status**: ✅ v1.2.0 - Production Ready
**Last Updated**: August 2026
