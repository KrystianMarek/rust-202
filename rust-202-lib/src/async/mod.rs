//! # Advanced Async Rust
//!
//! This module showcases cutting-edge async/await patterns using Rust 1.75+ features.
//! It demonstrates production-grade async code with explicit comparisons to Python's
//! asyncio, Go's goroutines, and C's event loops.
//!
//! ## Key Features
//!
//! - **Zero-cost futures**: No heap allocation for simple futures
//! - **Compile-time safety**: Send/Sync bounds enforced at compile-time
//! - **Fearless concurrency**: No data races, no UB
//! - **Composable**: Streams, combinators, and pipelines
//!
//! ## Comparison
//!
//! - **vs Python asyncio**: No GC pauses, compile-time safety
//! - **vs Go goroutines**: Explicit Send bounds, zero-cost abstraction
//! - **vs C event loops**: Memory-safe, no manual state machines
//!
//! ## Example
//!
//! ```rust,no_run
//! # #[cfg(feature = "async-tokio")]
//! # async fn example() {
//! use rust_202::r#async::basics::async_hello;
//!
//! let result = async_hello("World").await;
//! println!("{}", result);
//! # }
//! ```

pub mod basics;
pub mod concurrency;
pub mod io;
pub mod patterns;
pub mod pinning;
pub mod streams;
pub mod traits;

// Re-export commonly used items
pub use basics::*;
pub use streams::*;
pub use traits::*;
