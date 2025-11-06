//! # Domain-Specific Languages
//!
//! This module demonstrates embedding interpreters in Rust for DSL use cases.
//! Primary focus: Python embedding with PyO3 for configuration, plugins, and scripting.
//!
//! ## Why Embed Python in Rust?
//!
//! - **Configuration**: Use Python for complex config (vs TOML/YAML limits)
//! - **Plugins**: User-extensible systems without recompilation
//! - **Data Pipelines**: Leverage Python's data ecosystem
//! - **ML/AI**: Call Python ML models from Rust
//! - **Scripting**: Runtime flexibility with compile-time safety
//!
//! ## Comparison
//!
//! - **vs Pure Rust**: Python adds runtime flexibility
//! - **vs Pure Python**: Rust provides performance + safety
//! - **vs Go**: Go has no mature Python embedding
//! - **vs C**: Rust's PyO3 is safer, higher-level
//!
//! ## Example
//!
//! ```rust,no_run
//! # #[cfg(feature = "python-dsl")]
//! # fn example() -> pyo3::PyResult<()> {
//! use rust_202::dsl::python::eval_simple;
//!
//! let result = eval_simple("2 + 2")?;
//! println!("Python says: {}", result);
//! # Ok(())
//! # }
//! ```

pub mod python;

pub use python::*;

