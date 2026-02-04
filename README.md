# Open-Meteo MCP (Rust)

A lightweight, high-performance Model Context Protocol (MCP) server providing weather, snow conditions, air quality, and location data via the [Open-Meteo API](https://open-meteo.com/).

**Rust port of [open-meteo-mcp-java](https://github.com/schlpbch/open-meteo-mcp-java)** — featuring a single 8-10 MB static binary with <100ms cold start.

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

### MCP Tools (Phase 2)
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

### MCP Resources (Phase 3)
- `weather://codes` — WMO weather code reference
- `weather://parameters` — Available API parameters
- `weather://aqi-reference` — AQI scales and health recommendations
- `weather://swiss-locations` — Swiss cities, mountains, passes

### MCP Prompts (Phase 3)
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
cargo build

# Test
cargo test

# Format check
cargo fmt --check

# Lint
cargo clippy --all-targets

# Coverage
cargo llvm-cov --html  # Report in target/llvm-cov/html/index.html
```

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

| Phase | Duration | Status |
|-------|----------|--------|
| 0 | 1 day | ✅ Scaffolding |
| 1 | 2-3 days | ⏳ HTTP clients |
| 2 | 3-5 days | ⏳ Tools |
| 3 | 1-2 days | ⏳ Resources & prompts |
| 4 | 1-2 days | ⏳ Transport layer |
| 5 | 2-3 days | ⏳ Testing & coverage |
| 6 | 1 day | ⏳ Docker & CI |

**Total**: ~2-3 weeks for feature parity with Java v2.0.2

## Comparison with Java Version

| Aspect | Java v2.0.2 | Rust v0.1.0 |
|--------|-------------|------------|
| Binary Size | 50 MB + JVM | ~8 MB |
| Cold Start | 2-5s | <100ms |
| Memory (idle) | ~150 MB | ~5-10 MB |
| Dependencies | 100+ (Maven) | ~16 direct (Cargo) |
| Transport | REST + MCP + Chat | STDIO + SSE |
| Chat Endpoint | ✅ Included | ❌ Client-side only |
| Test Coverage | 72% (426 tests) | 72% target |

## Documentation

- [CLAUDE.md](CLAUDE.md) — Development guide
- [spec/ADR_COMPENDIUM.md](spec/ADR_COMPENDIUM.md) — Architecture decisions
- [Open-Meteo API](https://open-meteo.com/en/docs) — Weather data source

## License

Apache License 2.0 — See [LICENSE](LICENSE) file

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Support

- **Issues**: [GitHub Issues](https://github.com/schlpbch/open-meteo-mcp-rust/issues)
- **Discussions**: [GitHub Discussions](https://github.com/schlpbch/open-meteo-mcp-rust/discussions)

---

**Maintained by**: [@schlpbch](https://github.com/schlpbch)
**Status**: Phase 0 (Beta) — Feature parity incoming
**Last Updated**: February 2026
