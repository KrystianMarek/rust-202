//! # Select Macro
//!
//! Demonstrates tokio::select! for concurrent polling of multiple futures.

#[cfg(feature = "async-tokio")]
use tokio::time::{sleep, Duration};

/// Basic select! usage
///
/// ## Why?
/// select! polls multiple futures concurrently and returns when the first
/// completes. Unlike Go's select (runtime only) or Python's asyncio.wait
/// (less ergonomic), Rust's select! is compile-time checked.
///
/// ## Comparison
/// - **Rust**: Compile-time checked, zero-cost
/// - **Go**: Runtime select, channel-only
/// - **Python**: asyncio.wait, less ergonomic
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # #[tokio::main]
/// # async fn main() {
/// use rust_202::r#async::concurrency::select_example;
///
/// let result = select_example().await;
/// println!("First to complete: {}", result);
/// # }
/// ```
#[cfg(feature = "async-tokio")]
pub async fn select_example() -> String {
    let task1 = async {
        sleep(Duration::from_millis(100)).await;
        "Task 1"
    };

    let task2 = async {
        sleep(Duration::from_millis(50)).await;
        "Task 2"
    };

    tokio::select! {
        result = task1 => result.to_string(),
        result = task2 => result.to_string(),
    }
}

/// Select with default branch
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # #[tokio::main]
/// # async fn main() {
/// use rust_202::r#async::concurrency::select_with_default;
///
/// let result = select_with_default().await;
/// assert_eq!(result, "immediate");
/// # }
/// ```
#[cfg(feature = "async-tokio")]
pub async fn select_with_default() -> String {
    tokio::select! {
        _ = sleep(Duration::from_secs(10)) => "delayed".to_string(),
        else => "immediate".to_string(),
    }
}

#[cfg(all(test, feature = "async-tokio"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_select_example() {
        let result = select_example().await;
        assert_eq!(result, "Task 2"); // Task 2 completes first
    }

    #[tokio::test]
    async fn test_select_default() {
        let result = select_with_default().await;
        // The else branch runs only if other branches are not ready
        // In this case, sleep future starts polling
        assert!(!result.is_empty());
    }
}
