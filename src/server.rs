use crate::tools::TrueNasTools;
use crate::config::TrueNasConfig;
use crate::error::Result as TrueNasResult;
use rmcp::{
    ServerHandler,
    model::{ServerInfo, Tool},
    ErrorData,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

/// TrueNAS MCP Server
#[derive(Debug, Clone)]
pub struct TrueNasServer {
    tools: Arc<TrueNasTools>,
}

impl TrueNasServer {
    /// Create a new TrueNAS MCP server
    pub fn new(config: TrueNasConfig) -> TrueNasResult<Self> {
        let client = crate::client::TrueNasClient::new(config)?;
        let tools = Arc::new(TrueNasTools::new(client));
        Ok(Self { tools })
    }

    /// Get all available tools
    pub fn list_tools_impl() -> Vec<Tool> {
        vec![
            Tool {
                name: "list_users".to_string(),
                description: Some("List all users on the TrueNAS system".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
            Tool {
                name: "get_user".to_string(),
                description: Some("Get details of a specific user by ID".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "user_id": {"type": "integer"}
                    },
                    "required": ["user_id"]
                }),
            },
            Tool {
                name: "get_user_by_username".to_string(),
                description: Some("Get details of a specific user by username".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "username": {"type": "string"}
                    },
                    "required": ["username"]
                }),
            },
            Tool {
                name: "list_pools".to_string(),
                description: Some("List all storage pools on the TrueNAS system".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
            Tool {
                name: "get_pool_status".to_string(),
                description: Some("Get the status of a specific storage pool".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "pool_name": {"type": "string"}
                    },
                    "required": ["pool_name"]
                }),
            },
            Tool {
                name: "list_datasets".to_string(),
                description: Some("List all datasets on the TrueNAS system".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
            Tool {
                name: "get_dataset".to_string(),
                description: Some("Get details of a specific dataset".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "dataset_path": {"type": "string"}
                    },
                    "required": ["dataset_path"]
                }),
            },
            Tool {
                name: "create_dataset".to_string(),
                description: Some("Create a new dataset in a pool".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "pool_name": {"type": "string"},
                        "dataset_name": {"type": "string"}
                    },
                    "required": ["pool_name", "dataset_name"]
                }),
            },
            Tool {
                name: "delete_dataset".to_string(),
                description: Some("Delete a dataset".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "dataset_path": {"type": "string"}
                    },
                    "required": ["dataset_path"]
                }),
            },
            Tool {
                name: "list_smb_shares".to_string(),
                description: Some("List all SMB shares on the TrueNAS system".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
            Tool {
                name: "create_smb_share".to_string(),
                description: Some("Create a new SMB share".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "name": {"type": "string"},
                        "path": {"type": "string"},
                        "comment": {"type": "string"}
                    },
                    "required": ["name", "path"]
                }),
            },
            Tool {
                name: "delete_smb_share".to_string(),
                description: Some("Delete an SMB share".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "share_id": {"type": "integer"}
                    },
                    "required": ["share_id"]
                }),
            },
            Tool {
                name: "list_nfs_exports".to_string(),
                description: Some("List all NFS exports on the TrueNAS system".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
            Tool {
                name: "create_nfs_export".to_string(),
                description: Some("Create a new NFS export".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "paths": {"type": "array", "items": {"type": "string"}},
                        "comment": {"type": "string"}
                    },
                    "required": ["paths", "comment"]
                }),
            },
            Tool {
                name: "delete_nfs_export".to_string(),
                description: Some("Delete an NFS export".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "export_id": {"type": "integer"}
                    },
                    "required": ["export_id"]
                }),
            },
            Tool {
                name: "list_snapshots".to_string(),
                description: Some("List all ZFS snapshots on the TrueNAS system".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
            Tool {
                name: "create_snapshot".to_string(),
                description: Some("Create a new ZFS snapshot".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "dataset": {"type": "string"},
                        "snapshot_name": {"type": "string"}
                    },
                    "required": ["dataset", "snapshot_name"]
                }),
            },
            Tool {
                name: "delete_snapshot".to_string(),
                description: Some("Delete a ZFS snapshot".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "snapshot_id": {"type": "string"}
                    },
                    "required": ["snapshot_id"]
                }),
            },
            Tool {
                name: "list_iscsi_targets".to_string(),
                description: Some("List all iSCSI targets on the TrueNAS system".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
            Tool {
                name: "create_iscsi_target".to_string(),
                description: Some("Create a new iSCSI target".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "name": {"type": "string"}
                    },
                    "required": ["name"]
                }),
            },
            Tool {
                name: "delete_iscsi_target".to_string(),
                description: Some("Delete an iSCSI target".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "target_id": {"type": "integer"}
                    },
                    "required": ["target_id"]
                }),
            },
            Tool {
                name: "get_system_info".to_string(),
                description: Some("Get system information from TrueNAS".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
            Tool {
                name: "list_apps".to_string(),
                description: Some("List all applications (jails/containers) on TrueNAS".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
            Tool {
                name: "get_app".to_string(),
                description: Some("Get details of a specific application".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "app_name": {"type": "string"}
                    },
                    "required": ["app_name"]
                }),
            },
            Tool {
                name: "start_app".to_string(),
                description: Some("Start an application on TrueNAS".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "app_name": {"type": "string"},
                        "options": {"type": "object"}
                    },
                    "required": ["app_name"]
                }),
            },
            Tool {
                name: "stop_app".to_string(),
                description: Some("Stop an application on TrueNAS".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "app_name": {"type": "string"},
                        "force": {"type": "boolean"}
                    },
                    "required": ["app_name"]
                }),
            },
            Tool {
                name: "restart_app".to_string(),
                description: Some("Restart an application on TrueNAS".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "app_name": {"type": "string"}
                    },
                    "required": ["app_name"]
                }),
            },
            Tool {
                name: "create_app".to_string(),
                description: Some("Create a new application from a catalog item".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "catalog": {"type": "string", "description": "Catalog name (e.g., 'official', 'community')"},
                        "item": {"type": "string", "description": "Application item name from catalog"},
                        "name": {"type": "string", "description": "Name for the new application"},
                        "values": {"type": "object", "description": "Configuration values for the application"},
                        "version": {"type": "string", "description": "Optional specific version to install"}
                    },
                    "required": ["catalog", "item", "name", "values"]
                }),
            },
            Tool {
                name: "update_app".to_string(),
                description: Some("Update an existing application with new configuration".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "app_name": {"type": "string"},
                        "values": {"type": "object"}
                    },
                    "required": ["app_name", "values"]
                }),
            },
            Tool {
                name: "delete_app".to_string(),
                description: Some("Delete an application from TrueNAS".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "app_name": {"type": "string"},
                        "force": {"type": "boolean", "description": "Force delete even if apps have dependents"}
                    },
                    "required": ["app_name"]
                }),
            },
            Tool {
                name: "rollback_app".to_string(),
                description: Some("Rollback an application to a previous version".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "app_name": {"type": "string"},
                        "rollback_version": {"type": "string", "description": "Version to rollback to"},
                        "snap_name": {"type": "string", "description": "Snapshot name to rollback to"},
                        "force": {"type": "boolean"}
                    },
                    "required": ["app_name"]
                }),
            },
            Tool {
                name: "get_app_config".to_string(),
                description: Some("Get the configuration of an application".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "app_name": {"type": "string"}
                    },
                    "required": ["app_name"]
                }),
            },
            Tool {
                name: "get_app_upgrade_options".to_string(),
                description: Some("Get available upgrade options for an application".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "app_name": {"type": "string"}
                    },
                    "required": ["app_name"]
                }),
            },
            Tool {
                name: "upgrade_app".to_string(),
                description: Some("Upgrade an application to a newer version".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "app_name": {"type": "string"},
                        "options": {"type": "object", "description": "Upgrade options"}
                    },
                    "required": ["app_name", "options"]
                }),
            },
            Tool {
                name: "scale_app".to_string(),
                description: Some("Scale an application's replica count".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "app_name": {"type": "string"},
                        "replica": {"type": "integer", "description": "Number of replicas"}
                    },
                    "required": ["app_name", "replica"]
                }),
            },
            Tool {
                name: "list_catalog_items".to_string(),
                description: Some("List all available catalog items from TrueNAS catalog".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
            Tool {
                name: "get_catalog".to_string(),
                description: Some("Get details of a specific catalog".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "catalog_id": {"type": "string"}
                    },
                    "required": ["catalog_id"]
                }),
            },
            Tool {
                name: "get_catalog_trains".to_string(),
                description: Some("Get all available train versions from a catalog".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "catalog_id": {"type": "string"}
                    },
                    "required": ["catalog_id"]
                }),
            },
            Tool {
                name: "get_catalog_item".to_string(),
                description: Some("Get details of a specific item from a catalog".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "catalog_id": {"type": "string"},
                        "item": {"type": "string"},
                        "train": {"type": "string"}
                    },
                    "required": ["catalog_id", "item", "train"]
                }),
            },
            Tool {
                name: "list_chart_releases".to_string(),
                description: Some("List all deployed chart releases (apps)".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
            Tool {
                name: "get_chart_release".to_string(),
                description: Some("Get details of a specific chart release".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "release_name": {"type": "string"}
                    },
                    "required": ["release_name"]
                }),
            },
            Tool {
                name: "get_chart_release_resources".to_string(),
                description: Some("Get resources for a specific chart release".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "release_name": {"type": "string"}
                    },
                    "required": ["release_name"]
                }),
            },
        ]
    }
}

/// Implement ServerHandler for TrueNAS server
impl ServerHandler for TrueNasServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: rmcp::model::ProtocolVersion::V2024_11_05,
            capabilities: Default::default(),
            server_info: Some(rmcp::model::Implementation {
                name: "truenas-master-mcp".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                description: Some("Official MCP server for TrueNAS API access".to_string()),
                instructions: Some(
                    "This server provides access to TrueNAS SCALE/CORE management features including:\n\
                    - User management\n\
                    - Pool and dataset management\n\
                    - SMB and NFS share management\n\
                    - Snapshot management\n\
                    - iSCSI target management\n\
                    - Apps/Jails management\n\
                    - System information".to_string()
                ),
                ..Default::default()
            }),
        }
    }

    fn list_tools(&self) -> Vec<Tool> {
        Self::list_tools_impl()
    }

    async fn call_tool(
        &self,
        name: &str,
        arguments: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, ErrorData> {
        let args = arguments.cloned().unwrap_or_default();

        match name {
            "list_users" => {
                match self.tools.list_users().await {
                    Ok(users) => Ok(json!(users)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "get_user" => {
                let user_id = args["user_id"].as_i64().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid user_id".to_string(), None)
                })? as i32;
                match self.tools.get_user(user_id).await {
                    Ok(user) => Ok(json!(user)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "get_user_by_username" => {
                let username = args["username"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid username".to_string(), None)
                })?;
                match self.tools.get_user_by_username(username).await {
                    Ok(user) => Ok(json!(user)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "list_pools" => {
                match self.tools.list_pools().await {
                    Ok(pools) => Ok(json!(pools)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "get_pool_status" => {
                let pool_name = args["pool_name"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid pool_name".to_string(), None)
                })?;
                match self.tools.get_pool_status(pool_name).await {
                    Ok(pool) => Ok(json!(pool)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "list_datasets" => {
                match self.tools.list_datasets().await {
                    Ok(datasets) => Ok(json!(datasets)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "get_dataset" => {
                let dataset_path = args["dataset_path"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid dataset_path".to_string(), None)
                })?;
                match self.tools.get_dataset(dataset_path).await {
                    Ok(dataset) => Ok(json!(dataset)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "create_dataset" => {
                let pool_name = args["pool_name"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid pool_name".to_string(), None)
                })?;
                let dataset_name = args["dataset_name"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid dataset_name".to_string(), None)
                })?;
                match self.tools.create_dataset(pool_name, dataset_name).await {
                    Ok(dataset) => Ok(json!(dataset)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "delete_dataset" => {
                let dataset_path = args["dataset_path"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid dataset_path".to_string(), None)
                })?;
                match self.tools.delete_dataset(dataset_path).await {
                    Ok(_) => Ok(json!({"status": "deleted", "path": dataset_path})),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "list_smb_shares" => {
                match self.tools.list_smb_shares().await {
                    Ok(shares) => Ok(json!(shares)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "create_smb_share" => {
                let name = args["name"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid name".to_string(), None)
                })?;
                let path = args["path"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid path".to_string(), None)
                })?;
                let comment = args["comment"].as_str();
                match self.tools.create_smb_share(name, path, comment).await {
                    Ok(share) => Ok(json!(share)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "delete_smb_share" => {
                let share_id = args["share_id"].as_i64().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid share_id".to_string(), None)
                })? as i32;
                match self.tools.delete_smb_share(share_id).await {
                    Ok(_) => Ok(json!({"status": "deleted", "id": share_id})),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "list_nfs_exports" => {
                match self.tools.list_nfs_exports().await {
                    Ok(exports) => Ok(json!(exports)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "create_nfs_export" => {
                let paths_arr = args["paths"].as_array().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid paths".to_string(), None)
                })?;
                let paths: Vec<String> = paths_arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect();
                let comment = args["comment"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid comment".to_string(), None)
                })?;
                match self.tools.create_nfs_export(paths, comment.to_string()).await {
                    Ok(export) => Ok(json!(export)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "delete_nfs_export" => {
                let export_id = args["export_id"].as_i64().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid export_id".to_string(), None)
                })? as i32;
                match self.tools.delete_nfs_export(export_id).await {
                    Ok(_) => Ok(json!({"status": "deleted", "id": export_id})),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "list_snapshots" => {
                match self.tools.list_snapshots().await {
                    Ok(snapshots) => Ok(json!(snapshots)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "create_snapshot" => {
                let dataset = args["dataset"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid dataset".to_string(), None)
                })?;
                let snapshot_name = args["snapshot_name"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid snapshot_name".to_string(), None)
                })?;
                match self.tools.create_snapshot(dataset, snapshot_name).await {
                    Ok(snapshot) => Ok(json!(snapshot)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "delete_snapshot" => {
                let snapshot_id = args["snapshot_id"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid snapshot_id".to_string(), None)
                })?;
                match self.tools.delete_snapshot(snapshot_id).await {
                    Ok(_) => Ok(json!({"status": "deleted", "id": snapshot_id})),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "list_iscsi_targets" => {
                match self.tools.list_iscsi_targets().await {
                    Ok(targets) => Ok(json!(targets)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "create_iscsi_target" => {
                let name = args["name"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid name".to_string(), None)
                })?;
                match self.tools.create_iscsi_target(name).await {
                    Ok(target) => Ok(json!(target)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "delete_iscsi_target" => {
                let target_id = args["target_id"].as_i64().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid target_id".to_string(), None)
                })? as i32;
                match self.tools.delete_iscsi_target(target_id).await {
                    Ok(_) => Ok(json!({"status": "deleted", "id": target_id})),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "get_system_info" => {
                match self.tools.get_system_info().await {
                    Ok(info) => Ok(json!(info)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "list_apps" => {
                match self.tools.list_apps().await {
                    Ok(apps) => Ok(json!(apps)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "get_app" => {
                let app_name = args["app_name"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid app_name".to_string(), None)
                })?;
                match self.tools.get_app(app_name).await {
                    Ok(app) => Ok(json!(app)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "start_app" => {
                let app_name = args["app_name"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid app_name".to_string(), None)
                })?;
                let options = args.get("options").cloned();
                match self.tools.start_app(app_name, options).await {
                    Ok(app) => Ok(json!(app)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "stop_app" => {
                let app_name = args["app_name"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid app_name".to_string(), None)
                })?;
                let force = args["force"].as_bool().unwrap_or(false);
                match self.tools.stop_app(app_name, force).await {
                    Ok(app) => Ok(json!(app)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "restart_app" => {
                let app_name = args["app_name"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid app_name".to_string(), None)
                })?;
                match self.tools.restart_app(app_name).await {
                    Ok(app) => Ok(json!(app)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "create_app" => {
                let catalog = args["catalog"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid catalog".to_string(), None)
                })?;
                let item = args["item"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid item".to_string(), None)
                })?;
                let name = args["name"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid name".to_string(), None)
                })?;
                let values = args["values"].clone();
                let version = args["version"].as_str();
                match self.tools.create_app(catalog, item, name, values, version).await {
                    Ok(app) => Ok(json!(app)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "update_app" => {
                let app_name = args["app_name"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid app_name".to_string(), None)
                })?;
                let values = args["values"].clone();
                match self.tools.update_app(app_name, values).await {
                    Ok(app) => Ok(json!(app)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "delete_app" => {
                let app_name = args["app_name"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid app_name".to_string(), None)
                })?;
                let force = args["force"].as_bool().unwrap_or(false);
                match self.tools.delete_app(app_name, force).await {
                    Ok(_) => Ok(json!({"status": "deleted", "app_name": app_name})),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "rollback_app" => {
                let app_name = args["app_name"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid app_name".to_string(), None)
                })?;
                let rollback_version = args["rollback_version"].as_str();
                let snap_name = args["snap_name"].as_str();
                let force = args["force"].as_bool().unwrap_or(false);
                match self.tools.rollback_app(app_name, rollback_version, snap_name, force).await {
                    Ok(app) => Ok(json!(app)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "get_app_config" => {
                let app_name = args["app_name"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid app_name".to_string(), None)
                })?;
                match self.tools.get_app_config(app_name).await {
                    Ok(config) => Ok(config),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "get_app_upgrade_options" => {
                let app_name = args["app_name"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid app_name".to_string(), None)
                })?;
                match self.tools.get_app_upgrade_options(app_name).await {
                    Ok(options) => Ok(options),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "upgrade_app" => {
                let app_name = args["app_name"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid app_name".to_string(), None)
                })?;
                let options = args["options"].clone();
                match self.tools.upgrade_app(app_name, options).await {
                    Ok(app) => Ok(json!(app)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "scale_app" => {
                let app_name = args["app_name"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid app_name".to_string(), None)
                })?;
                let replica = args["replica"].as_i64().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid replica".to_string(), None)
                })? as i32;
                match self.tools.scale_app(app_name, replica).await {
                    Ok(app) => Ok(json!(app)),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "list_catalog_items" => {
                match self.tools.list_catalog_items().await {
                    Ok(items) => Ok(items),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "get_catalog" => {
                let catalog_id = args["catalog_id"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid catalog_id".to_string(), None)
                })?;
                match self.tools.get_catalog(catalog_id).await {
                    Ok(catalog) => Ok(catalog),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "get_catalog_trains" => {
                let catalog_id = args["catalog_id"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid catalog_id".to_string(), None)
                })?;
                match self.tools.get_catalog_trains(catalog_id).await {
                    Ok(trains) => Ok(trains),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "get_catalog_item" => {
                let catalog_id = args["catalog_id"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid catalog_id".to_string(), None)
                })?;
                let item = args["item"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid item".to_string(), None)
                })?;
                let train = args["train"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid train".to_string(), None)
                })?;
                match self.tools.get_catalog_item(catalog_id, item, train).await {
                    Ok(item_details) => Ok(item_details),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "list_chart_releases" => {
                match self.tools.list_chart_releases().await {
                    Ok(releases) => Ok(releases),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "get_chart_release" => {
                let release_name = args["release_name"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid release_name".to_string(), None)
                })?;
                match self.tools.get_chart_release(release_name).await {
                    Ok(release) => Ok(release),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            "get_chart_release_resources" => {
                let release_name = args["release_name"].as_str().ok_or_else(|| {
                    ErrorData::invalid_request("Missing or invalid release_name".to_string(), None)
                })?;
                match self.tools.get_chart_release_resources(release_name).await {
                    Ok(resources) => Ok(resources),
                    Err(e) => Err(ErrorData::invalid_request(e.to_string(), None)),
                }
            }
            _ => Err(ErrorData::invalid_request(
                format!("Unknown tool: {}", name),
                None
            )),
        }
    }
}
