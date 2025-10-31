//! # Singleton Pattern
//!
//! Thread-safe singleton implementation using `OnceCell`.

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::HashMap;

/// Global configuration singleton
///
/// ## Why?
/// Unlike Python's module-level variables or Go's sync.Once,
/// Rust's `Lazy` provides compile-time guaranteed thread safety.
/// The type system ensures no data races.
///
/// ## Example
/// ```rust
/// use rust_202::oop::patterns::singleton::Config;
///
/// Config::set("api_key", "secret123");
/// assert_eq!(Config::get("api_key"), Some("secret123".to_string()));
/// ```
pub struct Config;

static INSTANCE: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| RwLock::new(HashMap::new()));

impl Config {
    /// Get a configuration value
    pub fn get(key: &str) -> Option<String> {
        INSTANCE.read().get(key).cloned()
    }

    /// Set a configuration value
    pub fn set(key: &str, value: &str) {
        INSTANCE.write().insert(key.to_string(), value.to_string());
    }

    /// Check if a key exists
    pub fn has(key: &str) -> bool {
        INSTANCE.read().contains_key(key)
    }

    /// Clear all configuration
    pub fn clear() {
        INSTANCE.write().clear();
    }
}

/// Database connection pool singleton
///
/// ## Example
/// ```rust
/// use rust_202::oop::patterns::singleton::ConnectionPool;
///
/// let pool = ConnectionPool::instance();
/// assert_eq!(pool.max_connections(), 10);
/// ```
pub struct ConnectionPool {
    max_connections: usize,
}

impl ConnectionPool {
    /// Get the singleton instance
    pub fn instance() -> &'static Self {
        static POOL: Lazy<ConnectionPool> = Lazy::new(|| ConnectionPool {
            max_connections: 10,
        });
        &POOL
    }

    /// Get max connections
    pub fn max_connections(&self) -> usize {
        self.max_connections
    }

    /// Simulate getting a connection
    pub fn get_connection(&self) -> Connection {
        Connection::new(self.max_connections)
    }
}

/// A database connection (example)
#[derive(Debug)]
pub struct Connection {
    id: usize,
}

impl Connection {
    fn new(id: usize) -> Self {
        Self { id }
    }

    /// Get connection ID
    pub fn id(&self) -> usize {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_singleton() {
        Config::clear();
        Config::set("test_key", "test_value");
        assert_eq!(Config::get("test_key"), Some("test_value".to_string()));
        assert!(Config::has("test_key"));
        Config::clear();
        assert!(!Config::has("test_key"));
    }

    #[test]
    fn test_connection_pool() {
        let pool = ConnectionPool::instance();
        assert_eq!(pool.max_connections(), 10);
        let conn = pool.get_connection();
        assert_eq!(conn.id(), 10);
    }
}
