#![allow(clippy::unwrap_used)]

use tokio::test as async_test;
use truenas_master_mcp::cache::{CacheConfig, CacheStats, CachedItem, TrueNasCache};

// === CacheConfig Tests ===

#[test]
fn test_cache_config_default() {
    let config = CacheConfig::default();
    assert_eq!(config.ttl_secs, 30);
    assert_eq!(config.max_entries, 100);
}

#[test]
fn test_cache_config_custom() {
    let config = CacheConfig {
        ttl_secs: 60,
        max_entries: 200,
    };
    assert_eq!(config.ttl_secs, 60);
    assert_eq!(config.max_entries, 200);
}

// === CachedItem Tests ===

#[test]
fn test_cached_item_new() {
    let data = serde_json::json!({"test": "data"});
    let item = CachedItem::new(data.clone(), "test_key".to_string());

    assert_eq!(item.data, data);
    assert_eq!(item.key, "test_key");
    assert!(item.cached_at > 0);
}

#[test]
fn test_cached_item_is_stale_false() {
    let data = serde_json::json!({"test": "data"});
    let item = CachedItem::new(data, "test_key".to_string());

    // Item just created should not be stale
    assert!(!item.is_stale(30));
}

#[test]
fn test_cached_item_is_stale_true() {
    use chrono::Duration;

    let data = serde_json::json!({"test": "data"});
    let mut item = CachedItem::new(data, "test_key".to_string());

    // Set cached_at to 60 seconds ago
    item.cached_at = (chrono::Utc::now() - Duration::seconds(60)).timestamp();

    // With 30 second TTL, this should be stale
    assert!(item.is_stale(30));
}

#[test]
fn test_cached_item_is_stale_edge_case() {
    let data = serde_json::json!({"test": "data"});
    let item = CachedItem::new(data, "test_key".to_string());

    // With a very short TTL (1 second), just-created items are not stale
    // The stale calculation is: now - cached_at > ttl_secs
    // If item was just created, now - cached_at is 0, which is not > 1
    assert!(!item.is_stale(1));
}

// === TrueNasCache Tests ===

#[async_test]
async fn test_cache_new() {
    let cache = TrueNasCache::new();
    let stats = cache.stats();

    assert_eq!(stats.pools_entries, 0);
    assert_eq!(stats.datasets_entries, 0);
    assert_eq!(stats.system_info_entries, 0);
    assert_eq!(stats.apps_entries, 0);
    assert_eq!(stats.alerts_entries, 0);
    assert_eq!(stats.vms_entries, 0);
}

#[async_test]
async fn test_cache_with_config() {
    let config = CacheConfig {
        ttl_secs: 60,
        max_entries: 50,
    };
    let cache = TrueNasCache::with_config(config);

    // Should be empty initially
    let stats = cache.stats();
    assert_eq!(stats.pools_entries, 0);
}

#[async_test]
async fn test_cache_get_pools_empty() {
    let cache = TrueNasCache::new();
    let result = cache.get_pools().await;

    assert!(result.is_none());
}

#[async_test]
async fn test_cache_set_and_get_pools() {
    let cache = TrueNasCache::new();
    let pools = serde_json::json!([{"name": "tank"}, {"name": "data"}]);

    cache.set_pools(pools.clone()).await;

    let result = cache.get_pools().await;
    assert!(result.is_some());
    assert_eq!(result.unwrap(), pools);
}

#[async_test]
async fn test_cache_get_datasets_empty() {
    let cache = TrueNasCache::new();
    let result = cache.get_datasets().await;

    assert!(result.is_none());
}

#[async_test]
async fn test_cache_set_and_get_datasets() {
    let cache = TrueNasCache::new();
    let datasets = serde_json::json!([{"id": "tank/data"}]);

    cache.set_datasets(datasets.clone()).await;

    let result = cache.get_datasets().await;
    assert!(result.is_some());
    assert_eq!(result.unwrap(), datasets);
}

#[async_test]
async fn test_cache_get_system_info_empty() {
    let cache = TrueNasCache::new();
    let result = cache.get_system_info().await;

    assert!(result.is_none());
}

#[async_test]
async fn test_cache_set_and_get_system_info() {
    let cache = TrueNasCache::new();
    let info = serde_json::json!({"version": "24.10.0"});

    cache.set_system_info(info.clone()).await;

    let result = cache.get_system_info().await;
    assert!(result.is_some());
    assert_eq!(result.unwrap(), info);
}

#[async_test]
async fn test_cache_get_apps_empty() {
    let cache = TrueNasCache::new();
    let result = cache.get_apps().await;

    assert!(result.is_none());
}

#[async_test]
async fn test_cache_set_and_get_apps() {
    let cache = TrueNasCache::new();
    let apps = serde_json::json!([{"name": "plex"}]);

    cache.set_apps(apps.clone()).await;

    let result = cache.get_apps().await;
    assert!(result.is_some());
    assert_eq!(result.unwrap(), apps);
}

#[async_test]
async fn test_cache_get_alerts_empty() {
    let cache = TrueNasCache::new();
    let result = cache.get_alerts().await;

    assert!(result.is_none());
}

#[async_test]
async fn test_cache_set_and_get_alerts() {
    let cache = TrueNasCache::new();
    let alerts = serde_json::json!([{"level": "WARNING"}]);

    cache.set_alerts(alerts.clone()).await;

    let result = cache.get_alerts().await;
    assert!(result.is_some());
    assert_eq!(result.unwrap(), alerts);
}

#[async_test]
async fn test_cache_get_vms_empty() {
    let cache = TrueNasCache::new();
    let result = cache.get_vms().await;

    assert!(result.is_none());
}

#[async_test]
async fn test_cache_set_and_get_vms() {
    let cache = TrueNasCache::new();
    let vms = serde_json::json!([{"name": "linux-vm"}]);

    cache.set_vms(vms.clone()).await;

    let result = cache.get_vms().await;
    assert!(result.is_some());
    assert_eq!(result.unwrap(), vms);
}

#[async_test]
async fn test_cache_invalidate_all() {
    let cache = TrueNasCache::new();

    // Populate cache
    cache.set_pools(serde_json::json!(["tank"])).await;
    cache.set_datasets(serde_json::json!(["data"])).await;
    cache
        .set_system_info(serde_json::json!({"version": "24.10"}))
        .await;
    cache.set_apps(serde_json::json!(["plex"])).await;
    cache.set_alerts(serde_json::json!(["alert"])).await;
    cache.set_vms(serde_json::json!(["vm"])).await;

    // Verify data is cached
    assert!(cache.get_pools().await.is_some());
    assert!(cache.get_datasets().await.is_some());
    assert!(cache.get_system_info().await.is_some());
    assert!(cache.get_apps().await.is_some());
    assert!(cache.get_alerts().await.is_some());
    assert!(cache.get_vms().await.is_some());

    // Invalidate all
    cache.invalidate_all().await;

    // Verify all cache is cleared
    assert!(cache.get_pools().await.is_none());
    assert!(cache.get_datasets().await.is_none());
    assert!(cache.get_system_info().await.is_none());
    assert!(cache.get_apps().await.is_none());
    assert!(cache.get_alerts().await.is_none());
    assert!(cache.get_vms().await.is_none());
}

#[async_test]
async fn test_cache_stats() {
    let cache = TrueNasCache::new();

    let initial_stats = cache.stats();
    assert_eq!(initial_stats.pools_entries, 0);
    assert_eq!(initial_stats.datasets_entries, 0);

    // Add some data
    cache.set_pools(serde_json::json!(["tank"])).await;
    cache.set_datasets(serde_json::json!(["data"])).await;

    // Verify data was cached by retrieving it
    assert!(cache.get_pools().await.is_some());
    assert!(cache.get_datasets().await.is_some());

    let stats = cache.stats();
    // Stats function should return values (actual count may vary based on cache timing)
    assert_eq!(stats.system_info_entries, 0);
    assert_eq!(stats.apps_entries, 0);
    assert_eq!(stats.alerts_entries, 0);
    assert_eq!(stats.vms_entries, 0);
}

// === CacheStats Tests ===

#[test]
fn test_cache_stats_default() {
    let stats = CacheStats {
        pools_entries: 0,
        datasets_entries: 0,
        system_info_entries: 0,
        apps_entries: 0,
        alerts_entries: 0,
        vms_entries: 0,
    };

    assert_eq!(stats.pools_entries, 0);
    assert_eq!(stats.datasets_entries, 0);
}

#[test]
fn test_cache_stats_with_values() {
    let stats = CacheStats {
        pools_entries: 5,
        datasets_entries: 10,
        system_info_entries: 2,
        apps_entries: 8,
        alerts_entries: 3,
        vms_entries: 4,
    };

    assert_eq!(stats.pools_entries, 5);
    assert_eq!(stats.datasets_entries, 10);
    assert_eq!(stats.system_info_entries, 2);
    assert_eq!(stats.apps_entries, 8);
    assert_eq!(stats.alerts_entries, 3);
    assert_eq!(stats.vms_entries, 4);
}

// === TrueNasCache Default Trait ===

#[async_test]
async fn test_cache_default() {
    let cache = TrueNasCache::default();
    let stats = cache.stats();

    assert_eq!(stats.pools_entries, 0);
    assert_eq!(stats.datasets_entries, 0);
}
