//! # Gang of Four Design Patterns in Rust
//!
//! Classic GoF patterns adapted to Rust's ownership model and trait system.

pub mod adapter;
pub mod builder;
pub mod factory;
pub mod observer;
pub mod singleton;
pub mod strategy;

pub use adapter::*;
pub use builder::*;
pub use factory::*;
pub use observer::*;
pub use singleton::*;
pub use strategy::*;
