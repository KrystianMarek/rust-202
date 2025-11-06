//! # Const Atomic Operations
//!
//! Demonstrates compile-time const operations and atomic behaviors
//! that became stable in recent Rust versions.

use std::sync::atomic::{AtomicUsize, Ordering};

/// Compile-time constant computation using const functions
///
/// ## Why?
/// Const functions allow computation at compile-time, eliminating runtime overhead.
/// This is a zero-cost abstraction unique to Rust vs. Python's runtime evaluation.
///
/// ## Example
/// ```rust
/// use rust_202::rust_191::const_atomics::ConstMath;
///
/// const RESULT: u64 = ConstMath::mul_add(5, 10, 3); // Computed at compile-time
/// assert_eq!(RESULT, 53);
/// ```
pub struct ConstMath;

impl ConstMath {
    /// Multiply and add: (a * b) + c
    ///
    /// This is evaluated at compile-time when used in const contexts.
    pub const fn mul_add(a: u64, b: u64, c: u64) -> u64 {
        a.wrapping_mul(b).wrapping_add(c)
    }

    /// Compile-time power calculation
    pub const fn pow(base: u64, exp: u32) -> u64 {
        base.pow(exp)
    }

    /// Const array initialization example
    pub const fn create_lookup_table() -> [u64; 10] {
        let mut table = [0u64; 10];
        let mut i = 0;
        while i < 10 {
            table[i] = Self::pow(2, i as u32);
            i += 1;
        }
        table
    }
}

/// Atomic operations for lock-free concurrency
///
/// ## Why?
/// Atomics provide thread-safe operations without locks, crucial for performance.
/// Unlike Go's mutex-heavy approach or C's manual memory barriers,
/// Rust's atomics are type-safe and prevent data races at compile-time.
///
/// ## Example
/// ```rust
/// use rust_202::rust_191::const_atomics::AtomicCounter;
///
/// let counter = AtomicCounter::new();
/// counter.increment();
/// assert_eq!(counter.get(), 1);
/// ```
pub struct AtomicCounter {
    count: AtomicUsize,
}

impl AtomicCounter {
    /// Create a new atomic counter
    pub const fn new() -> Self {
        Self {
            count: AtomicUsize::new(0),
        }
    }

    /// Increment the counter atomically
    ///
    /// Uses Release ordering to ensure visibility to other threads
    pub fn increment(&self) -> usize {
        self.count.fetch_add(1, Ordering::Release)
    }

    /// Get current value
    ///
    /// Uses Acquire ordering to see updates from other threads
    pub fn get(&self) -> usize {
        self.count.load(Ordering::Acquire)
    }

    /// Compare and swap operation
    ///
    /// Returns true if the swap was successful
    pub fn compare_and_swap(&self, expected: usize, new: usize) -> bool {
        self.count
            .compare_exchange(expected, new, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }
}

impl Default for AtomicCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// Compile-time lookup tables for performance
///
/// ## Benchmark Note
/// This lookup table is initialized at compile-time, unlike Python's runtime dict
/// or Go's runtime map initialization.
pub const POWERS_OF_TWO: [u64; 10] = ConstMath::create_lookup_table();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_mul_add() {
        const RESULT: u64 = ConstMath::mul_add(5, 10, 3);
        assert_eq!(RESULT, 53);
    }

    #[test]
    fn test_const_pow() {
        const RESULT: u64 = ConstMath::pow(2, 10);
        assert_eq!(RESULT, 1024);
    }

    #[test]
    fn test_lookup_table() {
        assert_eq!(POWERS_OF_TWO[0], 1);
        assert_eq!(POWERS_OF_TWO[5], 32);
        assert_eq!(POWERS_OF_TWO[9], 512);
    }

    #[test]
    fn test_atomic_counter() {
        let counter = AtomicCounter::new();
        assert_eq!(counter.get(), 0);

        counter.increment();
        counter.increment();
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn test_compare_and_swap() {
        let counter = AtomicCounter::new();
        assert!(counter.compare_and_swap(0, 5));
        assert_eq!(counter.get(), 5);
        assert!(!counter.compare_and_swap(0, 10));
        assert_eq!(counter.get(), 5);
    }
}
