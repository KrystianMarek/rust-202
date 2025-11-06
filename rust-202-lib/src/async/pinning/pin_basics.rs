//! # Pin Basics
//!
//! Core pinning concepts for safe self-referential structures.

use pin_project_lite::pin_project;
use std::marker::PhantomPinned;
use std::pin::Pin;

/// Demonstrates Pin<Box<T>>
///
/// ## Why?
/// Pin prevents moving of self-referential structures, which is crucial
/// for async code. Unlike C (manual management) or Go (GC moves objects),
/// Rust's Pin provides compile-time safety.
///
/// ## Example
/// ```rust
/// use rust_202::r#async::pinning::pinned_value;
///
/// let pinned = pinned_value(42);
/// assert_eq!(*pinned, 42);
/// ```
pub fn pinned_value(value: i32) -> Pin<Box<i32>> {
    Box::pin(value)
}

pin_project! {
    /// Self-referential structure example
    ///
    /// ## Why?
    /// Demonstrates safe self-references using pin-project. This pattern
    /// is common in async iterators and generators.
    pub struct SelfReferential {
        value: String,
        #[pin]
        _marker: PhantomPinned,
    }
}

impl SelfReferential {
    /// Create a new self-referential structure
    pub fn new(value: String) -> Pin<Box<Self>> {
        Box::pin(Self {
            value,
            _marker: PhantomPinned,
        })
    }

    /// Get the value
    pub fn value(self: Pin<&Self>) -> &str {
        &self.get_ref().value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pinned_value() {
        let pinned = pinned_value(100);
        assert_eq!(*pinned, 100);
    }

    #[test]
    fn test_self_referential() {
        let data = SelfReferential::new("test".to_string());
        assert_eq!(data.as_ref().value(), "test");
    }
}
