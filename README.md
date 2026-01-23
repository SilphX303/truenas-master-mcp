# TrueNAS Master MCP

A Model Context Protocol (MCP) server for managing TrueNAS systems through AI assistants.

## Features

This MCP server provides access to TrueNAS API functionality including:

- **User Management**: List, get users by ID or username
- **Pool Management**: List pools, get pool status
- **Dataset Management**: List, create, get, delete datasets
- **SMB Shares**: List, create, delete SMB shares
- **NFS Exports**: List, create, delete NFS exports
- **ZFS Snapshots**: List, create, delete snapshots
- **iSCSI Targets**: List, create, delete iSCSI targets
- **System Information**: Get system info including version, hostname, uptime

## Installation

### Prerequisites

- Rust 1.85 or later
- TrueNAS SCALE or TrueNAS CORE with API access
- API key or username/password for authentication

### Build from source

```bash
# Clone the repository
git clone https://github.com/hongkongkiwi/truenas-master-mcp.git
cd truenas-master-mcp

# Build the project
cargo build --release

# The binary will be at target/release/truenas-master-mcp
```

## Configuration

The server is configured via environment variables:

| Variable | Required | Description | Default |
|----------|----------|-------------|---------|
| `TRUENAS_SERVER_URL` | Yes | TrueNAS server URL | `http://localhost` |
| `TRUENAS_API_KEY` | No* | API key for authentication | - |
| `TRUENAS_USERNAME` | No* | Username for basic auth | - |
| `TRUENAS_PASSWORD` | No* | Password for basic auth | - |
| `TRUENAS_VERIFY_SSL` | No | Verify SSL certificates | `true` |
| `TRUENAS_TIMEOUT` | No | Request timeout in seconds | `30` |
| `TRUENAS_VERSION` | No | TrueNAS version (scale/core) | `scale` |

*Either `TRUENAS_API_KEY` OR both `TRUENAS_USERNAME` and `TRUENAS_PASSWORD` must be provided.

### Example .env file

```bash
# Using API key
TRUENAS_SERVER_URL=https://truenas.local
TRUENAS_API_KEY=your-api-key-here

# OR using username/password
TRUENAS_SERVER_URL=https://truenas.local
TRUENAS_USERNAME=admin
TRUENAS_PASSWORD=your-password

# Optional settings
TRUENAS_VERIFY_SSL=true
TRUENAS_TIMEOUT=30
```

## Usage

### Running the server

```bash
# Load environment variables from .env file
export $(cat .env | xargs)

# Run the server
cargo run
```

The server communicates via stdio, which is the standard for MCP servers.

### Using with Claude Desktop

Add to your Claude Desktop config file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "truenas": {
      "command": "/path/to/truenas-master-mcp",
      "env": {
        "TRUENAS_SERVER_URL": "https://truenas.local",
        "TRUENAS_API_KEY": "your-api-key-here"
      }
    }
  }
}
```

### Using with other MCP clients

The server implements the standard Model Context Protocol. Refer to your MCP client's documentation for how to connect servers.

## Available Tools

### User Management
- `list_users` - List all users
- `get_user` - Get user by ID
- `get_user_by_username` - Get user by username

### Pool Management
- `list_pools` - List all storage pools
- `get_pool_status` - Get status of a specific pool

### Dataset Management
- `list_datasets` - List all datasets
- `get_dataset` - Get details of a specific dataset
- `create_dataset` - Create a new dataset
- `delete_dataset` - Delete a dataset

### SMB Shares
- `list_smb_shares` - List all SMB shares
- `create_smb_share` - Create a new SMB share
- `delete_smb_share` - Delete an SMB share

### NFS Exports
- `list_nfs_exports` - List all NFS exports
- `create_nfs_export` - Create a new NFS export
- `delete_nfs_export` - Delete an NFS export

### Snapshots
- `list_snapshots` - List all ZFS snapshots
- `create_snapshot` - Create a new snapshot
- `delete_snapshot` - Delete a snapshot

### iSCSI Targets
- `list_iscsi_targets` - List all iSCSI targets
- `create_iscsi_target` - Create a new iSCSI target
- `delete_iscsi_target` - Delete an iSCSI target

### Application Management (SCALE)
- `list_apps` - List all applications
- `get_app` - Get details of a specific application
- `start_app` - Start an application
- `stop_app` - Stop an application
- `restart_app` - Restart an application
- `create_app` - Create a new application from catalog
- `update_app` - Update an application configuration
- `delete_app` - Delete an application
- `rollback_app` - Rollback an application to previous version
- `get_app_config` - Get application configuration
- `get_app_upgrade_options` - Get available upgrade options
- `upgrade_app` - Upgrade an application
- `scale_app` - Scale application replica count

### Catalogs and Chart Releases
- `list_catalog_items` - List available catalog items
- `get_catalog` - Get catalog details
- `get_catalog_trains` - Get catalog train versions
- `get_catalog_item` - Get specific catalog item details
- `list_chart_releases` - List deployed chart releases
- `get_chart_release` - Get chart release details
- `get_chart_release_resources` - Get chart release resources

### System Information
- `get_system_info` - Get system information

## Development

### Running tests

```bash
cargo test
```

### Checking code

```bash
cargo fmt
cargo clippy
```

### Building for release

```bash
cargo build --release
```

## API Compatibility

This server supports both TrueNAS SCALE and TrueNAS CORE via the REST API v2.0. Set `TRUENAS_VERSION=scale` (default) for TrueNAS SCALE or `TRUENAS_VERSION=core` for TrueNAS CORE.

The server provides 40+ tools covering:
- User, pool, dataset, share, snapshot, and iSCSI management
- Application management (SCALE only)
- Catalog and chart release access

## License

MIT

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
