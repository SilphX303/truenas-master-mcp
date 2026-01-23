mod client;
mod config;
mod error;
mod tools;

use crate::config::TrueNasConfig;
use crate::tools::TrueNasTools;
use anyhow::Context;
use clap::Parser;
use serde_json::{json, Value};
use std::sync::Arc;
use std::str::FromStr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

/// Tool categories for access control
#[derive(Debug, Clone, PartialEq)]
pub enum ToolCategory {
    Users,
    Pools,
    Datasets,
    Shares,
    Snapshots,
    Iscsi,
    Apps,
    System,
    All,
}

impl FromStr for ToolCategory {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "users" => Ok(ToolCategory::Users),
            "pools" => Ok(ToolCategory::Pools),
            "datasets" | "dataset" => Ok(ToolCategory::Datasets),
            "shares" | "share" => Ok(ToolCategory::Shares),
            "snapshots" | "snapshot" => Ok(ToolCategory::Snapshots),
            "iscsi" => Ok(ToolCategory::Iscsi),
            "apps" | "app" => Ok(ToolCategory::Apps),
            "system" => Ok(ToolCategory::System),
            "all" => Ok(ToolCategory::All),
            _ => Err("Unknown category"),
        }
    }
}

/// Tool access control configuration
#[derive(Debug, Clone)]
pub struct ToolConfig {
    pub readonly: bool,
    pub enabled_categories: Vec<ToolCategory>,
    pub disabled_categories: Vec<ToolCategory>,
}

impl Default for ToolConfig {
    fn default() -> Self {
        Self {
            readonly: false,
            enabled_categories: vec![ToolCategory::All],
            disabled_categories: vec![],
        }
    }
}

impl ToolConfig {
    /// Create config from environment variables
    pub fn from_env() -> Self {
        let readonly = std::env::var("TRUENAS_READONLY")
            .map(|v| v.to_lowercase() == "true")
            .unwrap_or(false);

        let enabled_categories: Vec<ToolCategory> = std::env::var("TRUENAS_ENABLED_CATEGORIES")
            .ok()
            .map(|v| v.split(',').filter_map(|c| ToolCategory::from_str(c).ok()).collect())
            .unwrap_or_default();

        let disabled_categories: Vec<ToolCategory> = std::env::var("TRUENAS_DISABLED_CATEGORIES")
            .ok()
            .map(|v| v.split(',').filter_map(|c| ToolCategory::from_str(c).ok()).collect())
            .unwrap_or_default();

        Self {
            readonly,
            enabled_categories: if enabled_categories.is_empty() { vec![ToolCategory::All] } else { enabled_categories },
            disabled_categories,
        }
    }

    /// Check if a category is allowed
    pub fn is_category_allowed(&self, category: &ToolCategory) -> bool {
        // Check if category is disabled
        if self.disabled_categories.contains(category) || self.disabled_categories.contains(&ToolCategory::All) {
            return false;
        }

        // Check if category is enabled (or All is enabled)
        if self.enabled_categories.contains(&ToolCategory::All) {
            return true;
        }

        self.enabled_categories.contains(category)
    }

    /// Check if a tool can be executed (considering readonly mode)
    pub fn can_execute(&self, category: &ToolCategory, is_modification: bool) -> Result<(), String> {
        // Check category access
        if !self.is_category_allowed(category) {
            return Err(format!("Category {:?} is not enabled", category));
        }

        // Check readonly mode for modification tools
        if self.readonly && is_modification {
            return Err("Server is in readonly mode - modification tools are disabled".to_string());
        }

        Ok(())
    }
}

/// TrueNAS MCP Server
#[derive(Parser, Debug)]
#[command(name = "truenas-master-mcp")]
#[command(about = "Official MCP server for TrueNAS API access", long_about = None)]
struct Args {
    /// Transport type to use: stdio, http, or sse
    #[arg(short, long, default_value = "stdio")]
    transport: String,

    /// Host to bind to (for http/sse transports)
    #[arg(short, long, default_value = "127.0.0.1")]
    host: String,

    /// Port to bind to (for http/sse transports)
    #[arg(short, long, default_value = "3000")]
    port: u16,

    /// Enable readonly mode (disables all modification tools)
    #[arg(long)]
    readonly: bool,

    /// Enable specific tool categories (comma-separated: users,pools,datasets,shares,snapshots,iscsi,apps,system)
    #[arg(long)]
    enable_category: Vec<String>,

    /// Disable specific tool categories (comma-separated)
    #[arg(long)]
    disable_category: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build tool config from CLI args and environment
    let env_config = ToolConfig::from_env();
    let cli_enabled: Vec<ToolCategory> = args.enable_category
        .iter()
        .filter_map(|c| ToolCategory::from_str(c).ok())
        .collect();
    let cli_disabled: Vec<ToolCategory> = args.disable_category
        .iter()
        .filter_map(|c| ToolCategory::from_str(c).ok())
        .collect();

    let tool_config = ToolConfig {
        readonly: args.readonly || env_config.readonly,
        enabled_categories: if cli_enabled.is_empty() { env_config.enabled_categories } else { cli_enabled },
        disabled_categories: if cli_disabled.is_empty() { env_config.disabled_categories } else { cli_disabled },
    };

    info!("Starting TrueNAS MCP Server with {} transport", args.transport);
    if tool_config.readonly {
        info!("Readonly mode ENABLED - modification tools are disabled");
    }
    info!("Enabled categories: {:?}", tool_config.enabled_categories);
    if !tool_config.disabled_categories.is_empty() {
        info!("Disabled categories: {:?}", tool_config.disabled_categories);
    }

    // Load configuration
    let config = TrueNasConfig::from_env()
        .context("Failed to load configuration from environment")?;

    info!("Connecting to TrueNAS at: {}", config.server_url);

    // Create the TrueNAS server handler with tool config
    let server = Arc::new(TrueNasServerImpl::new(config, tool_config)?);

    match args.transport.as_str() {
        "stdio" => {
            run_stdio(server).await?;
        }
        "http" => {
            run_http(server, &args.host, args.port).await?;
        }
        "sse" => {
            run_sse(server, &args.host, args.port).await?;
        }
        _ => {
            return Err(anyhow::anyhow!("Unknown transport type: {}. Use: stdio, http, or sse", args.transport));
        }
    }

    Ok(())
}

/// TrueNAS MCP Server implementation
#[derive(Clone)]
struct TrueNasServerImpl {
    tools: Arc<TrueNasTools>,
    tool_config: ToolConfig,
}

impl TrueNasServerImpl {
    fn new(config: TrueNasConfig, tool_config: ToolConfig) -> anyhow::Result<Self> {
        let client = crate::client::TrueNasClient::new(config)?;
        let tools = Arc::new(TrueNasTools::new(client));
        Ok(Self { tools, tool_config })
    }

    fn get_server_info(&self) -> Value {
        json!({
            "name": "truenas-master-mcp",
            "version": env!("CARGO_PKG_VERSION"),
            "description": "Official MCP server for TrueNAS API access",
            "readonly": self.tool_config.readonly,
            "enabled_categories": format!("{:?}", self.tool_config.enabled_categories),
            "instructions": "This server provides access to TrueNAS SCALE/CORE management features including:\n- User management\n- Pool and dataset management\n- SMB and NFS share management\n- Snapshot management\n- iSCSI target management\n- Apps/Jails management\n- System information\n\nUse --readonly flag or TRUENAS_READONLY=true to disable modification tools."
        })
    }

    /// Get tool category and whether it's a modification tool
    fn get_tool_info(name: &str) -> (ToolCategory, bool) {
        match name {
            // Users - read operations
            "list_users" | "get_user" | "get_user_by_username" => (ToolCategory::Users, false),
            // Users - modification operations
            "create_user" | "update_user" | "delete_user" => (ToolCategory::Users, true),
            // Pools
            "list_pools" | "get_pool_status" => (ToolCategory::Pools, false),
            "scrub_pool" => (ToolCategory::Pools, true),
            // Datasets
            "list_datasets" | "get_dataset" => (ToolCategory::Datasets, false),
            "create_dataset" | "delete_dataset" | "update_dataset" => (ToolCategory::Datasets, true),
            // Shares
            "list_smb_shares" | "list_nfs_exports" | "get_smb_share" | "get_nfs_export" => (ToolCategory::Shares, false),
            "create_smb_share" | "delete_smb_share" | "create_nfs_export" | "delete_nfs_export" => (ToolCategory::Shares, true),
            // Snapshots
            "list_snapshots" | "get_snapshot" => (ToolCategory::Snapshots, false),
            "create_snapshot" | "delete_snapshot" | "rollback_snapshot" | "clone_snapshot" => (ToolCategory::Snapshots, true),
            // iSCSI
            "list_iscsi_targets" | "list_iscsi_luns" | "list_iscsi_portals" => (ToolCategory::Iscsi, false),
            "create_iscsi_target" | "delete_iscsi_target" | "create_iscsi_lun" | "delete_iscsi_lun" => (ToolCategory::Iscsi, true),
            // Apps
            "list_apps" | "get_app" | "get_app_config" | "list_app_catalogs" | "list_chart_releases" => (ToolCategory::Apps, false),
            "create_app" | "update_app" | "delete_app" | "start_app" | "stop_app" | "restart_app" | "rollback_app" => (ToolCategory::Apps, true),
            // Jails (CORE)
            "list_jails" | "get_jail" | "list_jail_fstabs" => (ToolCategory::Apps, false),
            "create_jail" | "update_jail" | "delete_jail" | "start_jail" | "stop_jail" | "clone_jail" => (ToolCategory::Apps, true),
            // System
            "get_system_info" | "get_system_version" | "get_alerts" => (ToolCategory::System, false),
            "update_system" | "reboot" | "shutdown" => (ToolCategory::System, true),
            // Default
            _ => (ToolCategory::All, false),
        }
    }

    /// Check if a tool can be executed
    fn check_tool_access(&self, name: &str) -> Result<(), String> {
        let (category, is_modification) = Self::get_tool_info(name);
        self.tool_config.can_execute(&category, is_modification)
    }

    fn list_tools(&self) -> Value {
        json!([
            {"name": "list_users", "description": "List all users on the TrueNAS system", "inputSchema": {"type": "object"}},
            {"name": "get_user", "description": "Get details of a specific user by ID", "inputSchema": {"type": "object", "properties": {"user_id": {"type": "integer"}}, "required": ["user_id"]}},
            {"name": "get_user_by_username", "description": "Get details of a specific user by username", "inputSchema": {"type": "object", "properties": {"username": {"type": "string"}}, "required": ["username"]}},
            {"name": "list_pools", "description": "List all storage pools on the TrueNAS system", "inputSchema": {"type": "object"}},
            {"name": "get_pool_status", "description": "Get the status of a specific storage pool", "inputSchema": {"type": "object", "properties": {"pool_name": {"type": "string"}}, "required": ["pool_name"]}},
            {"name": "list_datasets", "description": "List all datasets on the TrueNAS system", "inputSchema": {"type": "object"}},
            {"name": "get_dataset", "description": "Get details of a specific dataset", "inputSchema": {"type": "object", "properties": {"dataset_path": {"type": "string"}}, "required": ["dataset_path"]}},
            {"name": "create_dataset", "description": "Create a new dataset in a pool", "inputSchema": {"type": "object", "properties": {"pool_name": {"type": "string"}, "dataset_name": {"type": "string"}}, "required": ["pool_name", "dataset_name"]}},
            {"name": "delete_dataset", "description": "Delete a dataset", "inputSchema": {"type": "object", "properties": {"dataset_path": {"type": "string"}}, "required": ["dataset_path"]}},
            {"name": "list_smb_shares", "description": "List all SMB shares on the TrueNAS system", "inputSchema": {"type": "object"}},
            {"name": "create_smb_share", "description": "Create a new SMB share", "inputSchema": {"type": "object", "properties": {"name": {"type": "string"}, "path": {"type": "string"}, "comment": {"type": "string"}}, "required": ["name", "path"]}},
            {"name": "delete_smb_share", "description": "Delete an SMB share", "inputSchema": {"type": "object", "properties": {"share_id": {"type": "integer"}}, "required": ["share_id"]}},
            {"name": "list_nfs_exports", "description": "List all NFS exports on the TrueNAS system", "inputSchema": {"type": "object"}},
            {"name": "create_nfs_export", "description": "Create a new NFS export", "inputSchema": {"type": "object", "properties": {"paths": {"type": "array", "items": {"type": "string"}}, "comment": {"type": "string"}}, "required": ["paths", "comment"]}},
            {"name": "delete_nfs_export", "description": "Delete an NFS export", "inputSchema": {"type": "object", "properties": {"export_id": {"type": "integer"}}, "required": ["export_id"]}},
            {"name": "list_snapshots", "description": "List all ZFS snapshots on the TrueNAS system", "inputSchema": {"type": "object"}},
            {"name": "create_snapshot", "description": "Create a new ZFS snapshot", "inputSchema": {"type": "object", "properties": {"dataset": {"type": "string"}, "snapshot_name": {"type": "string"}}, "required": ["dataset", "snapshot_name"]}},
            {"name": "delete_snapshot", "description": "Delete a ZFS snapshot", "inputSchema": {"type": "object", "properties": {"snapshot_id": {"type": "string"}}, "required": ["snapshot_id"]}},
            {"name": "list_iscsi_targets", "description": "List all iSCSI targets on the TrueNAS system", "inputSchema": {"type": "object"}},
            {"name": "create_iscsi_target", "description": "Create a new iSCSI target", "inputSchema": {"type": "object", "properties": {"name": {"type": "string"}}, "required": ["name"]}},
            {"name": "delete_iscsi_target", "description": "Delete an iSCSI target", "inputSchema": {"type": "object", "properties": {"target_id": {"type": "integer"}}, "required": ["target_id"]}},
            {"name": "get_system_info", "description": "Get system information from TrueNAS", "inputSchema": {"type": "object"}},
            {"name": "list_apps", "description": "List all applications (jails/containers) on TrueNAS", "inputSchema": {"type": "object"}},
            {"name": "get_app", "description": "Get details of a specific application", "inputSchema": {"type": "object", "properties": {"app_name": {"type": "string"}}, "required": ["app_name"]}},
            {"name": "start_app", "description": "Start an application on TrueNAS", "inputSchema": {"type": "object", "properties": {"app_name": {"type": "string"}, "options": {"type": "object"}}, "required": ["app_name"]}},
            {"name": "stop_app", "description": "Stop an application on TrueNAS", "inputSchema": {"type": "object", "properties": {"app_name": {"type": "string"}, "force": {"type": "boolean"}}, "required": ["app_name"]}},
            {"name": "restart_app", "description": "Restart an application on TrueNAS", "inputSchema": {"type": "object", "properties": {"app_name": {"type": "string"}}, "required": ["app_name"]}}
        ])
    }

    async fn call_tool(&self, name: &str, arguments: &Value) -> Result<Value, String> {
        // Check tool access permissions
        self.check_tool_access(name)
            .map_err(|e| format!("Access denied: {}", e))?;

        match name {
            "list_users" => {
                match self.tools.list_users().await {
                    Ok(users) => Ok(json!(users)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "get_user" => {
                let user_id = arguments["user_id"].as_i64().ok_or("Missing user_id")? as i32;
                match self.tools.get_user(user_id).await {
                    Ok(user) => Ok(json!(user)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "get_user_by_username" => {
                let username = arguments["username"].as_str().ok_or("Missing username")?;
                match self.tools.get_user_by_username(username).await {
                    Ok(user) => Ok(json!(user)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "list_pools" => {
                match self.tools.list_pools().await {
                    Ok(pools) => Ok(json!(pools)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "get_pool_status" => {
                let pool_name = arguments["pool_name"].as_str().ok_or("Missing pool_name")?;
                match self.tools.get_pool_status(pool_name).await {
                    Ok(pool) => Ok(json!(pool)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "list_datasets" => {
                match self.tools.list_datasets().await {
                    Ok(datasets) => Ok(json!(datasets)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "get_dataset" => {
                let dataset_path = arguments["dataset_path"].as_str().ok_or("Missing dataset_path")?;
                match self.tools.get_dataset(dataset_path).await {
                    Ok(dataset) => Ok(json!(dataset)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "create_dataset" => {
                let pool_name = arguments["pool_name"].as_str().ok_or("Missing pool_name")?;
                let dataset_name = arguments["dataset_name"].as_str().ok_or("Missing dataset_name")?;
                match self.tools.create_dataset(pool_name, dataset_name).await {
                    Ok(dataset) => Ok(json!(dataset)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "delete_dataset" => {
                let dataset_path = arguments["dataset_path"].as_str().ok_or("Missing dataset_path")?;
                match self.tools.delete_dataset(dataset_path).await {
                    Ok(_) => Ok(json!({"status": "deleted", "path": dataset_path})),
                    Err(e) => Err(e.to_string()),
                }
            }
            "list_smb_shares" => {
                match self.tools.list_smb_shares().await {
                    Ok(shares) => Ok(json!(shares)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "create_smb_share" => {
                let name = arguments["name"].as_str().ok_or("Missing name")?;
                let path = arguments["path"].as_str().ok_or("Missing path")?;
                let comment = arguments["comment"].as_str();
                match self.tools.create_smb_share(name, path, comment).await {
                    Ok(share) => Ok(json!(share)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "delete_smb_share" => {
                let share_id = arguments["share_id"].as_i64().ok_or("Missing share_id")? as i32;
                match self.tools.delete_smb_share(share_id).await {
                    Ok(_) => Ok(json!({"status": "deleted", "id": share_id})),
                    Err(e) => Err(e.to_string()),
                }
            }
            "list_nfs_exports" => {
                match self.tools.list_nfs_exports().await {
                    Ok(exports) => Ok(json!(exports)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "create_nfs_export" => {
                let paths_arr = arguments["paths"].as_array().ok_or("Missing paths")?;
                let paths: Vec<String> = paths_arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect();
                let comment = arguments["comment"].as_str().ok_or("Missing comment")?;
                match self.tools.create_nfs_export(paths, comment.to_string()).await {
                    Ok(export) => Ok(json!(export)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "delete_nfs_export" => {
                let export_id = arguments["export_id"].as_i64().ok_or("Missing export_id")? as i32;
                match self.tools.delete_nfs_export(export_id).await {
                    Ok(_) => Ok(json!({"status": "deleted", "id": export_id})),
                    Err(e) => Err(e.to_string()),
                }
            }
            "list_snapshots" => {
                match self.tools.list_snapshots().await {
                    Ok(snapshots) => Ok(json!(snapshots)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "create_snapshot" => {
                let dataset = arguments["dataset"].as_str().ok_or("Missing dataset")?;
                let snapshot_name = arguments["snapshot_name"].as_str().ok_or("Missing snapshot_name")?;
                match self.tools.create_snapshot(dataset, snapshot_name).await {
                    Ok(snapshot) => Ok(json!(snapshot)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "delete_snapshot" => {
                let snapshot_id = arguments["snapshot_id"].as_str().ok_or("Missing snapshot_id")?;
                match self.tools.delete_snapshot(snapshot_id).await {
                    Ok(_) => Ok(json!({"status": "deleted", "id": snapshot_id})),
                    Err(e) => Err(e.to_string()),
                }
            }
            "list_iscsi_targets" => {
                match self.tools.list_iscsi_targets().await {
                    Ok(targets) => Ok(json!(targets)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "create_iscsi_target" => {
                let name = arguments["name"].as_str().ok_or("Missing name")?;
                match self.tools.create_iscsi_target(name).await {
                    Ok(target) => Ok(json!(target)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "delete_iscsi_target" => {
                let target_id = arguments["target_id"].as_i64().ok_or("Missing target_id")? as i32;
                match self.tools.delete_iscsi_target(target_id).await {
                    Ok(_) => Ok(json!({"status": "deleted", "id": target_id})),
                    Err(e) => Err(e.to_string()),
                }
            }
            "get_system_info" => {
                match self.tools.get_system_info().await {
                    Ok(info) => Ok(json!(info)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "list_apps" => {
                match self.tools.list_apps().await {
                    Ok(apps) => Ok(json!(apps)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "get_app" => {
                let app_name = arguments["app_name"].as_str().ok_or("Missing app_name")?;
                match self.tools.get_app(app_name).await {
                    Ok(app) => Ok(json!(app)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "start_app" => {
                let app_name = arguments["app_name"].as_str().ok_or("Missing app_name")?;
                let options = arguments.get("options").cloned();
                match self.tools.start_app(app_name, options).await {
                    Ok(app) => Ok(json!(app)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "stop_app" => {
                let app_name = arguments["app_name"].as_str().ok_or("Missing app_name")?;
                let force = arguments["force"].as_bool().unwrap_or(false);
                match self.tools.stop_app(app_name, force).await {
                    Ok(app) => Ok(json!(app)),
                    Err(e) => Err(e.to_string()),
                }
            }
            "restart_app" => {
                let app_name = arguments["app_name"].as_str().ok_or("Missing app_name")?;
                match self.tools.restart_app(app_name).await {
                    Ok(app) => Ok(json!(app)),
                    Err(e) => Err(e.to_string()),
                }
            }
            _ => Err(format!("Unknown tool: {}", name)),
        }
    }
}

/// Run server with stdio transport
async fn run_stdio(server: Arc<TrueNasServerImpl>) -> anyhow::Result<()> {
    info!("Starting TrueNAS MCP Server on stdio");

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let mut reader = BufReader::new(stdin);
    let mut writer = stdout;

    loop {
        let mut line = String::new();
        reader.read_line(&mut line).await?;
        if line.is_empty() {
            continue;
        }

        let request: Value = serde_json::from_str(&line)
            .context("Failed to parse JSON-RPC request")?;

        let response = handle_request(&server, request).await?;

        writer.write_all(response.as_bytes()).await?;
        writer.flush().await?;
    }
}

/// Handle MCP JSON-RPC request
async fn handle_request(server: &TrueNasServerImpl, request: Value) -> anyhow::Result<String> {
    let method = request["method"].as_str().context("Missing method")?;
    let id = request.get("id").cloned().unwrap_or(json!(null));

    let result = match method {
        "initialize" => {
            let capabilities = json!({
                "tools": {},
                "resources": {}
            });
            json!({
                "serverInfo": server.get_server_info(),
                "capabilities": capabilities
            })
        }
        "tools/list" => {
            json!({
                "tools": server.list_tools()
            })
        }
        "tools/call" => {
            let params = request.get("params").context("Missing params")?;
            let name = params["name"].as_str().context("Missing tool name")?;
            let empty_args = json!({});
            let arguments = params.get("arguments").unwrap_or(&empty_args);
            server.call_tool(name, arguments).await
                .map_err(|e| anyhow::anyhow!("Tool error: {}", e))?
        }
        _ => return Err(anyhow::anyhow!("Unknown method: {}", method)),
    };

    Ok(json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result
    }).to_string())
}

/// Run server with HTTP transport (placeholder)
async fn run_http(_server: Arc<TrueNasServerImpl>, host: &str, port: u16) -> anyhow::Result<()> {
    info!("Starting TrueNAS MCP Server on HTTP {}:{}", host, port);

    // Build Axum app
    let app = axum::Router::new()
        .route("/", axum::routing::get(|| async { axum::Json(json!({"status": "TrueNAS MCP Server running"})) }))
        .route("/mcp", axum::routing::post(mcp_http_handler))
        .layer(tower_http::cors::CorsLayer::permissive());

    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await
        .context(format!("Failed to bind to {}", addr))?;

    info!("TrueNAS HTTP MCP server listening on {}", addr);

    axum::serve(listener, app).await
        .context("HTTP server error")?;

    Ok(())
}

/// MCP HTTP message handler
async fn mcp_http_handler(
    axum::Json(_request): axum::Json<Value>,
) -> impl axum::response::IntoResponse {
    // TODO: Implement proper MCP HTTP protocol handling
    axum::Json(json!({
        "jsonrpc": "2.0",
        "id": null,
        "result": {"status": "ok", "message": "MCP HTTP endpoint"}
    }))
}

/// Run server with SSE transport (placeholder)
async fn run_sse(_server: Arc<TrueNasServerImpl>, host: &str, port: u16) -> anyhow::Result<()> {
    info!("Starting TrueNAS MCP Server with SSE on {}:{}", host, port);

    // Build Axum app with SSE support
    let app = axum::Router::new()
        .route("/sse", axum::routing::get(sse_handler))
        .route("/messages", axum::routing::post(message_handler))
        .layer(tower_http::cors::CorsLayer::permissive());

    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await
        .context(format!("Failed to bind to {}", addr))?;

    info!("TrueNAS SSE MCP server listening on {}", addr);

    axum::serve(listener, app).await
        .context("SSE server error")?;

    Ok(())
}

/// SSE endpoint handler
async fn sse_handler() -> impl axum::response::IntoResponse {
    use axum::response::sse::{Event, Sse};
    use futures_util::stream;

    let stream = stream::once(async move {
        Ok::<_, std::convert::Infallible>(Event::default().data("MCP SSE connection established"))
    });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(std::time::Duration::from_secs(15))
            .text("keepalive")
    )
}

/// Message handler for POST requests
async fn message_handler(
    axum::Json(_message): axum::Json<Value>,
) -> impl axum::response::IntoResponse {
    axum::Json(json!({
        "jsonrpc": "2.0",
        "id": null,
        "result": {"status": "ok", "message": "MCP SSE message received"}
    }))
}
