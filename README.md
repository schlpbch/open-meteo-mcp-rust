# Open-Meteo MCP (Rust)

A lightweight, high-performance Model Context Protocol (MCP) server providing weather, snow conditions, air quality, and location data via the [Open-Meteo API](https://open-meteo.com/).

**Rust port of [open-meteo-mcp-java](https://github.com/schlpbch/open-meteo-mcp-java)** — featuring a 26.4 MB Docker image with <100ms cold start and 258 comprehensive tests.

**Status**: ✅ v2.0.0 - Production Ready | 72% test coverage | Feature-complete parity with Java v2.0.2

## Quick Start

### Prerequisites

- Rust 1.75+ ([install](https://rustup.rs/))
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

#### HTTP (SSE Transport)

```bash
# Terminal 1: Start server
cargo run -- --transport sse --port 8888

# Terminal 2: Test
curl http://localhost:8888/health
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

# Test (258 tests)
cargo test --all-features

# Format check
cargo fmt --check

# Lint
cargo clippy --all-targets

# Coverage report
cargo llvm-cov --html  # Report in target/llvm-cov/html/index.html
```

### Test Suite (258 tests)

- **Library Tests** (78): Configuration, error handling, types
- **Phase 4 Tool Handler Tests** (91): Parameter validation, boundary testing
- **Phase 5 Service Layer Tests** (89): Error conversion, configuration management, service orchestration

### Project Structure

```
src/
├── main.rs          # CLI entry point
├── lib.rs           # Library root
├── config.rs        # Configuration
├── error.rs         # Error types
├── service.rs       # Core service
├── tools/           # Tool implementations
├── client/          # HTTP clients (Phase 1)
├── resources/       # MCP resources (Phase 3)
├── prompts/         # MCP prompts (Phase 3)
└── transport/       # Transport implementations (Phase 4)
```

## Architecture

See [spec/ADR_COMPENDIUM.md](spec/ADR_COMPENDIUM.md) for architecture decisions.

**Key Decisions:**
- **Async**: Tokio 1.x runtime
- **HTTP**: reqwest with connection pooling
- **MCP SDK**: Official `rmcp` 0.3+
- **Transport**: STDIO (Phase 0), SSE (Phase 4)
- **Testing**: Unit + Integration, 72% coverage target

## Development Roadmap

| Phase | Deliverable | Status |
| --- | --- | --- |
| 0 | Scaffolding, ping tool, CI | ✅ Complete |
| 1 | HTTP clients for 5 APIs | ✅ Complete |
| 2 | 11 tool implementations | ✅ Complete |
| 3 | Resources & prompts | ✅ Complete |
| 4 | STDIO + SSE transports | ✅ Complete |
| 5 | Testing & coverage (72%) | ✅ Complete (258 tests) |
| 6 | Docker & CD pipeline | ✅ Complete |

**Status**: All phases complete | Feature-complete parity with Java v2.0.2

## Comparison with Java Version

| Aspect | Java v2.0.2 | Rust v2.0.0 |
| --- | --- | --- |
| Binary Size | 50 MB + JVM | 26.4 MB (Docker) |
| Cold Start | 2-5s | <100ms |
| Memory (idle) | ~150 MB | 50-100 MB |
| Dependencies | 100+ (Maven) | ~16 direct (Cargo) |
| Transport | REST + MCP + Chat | STDIO + SSE |
| Chat Endpoint | ✅ Included | ❌ Client-side only |
| Test Coverage | 72% (426 tests) | ✅ 72% (258 tests) |
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
**Status**: ✅ v2.0.0 - Production Ready
**Last Updated**: February 2026
