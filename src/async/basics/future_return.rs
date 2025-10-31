//! # Returning impl Future
//!
//! Demonstrates explicit Future returns without heap allocation.

use std::future::Future;
use std::pin::Pin;

/// Return unboxed future
///
/// ## Why?
/// `-> impl Future` avoids heap allocation (Box). The compiler knows the exact
/// size at compile-time, enabling stack allocation. Unlike Python's coroutines
/// which are always heap-allocated.
///
/// ## Comparison
/// - **Rust**: Stack-allocated future, zero-cost
/// - **Python**: Heap-allocated coroutine with GC
/// - **Go**: Goroutine with minimum 2KB stack
///
/// ## Example
/// ```rust
/// # async fn example() {
/// use rust_202::r#async::basics::unboxed_future;
///
/// let result = unboxed_future(42).await;
/// assert_eq!(result, 84);
/// # }
/// ```
pub async fn unboxed_future(value: i32) -> i32 {
    value * 2
}

/// Return boxed future (when necessary)
///
/// ## Why?
/// Use `Box<dyn Future>` when you need trait objects or dynamic dispatch.
/// This has heap allocation cost but enables runtime polymorphism.
///
/// ## Example
/// ```rust
/// # async fn example() {
/// use rust_202::r#async::basics::boxed_future;
///
/// let result = boxed_future(10).await;
/// assert_eq!(result, 20);
/// # }
/// ```
pub fn boxed_future(value: i32) -> Pin<Box<dyn Future<Output = i32> + Send>> {
    Box::pin(async move { value * 2 })
}

/// Conditional future based on runtime value
///
/// ## Why?
/// Demonstrates returning different future types. Rust's type system
/// ensures both branches return the same type.
///
/// ## Example
/// ```rust
/// # async fn example() {
/// use rust_202::r#async::basics::conditional_future;
///
/// assert_eq!(conditional_future(true, 5).await, 10);
/// assert_eq!(conditional_future(false, 5).await, 5);
/// # }
/// ```
pub async fn conditional_future(double: bool, value: i32) -> i32 {
    if double {
        value * 2
    } else {
        value
    }
}

/// Future with explicit Send + 'static bounds
///
/// ## Why?
/// Explicit bounds ensure the future can be spawned on a multi-threaded
/// runtime and doesn't capture references. Compile-time enforcement
/// prevents data races.
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # #[tokio::main]
/// # async fn main() {
/// use rust_202::r#async::basics::spawnable_future;
///
/// let handle = tokio::spawn(spawnable_future(100));
/// let result = handle.await.unwrap();
/// assert_eq!(result, 200);
/// # }
/// ```
pub async fn spawnable_future(value: i32) -> i32 {
    value * 2
}

#[cfg(all(test, feature = "async-tokio"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_unboxed_future() {
        let result = unboxed_future(21).await;
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_boxed_future() {
        let result = boxed_future(5).await;
        assert_eq!(result, 10);
    }

    #[tokio::test]
    async fn test_conditional_future() {
        assert_eq!(conditional_future(true, 3).await, 6);
        assert_eq!(conditional_future(false, 3).await, 3);
    }

    #[tokio::test]
    async fn test_spawnable() {
        let handle = tokio::spawn(spawnable_future(50));
        let result = handle.await.unwrap();
        assert_eq!(result, 100);
    }
}
