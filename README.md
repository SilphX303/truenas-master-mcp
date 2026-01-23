# TrueNAS Master MCP

A Model Context Protocol (MCP) server for managing TrueNAS systems through AI assistants.

## Features

This MCP server provides access to TrueNAS API functionality including:

- **User Management**: List, get, create, update, delete users and groups
- **Pool Management**: List pools, get pool status
- **Dataset Management**: List, create, get, delete datasets
- **SMB Shares**: List, create, delete SMB shares
- **NFS Exports**: List, create, delete NFS exports
- **ZFS Snapshots**: List, create, delete snapshots
- **iSCSI Targets**: List, create, delete iSCSI targets
- **System Information**: Get system info, alerts, updates
- **VM Management**: List, start, stop, restart virtual machines
- **Network Management**: List interfaces, routes, DNS configuration
- **Service Management**: List, start, stop, restart services
- **Disk Management**: List disks and get disk details
- **Certificate Management**: List certificates
- **Replication**: List and run replication tasks
- **Cloud Sync**: List and run cloud sync tasks

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

### CLI Options

The server also supports command-line options:

| Option | Description | Default |
|--------|-------------|---------|
| `-t, --transport` | Transport type: stdio, http, sse | stdio |
| `-h, --host` | Host to bind to (http/sse only) | 127.0.0.1 |
| `-p, --port` | Port to bind to (http/sse only) | 3000 |
| `--truenas-version` | TrueNAS version: scale or core | scale |
| `--readonly` | Enable readonly mode | false |
| `-c, --config-file` | Path to config file (JSON/YAML) | - |
| `-l, --log-level` | Log level: debug, info, warn, error | info |
| `-v, --verbose` | Enable verbose output | false |
| `--insecure-ssl` | Disable SSL certificate verification | false |
| `--timeout` | API request timeout in seconds | 30 |
| `--pretty` | Enable JSON pretty printing | false |

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
- `get_alerts` - Get system alerts
- `check_for_updates` - Check for system updates
- `reboot_system` - Reboot the system
- `shutdown_system` - Shutdown the system

### Group Management
- `list_groups` - List all groups
- `get_group` - Get group by ID
- `get_group_by_name` - Get group by name
- `create_group` - Create a new group
- `delete_group` - Delete a group

### VM Management
- `list_vms` - List all virtual machines
- `get_vm` - Get VM details by ID
- `start_vm` - Start a virtual machine
- `stop_vm` - Stop a virtual machine
- `restart_vm` - Restart a virtual machine

### Network Management
- `list_interfaces` - List all network interfaces
- `list_routes` - List all network routes
- `get_dns` - Get DNS configuration

### Service Management
- `list_services` - List all services
- `get_service` - Get service details
- `start_service` - Start a service
- `stop_service` - Stop a service
- `restart_service` - Restart a service

### Disk Management
- `list_disks` - List all disks
- `get_disk` - Get disk details by name

### Certificate Management
- `list_certificates` - List all certificates
- `get_certificate` - Get certificate details by ID

### Replication
- `list_replication_tasks` - List all replication tasks
- `run_replication_task` - Run a replication task

### Cloud Sync
- `list_cloudsync_tasks` - List all cloud sync tasks
- `run_cloudsync_task` - Run a cloud sync task

### Other
- `get_enclosure` - Get enclosure information
- `get_support` - Get support information

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

The server provides 80+ tools covering:
- User and group management
- Pool, dataset, share, snapshot, and iSCSI management
- VM, network, and service management
- Application management (SCALE only)
- Catalog and chart release access
- Replication and cloud sync
- Disk and certificate management

## License

MIT

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
