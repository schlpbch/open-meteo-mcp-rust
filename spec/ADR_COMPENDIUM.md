# Architecture Decision Records (ADRs) — Open-Meteo MCP Rust

**Project:** `open-meteo-mcp-rust` **Status:** Initial ADRs for v0.1.0, with
in-place updates as decisions evolved (see ADR-003, ADR-004, ADR-008, ADR-011)
**Last Updated:** 2026-08-20

---

## ADR-001: Use Tokio as Async Runtime

**Status:** ACCEPTED

**Context:** Rust has multiple async runtimes: tokio, async-std, embassy,
glommio. The Java project uses Spring's `VirtualThreads` for lightweight
concurrency. We need a runtime that:

- Handles 100+ concurrent MCP clients
- Works with most async libraries (serde, reqwest, axum, rmcp)
- Has excellent performance and production maturity

**Decision:** Use **Tokio 1.x** as the primary async runtime.

**Rationale:**

- De facto standard in Rust ecosystem (98% of async crates)
- Excellent performance: ~100ns per task spawn
- Production-proven: Discord, Cloudflare, AWS use Tokio
- Works seamlessly with `reqwest`, `axum`, `rmcp`
- Strong community and regular updates

**Consequences:**

- ✅ Can use Tokio features: `spawn`, `select!`, `join!`
- ✅ Simplified dependency management
- ❌ Not suitable for real-time/hard-latency systems (use embassy for that)
- ❌ Single-threaded event loop (but multi-threaded mode available)

**Alternatives Considered:**

- **async-std**: Smaller, but less mature; fewer ecosystem integrations
- **embassy**: Designed for embedded systems, overkill for server
- **glommio**: Thread-per-core model, but requires significant refactoring

**Related ADRs:** ADR-002 (Concurrency Model)

---

## ADR-002: Concurrency Model — Per-Request Tasks, Shared Client Pool

**Status:** ACCEPTED

**Context:** The Java version uses Spring's `WebClient` (single pool) and
virtual threads for request handling. In Rust, we need a clear model for:

- How many concurrent requests to allow
- Connection pooling strategy
- Per-request overhead

**Decision:**

- Single shared `reqwest::Client` (Arc<Client>) with built-in connection pooling
- Spawn a Tokio task per MCP request (lightweight, <1KB each)
- Default to 64 concurrent requests, configurable via env var

**Rationale:**

- `reqwest::Client` is thread-safe and reuses HTTP connections
- Tokio tasks are ~100x lighter than OS threads
- Backpressure: if too many requests, queue them gracefully
- Connection pooling reduces DNS lookups and TLS handshakes

**Code Pattern:**

```rust
// src/service.rs
pub struct OpenMeteoService {
    client: Arc<reqwest::Client>,  // Shared pool
    config: Config,
}

// Per-request handling
impl OpenMeteoService {
    pub async fn get_weather(&self, req: WeatherRequest) -> Result<WeatherResponse> {
        // Spawned as: tokio::spawn(async move { ... })
        // Runs on worker thread from pool
        self.client.get(...).send().await
    }
}
```

**Consequences:**

- ✅ Automatic backpressure via Tokio queue
- ✅ Minimal memory overhead per request
- ❌ If one request blocks, others can progress (but kernel scheduler will
  rebalance)
- ❌ Not ideal for CPU-bound work (but this is I/O-bound)

**Alternatives Considered:**

- Per-request OS thread: 2-8 MB overhead per thread, max ~1000 threads
- Async channels with bounded queue: More explicit backpressure, more code

**Related ADRs:** ADR-001 (Tokio Runtime)

---

## ADR-003: Transport Layer — STDIO for Phase 0-1, SSE in Phase 4, Defer Streamable HTTP

**Status:** ACCEPTED — superseded in practice, August 2026

**Update (August 2026):** `rmcp`'s streamable HTTP matured well before v1.0 and is no
longer experimental (see ADR-004 update). The `/sse` HTTP endpoint (`src/transport/sse.rs`)
now serves `rmcp::transport::streamable_http_server::StreamableHttpService` directly
(POST, JSON-RPC) rather than the plain GET SSE stream this ADR anticipated — the
deferred "Streamable HTTP" row below effectively arrived under the existing `/sse`
route name instead of a new one. The rationale and STDIO-first sequencing below stand;
only the "defer streamable HTTP" consequence is no longer current.

**Context:** MCP supports multiple transports for client-server communication:

1. **STDIO** — single bidirectional pipe (Claude Desktop, CLI clients)
2. **SSE** — Server-Sent Events (web clients, HTTP)
3. **Streamable HTTP** (new) — chunked bidirectional HTTP

Each has tradeoffs:

| Transport   | Latency  | Complexity | Claude Desktop | Web Support | Standardization  |
| ----------- | -------- | ---------- | -------------- | ----------- | ---------------- |
| STDIO       | <10ms    | Low        | ✅ Native      | ❌ No       | ✅ MCP Spec      |
| SSE         | 50-200ms | Medium     | ✅ Via bridge  | ✅ Yes      | ✅ HTTP Standard |
| Stream HTTP | <50ms    | High       | ✅ Maybe       | ✅ Yes      | ⚠️ Experimental  |

**Decision:**

- **Phase 0-1:** STDIO only (validates end-to-end tool invocation)
- **Phase 4:** Add SSE transport for web clients
- **v1.0+:** Defer streamable HTTP; add if user demand exists

**Rationale:**

- STDIO is simplest for initial bringup
- SSE is proven, widely used, HTTP-standard
- Streamable HTTP in `rmcp` is still experimental; not recommended for v0.1
- Can switch to streamable HTTP later without breaking clients

**Consequences:**

- ✅ STDIO gets product to Claude Desktop quickly
- ✅ SSE enables web/remote clients
- ❌ Streamable HTTP deferred (can add v0.2)
- ❌ Clients must choose STDIO or SSE (no auto-negotiation in v0.1)

**Code Pattern — Transport Abstraction:**

```rust
// src/transport/mod.rs
pub trait Transport {
    async fn run(&self, server: ServerImpl) -> Result<()>;
}

pub struct StdioTransport;
pub struct SseTransport { port: u16 }

impl Transport for StdioTransport {
    async fn run(&self, server: ServerImpl) -> Result<()> {
        rmcp::transport::stdio(server).await
    }
}

impl Transport for SseTransport {
    async fn run(&self, server: ServerImpl) -> Result<()> {
        axum_sse_server(server, self.port).await
    }
}

// main.rs
let transport: Box<dyn Transport> = match args.transport.as_str() {
    "stdio" => Box::new(StdioTransport),
    "sse" => Box::new(SseTransport { port: args.port }),
    _ => panic!("Unknown transport"),
};

transport.run(server).await?;
```

**Alternatives Considered:**

- Streamable HTTP from day one: Adds complexity, `rmcp` support is experimental
- Custom HTTP implementation: Reinvents the wheel, security risk

**Related ADRs:** ADR-004 (MCP SDK Choice)

---

## ADR-004: MCP SDK — Use Official `rmcp` 0.3+

**Status:** ACCEPTED — updated August 2026

**Update (August 2026):** The dependency stayed pinned at `rmcp` 1.3 without ever
being wired into the code for several releases — `ServerHandler` was never
implemented, so STDIO/SSE just blocked on a shutdown signal. As of this update the
crate is on `rmcp` 3.1 and is actually wired up in `src/mcp.rs`: a `#[tool_router]`
impl provides the 11 domain tools plus `ping`, and a manual `ServerHandler` impl
handles resources and prompts (rmcp has no macro for those). See `src/mcp.rs` for
the current pattern before following the example below, which reflects the
original 0.3-era plan.

**Context:** The `rmcp` crate is the official Rust SDK for MCP (merged from
4t145/rmcp). Alternatives:

- Official `rmcp`: Maintained, `#[tool]` macro, tokio-based
- Custom MCP implementation: Full control, but 2-3 weeks dev time
- Other community forks: Unclear maintenance status

**Decision:** Use **official `rmcp` 0.3+** from `modelcontextprotocol/rust-sdk`.

**Rationale:**

- Official endorsement from Anthropic
- `#[tool]` macro maps cleanly to `@Tool` in Spring AI
- Built on Tokio (aligns with ADR-001)
- SSE and STDIO transports built-in
- Active maintenance and bug fixes

**Cargo Dependency:**

```toml
[dependencies]
rmcp = { version = "0.3", features = ["server", "transport-io", "transport-sse-server"] }
```

**Consequences:**

- ✅ Official protocol compliance guaranteed
- ✅ Macro-based tool definition is elegant
- ✅ Automatic protocol version negotiation
- ❌ Tied to upstream release schedule
- ❌ If rmcp has a bug, we wait for fix or fork

**Version Pinning Strategy:**

- Pin to `0.3` (semver: allows 0.3.x patches)
- Review breaking changes in 0.4+ before upgrade
- Maintain compatibility shim in `src/rmcp_compat.rs` if needed

**Alternatives Considered:**

- Custom implementation: ~1000 LOC, but reinvents JSON-RPC, protocol handling
- Forking `rmcp`: Support burden, duplicated effort

**Related ADRs:** ADR-003 (Transport), ADR-008 (Tool Definition)

---

## ADR-005: Code Organization — Monolithic Crate v0.1, Workspace at v1.0+

**Status:** ACCEPTED

**Context:** Should we use a Cargo workspace from day one, or start monolithic?

| Approach   | Pros                                    | Cons                                |
| ---------- | --------------------------------------- | ----------------------------------- |
| Monolithic | Simple, fast builds, fewer interdeps    | Can get unwieldy at 5K+ LOC         |
| Workspace  | Modular, parallel builds, reusable libs | Overhead, interdep complexity early |

**Decision:** Start with **monolithic crate** (Phase 0-5), reevaluate at v1.0.

**Rationale:**

- Single `Cargo.toml` is simpler for initial development
- ~2000 LOC fits comfortably in one crate
- No premature modularization
- Easier to refactor later if needed

**When to split into workspace:**

- Single crate exceeds 5000 LOC
- Separate client library requested by users
- Multiple backends needed (e.g., `open-meteo-mcp-core`,
  `open-meteo-mcp-server`)

**Expected Structure (v0.1):**

```
open-meteo-mcp/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── client/
│   ├── tools/
│   ├── resources/
│   ├── transport/
│   └── types/
```

**Consequences:**

- ✅ Simple dependency resolution
- ✅ Fast to build and iterate
- ❌ All features in one binary (can't do lightweight client-only install)
- ❌ Testing tools separately harder

**Alternatives Considered:**

- Workspace from v0.1: Extra management overhead for ~500 LOC codebase
- Submodules: Git complexity, not Rust idiomatic

**Related ADRs:** ADR-006 (Release Strategy)

---

## ADR-006: Release Strategy — GitHub Releases (v0.1), crates.io (v0.2+), cargo-binstall (v1.0+)

**Status:** ACCEPTED

**Context:** How to distribute the binary? Options:

1. **GitHub Releases only** — simple, works for early versions
2. **crates.io** — discoverable, `cargo install`, but source build
3. **cargo-binstall** — pre-built binaries, one-command install
4. **Homebrew/package managers** — distribution but maintenance burden

**Decision:** Phased approach:

- **v0.1** (Beta): GitHub Releases only + Docker
- **v0.2** (RC): Add crates.io publishing
- **v1.0** (Stable): Add cargo-binstall

**Rationale:**

- v0.1 is experimental; GitHub Releases sufficient
- v0.2 adds stability; publish to crates.io
- v1.0 is production; cargo-binstall gives users instant install

**GitHub Releases (All Versions):**

```bash
# Automated via GitHub Actions (on git tag)
# Builds:
- open-meteo-mcp-0.1.0-linux-x86_64 (~8 MB)
- open-meteo-mcp-0.1.0-darwin-aarch64 (~8 MB)
- open-meteo-mcp-0.1.0-windows-x86_64.exe (~8 MB)
- open-meteo-mcp-0.1.0.tar.gz (source)
```

**crates.io (v0.2+):**

```bash
cargo publish  # Automatic in CD
cargo install open-meteo-mcp  # 30-second compile
```

**cargo-binstall (v1.0+):**

```bash
cargo binstall open-meteo-mcp  # Download pre-built, 1 second install
# Requires crate-v1.toml config
```

**Consequences:**

- ✅ Low overhead for v0.1 (GitHub only)
- ✅ Discoverability at v0.2 (crates.io)
- ✅ Frictionless install at v1.0 (cargo-binstall)
- ❌ Maintenance: Keep release process automated
- ❌ May need to maintain binaries on multiple platforms

**Alternatives Considered:**

- crates.io from v0.1: Adds complexity, users expect stability
- Homebrew from v0.1: Maintenance burden, macOS-only initially

**Related ADRs:** ADR-007 (CI/CD Pipeline)

---

## ADR-007: CI/CD Pipeline — GitHub Actions with Cross-Compilation

**Status:** ACCEPTED

**Context:** We need automated builds for multiple targets (Linux x86_64, macOS
ARM64, Windows x86_64). Options:

- GitHub Actions (free, cross-compile via `cross`)
- Manual release builds (error-prone)
- Other CI (GitLab, CircleCI, cost)

**Decision:** Use **GitHub Actions** with `cross` crate for cross-compilation.

**Build Matrix:**

```yaml
# .github/workflows/release.yml
strategy:
  matrix:
    include:
      - target: x86_64-unknown-linux-gnu
        os: ubuntu-latest
      - target: aarch64-apple-darwin
        os: macos-latest-large
      - target: x86_64-pc-windows-msvc
        os: windows-latest
```

**Rationale:**

- GitHub Actions free tier covers our needs
- `cross` crate simplifies cross-compilation (no manual toolchain setup)
- Automated on every tag (v0.1.0, v0.2.0, etc.)
- Consistent builds (same environment every time)

**Release Workflow:**

1. Merge PR to `main`
2. Create git tag: `git tag v0.1.0`
3. Push tag: `git push origin v0.1.0`
4. GitHub Actions:
   - Runs tests (all targets)
   - Builds release binaries
   - Creates GitHub Release with artifacts
   - (v0.2+) Publishes to crates.io

**Consequences:**

- ✅ Automated, reliable releases
- ✅ Consistent builds across platforms
- ❌ 10-15 min per release (time to compile all targets)
- ❌ Requires GitHub Actions credit (free tier sufficient)

**Alternatives Considered:**

- Manual release builds: Error-prone, hard to reproduce
- CI/CD service (CircleCI, etc.): Cost, complexity

**Related ADRs:** ADR-006 (Release Strategy)

---

## ADR-008: Tool Definition — `#[tool]` Macro with serde/schemars for Parameter Validation

**Status:** ACCEPTED — pattern updated August 2026

**Update (August 2026):** The `#[tool]` macro in `rmcp` 3.x takes a single
`Parameters<T>`-wrapped struct argument, not `#[tool_param]`-annotated individual
scalar arguments as sketched below. The actual implementation in `src/mcp.rs`
splits each tool into two layers: the business-logic method (in `src/tools/*.rs`,
unchanged scalar-argument signature, unaware of `rmcp`) plus a thin `#[tool]`
wrapper in `src/mcp.rs` that destructures `Parameters<XParams>` and calls it. This
keeps `rmcp`'s wire types out of the business-logic layer and its 280 tests. See
`src/mcp.rs` for the current pattern; the code block below is the original 0.3-era
sketch.

**Context:** How to define MCP tools? The `rmcp` crate provides `#[tool]` macro
which:

- Generates tool metadata (name, description, parameters)
- Validates input against JSON schema
- Returns `CallToolResult`

**Decision:** Use **`#[tool]` macro** for all tool definitions, with `serde` for
deserialization and `schemars` for schema generation.

**Tool Definition Pattern:**

```rust
#[tool(description = "Get weather forecast with temperature, precipitation, wind")]
pub async fn get_weather(
    &self,
    #[tool_param(description = "Latitude (-90 to 90)")]
    latitude: f64,

    #[tool_param(description = "Longitude (-180 to 180)")]
    longitude: f64,

    #[tool_param(description = "Number of forecast days")]
    #[serde(default = "default_forecast_days")]
    forecast_days: Option<u8>,
) -> Result<CallToolResult, McpError> {
    // Validate coordinates
    if !is_valid_coords(latitude, longitude) {
        return Err(McpError::InvalidParameter(
            format!("Invalid coordinates: {}, {}", latitude, longitude)
        ));
    }

    let req = WeatherRequest {
        latitude,
        longitude,
        forecast_days: forecast_days.unwrap_or(7),
        ..Default::default()
    };

    let resp = self.client.get_weather(&req).await?;
    let json = serde_json::to_string_pretty(&resp)?;

    Ok(CallToolResult::success(vec![
        Content::text(json)
    ]))
}
```

**Parameter Validation Strategy:**

- **Schema validation**: `schemars` auto-generates JSON schema from Rust types
- **Runtime validation**: Custom validators for domain logic (e.g., coordinate
  ranges)
- **Deserialization**: `serde` with `#[serde(default)]` for optional fields

**Consequences:**

- ✅ Type-safe, compile-checked tool definitions
- ✅ Automatic JSON schema generation
- ✅ Client knows valid parameter ranges upfront
- ❌ Complex nested types can be verbose
- ❌ Macro complexity if extended later

**Alternatives Considered:**

- Manual JSON schema: Error-prone, duplicated logic
- Reflection at runtime: Loses compile-time safety

**Related ADRs:** ADR-004 (MCP SDK), ADR-009 (Error Handling)

---

## ADR-009: Error Handling — `thiserror` for Domain Errors, `anyhow` for Context

**Status:** ACCEPTED

**Context:** Rust errors need to be:

- User-friendly (tool responses)
- Debuggable (tracing, logs)
- Convertible to MCP protocol errors

**Decision:**

- Use **`thiserror`** for domain errors (API errors, validation)
- Use **`anyhow`** for contextual errors (I/O, config parsing)
- Convert all to `McpError` at tool boundary

**Error Types:**

```rust
// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenMeteoError {
    #[error("Invalid coordinates: latitude {lat}, longitude {lon}")]
    InvalidCoordinates { lat: f64, lon: f64 },

    #[error("API error {code}: {message}")]
    ApiError { code: u16, message: String },

    #[error("Rate limited: retry after {seconds}s")]
    RateLimit { seconds: u64 },

    #[error("HTTP client error")]
    HttpClient(#[from] reqwest::Error),

    #[error("JSON error")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, OpenMeteoError>;

// Convert to MCP error at tool boundary
impl From<OpenMeteoError> for McpError {
    fn from(err: OpenMeteoError) -> Self {
        match err {
            OpenMeteoError::InvalidCoordinates { .. } => {
                McpError::InvalidRequest(err.to_string())
            }
            OpenMeteoError::RateLimit { .. } => {
                McpError::InternalError(err.to_string())
            }
            _ => McpError::InternalError(err.to_string()),
        }
    }
}
```

**Error Handling in Tools:**

```rust
#[tool(description = "Get weather forecast")]
pub async fn get_weather(
    &self,
    latitude: f64,
    longitude: f64,
) -> Result<CallToolResult, McpError> {
    // Domain errors auto-convert to McpError via From impl
    let resp = self.client
        .get_weather(&WeatherRequest { latitude, longitude, ..Default::default() })
        .await?;  // <- OpenMeteoError::InvalidCoordinates auto-converts here

    Ok(CallToolResult::success(vec![
        Content::text(serde_json::to_string_pretty(&resp)?)
    ]))
}
```

**Consequences:**

- ✅ Clear error semantics at each layer
- ✅ Easy to map domain errors to user-facing messages
- ✅ Automatic stack traces with `anyhow`
- ❌ Some boilerplate for From implementations
- ❌ Multiple error types (but manageable with type aliases)

**Alternatives Considered:**

- Single error enum: Loses structure, hard to extend
- `miette` for fancy error reporting: Nice but overkill for MCP

**Related ADRs:** ADR-010 (Logging & Observability)

---

## ADR-010: Logging & Observability — Structured Logging with Tracing

**Status:** ACCEPTED

**Context:** The Java version uses SLF4J with structured JSON logging. In Rust,
we need:

- Structured JSON logs for production
- Log levels configurable via env vars
- Tracing for span-based debugging

**Decision:** Use **`tracing`** + **`tracing-subscriber`** with JSON formatter.

**Configuration:**

```rust
// src/lib.rs or main.rs
use tracing_subscriber::EnvFilter;

fn init_tracing(log_level: Option<String>) {
    let filter = EnvFilter::try_from_env("LOG_LEVEL")
        .unwrap_or_else(|_| {
            EnvFilter::new(log_level.unwrap_or_else(|| "info".to_string()))
        });

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)  // stderr for logs
        .json()  // Structured JSON output
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .init();
}
```

**Usage in Tools:**

```rust
#[tool(description = "Get weather")]
pub async fn get_weather(&self, lat: f64, lon: f64) -> Result<CallToolResult, McpError> {
    tracing::info!(latitude = lat, longitude = lon, "Fetching weather forecast");

    let resp = self.client.get_weather(&req).await
        .inspect_err(|e| {
            tracing::error!(error = ?e, "Failed to fetch weather");
        })?;

    tracing::debug!(response_size = resp.len(), "Weather response received");

    Ok(CallToolResult::success(vec![Content::text(resp)]))
}
```

**JSON Output Example:**

```json
{
  "timestamp": "2026-02-04T12:34:56Z",
  "level": "INFO",
  "target": "open_meteo_mcp::tools::weather",
  "message": "Fetching weather forecast",
  "latitude": 48.1,
  "longitude": 11.6,
  "thread_id": 2
}
```

**Consequences:**

- ✅ Structured logs for log aggregation (ELK, Datadog)
- ✅ Easy filtering by log level, target, fields
- ✅ Span-based tracing for performance debugging
- ❌ JSON output is verbose (not human-readable without jq)
- ❌ Slight performance overhead (but negligible)

**Alternatives Considered:**

- `log` crate: Simpler, but less structured
- `slog`: More heavyweight, over-engineered

**Related ADRs:** ADR-009 (Error Handling)

---

## ADR-011: Testing Strategy — Unit + Integration + Fixtures

**Status:** ACCEPTED — mocking layer not adopted in practice, corrected August 2026

**Update (August 2026):** The `mockall`-based unit-test layer sketched below was
never implemented — `mockall` was a declared dev-dependency with zero references
anywhere in `src/` or `tests/`, and has since been removed from `Cargo.toml`.
Tool-handler tests (`tests/tools_*_handler.rs`, 105 tests) instead call the live
Open-Meteo API directly with no mocking; this makes them flake in
network-restricted environments (see ARCHITECTURE.md's Troubleshooting section)
but keeps them honest against the real API contract. `wiremock` is used, but only
by the client-layer tests. Current total: 280 tests, 72% coverage target unchanged.

**Context:** Target: 72% code coverage (parity with Java version). Need strategy
for:

- Unit testing individual tools
- Integration testing tool → HTTP client → API
- Mocking external APIs

**Decision:** Three-layer testing:

1. **Unit tests** — Tool logic with mocked client
2. **Integration tests** — Tool → HTTP client with `wiremock`
3. **Fixtures** — Recorded API responses for deterministic tests

**Test Structure:**

```
tests/
├── integration/
│   ├── tools_test.rs
│   ├── transport_test.rs
│   └── fixtures/
│       ├── weather_response.json
│       ├── geocode_response.json
│       └── ...
src/
├── tools/
│   └── weather.rs
│       └── #[cfg(test)] mod tests { ... }
```

**Unit Test Pattern:**

```rust
// src/tools/weather.rs
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_get_weather_success() {
        let mut mock_client = MockOpenMeteoClient::new();
        mock_client
            .expect_get_weather()
            .with(eq(WeatherRequest { latitude: 48.1, longitude: 11.6, .. }))
            .times(1)
            .returning(|_| {
                Ok(WeatherResponse {
                    latitude: 48.1,
                    longitude: 11.6,
                    current: Some(CurrentWeather { temperature: 5.2, .. }),
                    ..Default::default()
                })
            });

        let service = OpenMeteoService::with_client(mock_client, Config::default());
        let result = service.get_weather(48.1, 11.6).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_weather_invalid_coordinates() {
        let service = OpenMeteoService::new(Config::default()).await.unwrap();
        let result = service.get_weather(999.0, 999.0).await;

        assert!(matches!(result, Err(OpenMeteoError::InvalidCoordinates { .. })));
    }
}
```

**Integration Test Pattern:**

```rust
// tests/integration/tools_test.rs
#[tokio::test]
async fn test_get_weather_integration() {
    let mock_server = MockServer::start().await;

    let weather_json = std::fs::read_to_string("tests/fixtures/weather_response.json")
        .expect("Fixture not found");

    Mock::given(path("/v1/forecast"))
        .respond_with(ResponseTemplate::new(200).set_body_string(weather_json))
        .mount(&mock_server)
        .await;

    let client = OpenMeteoClient::with_base_url(mock_server.uri());
    let service = OpenMeteoService::with_client(client, Config::default());

    let result = service.get_weather(48.1, 11.6).await;

    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(resp.latitude, 48.1);
}
```

**Coverage Target:**

- Phase 2 (Tools): 75% (exclude edge cases in error handling)
- Phase 5 (Full suite): 72% (match Java version)
- Tool: `cargo llvm-cov` with `--html` report

**Consequences:**

- ✅ High confidence in correctness
- ✅ Fixtures allow offline testing
- ✅ Fast unit tests, deterministic integration tests
- ❌ Maintaining fixtures as APIs evolve
- ❌ Coverage overhead (some tests are redundant)

**Alternatives Considered:**

- Only unit tests: Misses integration bugs
- Only integration tests: Slow, flaky if APIs change

**Related ADRs:** ADR-008 (Tool Definition)

---

## ADR-012: Dependencies — Minimize, Pin Major Versions, Audit Regularly

**Status:** ACCEPTED

**Context:** Rust crate ecosystem is vast. We need a policy for:

- How many direct dependencies?
- Version pinning strategy?
- Security updates?

**Decision:**

- Target **≤30 direct dependencies** (vs. 100+ in Maven)
- Pin **major versions** (allow minor/patch)
- **Audit monthly** via `cargo-audit`, **check security advisories weekly**

**Dependency Philosophy:**

- Use `std` library where possible (don't add crate for 50 LOC)
- Prefer smaller, focused crates (e.g., `thiserror` not `eyre`)
- Avoid beta/pre-release deps

**Current Direct Dependencies (August 2026, updated from the Phase 0 snapshot):**

```toml
[dependencies]
rmcp = "3.1"           # Official MCP SDK — now actually wired into ServerHandler (src/mcp.rs)
tokio = "1"            # Async runtime
reqwest = "0.13"       # HTTP client
serde = "1"            # Serialization
serde_json = "1"       # JSON support
schemars = "1"         # JSON schema generation
axum = "0.8"           # Web framework
tower = "0.5"          # Middleware
tower-http = "0.7"     # HTTP utilities
thiserror = "2"        # Error types
anyhow = "1"           # Error context
dotenvy = "0.15"       # .env loading
envy = "0.4"           # Env var deserialization
tracing = "0.1"        # Structured logging
tracing-subscriber = "0.3"
chrono = "0.4"         # DateTime
clap = "4.5"           # CLI argument parsing
futures = "0.3"        # Async utilities

# Total: 17 direct, ~159 transitive (176 unique packages in the runtime tree)
```

`mockall` was a declared dev-dependency with zero actual usage (see the ADR-011
update above) and has been removed.

**Security Audit Process:**

```bash
# Weekly: Check for advisories
cargo audit --deny warnings

# Monthly: Update dependencies
cargo update

# Before release: Full security review
cargo audit && cargo outdated
```

**Consequences:**

- ✅ Small dependency tree = less surface area
- ✅ Faster builds (fewer crates to compile)
- ✅ Easier to audit and maintain
- ❌ Can't use every trendy crate
- ❌ May need to implement small utilities

**Alternatives Considered:**

- Minimal deps (only std + rmcp): Reinvents too much (HTTP, JSON, etc.)
- Max deps (add every useful crate): Bloated, security risk

**Related ADRs:** ADR-006 (Release Strategy)

---

## Summary Table

| ADR | Title             | Decision                                                | Status   |
| --- | ----------------- | ------------------------------------------------------- | -------- |
| 001 | Async Runtime     | Tokio 1.x                                               | ACCEPTED |
| 002 | Concurrency Model | Per-request tasks, shared client                        | ACCEPTED |
| 003 | Transport Layer   | STDIO + streamable HTTP (`/sse`), both via `rmcp`       | ACCEPTED (updated Aug 2026) |
| 004 | MCP SDK           | Official `rmcp` 3.1, wired into `ServerHandler`         | ACCEPTED (updated Aug 2026) |
| 005 | Code Organization | Monolithic v0.1, workspace at v1.0+                     | ACCEPTED |
| 006 | Release Strategy  | GH Releases v0.1, crates.io v0.2+, cargo-binstall v1.0+ | ACCEPTED |
| 007 | CI/CD             | GitHub Actions + cross-compilation                      | ACCEPTED |
| 008 | Tool Definition   | `#[tool]` macro (`Parameters<T>`) + serde + schemars    | ACCEPTED (updated Aug 2026) |
| 009 | Error Handling    | `thiserror` + `anyhow`, convert to `McpError`           | ACCEPTED |
| 010 | Logging           | Structured JSON with `tracing`                          | ACCEPTED |
| 011 | Testing           | Unit + Integration (no mocking layer), 280 tests, 72% coverage | ACCEPTED (updated Aug 2026) |
| 012 | Dependencies      | ≤30 direct, pin major versions, audit monthly           | ACCEPTED |

---

**Status (August 2026):** All phases (0–6) plus the `rmcp` 3.1 SDK wiring
described in the ADR-003/004/008/011 updates above are complete. See
[CLAUDE.md](../CLAUDE.md) and [ARCHITECTURE.md](../ARCHITECTURE.md) for the
current system state.
