//! # Stream Basics
//!
//! Core Stream trait usage and custom stream implementations.

use futures::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Simple counter stream
///
/// ## Why?
/// Streams are async iterators with backpressure. Unlike Go channels
/// (no backpressure) or Python async generators (GC overhead), Rust
/// streams are zero-cost and composable.
///
/// ## Comparison
/// - **Rust**: Zero-cost, compile-time size, backpressure
/// - **Python**: `async for`, GC'd generator
/// - **Go**: Unbuffered channel blocks sender
///
/// ## Example
/// ```rust
/// # async fn example() {
/// use rust_202::r#async::streams::CounterStream;
/// use futures::StreamExt;
///
/// let mut stream = CounterStream::new(5);
/// let values: Vec<u32> = stream.collect().await;
/// assert_eq!(values, vec![0, 1, 2, 3, 4]);
/// # }
/// ```
pub struct CounterStream {
    current: u32,
    max: u32,
}

impl CounterStream {
    /// Create a new counter stream
    pub fn new(max: u32) -> Self {
        Self { current: 0, max }
    }
}

impl Stream for CounterStream {
    type Item = u32;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.current < self.max {
            let value = self.current;
            self.current += 1;
            Poll::Ready(Some(value))
        } else {
            Poll::Ready(None)
        }
    }
}

/// Range stream with step
///
/// ## Example
/// ```rust
/// # async fn example() {
/// use rust_202::r#async::streams::RangeStream;
/// use futures::StreamExt;
///
/// let stream = RangeStream::new(0, 10, 2);
/// let values: Vec<i32> = stream.collect().await;
/// assert_eq!(values, vec![0, 2, 4, 6, 8]);
/// # }
/// ```
pub struct RangeStream {
    current: i32,
    end: i32,
    step: i32,
}

impl RangeStream {
    /// Create a new range stream
    pub fn new(start: i32, end: i32, step: i32) -> Self {
        Self {
            current: start,
            end,
            step,
        }
    }
}

impl Stream for RangeStream {
    type Item = i32;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.current < self.end {
            let value = self.current;
            self.current += self.step;
            Poll::Ready(Some(value))
        } else {
            Poll::Ready(None)
        }
    }
}

/// Fibonacci stream
///
/// ## Why?
/// Demonstrates stateful stream. Each poll advances the internal state
/// without heap allocation (unlike Python generators).
///
/// ## Example
/// ```rust
/// # async fn example() {
/// use rust_202::r#async::streams::FibonacciStream;
/// use futures::StreamExt;
///
/// let stream = FibonacciStream::new();
/// let values: Vec<u64> = stream.take(6).collect().await;
/// assert_eq!(values, vec![0, 1, 1, 2, 3, 5]);
/// # }
/// ```
pub struct FibonacciStream {
    curr: u64,
    next: u64,
}

impl FibonacciStream {
    /// Create a new Fibonacci stream
    pub fn new() -> Self {
        Self { curr: 0, next: 1 }
    }
}

impl Default for FibonacciStream {
    fn default() -> Self {
        Self::new()
    }
}

impl Stream for FibonacciStream {
    type Item = u64;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current.saturating_add(self.next);
        Poll::Ready(Some(current))
    }
}

#[cfg(all(test, feature = "async-tokio"))]
mod tests {
    use super::*;
    use futures::StreamExt;

    #[tokio::test]
    async fn test_counter_stream() {
        let stream = CounterStream::new(5);
        let values: Vec<u32> = stream.collect().await;
        assert_eq!(values, vec![0, 1, 2, 3, 4]);
    }

    #[tokio::test]
    async fn test_range_stream() {
        let stream = RangeStream::new(0, 10, 2);
        let values: Vec<i32> = stream.collect().await;
        assert_eq!(values, vec![0, 2, 4, 6, 8]);
    }

    #[tokio::test]
    async fn test_fibonacci_stream() {
        let stream = FibonacciStream::new();
        let values: Vec<u64> = stream.take(10).collect().await;
        assert_eq!(values, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }
}

