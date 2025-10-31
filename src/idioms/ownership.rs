//! # Ownership and Borrowing Patterns
//!
//! Demonstrates Rust's ownership system and common borrowing patterns.

/// RAII (Resource Acquisition Is Initialization) example
///
/// ## Why?
/// RAII in Rust guarantees resource cleanup without garbage collection.
/// Unlike Python's `__del__` (unreliable timing) or C's manual free
/// (error-prone), Rust's Drop trait is deterministic and automatic.
///
/// ## Example
/// ```rust
/// use rust_202::idioms::ownership::FileHandle;
///
/// {
///     let handle = FileHandle::open("data.txt");
///     handle.write("Hello");
/// } // File automatically closed here via Drop
/// ```
pub struct FileHandle {
    filename: String,
}

impl FileHandle {
    /// Open a file
    pub fn open(filename: &str) -> Self {
        println!("Opening file: {}", filename);
        Self {
            filename: filename.to_string(),
        }
    }

    /// Write to the file
    pub fn write(&self, data: &str) {
        println!("Writing to {}: {}", self.filename, data);
    }
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        println!("Closing file: {}", self.filename);
    }
}

/// Smart pointer example with automatic cleanup
///
/// ## Example
/// ```rust
/// use rust_202::idioms::ownership::ConnectionPool;
///
/// let pool = ConnectionPool::new(5);
/// let conn = pool.acquire();
/// conn.execute("SELECT * FROM users");
/// // Connection automatically returned to pool when dropped
/// ```
pub struct ConnectionPool {
    size: usize,
}

impl ConnectionPool {
    /// Create a new connection pool
    pub fn new(size: usize) -> Self {
        Self { size }
    }

    /// Acquire a connection
    pub fn acquire(&self) -> PooledConnection {
        PooledConnection { id: self.size }
    }
}

/// A connection that returns to pool on drop
pub struct PooledConnection {
    id: usize,
}

impl PooledConnection {
    /// Execute a query
    pub fn execute(&self, query: &str) {
        println!("Executing query on connection {}: {}", self.id, query);
    }
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        println!("Returning connection {} to pool", self.id);
    }
}

/// Move semantics example
///
/// ## Why?
/// Move semantics transfer ownership without copying, enabling zero-cost
/// resource management. Unlike Python's reference counting or Go's copying,
/// Rust moves are truly zero-cost.
pub struct MoveSemantics;

impl MoveSemantics {
    /// Example demonstrating move
    ///
    /// ## Example
    /// ```rust
    /// use rust_202::idioms::ownership::MoveSemantics;
    ///
    /// let result = MoveSemantics::transfer_ownership(vec![1, 2, 3]);
    /// assert_eq!(result, vec![1, 2, 3, 4]);
    /// ```
    pub fn transfer_ownership(mut data: Vec<i32>) -> Vec<i32> {
        // `data` was moved into this function
        data.push(4);
        data // Ownership transferred to caller
    }

    /// Borrowing example
    pub fn borrow_immutably(data: &[i32]) -> i32 {
        data.iter().sum()
    }

    /// Mutable borrowing example
    pub fn borrow_mutably(data: &mut Vec<i32>) {
        data.push(100);
    }
}

/// Interior mutability with Cell/RefCell pattern
///
/// ## Why?
/// Interior mutability allows mutation through shared references when
/// borrow checker rules are too restrictive. Unlike C's const_cast
/// (unsafe) or Go's no compile-time safety, Rust's RefCell provides
/// runtime-checked interior mutability.
///
/// ## Example
/// ```rust
/// use rust_202::idioms::ownership::Counter;
///
/// let counter = Counter::new();
/// counter.increment();
/// counter.increment();
/// assert_eq!(counter.get(), 2);
/// ```
use std::cell::Cell;

/// A counter with interior mutability using Cell
pub struct Counter {
    count: Cell<u32>,
}

impl Counter {
    /// Create a new counter
    pub fn new() -> Self {
        Self {
            count: Cell::new(0),
        }
    }

    /// Increment the counter
    pub fn increment(&self) {
        let current = self.count.get();
        self.count.set(current + 1);
    }

    /// Get current count
    pub fn get(&self) -> u32 {
        self.count.get()
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

/// Cow (Clone on Write) pattern
///
/// ## Why?
/// Cow allows avoiding clones when data isn't modified, optimizing
/// for the common case. This is more efficient than Python's
/// copy-on-write or Go's []byte vs string conversions.
///
/// ## Example
/// ```rust
/// use rust_202::idioms::ownership::process_string;
///
/// let s = "hello";
/// let result = process_string(s.into(), false);
/// // No clone occurred
/// ```
use std::borrow::Cow;

/// Process a string, converting to uppercase if specified
///
/// Uses Cow to avoid cloning when no modification is needed.
pub fn process_string(data: Cow<str>, uppercase: bool) -> Cow<str> {
    if uppercase {
        // Clone only if needed
        Cow::Owned(data.to_uppercase())
    } else {
        // No clone, just return borrowed data
        data
    }
}

/// Reference counting with Rc/Arc
///
/// ## Example
/// ```rust
/// use rust_202::idioms::ownership::SharedData;
/// use std::rc::Rc;
///
/// let data = SharedData::new(vec![1, 2, 3]);
/// let clone1 = data.clone();
/// let clone2 = data.clone();
/// // All clones share the same data
/// ```
use std::rc::Rc;

/// Wrapper for reference-counted shared data
#[derive(Clone)]
pub struct SharedData<T> {
    data: Rc<T>,
}

impl<T> SharedData<T> {
    /// Create new shared data
    pub fn new(data: T) -> Self {
        Self {
            data: Rc::new(data),
        }
    }

    /// Get a reference to the data
    pub fn get(&self) -> &T {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_handle_raii() {
        {
            let handle = FileHandle::open("test.txt");
            handle.write("test data");
        } // Drop called here
    }

    #[test]
    fn test_connection_pool() {
        let pool = ConnectionPool::new(5);
        {
            let conn = pool.acquire();
            conn.execute("SELECT 1");
        } // Connection returned here
    }

    #[test]
    fn test_move_semantics() {
        let data = vec![1, 2, 3];
        let result = MoveSemantics::transfer_ownership(data);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_borrowing() {
        let data = vec![1, 2, 3];
        let sum = MoveSemantics::borrow_immutably(&data);
        assert_eq!(sum, 6);

        let mut data = vec![1, 2, 3];
        MoveSemantics::borrow_mutably(&mut data);
        assert_eq!(data, vec![1, 2, 3, 100]);
    }

    #[test]
    fn test_counter() {
        let counter = Counter::new();
        counter.increment();
        counter.increment();
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn test_cow() {
        let s = "hello";
        let result = process_string(s.into(), false);
        assert_eq!(result, "hello");

        let result = process_string(s.into(), true);
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_shared_data() {
        let data = SharedData::new(vec![1, 2, 3]);
        let clone = data.clone();
        assert_eq!(data.get(), clone.get());
    }
}
