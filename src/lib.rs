//! # Rust-202: Advanced Rust Concepts Library
//!
//! `rust-202` is an interactive, example-driven cheat sheet for advanced Rust concepts.
//! It covers Rust 1.91+ features, trait-based OOP paradigms, functional programming,
//! Gang of Four (GoF) design patterns adapted to Rust, and Rust-specific idioms.
//!
//! ## Quick Start
//!
//! ```rust
//! use rust_202::oop::composition::{CompositionExample, Circle};
//! use rust_202::functional::iterators::fibonacci_sequence;
//! use rust_202::idioms::error_handling::ResultExample;
//!
//! // Use trait-based composition
//! let circle = Circle::new(5.0);
//! let example = CompositionExample::new(Box::new(circle), "Red");
//!
//! // Functional programming with iterators
//! let fibs: Vec<u64> = fibonacci_sequence().take(10).collect();
//!
//! // Idiomatic error handling
//! let result = ResultExample::safe_divide(10, 2);
//! ```
//!
//! ## Modules
//!
//! - [`rust_191`] - Rust 1.91-specific features
//! - [`oop`] - Trait-based object-oriented programming
//! - [`functional`] - Functional programming approaches
//! - [`idioms`] - Rust-specific programming patterns
//! - [`differentiators`] - What sets Rust apart from Python, Go, and C

#![warn(missing_docs)]
#![warn(clippy::all)]

/// Rust 1.91+ specific features and stabilizations
pub mod rust_191;

/// Object-oriented programming using traits and composition
pub mod oop;

/// Functional programming patterns in Rust
pub mod functional;

/// Rust-specific idioms and best practices
pub mod idioms;

/// Features that differentiate Rust from Python, Go, and C
pub mod differentiators;

/// Advanced async/await patterns and concurrent programming
#[cfg(feature = "async-tokio")]
pub mod r#async;

// Re-export commonly used items for convenience
pub use functional::iterators::fibonacci_sequence;
pub use idioms::error_handling::ResultExample;
pub use oop::composition::CompositionExample;
