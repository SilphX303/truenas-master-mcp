use crate::client::TrueNasClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// TrueNAS API response types
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub uid: i32,
    #[serde(default)]
    pub home: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub full_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pool {
    pub name: String,
    pub guid: String,
    pub status: String,
    pub size: u64,
    pub free: u64,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dataset {
    pub name: String,
    pub pool: String,
    #[serde(default)]
    pub mountpoint: Option<String>,
    #[serde(default)]
    pub comments: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SmbShare {
    pub id: i32,
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NfsExport {
    pub id: i32,
    pub paths: Vec<String>,
    pub comment: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Snapshot {
    pub name: String,
    pub pool: String,
    pub dataset: String,
    pub creation: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IscsiTarget {
    pub id: i32,
    pub name: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SystemInfo {
    pub version: String,
    pub hostname: String,
    #[serde(default)]
    pub cpu_model: Option<String>,
    #[serde(default)]
    pub uptime_seconds: Option<u64>,
}

/// App information for TrueNAS apps/jails
#[derive(Debug, Deserialize, Serialize)]
pub struct AppInfo {
    pub name: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default)]
    pub image: Option<String>,
}

/// Tool handlers for TrueNAS operations
#[derive(Debug)]
pub struct TrueNasTools {
    client: TrueNasClient,
}

impl TrueNasTools {
    pub fn new(client: TrueNasClient) -> Self {
        Self { client }
    }

    // === User Management ===

    pub async fn list_users(&self) -> Result<Vec<User>> {
        self.client.get("/api/v2.0/user").await
    }

    pub async fn get_user(&self, user_id: i32) -> Result<User> {
        self.client.get(&format!("/api/v2.0/user/{}", user_id)).await
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<User> {
        let users: Vec<User> = self.client.get("/api/v2.0/user").await?;
        users.into_iter()
            .find(|u| u.username == username)
            .ok_or_else(|| crate::error::TrueNasError::NotFound(format!("User '{}' not found", username)))
    }

    // === Pool Management ===

    pub async fn list_pools(&self) -> Result<Vec<Pool>> {
        self.client.get("/api/v2.0/pool").await
    }

    pub async fn get_pool_status(&self, pool_name: &str) -> Result<Pool> {
        self.client.get(&format!("/api/v2.0/pool/{}", pool_name)).await
    }

    // === Dataset Management ===

    pub async fn list_datasets(&self) -> Result<Vec<Dataset>> {
        self.client.get("/api/v2.0/pool/dataset").await
    }

    pub async fn get_dataset(&self, dataset_path: &str) -> Result<Dataset> {
        let encoded = urlencoding::encode(dataset_path);
        self.client.get(&format!("/api/v2.0/pool/dataset/{}", encoded)).await
    }

    pub async fn create_dataset(&self, pool_name: &str, dataset_name: &str) -> Result<Dataset> {
        #[derive(Serialize)]
        struct CreateDatasetRequest {
            name: String,
        }
        let full_name = format!("{}/{}", pool_name, dataset_name);
        self.client.post("/api/v2.0/pool/dataset", &CreateDatasetRequest {
            name: full_name,
        }).await
    }

    pub async fn delete_dataset(&self, dataset_path: &str) -> Result<()> {
        let encoded = urlencoding::encode(dataset_path);
        self.client.delete(&format!("/api/v2.0/pool/dataset/{}", encoded)).await
    }

    // === SMB Shares ===

    pub async fn list_smb_shares(&self) -> Result<Vec<SmbShare>> {
        self.client.get("/api/v2.0/sharing/smb").await
    }

    pub async fn create_smb_share(&self, name: &str, path: &str, comment: Option<&str>) -> Result<SmbShare> {
        #[derive(Serialize)]
        struct CreateSmbRequest {
            name: String,
            path: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            comment: Option<String>,
        }
        self.client.post("/api/v2.0/sharing/smb", &CreateSmbRequest {
            name: name.to_string(),
            path: path.to_string(),
            comment: comment.map(|c| c.to_string()),
        }).await
    }

    pub async fn delete_smb_share(&self, share_id: i32) -> Result<()> {
        self.client.delete(&format!("/api/v2.0/sharing/smb/{}", share_id)).await
    }

    // === NFS Exports ===

    pub async fn list_nfs_exports(&self) -> Result<Vec<NfsExport>> {
        self.client.get("/api/v2.0/sharing/nfs").await
    }

    pub async fn create_nfs_export(&self, paths: Vec<String>, comment: String) -> Result<NfsExport> {
        #[derive(Serialize)]
        struct CreateNfsRequest {
            paths: Vec<String>,
            comment: String,
        }
        self.client.post("/api/v2.0/sharing/nfs", &CreateNfsRequest {
            paths,
            comment,
        }).await
    }

    pub async fn delete_nfs_export(&self, export_id: i32) -> Result<()> {
        self.client.delete(&format!("/api/v2.0/sharing/nfs/{}", export_id)).await
    }

    // === Snapshots ===

    pub async fn list_snapshots(&self) -> Result<Vec<Snapshot>> {
        self.client.get("/api/v2.0/zfs/snapshot").await
    }

    pub async fn create_snapshot(&self, dataset: &str, snapshot_name: &str) -> Result<Snapshot> {
        #[derive(Serialize)]
        struct CreateSnapshotRequest {
            dataset: String,
            name: String,
        }
        self.client.post("/api/v2.0/zfs/snapshot", &CreateSnapshotRequest {
            dataset: dataset.to_string(),
            name: snapshot_name.to_string(),
        }).await
    }

    pub async fn delete_snapshot(&self, snapshot_id: &str) -> Result<()> {
        self.client.delete(&format!("/api/v2.0/zfs/snapshot/{}", snapshot_id)).await
    }

    // === iSCSI Targets ===

    pub async fn list_iscsi_targets(&self) -> Result<Vec<IscsiTarget>> {
        self.client.get("/api/v2.0/iscsi/target").await
    }

    pub async fn create_iscsi_target(&self, name: &str) -> Result<IscsiTarget> {
        #[derive(Serialize)]
        struct CreateIscsiRequest {
            name: String,
        }
        self.client.post("/api/v2.0/iscsi/target", &CreateIscsiRequest {
            name: name.to_string(),
        }).await
    }

    pub async fn delete_iscsi_target(&self, target_id: i32) -> Result<()> {
        self.client.delete(&format!("/api/v2.0/iscsi/target/{}", target_id)).await
    }

    // === System Information ===

    pub async fn get_system_info(&self) -> Result<SystemInfo> {
        self.client.get("/api/v2.0/system/info").await
    }

    // === Apps (Jails/Containers) ===

    /// List all applications/jails on TrueNAS
    pub async fn list_apps(&self) -> Result<Vec<AppInfo>> {
        // For TrueNAS SCALE with apps (Kubernetes/Helm charts)
        #[derive(Deserialize)]
        struct ScaleAppResponse {
            #[serde(default)]
            name: String,
            #[serde(default)]
            version: Option<String>,
            #[serde(default)]
            state: Option<String>,
            #[serde(default)]
            description: Option<String>,
        }

        #[derive(Deserialize)]
        struct ScaleAppsList {
            #[serde(default)]
            apps: Vec<ScaleAppResponse>,
        }

        // Try SCALE apps endpoint first
        let scale_result: Option<ScaleAppsList> = self.client.get("/api/v2.0/app").await.ok();
        if let Some(response) = scale_result {
            return Ok(response.apps.into_iter().map(|app| AppInfo {
                name: app.name,
                version: app.version,
                state: app.state,
                description: app.description,
                port: None,
                image: None,
            }).collect());
        }

        // Fall back to CORE jail endpoint
        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct JailResponse {
            #[serde(default)]
            id: i32,
            #[serde(default)]
            name: String,
            #[serde(default)]
            state: String,
        }

        #[derive(Deserialize)]
        struct JailsList {
            #[serde(default)]
            #[serde(rename = "jails")]
            jails_list: Vec<JailResponse>,
        }

        let jails: JailsList = self.client.get("/api/v2.0/jail").await
            .unwrap_or(JailsList { jails_list: vec![] });

        Ok(jails.jails_list.into_iter().map(|jail| AppInfo {
            name: jail.name,
            version: None,
            state: Some(jail.state),
            description: None,
            port: None,
            image: None,
        }).collect())
    }

    /// Get details of a specific application
    pub async fn get_app(&self, app_name: &str) -> Result<AppInfo> {
        let encoded = urlencoding::encode(app_name);

        // Try SCALE app endpoint first
        #[derive(Deserialize)]
        struct ScaleAppDetail {
            #[serde(default)]
            name: String,
            #[serde(default)]
            version: Option<String>,
            #[serde(default)]
            state: Option<String>,
            #[serde(default)]
            description: Option<String>,
            #[serde(default)]
            port: Option<u16>,
            #[serde(default)]
            image: Option<String>,
        }

        let scale_result: Option<ScaleAppDetail> = self.client.get(&format!("/api/v2.0/app/{}", encoded)).await.ok();
        if let Some(app) = scale_result {
            return Ok(AppInfo {
                name: app.name,
                version: app.version,
                state: app.state,
                description: app.description,
                port: app.port,
                image: app.image,
            });
        }

        // Fall back to CORE jail endpoint
        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct JailDetail {
            #[serde(default)]
            id: i32,
            #[serde(default)]
            name: String,
            #[serde(default)]
            state: String,
        }

        let jail: JailDetail = self.client.get(&format!("/api/v2.0/jail/{}", encoded)).await?;

        Ok(AppInfo {
            name: jail.name,
            version: None,
            state: Some(jail.state),
            description: None,
            port: None,
            image: None,
        })
    }

    /// Start an application
    pub async fn start_app(&self, app_name: &str, options: Option<serde_json::Value>) -> Result<AppInfo> {
        let encoded = urlencoding::encode(app_name);

        #[derive(Serialize)]
        struct StartRequest {
            #[serde(skip_serializing_if = "Option::is_none")]
            options: Option<serde_json::Value>,
        }

        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct StartResponse {
            #[serde(default)]
            name: String,
            #[serde(default)]
            state: Option<String>,
        }

        // Try SCALE endpoint
        let _response: StartResponse = self.client.post(
            &format!("/api/v2.0/app/{}/start", encoded),
            &StartRequest { options }
        ).await?;

        self.get_app(app_name).await
    }

    /// Stop an application
    pub async fn stop_app(&self, app_name: &str, force: bool) -> Result<AppInfo> {
        let encoded = urlencoding::encode(app_name);

        #[derive(Serialize)]
        struct StopRequest {
            force: bool,
        }

        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct StopResponse {
            #[serde(default)]
            name: String,
            #[serde(default)]
            state: Option<String>,
        }

        // Try SCALE endpoint
        let _response: StopResponse = self.client.post(
            &format!("/api/v2.0/app/{}/stop", encoded),
            &StopRequest { force }
        ).await?;

        self.get_app(app_name).await
    }

    /// Restart an application
    pub async fn restart_app(&self, app_name: &str) -> Result<AppInfo> {
        let encoded = urlencoding::encode(app_name);

        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct RestartResponse {
            #[serde(default)]
            name: String,
            #[serde(default)]
            state: Option<String>,
        }

        // Try SCALE endpoint
        let _response: RestartResponse = self.client.post(
            &format!("/api/v2.0/app/{}/restart", encoded),
            &()
        ).await?;

        self.get_app(app_name).await
    }
}
