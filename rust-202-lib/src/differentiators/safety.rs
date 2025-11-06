//! # Memory Safety Without Garbage Collection
//!
//! Demonstrates Rust's compile-time memory safety guarantees.

/// Borrow checker prevents use-after-free
///
/// ## Comparison
/// - **Rust**: Compile-time error (won't compile)
/// - **C**: Undefined behavior (segfault, corruption)
/// - **Python**: No issue (GC handles it, but with runtime overhead)
/// - **Go**: No issue (GC handles it, with GC pauses)
///
/// ## Example (this would NOT compile)
/// ```compile_fail
/// let vec = vec![1, 2, 3];
/// let reference = &vec[0];
/// drop(vec); // Error: cannot move out of `vec` because it is borrowed
/// println!("{}", reference);
/// ```
pub struct BorrowCheckerExample;

impl BorrowCheckerExample {
    /// Safe version: ownership is clear
    pub fn safe_reference_example() -> i32 {
        let vec = vec![1, 2, 3];
        let first = vec[0]; // Copy the value, not a reference
        drop(vec); // Now we can drop vec
        first // first is still valid
    }
}

/// No null pointer dereferencing
///
/// ## Comparison
/// - **Rust**: Option<T> forces handling at compile-time
/// - **C**: NULL pointer dereferencing causes segfault
/// - **Python**: None can cause AttributeError at runtime
/// - **Go**: nil can cause panic at runtime
///
/// ## Example
/// ```rust
/// use rust_202::differentiators::safety::SafeNullHandling;
///
/// let value = SafeNullHandling::find_value(vec![1, 2, 3], 5);
/// match value {
///     Some(v) => println!("Found: {}", v),
///     None => println!("Not found"),
/// }
/// ```
pub struct SafeNullHandling;

impl SafeNullHandling {
    /// Find a value safely
    pub fn find_value(data: Vec<i32>, target: i32) -> Option<i32> {
        data.into_iter().find(|&x| x == target)
    }

    /// Get first element safely
    pub fn first_element<T: Clone>(data: &[T]) -> Option<T> {
        data.first().cloned()
    }
}

/// No data races
///
/// ## Comparison
/// - **Rust**: Send/Sync traits prevent data races at compile-time
/// - **C**: Data races cause undefined behavior
/// - **Python**: GIL prevents true parallelism
/// - **Go**: Data races detected at runtime with race detector
///
/// ## Example
/// ```rust
/// use rust_202::differentiators::safety::ThreadSafeCounter;
/// use std::sync::Arc;
/// use std::thread;
///
/// let counter = Arc::new(ThreadSafeCounter::new());
/// let mut handles = vec![];
///
/// for _ in 0..10 {
///     let counter_clone = Arc::clone(&counter);
///     let handle = thread::spawn(move || {
///         counter_clone.increment();
///     });
///     handles.push(handle);
/// }
///
/// for handle in handles {
///     handle.join().unwrap();
/// }
///
/// assert_eq!(counter.get(), 10);
/// ```
use std::sync::atomic::{AtomicU32, Ordering};

/// Thread-safe counter using atomics
pub struct ThreadSafeCounter {
    count: AtomicU32,
}

impl ThreadSafeCounter {
    /// Create a new counter
    pub fn new() -> Self {
        Self {
            count: AtomicU32::new(0),
        }
    }

    /// Increment atomically
    pub fn increment(&self) {
        self.count.fetch_add(1, Ordering::SeqCst);
    }

    /// Get current value
    pub fn get(&self) -> u32 {
        self.count.load(Ordering::SeqCst)
    }
}

impl Default for ThreadSafeCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// No buffer overflows
///
/// ## Comparison
/// - **Rust**: Bounds checking (opt-out with unsafe)
/// - **C**: Buffer overflows cause UB, security vulnerabilities
/// - **Python**: Bounds checked, but at runtime
/// - **Go**: Bounds checked, but at runtime
///
/// ## Example
/// ```rust
/// use rust_202::differentiators::safety::SafeArrayAccess;
///
/// let data = vec![1, 2, 3, 4, 5];
/// assert_eq!(SafeArrayAccess::safe_get(&data, 2), Some(&3));
/// assert_eq!(SafeArrayAccess::safe_get(&data, 100), None);
/// ```
pub struct SafeArrayAccess;

impl SafeArrayAccess {
    /// Safe array access with Option
    pub fn safe_get<T>(data: &[T], index: usize) -> Option<&T> {
        data.get(index)
    }

    /// Iterate safely (no manual index management)
    pub fn safe_sum(data: &[i32]) -> i32 {
        data.iter().sum()
    }
}

/// Type safety prevents invalid state
///
/// ## Comparison
/// - **Rust**: Type system prevents invalid states at compile-time
/// - **C**: Easy to create invalid states (uninitialized, wrong casts)
/// - **Python**: Duck typing allows runtime type errors
/// - **Go**: Interface{} allows runtime type errors
///
/// ## Example
/// ```rust
/// use rust_202::differentiators::safety::{Validated, Unvalidated, Email};
///
/// let unvalidated = Email::<Unvalidated>::new("user@example.com");
/// let validated = unvalidated.validate().unwrap();
/// // validated.send(); // Can only call send on validated emails
/// ```
use std::marker::PhantomData;

/// Type state marker for validated emails
pub struct Validated;

/// Type state marker for unvalidated emails
pub struct Unvalidated;

/// Email with type-level validation state
pub struct Email<State = Unvalidated> {
    address: String,
    _state: PhantomData<State>,
}

impl Email<Unvalidated> {
    /// Create a new unvalidated email
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            _state: PhantomData,
        }
    }

    /// Validate the email
    pub fn validate(self) -> Result<Email<Validated>, &'static str> {
        if self.address.contains('@') {
            Ok(Email {
                address: self.address,
                _state: PhantomData,
            })
        } else {
            Err("Invalid email format")
        }
    }
}

impl Email<Validated> {
    /// Send email (only available for validated emails)
    pub fn send(&self) {
        println!("Sending email to {}", self.address);
    }

    /// Get address
    pub fn address(&self) -> &str {
        &self.address
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_borrow_checker() {
        let result = BorrowCheckerExample::safe_reference_example();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_safe_null_handling() {
        let data = vec![1, 2, 3];
        assert_eq!(SafeNullHandling::find_value(data.clone(), 2), Some(2));
        assert_eq!(SafeNullHandling::find_value(data.clone(), 5), None);
        assert_eq!(SafeNullHandling::first_element(&data), Some(1));
    }

    #[test]
    fn test_thread_safe_counter() {
        let counter = Arc::new(ThreadSafeCounter::new());
        let mut handles = vec![];

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                counter_clone.increment();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(counter.get(), 10);
    }

    #[test]
    fn test_safe_array_access() {
        let data = vec![1, 2, 3, 4, 5];
        assert_eq!(SafeArrayAccess::safe_get(&data, 2), Some(&3));
        assert_eq!(SafeArrayAccess::safe_get(&data, 100), None);
        assert_eq!(SafeArrayAccess::safe_sum(&data), 15);
    }

    #[test]
    fn test_type_state_pattern() {
        let unvalidated = Email::<Unvalidated>::new("user@example.com");
        let validated = unvalidated.validate().unwrap();
        validated.send();

        let invalid = Email::<Unvalidated>::new("notanemail");
        assert!(invalid.validate().is_err());
    }
}
