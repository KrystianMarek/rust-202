//! # Async Pipeline Pattern
//!
//! Stream-based data processing pipelines.

#[cfg(feature = "async-tokio")]
use futures::stream::{self, StreamExt};

/// Pipeline with map and filter
///
/// ## Why?
/// Async pipelines compose transformations without buffering entire datasets.
/// Unlike Python generators (GC overhead) or Go channels (no backpressure),
/// Rust streams provide zero-cost composition with backpressure.
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # async fn example() {
/// use rust_202::r#async::patterns::simple_pipeline;
///
/// let result = simple_pipeline(vec![1, 2, 3, 4, 5]).await;
/// assert_eq!(result, vec![4, 16]);
/// # }
/// ```
#[cfg(feature = "async-tokio")]
pub async fn simple_pipeline(numbers: Vec<i32>) -> Vec<i32> {
    stream::iter(numbers)
        .filter(|&x| async move { x % 2 == 0 })
        .map(|x| x * x)
        .collect()
        .await
}

/// Multi-stage pipeline
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "async-tokio")]
/// # async fn example() {
/// use rust_202::r#async::patterns::multi_stage_pipeline;
///
/// let result = multi_stage_pipeline(10).await;
/// assert!(result.len() <= 10);
/// # }
/// ```
#[cfg(feature = "async-tokio")]
pub async fn multi_stage_pipeline(count: usize) -> Vec<String> {
    stream::iter(0..count)
        .map(|x| x * 2)
        .filter(|&x| async move { x > 5 })
        .map(|x| format!("Value: {}", x))
        .collect()
        .await
}

#[cfg(all(test, feature = "async-tokio"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simple_pipeline() {
        let result = simple_pipeline(vec![1, 2, 3, 4, 5, 6]).await;
        assert_eq!(result, vec![4, 16, 36]);
    }

    #[tokio::test]
    async fn test_multi_stage() {
        let result = multi_stage_pipeline(10).await;
        assert!(!result.is_empty());
        assert!(result[0].contains("Value:"));
    }
}
