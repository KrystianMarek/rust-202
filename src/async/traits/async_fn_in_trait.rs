//! # Async fn in Trait (Native)
//!
//! Demonstrates native async fn in traits stabilized in Rust 1.75+.

/// Repository trait with async methods (native)
///
/// ## Why?
/// Native async fn in traits (stable since 1.75) eliminates the need for
/// async-trait macro, removing heap allocation overhead. Unlike Go interfaces
/// (no async) or Python protocols (no compile-time checking), this provides
/// zero-cost async abstraction with compile-time safety.
///
/// ## Comparison
/// - **Rust**: Zero-cost, compile-time checked, no Box
/// - **Python**: `abc.abstractmethod`, runtime checking only
/// - **Go**: No async in interfaces
///
/// ## Example
/// ```rust
/// # async fn example() {
/// use rust_202::r#async::traits::{Repository, InMemoryRepo};
///
/// let repo = InMemoryRepo::new();
/// let user = repo.find_by_id(1).await;
/// # }
/// ```
#[allow(async_fn_in_trait)]
pub trait Repository {
    /// Find a user by ID
    async fn find_by_id(&self, id: u64) -> Option<String>;

    /// Save a user
    async fn save(&mut self, id: u64, name: String) -> Result<(), String>;
}

/// In-memory repository implementation
pub struct InMemoryRepo {
    data: std::collections::HashMap<u64, String>,
}

impl InMemoryRepo {
    /// Create a new in-memory repository
    pub fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
        }
    }
}

impl Default for InMemoryRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl Repository for InMemoryRepo {
    async fn find_by_id(&self, id: u64) -> Option<String> {
        self.data.get(&id).cloned()
    }

    async fn save(&mut self, id: u64, name: String) -> Result<(), String> {
        self.data.insert(id, name);
        Ok(())
    }
}

/// Service trait demonstrating async with generics
///
/// ## Example
/// ```rust
/// # async fn example() {
/// use rust_202::r#async::traits::{DataService, SimpleService};
///
/// let service = SimpleService;
/// let result = service.process(vec![1, 2, 3]).await;
/// assert_eq!(result, 6);
/// # }
/// ```
#[allow(async_fn_in_trait)]
pub trait DataService<T> {
    /// Process data asynchronously
    async fn process(&self, data: Vec<T>) -> T
    where
        T: std::ops::Add<Output = T> + Default + Clone;
}

/// Simple service implementation
pub struct SimpleService;

impl<T> DataService<T> for SimpleService {
    async fn process(&self, data: Vec<T>) -> T
    where
        T: std::ops::Add<Output = T> + Default + Clone,
    {
        data.into_iter().fold(T::default(), |acc, x| acc + x)
    }
}

#[cfg(all(test, feature = "async-tokio"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_repo() {
        let mut repo = InMemoryRepo::new();

        assert!(repo.find_by_id(1).await.is_none());

        repo.save(1, "Alice".to_string()).await.unwrap();
        assert_eq!(repo.find_by_id(1).await, Some("Alice".to_string()));
    }

    #[tokio::test]
    async fn test_data_service() {
        let service = SimpleService;
        let result = service.process(vec![1, 2, 3, 4, 5]).await;
        assert_eq!(result, 15);
    }
}
