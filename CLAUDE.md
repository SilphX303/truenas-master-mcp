# CLAUDE.md

This file provides guidance to Claude Code when working with this repository.

## Project Overview

TrueNAS Master MCP is a Model Context Protocol (MCP) server written in Rust that enables AI assistants to interact with TrueNAS systems via their REST API. It provides tools for managing users, pools, datasets, shares, snapshots, applications, and more.

## Tech Stack

- **Language**: Rust 2024 Edition
- **MCP SDK**: rmcp 0.2
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

# Build with all features
cargo build --all-features

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

# Run cargo-deny (security audits)
cargo deny check
```

## Code Style

- Use `cargo fmt` for formatting (Rust standard)
- Use `cargo clippy` for linting - address all warnings
- Use `anyhow::Result<T>` for fallible operations in main
- Use `thiserror` for defining error types in `src/error.rs`
- Use `crate::error::Result<T>` for internal operations
- Use `async/await` with tokio for async operations
- Follow the existing patterns in `src/server.rs` for tool implementation

## Architecture

### Main Components

1. **`src/main.rs`** - Entry point, handles CLI args and transport selection
2. **`src/server.rs`** - MCP server implementation, tool definitions, and request handling
3. **`src/tools.rs`** - TrueNAS API operations (user, pool, dataset, share, etc.)
4. **`src/client.rs`** - HTTP client for TrueNAS API with auth support
5. **`src/config.rs`** - Configuration loading from environment variables
6. **`src/error.rs`** - Error type definitions with helper methods

### Transport Options

The server supports three transport modes:
- `stdio` - Standard input/output (default, for MCP clients)
- `http` - HTTP endpoint at `/mcp`
- `sse` - Server-Sent Events at `/sse` with POST at `/messages`

### Feature Flags

- `scale` - Enable TrueNAS SCALE-specific features (default)
- `core` - Enable TrueNAS CORE-specific features (default)
- `full` - Enable all features for release builds

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `TRUENAS_SERVER_URL` | Yes | TrueNAS server URL |
| `TRUENAS_API_KEY` | No* | API key for authentication |
| `TRUENAS_USERNAME` | No* | Username for basic auth |
| `TRUENAS_PASSWORD` | No* | Password for basic auth |
| `TRUENAS_VERIFY_SSL` | No | Verify SSL (default: true) |
| `TRUENAS_TIMEOUT` | No | Timeout in seconds (default: 30) |
| `TRUENAS_VERSION` | No | Version: scale/core (default: scale) |

*Either API key OR username/password required

## Tool Implementation Pattern

When adding a new tool:

1. Add the tool definition in `TrueNasServer` using the `#[tool]` macro in `src/server.rs`
2. Add a handler case in the tool router implementation
3. Add a method in `TrueNasTools` in `src/tools.rs` for the actual API call
4. Update `src/client.rs` if new HTTP methods are needed
5. Add test cases in `tests/` for the new functionality
6. Update the JSON schema for input validation if needed

## Error Handling

Use `TrueNasError` enum with specific error variants:
- `RequestError` - HTTP/network failures
- `ApiError` - API response errors with status code
- `AuthError` - Authentication failures
- `ConfigError` - Configuration issues
- `NotFound` - Resource not found
- `ValidationError` - Input validation failures
- `PermissionDenied` - Authorization failures
- `AlreadyExists` - Duplicate resource errors
- `TimeoutError` - Operation timeouts
- `PoolError` - Storage pool issues
- `DatasetError` - Dataset-related errors
- `VmError` - Virtual machine errors
- `ServiceError` - Service management errors
- `SystemError` - System-level errors
- `InternalError` - Unexpected internal errors

Helper methods available:
- `TrueNasError::not_found(resource, identifier)` - Create not found errors
- `TrueNasError::validation(message)` - Create validation errors
- `TrueNasError::permission_denied(operation)` - Create permission errors
- `TrueNasError::already_exists(resource, identifier)` - Create duplicate errors
- `TrueNasError::from_api_response(status, message)` - Create from API response

## Git Hooks

This project uses a **pre-commit hook** (`.git/hooks/pre-commit`) that runs:

- `cargo fmt --check` - Verify code formatting
- `cargo clippy` - Lint code (fails on warnings)
- `cargo check` - Verify code compiles

The hook is automatically active once the file exists in `.git/hooks/`. If it's not executable, run:
```bash
chmod +x .git/hooks/pre-commit
```

## GitHub Actions CI/CD

The project includes a comprehensive CI/CD workflow (`.github/workflows/ci.yml`):

### Jobs:
1. **Lint** - Code formatting and clippy checks
2. **Test** - Runs all tests with coverage reporting
3. **Build Multiplatform** - Builds for:
   - aarch64-unknown-linux-gnu
   - x86_64-unknown-linux-gnu
   - x86_64-apple-darwin
   - aarch64-apple-darwin
   - x86_64-pc-windows-msvc
4. **Security** - Runs cargo-audit and cargo-deny
5. **Publish** - Publishes to crates.io on version tags
6. **Release** - Creates GitHub releases with binaries

### Running Locally:
```bash
# Trigger workflow on main branch push
git push origin main

# Create a release
git tag v0.1.0
git push origin v0.1.0
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run doc tests
cargo test --doc

# Run tests with coverage
cargo tarpaulin --out Html
```

## Development Notes

- The rmcp 0.2 SDK uses `ServerHandler` trait with async methods
- Tools are defined using procedural macros
- Use `Arc<TrueNasTools>` for shared state across tool calls
- Configuration is loaded from environment at startup
- Sensitive data (API keys, passwords) is masked in debug output
