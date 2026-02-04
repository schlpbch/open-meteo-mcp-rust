# Multi-stage Docker build for open-meteo-mcp
# Stage 1: Builder - Compile the Rust application
# Stage 2: Runtime - Minimal image with just the compiled binary

# ===== BUILDER STAGE =====
# Using latest Rust image to support rmcp 0.3 with edition2024 compatibility
FROM rust:latest AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifest files
COPY Cargo.toml ./
COPY Cargo.lock* ./

# Copy source code
COPY src ./src

# Build optimized release binary
RUN cargo build --release

# ===== RUNTIME STAGE =====
# Use distroless image for security and size
FROM gcr.io/distroless/cc-debian12

LABEL maintainer="Andreas Schlapbach <schlpbch@gmail.com>"
LABEL description="Open-Meteo MCP Server - Weather data via Model Context Protocol"

# Copy compiled binary from builder
COPY --from=builder /app/target/release/open-meteo-mcp /app/open-meteo-mcp

# Copy environment file template
COPY .env.example /app/.env.example

# Set working directory
WORKDIR /app

# Expose port (documentation only, actual port configurable via PORT env var)
EXPOSE 8888

# Default environment variables
ENV TRANSPORT=sse \
    PORT=8888 \
    LOG_LEVEL=info \
    HOST=0.0.0.0 \
    TIMEOUT_SECS=30

# Health check (requires /bin/sh in distroless image - alternative is to use health endpoint)
# Note: distroless doesn't have shell, so we can't use simple curl health check
# Instead, orchestration tools should use HTTP GET on /health endpoint

# Run the application
ENTRYPOINT ["/app/open-meteo-mcp"]
CMD ["--transport", "sse"]
