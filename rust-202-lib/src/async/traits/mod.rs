//! # Async Traits
//!
//! Demonstrates async methods in traits using both async-trait macro
//! and native async fn in trait (Rust 1.75+).

pub mod async_fn_in_trait;
pub mod async_trait_macro;

pub use async_fn_in_trait::*;
pub use async_trait_macro::*;
