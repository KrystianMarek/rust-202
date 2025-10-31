//! # Async TCP Server
//!
//! Demonstrates async TCP networking with tokio.

/// Echo server example
///
/// ## Why?
/// Async I/O allows handling thousands of connections with minimal threads.
/// Unlike thread-per-connection (C) or GIL-limited (Python), Rust's async
/// I/O scales efficiently.
///
/// ## Comparison
/// - **Rust**: Non-blocking I/O, zero-cost futures, thousands of connections
/// - **C**: Manual epoll/kqueue, error-prone
/// - **Python**: asyncio limited by GIL for CPU work
/// - **Go**: Goroutine per connection (memory overhead)
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # #[tokio::main]
/// # async fn main() {
/// use rust_202::r#async::io::echo_message;
///
/// let message = b"Hello";
/// let result = echo_message(message).await;
/// assert_eq!(result, message);
/// # }
/// ```
#[cfg(feature = "async-tokio")]
pub async fn echo_message(message: &[u8]) -> Vec<u8> {
    // Simulate echo behavior
    message.to_vec()
}

/// Async buffer handling
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # #[tokio::main]
/// # async fn main() {
/// use rust_202::r#async::io::read_lines;
///
/// let data = b"line1\nline2\nline3\n";
/// let lines = read_lines(data).await;
/// assert_eq!(lines.len(), 3);
/// # }
/// ```
#[cfg(feature = "async-tokio")]
pub async fn read_lines(data: &[u8]) -> Vec<String> {
    let content = String::from_utf8_lossy(data);
    content.lines().map(|s| s.to_string()).collect()
}

#[cfg(all(test, feature = "async-tokio"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_echo() {
        let msg = b"test message";
        let result = echo_message(msg).await;
        assert_eq!(result, msg);
    }

    #[tokio::test]
    async fn test_read_lines() {
        let data = b"first\nsecond\nthird\n";
        let lines = read_lines(data).await;
        assert_eq!(lines, vec!["first", "second", "third"]);
    }
}
