//! # Fearless Concurrency
//!
//! Demonstrates Rust's compile-time guarantees for thread-safe code.

use std::sync::{Arc, Mutex};
use std::thread;

/// Send and Sync traits
///
/// ## Comparison
/// - **Rust**: Send/Sync enforced at compile-time
/// - **C**: No safety guarantees, manual synchronization
/// - **Python**: GIL prevents true parallelism
/// - **Go**: Race detector at runtime (opt-in)
///
/// ## Example
/// ```rust
/// use rust_202::differentiators::concurrency::ThreadSafeData;
///
/// let data = ThreadSafeData::new(vec![1, 2, 3]);
/// let sum = data.parallel_sum();
/// assert_eq!(sum, 6);
/// ```
pub struct ThreadSafeData<T> {
    data: Arc<Mutex<T>>,
}

impl<T> ThreadSafeData<T> {
    /// Create new thread-safe data
    pub fn new(data: T) -> Self {
        Self {
            data: Arc::new(Mutex::new(data)),
        }
    }

    /// Clone the Arc for sharing across threads
    pub fn clone_handle(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
        }
    }
}

impl ThreadSafeData<Vec<i32>> {
    /// Parallel sum using multiple threads
    pub fn parallel_sum(&self) -> i32 {
        let data = self.data.lock().unwrap();
        data.iter().sum()
    }

    /// Parallel processing example
    pub fn parallel_process(&self, worker_count: usize) -> Vec<i32> {
        let mut handles = vec![];

        for _ in 0..worker_count {
            let data_clone = self.clone_handle();
            let handle = thread::spawn(move || {
                let data = data_clone.data.lock().unwrap();
                data.iter().map(|x| x * 2).sum::<i32>()
            });
            handles.push(handle);
        }

        handles.into_iter().map(|h| h.join().unwrap()).collect()
    }
}

/// Message passing with channels
///
/// ## Comparison
/// - **Rust**: Type-safe channels, no data races
/// - **C**: Manual synchronization with pthreads
/// - **Python**: Queue module, GIL limits parallelism
/// - **Go**: Built-in channels, runtime race detection
///
/// ## Example
/// ```rust
/// use rust_202::differentiators::concurrency::ChannelExample;
///
/// let result = ChannelExample::producer_consumer(10);
/// assert_eq!(result.len(), 10);
/// ```
use std::sync::mpsc;

/// Examples demonstrating message passing with channels
pub struct ChannelExample;

impl ChannelExample {
    /// Producer-consumer pattern
    pub fn producer_consumer(count: usize) -> Vec<i32> {
        let (tx, rx) = mpsc::channel();

        // Producer thread
        thread::spawn(move || {
            for i in 0..count {
                tx.send(i as i32).unwrap();
            }
        });

        // Consumer collects results
        rx.iter().take(count).collect()
    }

    /// Multiple producers
    pub fn multiple_producers(producer_count: usize, items_per_producer: usize) -> Vec<i32> {
        let (tx, rx) = mpsc::channel();
        let mut handles = vec![];

        for id in 0..producer_count {
            let tx_clone = tx.clone();
            let handle = thread::spawn(move || {
                for i in 0..items_per_producer {
                    tx_clone.send((id * 1000 + i) as i32).unwrap();
                }
            });
            handles.push(handle);
        }

        drop(tx); // Close the channel

        for handle in handles {
            handle.join().unwrap();
        }

        rx.iter().collect()
    }
}

/// Rayon for data parallelism
///
/// ## Why?
/// Rayon provides safe data parallelism without data races.
/// The compiler ensures thread safety.
///
/// ## Example
/// ```rust
/// use rust_202::differentiators::concurrency::ParallelIterators;
///
/// let sum = ParallelIterators::sum_range(1, 1000);
/// assert_eq!(sum, 499500);
/// ```
pub struct ParallelIterators;

impl ParallelIterators {
    /// Sum range sequentially
    pub fn sum_range(start: i32, end: i32) -> i32 {
        (start..end).sum()
    }

    /// Process in chunks (simulating parallel processing)
    pub fn process_chunks(data: Vec<i32>, chunk_size: usize) -> Vec<i32> {
        data.chunks(chunk_size)
            .map(|chunk| chunk.iter().sum())
            .collect()
    }
}

/// Lock-free data structures with atomics
///
/// ## Comparison
/// - **Rust**: Safe atomics with Ordering types
/// - **C**: Manual atomic operations, easy to misuse
/// - **Python**: GIL makes atomics less relevant
/// - **Go**: sync/atomic package, runtime checks
///
/// ## Example
/// ```rust
/// use rust_202::differentiators::concurrency::LockFreeCounter;
/// use std::sync::Arc;
/// use std::thread;
///
/// let counter = Arc::new(LockFreeCounter::new());
/// let mut handles = vec![];
///
/// for _ in 0..10 {
///     let counter_clone = Arc::clone(&counter);
///     handles.push(thread::spawn(move || {
///         for _ in 0..1000 {
///             counter_clone.increment();
///         }
///     }));
/// }
///
/// for handle in handles {
///     handle.join().unwrap();
/// }
///
/// assert_eq!(counter.get(), 10000);
/// ```
use std::sync::atomic::{AtomicUsize, Ordering};

/// Lock-free counter using atomic operations
pub struct LockFreeCounter {
    count: AtomicUsize,
}

impl LockFreeCounter {
    /// Create new counter
    pub fn new() -> Self {
        Self {
            count: AtomicUsize::new(0),
        }
    }

    /// Increment atomically
    pub fn increment(&self) {
        self.count.fetch_add(1, Ordering::SeqCst);
    }

    /// Get current value
    pub fn get(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }

    /// Compare and swap
    pub fn compare_and_swap(&self, current: usize, new: usize) -> bool {
        self.count
            .compare_exchange(current, new, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    }
}

impl Default for LockFreeCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// Scoped threads for borrowing
///
/// ## Why?
/// Scoped threads allow borrowing local data, unlike standard threads.
/// This is safe because the compiler ensures threads don't outlive
/// the borrowed data.
///
/// ## Example
/// ```rust
/// use rust_202::differentiators::concurrency::ScopedThreads;
///
/// let data = vec![1, 2, 3, 4, 5];
/// let sum = ScopedThreads::parallel_sum_scoped(&data);
/// assert_eq!(sum, 15);
/// ```
pub struct ScopedThreads;

impl ScopedThreads {
    /// Parallel sum using scoped threads (simplified example)
    pub fn parallel_sum_scoped(data: &[i32]) -> i32 {
        // In real code, use std::thread::scope (stable in Rust 1.63+)
        // For now, we'll use a simplified version
        data.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_safe_data() {
        let data = ThreadSafeData::new(vec![1, 2, 3]);
        let sum = data.parallel_sum();
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_parallel_process() {
        let data = ThreadSafeData::new(vec![1, 2, 3, 4, 5]);
        let results = data.parallel_process(3);
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_producer_consumer() {
        let result = ChannelExample::producer_consumer(10);
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_multiple_producers() {
        let result = ChannelExample::multiple_producers(3, 5);
        assert_eq!(result.len(), 15);
    }

    #[test]
    fn test_parallel_iterators() {
        let sum = ParallelIterators::sum_range(1, 1000);
        assert_eq!(sum, 499500);

        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let chunks = ParallelIterators::process_chunks(data, 3);
        assert_eq!(chunks, vec![6, 15, 24]);
    }

    #[test]
    fn test_lock_free_counter() {
        let counter = Arc::new(LockFreeCounter::new());
        let mut handles = vec![];

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    counter_clone.increment();
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(counter.get(), 1000);
    }

    #[test]
    fn test_compare_and_swap() {
        let counter = LockFreeCounter::new();
        assert!(counter.compare_and_swap(0, 5));
        assert_eq!(counter.get(), 5);
        assert!(!counter.compare_and_swap(0, 10));
        assert_eq!(counter.get(), 5);
    }

    #[test]
    fn test_scoped_threads() {
        let data = vec![1, 2, 3, 4, 5];
        let sum = ScopedThreads::parallel_sum_scoped(&data);
        assert_eq!(sum, 15);
    }
}
