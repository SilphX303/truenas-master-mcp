# CLAUDE.md

This file provides guidance to Claude Code when working with this repository.

## Project Overview

TrueNAS Master MCP is a Model Context Protocol (MCP) server written in Rust that enables AI assistants to interact with TrueNAS systems via their REST API. It provides tools for managing users, pools, datasets, shares, snapshots, and applications.

## Tech Stack

- **Language**: Rust 2024 Edition
- **MCP SDK**: rmcp 0.8
- **HTTP Client**: reqwest 0.12
- **Async Runtime**: tokio 1.42
- **Server**: axum 0.8 (for HTTP/SSE transports)
- **CLI**: clap 4.5
- **Configuration**: config 0.15

## Key Commands

```bash
# Build the project
cargo build

# Build for release
cargo build --release

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run

# Format code
cargo fmt

# Lint code
cargo clippy

# Check code
cargo check
```

## Code Style

- Use `cargo fmt` for formatting (Rust standard)
- Use `cargo clippy` for linting - address all warnings
- Use `anyhow` for error handling (see `src/error.rs`)
- Use `thiserror` for defining error types
- Use `async/await` with tokio for async operations
- Follow the existing patterns in `src/server.rs` for tool implementation

## Architecture

### Main Components

1. **`src/main.rs`** - Entry point, handles CLI args and transport selection
2. **`src/server.rs`** - MCP server implementation, tool definitions, and request handling
3. **`src/tools.rs`** - TrueNAS API operations (user, pool, dataset, share, etc.)
4. **`src/client.rs`** - HTTP client for TrueNAS API with auth support
5. **`src/config.rs`** - Configuration loading from environment variables
6. **`src/error.rs`** - Error type definitions

### Transport Options

The server supports three transport modes:
- `stdio` - Standard input/output (default, for MCP clients)
- `http` - HTTP endpoint at `/mcp`
- `sse` - Server-Sent Events at `/sse` with POST at `/messages`

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `TRUENAS_SERVER_URL` | Yes | TrueNAS server URL |
| `TRUENAS_API_KEY` | No* | API key for authentication |
| `TRUENAS_USERNAME` | No* | Username for basic auth |
| `TRUENAS_PASSWORD` | No* | Password for basic auth |
| `TRUENAS_VERIFY_SSL` | No | Verify SSL (default: true) |
| `TRUENAS_TIMEOUT` | No | Timeout in seconds (default: 30) |

*Either API key OR username/password required

## Tool Implementation Pattern

When adding a new tool:

1. Add the tool definition in `TrueNasServer::list_tools_impl()` in `src/server.rs`
2. Add a handler case in `TrueNasServer::call_tool()`
3. Add a method in `TrueNasTools` in `src/tools.rs` for the actual API call
4. Update `src/client.rs` if new HTTP methods are needed
5. Add test cases in `tests/` if available

## Adding New Tools

Tools are defined as `Tool` structs with:
- `name`: Tool identifier (snake_case)
- `description`: Human-readable description
- `input_schema`: JSON Schema for parameters

See existing tools in `src/server.rs` for examples.

## Error Handling

- Use `anyhow::Result<T>` for fallible operations in main
- Use `crate::error::Result<T>` for internal operations
- Define specific errors in `TrueNasError` enum in `src/error.rs`

## Development Notes

- The rmcp 0.8 SDK uses `ServerHandler` trait with `get_info()`, `list_tools()`, and `call_tool()` methods
- Tools return `Result<serde_json::Value, rmcp::Error>`
- Use `Arc<TrueNasTools>` for shared state across tool calls
- Configuration is loaded from environment at startup

## Git Hooks

This project uses a custom pre-commit hook (`.git/hooks/pre-commit`) that runs:

- `cargo check` - Verify code compiles
- `cargo clippy` - Lint code (fails on warnings)

The hook is automatically active once the file exists in `.git/hooks/`.
