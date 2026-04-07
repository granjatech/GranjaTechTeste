use moka::future::Cache;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

use crate::errors::AppError;

/// CacheService wrapping moka::future::Cache (per D-04).
/// Uses two internal Cache instances for different TTL tiers (per D-05):
/// - short_cache: 5 min TTL (dashboard KPIs, avicultura dashboard)
/// - long_cache: 10 min TTL (heavy report endpoints)
pub struct CacheService {
    short_cache: Cache<String, String>, // 5 min TTL
    long_cache: Cache<String, String>,  // 10 min TTL
}

impl CacheService {
    pub fn new(max_capacity: u64) -> Self {
        Self {
            short_cache: Cache::builder()
                .max_capacity(max_capacity)
                .time_to_live(Duration::from_secs(5 * 60)) // 5 min
                .build(),
            long_cache: Cache::builder()
                .max_capacity(max_capacity)
                .time_to_live(Duration::from_secs(10 * 60)) // 10 min
                .build(),
        }
    }

    fn select_cache(&self, ttl: Duration) -> &Cache<String, String> {
        if ttl.as_secs() <= 5 * 60 {
            &self.short_cache
        } else {
            &self.long_cache
        }
    }

    /// Get a cached value by key. Returns None if not found or deserialization fails.
    pub async fn get<T: DeserializeOwned>(&self, key: &str, ttl: Duration) -> Option<T> {
        let cache = self.select_cache(ttl);
        cache
            .get(key)
            .await
            .and_then(|json| serde_json::from_str(&json).ok())
    }

    /// Set a cached value with the specified TTL tier.
    pub async fn set<T: Serialize>(&self, key: &str, value: &T, ttl: Duration) {
        let cache = self.select_cache(ttl);
        if let Ok(json) = serde_json::to_string(value) {
            cache.insert(key.to_string(), json).await;
        }
    }

    /// Remove a specific cache entry from both tiers.
    pub async fn remove(&self, key: &str) {
        self.short_cache.remove(key).await;
        self.long_cache.remove(key).await;
    }

    /// Remove entries by pattern. Note: moka does not support pattern removal natively.
    /// Logs a warning (matching .NET MemoryCacheService behavior which also warns).
    pub async fn remove_by_pattern(&self, _pattern: &str) {
        tracing::warn!("remove_by_pattern is not fully supported with moka cache");
    }

    /// Get cached value or compute and store it. This is the primary caching method.
    pub async fn get_or_set<T, F, Fut>(
        &self,
        key: &str,
        f: F,
        ttl: Duration,
    ) -> Result<T, AppError>
    where
        T: Serialize + DeserializeOwned,
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, AppError>>,
    {
        if let Some(cached) = self.get::<T>(key, ttl).await {
            tracing::debug!("Cache hit for key: {}", key);
            return Ok(cached);
        }
        tracing::debug!("Cache miss for key: {}", key);
        let value = f().await?;
        self.set(key, &value, ttl).await;
        Ok(value)
    }
}
