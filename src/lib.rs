//! # TrueNAS Master MCP Server
//!
//! An official Model Context Protocol (MCP) server for TrueNAS API access.
//! This server enables AI assistants to interact with TrueNAS SCALE and CORE systems
//! via their REST API v2.0.
//!
//! ## Features
//!
//! - **User Management**: List, create, update, and delete users and groups
//! - **Pool Management**: View storage pools and their status
//! - **Dataset Management**: Create, list, and delete ZFS datasets
//! - **Share Management**: Manage SMB and NFS shares
//! - **Snapshot Management**: Create and manage ZFS snapshots
//! - **iSCSI Management**: Configure iSCSI targets
//! - **App Management**: Deploy and manage applications (SCALE) or jails (CORE)
//! - **System Monitoring**: View system info, alerts, and updates
//! - **Network Management**: Configure interfaces, routes, and DNS
//! - **Service Management**: Start, stop, and restart services
//!
//! ## Usage
//!
//! ```bash
//! # Set environment variables
//! export TRUENAS_SERVER_URL="https://truenas.local"
//! export TRUENAS_API_KEY="your-api-key"
//!
//! # Run with stdio transport (default)
//! cargo run
//!
//! # Run with HTTP transport
//! cargo run -- --transport http --port 3000
//!
//! # Run in readonly mode
//! cargo run -- --readonly
//! ```
//!
//! ## Environment Variables
//!
//! | Variable | Required | Description |
//! |----------|----------|-------------|
//! | `TRUENAS_SERVER_URL` | Yes | TrueNAS server URL |
//! | `TRUENAS_API_KEY` | No* | API key for authentication |
//! | `TRUENAS_USERNAME` | No* | Username for basic auth |
//! | `TRUENAS_PASSWORD` | No* | Password for basic auth |
//! | `TRUENAS_VERIFY_SSL` | No | Verify SSL (default: true) |
//! | `TRUENAS_TIMEOUT` | No | Timeout in seconds (default: 30) |
//! | `TRUENAS_VERSION` | No | TrueNAS version (scale/core, default: scale) |
//! | `TRUENAS_READONLY` | No | Enable readonly mode |
//!
//! *Either API key OR username/password required

pub mod client;
pub mod config;
pub mod error;
pub mod server;
pub mod tools;
