//! # Join Handle and Task Spawning
//!
//! Demonstrates tokio::spawn and JoinHandle for concurrent task execution.

#[cfg(feature = "async-tokio")]
use tokio::task::JoinHandle;

/// Spawn concurrent tasks
///
/// ## Why?
/// tokio::spawn creates a new async task (green thread). Unlike OS threads
/// (heavy) or Go goroutines (GC'd), Tokio tasks are zero-cost and type-safe.
///
/// ## Comparison
/// - **Rust**: Zero-cost green threads, compile-time Send checking
/// - **Go**: Goroutines with 2KB+ stack, runtime scheduler
/// - **Python**: asyncio.create_task, single-threaded by default
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # #[tokio::main]
/// # async fn main() {
/// use rust_202::r#async::concurrency::spawn_tasks;
///
/// let sum = spawn_tasks(vec![1, 2, 3, 4, 5]).await;
/// assert_eq!(sum, 15);
/// # }
/// ```
#[cfg(feature = "async-tokio")]
pub async fn spawn_tasks(numbers: Vec<i32>) -> i32 {
    let mut handles = Vec::new();

    for num in numbers {
        let handle: JoinHandle<i32> = tokio::spawn(async move {
            // Simulate some async work
            tokio::task::yield_now().await;
            num
        });
        handles.push(handle);
    }

    let mut sum = 0;
    for handle in handles {
        sum += handle.await.unwrap();
    }

    sum
}

/// Parallel processing example
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # #[tokio::main]
/// # async fn main() {
/// use rust_202::r#async::concurrency::parallel_map;
///
/// let results = parallel_map(vec![1, 2, 3], |x| x * 2).await;
/// assert_eq!(results, vec![2, 4, 6]);
/// # }
/// ```
#[cfg(feature = "async-tokio")]
pub async fn parallel_map<T, F>(items: Vec<T>, f: F) -> Vec<T>
where
    T: Send + 'static,
    F: Fn(T) -> T + Send + Sync + 'static + Copy,
{
    let handles: Vec<_> = items
        .into_iter()
        .map(|item| tokio::spawn(async move { f(item) }))
        .collect();

    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await.unwrap());
    }

    results
}

#[cfg(all(test, feature = "async-tokio"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_spawn_tasks() {
        let sum = spawn_tasks(vec![10, 20, 30]).await;
        assert_eq!(sum, 60);
    }

    #[tokio::test]
    async fn test_parallel_map() {
        let results = parallel_map(vec![1, 2, 3, 4], |x| x * 10).await;
        assert_eq!(results, vec![10, 20, 30, 40]);
    }
}
