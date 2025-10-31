//! # Timeout and Cancellation
//!
//! Demonstrates timeout and cancellation patterns in async code.

#[cfg(feature = "async-tokio")]
use tokio::time::{timeout, Duration, sleep};

/// Timeout example
///
/// ## Why?
/// Timeouts prevent indefinite waiting. Rust's timeout is compile-time
/// checked and integrates with Result, unlike Python's asyncio.wait_for
/// (exception-based) or Go's context.WithTimeout (error-prone).
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # #[tokio::main]
/// # async fn main() {
/// use rust_202::r#async::concurrency::timeout_example;
///
/// let result = timeout_example(50).await;
/// assert!(result.is_ok());
///
/// let result = timeout_example(150).await;
/// assert!(result.is_err());
/// # }
/// ```
#[cfg(feature = "async-tokio")]
pub async fn timeout_example(delay_ms: u64) -> Result<String, &'static str> {
    let operation = async {
        sleep(Duration::from_millis(delay_ms)).await;
        "Success".to_string()
    };

    match timeout(Duration::from_millis(100), operation).await {
        Ok(result) => Ok(result),
        Err(_) => Err("Timeout"),
    }
}

/// Retry with exponential backoff
///
/// ## Why?
/// Common pattern for network operations. Rust's async makes this
/// composable and testable.
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # #[tokio::main]
/// # async fn main() {
/// use rust_202::r#async::concurrency::retry_with_backoff;
///
/// let result = retry_with_backoff(3, || async { Ok::<i32, ()>(42) }).await;
/// assert_eq!(result, Ok(42));
/// # }
/// ```
#[cfg(feature = "async-tokio")]
pub async fn retry_with_backoff<F, Fut, T, E>(
    max_attempts: u32,
    mut operation: F,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    let mut attempt = 0;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt >= max_attempts - 1 => return Err(e),
            Err(_) => {
                attempt += 1;
                let delay = 2_u64.pow(attempt);
                sleep(Duration::from_millis(delay * 10)).await;
            }
        }
    }
}

#[cfg(all(test, feature = "async-tokio"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_timeout_success() {
        let result = timeout_example(20).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_timeout_failure() {
        let result = timeout_example(200).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_retry() {
        let result = retry_with_backoff(3, || async { Ok::<_, ()>(100) }).await;
        assert_eq!(result, Ok(100));
    }
}

