//! # Python Embedding with PyO3
//!
//! Safe Python interpreter embedding in Rust applications.

pub mod embed;
pub mod call;
pub mod sandbox;

pub use embed::*;
pub use call::*;
pub use sandbox::*;

