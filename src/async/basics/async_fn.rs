//! # Async Functions and .await
//!
//! Demonstrates basic async fn usage, .await operator, and Send bounds.

use std::time::Duration;

#[cfg(feature = "async-tokio")]
use tokio::time::sleep;

/// Simple async function
///
/// ## Why?
/// `async fn` transforms the function into a state machine at compile-time,
/// with zero runtime overhead. Unlike Python's async def (GC'd closures)
/// or Go's goroutines (runtime-managed stacks), Rust futures are zero-cost.
///
/// ## Comparison
/// - **Rust**: Compile-time state machine, zero heap allocation
/// - **Python**: Runtime coroutine with GC overhead
/// - **Go**: 2KB+ stack per goroutine
/// - **C**: Manual state machine hell
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # #[tokio::main]
/// # async fn main() {
/// use rust_202::r#async::basics::async_hello;
///
/// let result = async_hello("World").await;
/// assert_eq!(result, "Hello, World!");
/// # }
/// ```
pub async fn async_hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Async function with delay
///
/// ## Why?
/// Demonstrates non-blocking sleep. The runtime can schedule other tasks
/// during the sleep period, unlike blocking sleep which holds the thread.
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # #[tokio::main]
/// # async fn main() {
/// use rust_202::r#async::basics::async_delay_hello;
///
/// let result = async_delay_hello("Rust", 100).await;
/// assert!(result.contains("Rust"));
/// # }
/// ```
#[cfg(feature = "async-tokio")]
pub async fn async_delay_hello(name: &str, delay_ms: u64) -> String {
    sleep(Duration::from_millis(delay_ms)).await;
    format!("Hello after {}ms, {}!", delay_ms, name)
}

/// Async function with Send bound
///
/// ## Why?
/// Send bound ensures the future can be safely moved between threads.
/// This is enforced at compile-time, unlike Go's runtime race detector.
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # #[tokio::main]
/// # async fn main() {
/// use rust_202::r#async::basics::send_async_task;
///
/// let result = send_async_task(42).await;
/// assert_eq!(result, 84);
/// # }
/// ```
#[cfg(feature = "async-tokio")]
pub async fn send_async_task(value: i32) -> i32
where
    // Explicit Send bound for multi-threaded runtime
{
    tokio::task::yield_now().await; // Yield to scheduler
    value * 2
}

/// Demonstrate chaining async operations
///
/// ## Why?
/// Shows composability of async functions. Each .await is a suspension point
/// where the runtime can schedule other work.
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # #[tokio::main]
/// # async fn main() {
/// use rust_202::r#async::basics::chain_async_ops;
///
/// let result = chain_async_ops(5).await;
/// assert_eq!(result, "Result: 10");
/// # }
/// ```
pub async fn chain_async_ops(value: i32) -> String {
    let doubled = async_double(value).await;
    let message = async_format(doubled).await;
    message
}

async fn async_double(x: i32) -> i32 {
    x * 2
}

async fn async_format(x: i32) -> String {
    format!("Result: {}", x)
}

/// Async error handling
///
/// ## Why?
/// Demonstrates Result with async. The ? operator works seamlessly
/// with async functions, unlike Python's exception handling or Go's
/// explicit error returns.
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # #[tokio::main]
/// # async fn main() {
/// use rust_202::r#async::basics::async_divide;
///
/// assert!(async_divide(10, 2).await.is_ok());
/// assert!(async_divide(10, 0).await.is_err());
/// # }
/// ```
pub async fn async_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

#[cfg(all(test, feature = "async-tokio"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_hello() {
        let result = async_hello("Test").await;
        assert_eq!(result, "Hello, Test!");
    }

    #[tokio::test]
    async fn test_async_delay() {
        let result = async_delay_hello("Test", 10).await;
        assert!(result.contains("Test"));
    }

    #[tokio::test]
    async fn test_send_task() {
        let result = send_async_task(21).await;
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_chain_ops() {
        let result = chain_async_ops(10).await;
        assert_eq!(result, "Result: 20");
    }

    #[tokio::test]
    async fn test_async_divide() {
        assert_eq!(async_divide(10, 2).await, Ok(5));
        assert!(async_divide(10, 0).await.is_err());
    }
}
