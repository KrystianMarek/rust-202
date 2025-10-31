//! # Stream Combinators
//!
//! Demonstrates stream transformation and composition.

#[cfg(feature = "async-tokio")]
use futures::stream::{self, Stream, StreamExt};

/// Map combinator example
///
/// ## Why?
/// Stream combinators compose without allocating intermediate collections.
/// Unlike Python list comprehensions (eager) or Go channels (require goroutines),
/// Rust streams are lazy and zero-cost.
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # async fn example() {
/// use rust_202::r#async::streams::map_stream_example;
///
/// let result = map_stream_example().await;
/// assert_eq!(result, vec![0, 2, 4, 6, 8]);
/// # }
/// ```
#[cfg(feature = "async-tokio")]
pub async fn map_stream_example() -> Vec<i32> {
    stream::iter(0..5)
        .map(|x| x * 2)
        .collect()
        .await
}

/// Filter combinator example
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # async fn example() {
/// use rust_202::r#async::streams::filter_stream_example;
///
/// let result = filter_stream_example().await;
/// assert_eq!(result, vec![0, 2, 4]);
/// # }
/// ```
#[cfg(feature = "async-tokio")]
pub async fn filter_stream_example() -> Vec<i32> {
    stream::iter(0..6)
        .filter(|&x| async move { x % 2 == 0 })
        .collect()
        .await
}

/// Chain combinator example
///
/// ## Why?
/// Demonstrates combining multiple streams. No heap allocation for
/// the combinator itself (unlike Python itertools.chain).
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # async fn example() {
/// use rust_202::r#async::streams::chain_stream_example;
///
/// let result = chain_stream_example().await;
/// assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
/// # }
/// ```
#[cfg(feature = "async-tokio")]
pub async fn chain_stream_example() -> Vec<i32> {
    let stream1 = stream::iter(vec![1, 2, 3]);
    let stream2 = stream::iter(vec![4, 5, 6]);

    stream1.chain(stream2).collect().await
}

/// Take/skip combinators
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # async fn example() {
/// use rust_202::r#async::streams::take_skip_example;
///
/// let result = take_skip_example().await;
/// assert_eq!(result, vec![3, 4, 5]);
/// # }
/// ```
#[cfg(feature = "async-tokio")]
pub async fn take_skip_example() -> Vec<i32> {
    stream::iter(0..10)
        .skip(3)
        .take(3)
        .collect()
        .await
}

/// Fold combinator (reduce)
///
/// ## Why?
/// Async reduction with zero intermediate allocations.
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # async fn example() {
/// use rust_202::r#async::streams::fold_stream_example;
///
/// let result = fold_stream_example().await;
/// assert_eq!(result, 45);
/// # }
/// ```
#[cfg(feature = "async-tokio")]
pub async fn fold_stream_example() -> i32 {
    stream::iter(0..10)
        .fold(0, |acc, x| async move { acc + x })
        .await
}

#[cfg(all(test, feature = "async-tokio"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_map_stream() {
        let result = map_stream_example().await;
        assert_eq!(result, vec![0, 2, 4, 6, 8]);
    }

    #[tokio::test]
    async fn test_filter_stream() {
        let result = filter_stream_example().await;
        assert_eq!(result, vec![0, 2, 4]);
    }

    #[tokio::test]
    async fn test_chain_stream() {
        let result = chain_stream_example().await;
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[tokio::test]
    async fn test_take_skip() {
        let result = take_skip_example().await;
        assert_eq!(result, vec![3, 4, 5]);
    }

    #[tokio::test]
    async fn test_fold() {
        let result = fold_stream_example().await;
        assert_eq!(result, 45);
    }
}

