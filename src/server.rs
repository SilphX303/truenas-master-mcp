use crate::tools::TrueNasTools;
use crate::config::TrueNasConfig;
use crate::error::Result as TrueNasResult;
use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    tool, tool_handler, tool_router,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// Request types for tools - each tool gets its own struct with JsonSchema
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetUserRequest {
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetUserByUsernameRequest {
    pub username: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetPoolStatusRequest {
    pub pool_name: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetDatasetRequest {
    pub dataset_path: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CreateDatasetRequest {
    pub pool_name: String,
    pub dataset_name: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct DeleteDatasetRequest {
    pub dataset_path: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CreateSmbShareRequest {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct DeleteSmbShareRequest {
    pub share_id: i32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CreateNfsExportRequest {
    pub paths: Vec<String>,
    pub comment: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct DeleteNfsExportRequest {
    pub export_id: i32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CreateSnapshotRequest {
    pub dataset: String,
    pub snapshot_name: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct DeleteSnapshotRequest {
    pub snapshot_id: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CreateIscsiTargetRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct DeleteIscsiTargetRequest {
    pub target_id: i32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetAppRequest {
    pub app_name: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct StartAppRequest {
    pub app_name: String,
    #[serde(default)]
    pub options: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct StopAppRequest {
    pub app_name: String,
    #[serde(default)]
    pub force: Option<bool>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RestartAppRequest {
    pub app_name: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CreateAppRequest {
    pub catalog: String,
    pub item: String,
    pub name: String,
    pub values: serde_json::Value,
    #[serde(default)]
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UpdateAppRequest {
    pub app_name: String,
    pub values: serde_json::Value,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct DeleteAppRequest {
    pub app_name: String,
    #[serde(default)]
    pub force: Option<bool>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RollbackAppRequest {
    pub app_name: String,
    #[serde(default)]
    pub rollback_version: Option<String>,
    #[serde(default)]
    pub snap_name: Option<String>,
    #[serde(default)]
    pub force: Option<bool>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetAppConfigRequest {
    pub app_name: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetAppUpgradeOptionsRequest {
    pub app_name: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UpgradeAppRequest {
    pub app_name: String,
    pub options: serde_json::Value,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ScaleAppRequest {
    pub app_name: String,
    pub replica: i32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetCatalogRequest {
    pub catalog_id: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetCatalogTrainsRequest {
    pub catalog_id: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetCatalogItemRequest {
    pub catalog_id: String,
    pub item: String,
    pub train: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetChartReleaseRequest {
    pub release_name: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetChartReleaseResourcesRequest {
    pub release_name: String,
}

/// TrueNAS MCP Server
#[derive(Debug, Clone)]
pub struct TrueNasServer {
    tools: Arc<TrueNasTools>,
    tool_router: ToolRouter<Self>,
}

impl TrueNasServer {
    /// Create a new TrueNAS MCP server
    pub fn new(config: TrueNasConfig) -> TrueNasResult<Self> {
        let client = crate::client::TrueNasClient::new(config)?;
        let tools = Arc::new(TrueNasTools::new(client));
        Ok(Self {
            tools,
            tool_router: Self::tool_router(),
        })
    }

    /// Create a server without connecting (for testing)
    pub fn new_mock() -> Self {
        let config = crate::config::TrueNasConfig::default();
        let client = crate::client::TrueNasClient::new(config).unwrap();
        let tools = Arc::new(TrueNasTools::new(client));
        Self {
            tools,
            tool_router: Self::tool_router(),
        }
    }
}

/// Implement ServerHandler using the tool_router macro
#[tool_handler(router = self.tool_router)]
impl ServerHandler for TrueNasServer {}

/// Tool router with all tool definitions
#[tool_router(router = tool_router)]
impl TrueNasServer {
    #[tool(name = "list_users", description = "List all users on the TrueNAS system")]
    async fn list_users(&self) {
        let _ = self.tools.list_users().await;
    }

    #[tool(name = "get_user", description = "Get details of a specific user by ID")]
    async fn get_user(&self, _req: Parameters<GetUserRequest>) {
        let _ = self.tools.get_user(_req.0.user_id).await;
    }

    #[tool(name = "get_user_by_username", description = "Get details of a specific user by username")]
    async fn get_user_by_username(&self, _req: Parameters<GetUserByUsernameRequest>) {
        let _ = self.tools.get_user_by_username(&_req.0.username).await;
    }

    #[tool(name = "list_pools", description = "List all storage pools on the TrueNAS system")]
    async fn list_pools(&self) {
        let _ = self.tools.list_pools().await;
    }

    #[tool(name = "get_pool_status", description = "Get the status of a specific storage pool")]
    async fn get_pool_status(&self, _req: Parameters<GetPoolStatusRequest>) {
        let _ = self.tools.get_pool_status(&_req.0.pool_name).await;
    }

    #[tool(name = "list_datasets", description = "List all datasets on the TrueNAS system")]
    async fn list_datasets(&self) {
        let _ = self.tools.list_datasets().await;
    }

    #[tool(name = "get_dataset", description = "Get details of a specific dataset")]
    async fn get_dataset(&self, _req: Parameters<GetDatasetRequest>) {
        let _ = self.tools.get_dataset(&_req.0.dataset_path).await;
    }

    #[tool(name = "create_dataset", description = "Create a new dataset in a pool")]
    async fn create_dataset(&self, _req: Parameters<CreateDatasetRequest>) {
        let _ = self.tools.create_dataset(&_req.0.pool_name, &_req.0.dataset_name).await;
    }

    #[tool(name = "delete_dataset", description = "Delete a dataset")]
    async fn delete_dataset(&self, _req: Parameters<DeleteDatasetRequest>) {
        let _ = self.tools.delete_dataset(&_req.0.dataset_path).await;
    }

    #[tool(name = "list_smb_shares", description = "List all SMB shares on the TrueNAS system")]
    async fn list_smb_shares(&self) {
        let _ = self.tools.list_smb_shares().await;
    }

    #[tool(name = "create_smb_share", description = "Create a new SMB share")]
    async fn create_smb_share(&self, _req: Parameters<CreateSmbShareRequest>) {
        let _ = self.tools.create_smb_share(&_req.0.name, &_req.0.path, _req.0.comment.as_deref()).await;
    }

    #[tool(name = "delete_smb_share", description = "Delete an SMB share")]
    async fn delete_smb_share(&self, _req: Parameters<DeleteSmbShareRequest>) {
        let _ = self.tools.delete_smb_share(_req.0.share_id).await;
    }

    #[tool(name = "list_nfs_exports", description = "List all NFS exports on the TrueNAS system")]
    async fn list_nfs_exports(&self) {
        let _ = self.tools.list_nfs_exports().await;
    }

    #[tool(name = "create_nfs_export", description = "Create a new NFS export")]
    async fn create_nfs_export(&self, _req: Parameters<CreateNfsExportRequest>) {
        let _ = self.tools.create_nfs_export(_req.0.paths, _req.0.comment).await;
    }

    #[tool(name = "delete_nfs_export", description = "Delete an NFS export")]
    async fn delete_nfs_export(&self, _req: Parameters<DeleteNfsExportRequest>) {
        let _ = self.tools.delete_nfs_export(_req.0.export_id).await;
    }

    #[tool(name = "list_snapshots", description = "List all ZFS snapshots on the TrueNAS system")]
    async fn list_snapshots(&self) {
        let _ = self.tools.list_snapshots().await;
    }

    #[tool(name = "create_snapshot", description = "Create a new ZFS snapshot")]
    async fn create_snapshot(&self, _req: Parameters<CreateSnapshotRequest>) {
        let _ = self.tools.create_snapshot(&_req.0.dataset, &_req.0.snapshot_name).await;
    }

    #[tool(name = "delete_snapshot", description = "Delete a ZFS snapshot")]
    async fn delete_snapshot(&self, _req: Parameters<DeleteSnapshotRequest>) {
        let _ = self.tools.delete_snapshot(&_req.0.snapshot_id).await;
    }

    #[tool(name = "list_iscsi_targets", description = "List all iSCSI targets on the TrueNAS system")]
    async fn list_iscsi_targets(&self) {
        let _ = self.tools.list_iscsi_targets().await;
    }

    #[tool(name = "create_iscsi_target", description = "Create a new iSCSI target")]
    async fn create_iscsi_target(&self, _req: Parameters<CreateIscsiTargetRequest>) {
        let _ = self.tools.create_iscsi_target(&_req.0.name).await;
    }

    #[tool(name = "delete_iscsi_target", description = "Delete an iSCSI target")]
    async fn delete_iscsi_target(&self, _req: Parameters<DeleteIscsiTargetRequest>) {
        let _ = self.tools.delete_iscsi_target(_req.0.target_id).await;
    }

    #[tool(name = "get_system_info", description = "Get system information from TrueNAS")]
    async fn get_system_info(&self) {
        let _ = self.tools.get_system_info().await;
    }

    #[tool(name = "list_apps", description = "List all applications (jails/containers) on TrueNAS")]
    async fn list_apps(&self) {
        let _ = self.tools.list_apps().await;
    }

    #[tool(name = "get_app", description = "Get details of a specific application")]
    async fn get_app(&self, _req: Parameters<GetAppRequest>) {
        let _ = self.tools.get_app(&_req.0.app_name).await;
    }

    #[tool(name = "start_app", description = "Start an application on TrueNAS")]
    async fn start_app(&self, _req: Parameters<StartAppRequest>) {
        let _ = self.tools.start_app(&_req.0.app_name, _req.0.options).await;
    }

    #[tool(name = "stop_app", description = "Stop an application on TrueNAS")]
    async fn stop_app(&self, _req: Parameters<StopAppRequest>) {
        let _ = self.tools.stop_app(&_req.0.app_name, _req.0.force.unwrap_or(false)).await;
    }

    #[tool(name = "restart_app", description = "Restart an application on TrueNAS")]
    async fn restart_app(&self, _req: Parameters<RestartAppRequest>) {
        let _ = self.tools.restart_app(&_req.0.app_name).await;
    }

    #[tool(name = "create_app", description = "Create a new application from a catalog item")]
    async fn create_app(&self, _req: Parameters<CreateAppRequest>) {
        let _ = self.tools.create_app(&_req.0.catalog, &_req.0.item, &_req.0.name, _req.0.values, _req.0.version.as_deref()).await;
    }

    #[tool(name = "update_app", description = "Update an existing application with new configuration")]
    async fn update_app(&self, _req: Parameters<UpdateAppRequest>) {
        let _ = self.tools.update_app(&_req.0.app_name, _req.0.values).await;
    }

    #[tool(name = "delete_app", description = "Delete an application from TrueNAS")]
    async fn delete_app(&self, _req: Parameters<DeleteAppRequest>) {
        let _ = self.tools.delete_app(&_req.0.app_name, _req.0.force.unwrap_or(false)).await;
    }

    #[tool(name = "rollback_app", description = "Rollback an application to a previous version")]
    async fn rollback_app(&self, _req: Parameters<RollbackAppRequest>) {
        let _ = self.tools.rollback_app(&_req.0.app_name, _req.0.rollback_version.as_deref(), _req.0.snap_name.as_deref(), _req.0.force.unwrap_or(false)).await;
    }

    #[tool(name = "get_app_config", description = "Get the configuration of an application")]
    async fn get_app_config(&self, _req: Parameters<GetAppConfigRequest>) {
        let _ = self.tools.get_app_config(&_req.0.app_name).await;
    }

    #[tool(name = "get_app_upgrade_options", description = "Get available upgrade options for an application")]
    async fn get_app_upgrade_options(&self, _req: Parameters<GetAppUpgradeOptionsRequest>) {
        let _ = self.tools.get_app_upgrade_options(&_req.0.app_name).await;
    }

    #[tool(name = "upgrade_app", description = "Upgrade an application to a newer version")]
    async fn upgrade_app(&self, _req: Parameters<UpgradeAppRequest>) {
        let _ = self.tools.upgrade_app(&_req.0.app_name, _req.0.options).await;
    }

    #[tool(name = "scale_app", description = "Scale an application's replica count")]
    async fn scale_app(&self, _req: Parameters<ScaleAppRequest>) {
        let _ = self.tools.scale_app(&_req.0.app_name, _req.0.replica).await;
    }

    #[tool(name = "list_catalog_items", description = "List all available catalog items from TrueNAS catalog")]
    async fn list_catalog_items(&self) {
        let _ = self.tools.list_catalog_items().await;
    }

    #[tool(name = "get_catalog", description = "Get details of a specific catalog")]
    async fn get_catalog(&self, _req: Parameters<GetCatalogRequest>) {
        let _ = self.tools.get_catalog(&_req.0.catalog_id).await;
    }

    #[tool(name = "get_catalog_trains", description = "Get all available train versions from a catalog")]
    async fn get_catalog_trains(&self, _req: Parameters<GetCatalogTrainsRequest>) {
        let _ = self.tools.get_catalog_trains(&_req.0.catalog_id).await;
    }

    #[tool(name = "get_catalog_item", description = "Get details of a specific item from a catalog")]
    async fn get_catalog_item(&self, _req: Parameters<GetCatalogItemRequest>) {
        let _ = self.tools.get_catalog_item(&_req.0.catalog_id, &_req.0.item, &_req.0.train).await;
    }

    #[tool(name = "list_chart_releases", description = "List all deployed chart releases (apps)")]
    async fn list_chart_releases(&self) {
        let _ = self.tools.list_chart_releases().await;
    }

    #[tool(name = "get_chart_release", description = "Get details of a specific chart release")]
    async fn get_chart_release(&self, _req: Parameters<GetChartReleaseRequest>) {
        let _ = self.tools.get_chart_release(&_req.0.release_name).await;
    }

    #[tool(name = "get_chart_release_resources", description = "Get resources for a specific chart release")]
    async fn get_chart_release_resources(&self, _req: Parameters<GetChartReleaseResourcesRequest>) {
        let _ = self.tools.get_chart_release_resources(&_req.0.release_name).await;
    }
}
