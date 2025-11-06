//! # Async Trait Macro
//!
//! Demonstrates the async-trait crate for compatibility and dyn async traits.

use async_trait::async_trait;

/// Cache trait using async-trait macro
///
/// ## Why?
/// The async-trait macro provides a workaround for async in trait objects (dyn Trait).
/// Useful for dynamic dispatch scenarios where you need trait objects.
///
/// ## Trade-off
/// - ✅ Works with dyn Trait
/// - ❌ Adds Box allocation overhead
/// - ⚠️  Use native async fn in trait when possible
///
/// ## Example
/// ```rust
/// # async fn example() {
/// use rust_202::r#async::traits::{Cache, MemoryCache};
///
/// let mut cache = MemoryCache::new();
/// cache.set("key1".to_string(), "value1".to_string()).await;
/// assert_eq!(cache.get("key1").await, Some("value1".to_string()));
/// # }
/// ```
#[async_trait]
pub trait Cache: Send + Sync {
    /// Get value from cache
    async fn get(&self, key: &str) -> Option<String>;

    /// Set value in cache
    async fn set(&mut self, key: String, value: String);

    /// Delete value from cache
    async fn delete(&mut self, key: &str) -> bool;
}

/// In-memory cache implementation
pub struct MemoryCache {
    data: std::collections::HashMap<String, String>,
}

impl MemoryCache {
    /// Create a new memory cache
    pub fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
        }
    }
}

impl Default for MemoryCache {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Cache for MemoryCache {
    async fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    async fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    async fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }
}

/// Demonstrates dyn async trait usage
///
/// ## Example
/// ```rust
/// # async fn example() {
/// use rust_202::r#async::traits::{use_cache_dyn, MemoryCache};
///
/// let cache = MemoryCache::new();
/// use_cache_dyn(Box::new(cache)).await;
/// # }
/// ```
pub async fn use_cache_dyn(mut cache: Box<dyn Cache>) {
    cache
        .set("dynamic".to_string(), "dispatch".to_string())
        .await;
    let value = cache.get("dynamic").await;
    assert_eq!(value, Some("dispatch".to_string()));
}

#[cfg(all(test, feature = "async-tokio"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_cache() {
        let mut cache = MemoryCache::new();

        assert!(cache.get("test").await.is_none());

        cache.set("test".to_string(), "value".to_string()).await;
        assert_eq!(cache.get("test").await, Some("value".to_string()));

        assert!(cache.delete("test").await);
        assert!(cache.get("test").await.is_none());
    }

    #[tokio::test]
    async fn test_dyn_cache() {
        let cache = MemoryCache::new();
        use_cache_dyn(Box::new(cache)).await;
    }
}
