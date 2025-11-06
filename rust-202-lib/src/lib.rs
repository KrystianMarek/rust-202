//! # Rust-202: Advanced Rust Concepts Library
//!
//! `rust-202` is an interactive, example-driven cheat sheet for advanced Rust concepts.
//! It covers Rust 1.75+ features, trait-based OOP paradigms, functional programming,
//! Gang of Four (GoF) design patterns adapted to Rust, and Rust-specific idioms.
//!
//! ## For Rust Beginners
//!
//! This library is organized into modules that demonstrate different aspects of Rust:
//!
//! - **Start with `idioms`** - Core Rust concepts (ownership, borrowing, error handling)
//! - **Then `oop`** - How to do object-oriented programming without inheritance
//! - **Next `functional`** - Rust's zero-cost functional programming
//! - **Advanced: `async`** - Modern async/await patterns
//! - **Expert: `dsl`** - Embedding other languages in Rust
//!
//! ## Quick Start
//!
//! ```rust
//! use rust_202::oop::composition::{CompositionExample, Circle};
//! use rust_202::functional::iterators::fibonacci_sequence;
//! use rust_202::idioms::error_handling::ResultExample;
//!
//! // Trait-based composition (Rust has no classes/inheritance!)
//! let circle = Circle::new(5.0);
//! let example = CompositionExample::new(Box::new(circle), "Red");
//! // Box<T> = heap allocation, like Java's 'new' but explicit
//!
//! // Functional programming with iterators (zero-cost!)
//! let fibs: Vec<u64> = fibonacci_sequence().take(10).collect();
//! // This compiles to the same code as a manual loop!
//!
//! // Idiomatic error handling (no exceptions in Rust!)
//! let result = ResultExample::safe_divide(10, 2);
//! // Returns Result<i32, String> - must handle errors explicitly
//! ```
//!
//! ## Modules
//!
//! - [`rust_191`] - Rust 1.75+ features (const, atomics, lifetimes)
//! - [`oop`] - Trait-based OOP (no inheritance, yes polymorphism!)
//! - [`functional`] - Functional programming (zero-cost abstractions)
//! - [`idioms`] - Rust-specific patterns (ownership, borrowing, RAII)
//! - [`differentiators`] - What makes Rust special vs Python/Go/C
//! - [`async`] - Modern async/await (zero-cost futures)
//! - [`dsl`] - Embedding Python safely in Rust

// Compiler directives (lints):
// #![warn(...)] tells the compiler to warn about certain code patterns
#![warn(missing_docs)]  // Warn if public items lack documentation
#![warn(clippy::all)]   // Enable all Clippy lints for code quality

// === MODULE DECLARATIONS ===
// In Rust, modules are declared with 'pub mod' to make them public.
// Each module corresponds to a directory or file in src/

/// Rust 1.75+ specific features and stabilizations
///
/// **For Beginners**: Start here to see modern Rust capabilities like
/// compile-time computation and atomic operations.
pub mod rust_191;

/// Object-oriented programming using traits and composition
///
/// **For Beginners**: Rust has no classes! Learn how traits provide polymorphism
/// and composition replaces inheritance.
pub mod oop;

/// Functional programming patterns in Rust
///
/// **For Beginners**: Rust's iterators are zero-cost abstractions - they compile
/// to the same code as manual loops but are more expressive.
pub mod functional;

/// Rust-specific idioms and best practices
///
/// **For Beginners**: START HERE! Learn ownership, borrowing, and error handling -
/// the core concepts that make Rust unique.
pub mod idioms;

/// Features that differentiate Rust from Python, Go, and C
///
/// **For Beginners**: See why Rust is special - memory safety without GC,
/// fearless concurrency, and zero-cost abstractions.
pub mod differentiators;

/// Advanced async/await patterns and concurrent programming
///
/// **For Beginners**: This is advanced! Learn basic Rust first, then come back
/// to see how async/await works without a runtime or garbage collector.
///
/// **Note**: Only available with the `async-tokio` feature flag
#[cfg(feature = "async-tokio")]  // Conditional compilation - only include if feature enabled
pub mod r#async;  // r#async because 'async' is a keyword, r# escapes it

/// Domain-Specific Languages: Python embedding and interop
///
/// **For Beginners**: Advanced topic! Shows how Rust can safely embed other
/// languages like Python, combining Rust's safety with Python's flexibility.
///
/// **Note**: Only available with the `python-dsl` feature flag
#[cfg(feature = "python-dsl")]  // Conditional compilation
pub mod dsl;

// === RE-EXPORTS ===
// Re-exporting allows users to access items directly from the crate root
// instead of having to know the full path.
//
// For example, instead of: use rust_202::functional::iterators::fibonacci_sequence;
// You can write: use rust_202::fibonacci_sequence;

/// Re-export: Fibonacci sequence iterator
///
/// **Beginner Note**: This demonstrates Rust's iterator pattern.
/// Iterators are lazy (don't compute until needed) and zero-cost
/// (compile to the same code as manual loops).
pub use functional::iterators::fibonacci_sequence;

/// Re-export: Error handling examples
///
/// **Beginner Note**: Rust has no exceptions! Instead, functions return
/// Result<T, E> to force explicit error handling at compile-time.
pub use idioms::error_handling::ResultExample;

/// Re-export: Composition example
///
/// **Beginner Note**: Rust has no inheritance! This shows how composition
/// (embedding structs) and traits (interfaces) replace class hierarchies.
pub use oop::composition::CompositionExample;
