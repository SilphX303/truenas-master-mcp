use moka::future::Cache;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// TTL for cached items in seconds
    pub ttl_secs: u64,
    /// Maximum cached entries
    pub max_entries: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            ttl_secs: 30, // 30 second TTL for real-time data
            max_entries: 100,
        }
    }
}

/// Cached item wrapper with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedItem<T> {
    /// The cached data
    pub data: T,
    /// When the item was cached
    pub cached_at: i64,
    /// Cache key
    pub key: String,
}

impl<T> CachedItem<T> {
    pub fn new(data: T, key: String) -> Self {
        Self {
            data,
            cached_at: chrono::Utc::now().timestamp(),
            key,
        }
    }

    /// Check if the cache entry is stale
    pub fn is_stale(&self, ttl_secs: u64) -> bool {
        let now = chrono::Utc::now().timestamp();
        now - self.cached_at > ttl_secs as i64
    }
}

/// In-memory cache for TrueNAS data
/// Optimized for AI queries that frequently access the same data
#[derive(Debug, Clone)]
pub struct TrueNasCache {
    /// Cache for pools data (short TTL - changes frequently)
    pools: Cache<String, CachedItem<serde_json::Value>>,
    /// Cache for datasets (medium TTL)
    datasets: Cache<String, CachedItem<serde_json::Value>>,
    /// Cache for system info (short TTL)
    system_info: Cache<String, CachedItem<serde_json::Value>>,
    /// Cache for apps (medium TTL)
    apps: Cache<String, CachedItem<serde_json::Value>>,
    /// Cache for alerts (very short TTL - important for AI)
    alerts: Cache<String, CachedItem<serde_json::Value>>,
    /// Cache for VMs (medium TTL)
    vms: Cache<String, CachedItem<serde_json::Value>>,
    /// Cache configuration
    config: CacheConfig,
}

impl TrueNasCache {
    /// Create a new cache with default configuration
    pub fn new() -> Self {
        Self::with_config(CacheConfig::default())
    }

    /// Create a cache with custom configuration
    pub fn with_config(config: CacheConfig) -> Self {
        Self {
            pools: Cache::builder()
                .max_capacity(config.max_entries)
                .time_to_live(Duration::from_secs(config.ttl_secs))
                .build(),
            datasets: Cache::builder()
                .max_capacity(config.max_entries)
                .time_to_live(Duration::from_secs(config.ttl_secs * 2))
                .build(),
            system_info: Cache::builder()
                .max_capacity(10)
                .time_to_live(Duration::from_secs(config.ttl_secs))
                .build(),
            apps: Cache::builder()
                .max_capacity(config.max_entries)
                .time_to_live(Duration::from_secs(config.ttl_secs * 3))
                .build(),
            alerts: Cache::builder()
                .max_capacity(50)
                .time_to_live(Duration::from_secs(10)) // Very short TTL for alerts
                .build(),
            vms: Cache::builder()
                .max_capacity(config.max_entries)
                .time_to_live(Duration::from_secs(config.ttl_secs * 2))
                .build(),
            config,
        }
    }

    /// Get cached pools if available and not stale
    pub async fn get_pools(&self) -> Option<serde_json::Value> {
        let item = self.pools.get("list_pools").await?;
        if item.is_stale(self.config.ttl_secs) {
            None
        } else {
            Some(item.data)
        }
    }

    /// Cache pools data
    pub async fn set_pools(&self, data: serde_json::Value) {
        self.pools
            .insert(
                "list_pools".to_string(),
                CachedItem::new(data, "list_pools".to_string()),
            )
            .await;
    }

    /// Get cached datasets if available
    pub async fn get_datasets(&self) -> Option<serde_json::Value> {
        let item = self.datasets.get("list_datasets").await?;
        if item.is_stale(self.config.ttl_secs * 2) {
            None
        } else {
            Some(item.data)
        }
    }

    /// Cache datasets data
    pub async fn set_datasets(&self, data: serde_json::Value) {
        self.datasets
            .insert(
                "list_datasets".to_string(),
                CachedItem::new(data, "list_datasets".to_string()),
            )
            .await;
    }

    /// Get cached system info if available
    pub async fn get_system_info(&self) -> Option<serde_json::Value> {
        let item = self.system_info.get("system_info").await?;
        if item.is_stale(self.config.ttl_secs) {
            None
        } else {
            Some(item.data)
        }
    }

    /// Cache system info
    pub async fn set_system_info(&self, data: serde_json::Value) {
        self.system_info
            .insert(
                "system_info".to_string(),
                CachedItem::new(data, "system_info".to_string()),
            )
            .await;
    }

    /// Get cached apps if available
    pub async fn get_apps(&self) -> Option<serde_json::Value> {
        let item = self.apps.get("list_apps").await?;
        if item.is_stale(self.config.ttl_secs * 3) {
            None
        } else {
            Some(item.data)
        }
    }

    /// Cache apps data
    pub async fn set_apps(&self, data: serde_json::Value) {
        self.apps
            .insert(
                "list_apps".to_string(),
                CachedItem::new(data, "list_apps".to_string()),
            )
            .await;
    }

    /// Get cached alerts if available (very important for AI monitoring)
    pub async fn get_alerts(&self) -> Option<serde_json::Value> {
        let item = self.alerts.get("alerts").await?;
        if item.is_stale(10) {
            None
        } else {
            Some(item.data)
        }
    }

    /// Cache alerts data
    pub async fn set_alerts(&self, data: serde_json::Value) {
        self.alerts
            .insert(
                "alerts".to_string(),
                CachedItem::new(data, "alerts".to_string()),
            )
            .await;
    }

    /// Get cached VMs if available
    pub async fn get_vms(&self) -> Option<serde_json::Value> {
        let item = self.vms.get("list_vms").await?;
        if item.is_stale(self.config.ttl_secs * 2) {
            None
        } else {
            Some(item.data)
        }
    }

    /// Cache VMs data
    pub async fn set_vms(&self, data: serde_json::Value) {
        self.vms
            .insert(
                "list_vms".to_string(),
                CachedItem::new(data, "list_vms".to_string()),
            )
            .await;
    }

    /// Invalidate all cached data
    pub async fn invalidate_all(&self) {
        self.pools.invalidate_all();
        self.datasets.invalidate_all();
        self.system_info.invalidate_all();
        self.apps.invalidate_all();
        self.alerts.invalidate_all();
        self.vms.invalidate_all();
    }

    /// Get cache statistics for monitoring
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            pools_entries: self.pools.entry_count(),
            datasets_entries: self.datasets.entry_count(),
            system_info_entries: self.system_info.entry_count(),
            apps_entries: self.apps.entry_count(),
            alerts_entries: self.alerts.entry_count(),
            vms_entries: self.vms.entry_count(),
        }
    }
}

/// Cache statistics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub pools_entries: u64,
    pub datasets_entries: u64,
    pub system_info_entries: u64,
    pub apps_entries: u64,
    pub alerts_entries: u64,
    pub vms_entries: u64,
}

impl Default for TrueNasCache {
    fn default() -> Self {
        Self::new()
    }
}
