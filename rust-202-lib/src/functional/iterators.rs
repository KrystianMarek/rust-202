//! # Advanced Iterator Patterns
//!
//! Demonstrates Rust's powerful iterator system with zero-cost abstractions.

use std::iter::Iterator;

/// Fibonacci sequence iterator
///
/// ## Why?
/// Iterators in Rust are zero-cost abstractions that compile to the same
/// code as hand-written loops, unlike Python's generator overhead or
/// Go's lack of iterator protocol.
///
/// ## Example
/// ```rust
/// use rust_202::functional::iterators::fibonacci_sequence;
///
/// let fibs: Vec<u64> = fibonacci_sequence().take(10).collect();
/// assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
/// ```
pub fn fibonacci_sequence() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

/// Fibonacci iterator implementation
#[derive(Debug, Clone)]
pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current.saturating_add(self.next);
        Some(current)
    }
}

/// Custom iterator that yields windows of elements
///
/// ## Example
/// ```rust
/// use rust_202::functional::iterators::SlidingWindow;
///
/// let data = vec![1, 2, 3, 4, 5];
/// let windows: Vec<Vec<i32>> = SlidingWindow::new(&data, 3).collect();
/// assert_eq!(windows.len(), 3);
/// ```
pub struct SlidingWindow<'a, T> {
    data: &'a [T],
    window_size: usize,
    position: usize,
}

impl<'a, T: Clone> SlidingWindow<'a, T> {
    /// Create a new sliding window iterator
    pub fn new(data: &'a [T], window_size: usize) -> Self {
        Self {
            data,
            window_size,
            position: 0,
        }
    }
}

impl<'a, T: Clone> Iterator for SlidingWindow<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position + self.window_size <= self.data.len() {
            let window = self.data[self.position..self.position + self.window_size].to_vec();
            self.position += 1;
            Some(window)
        } else {
            None
        }
    }
}

/// Advanced iterator combinators
///
/// ## Why?
/// Iterator chains in Rust are optimized at compile-time, often producing
/// better assembly than hand-written loops. This is impossible in Python
/// due to runtime interpretation or Go due to lack of iterator abstraction.
pub struct IteratorExamples;

impl IteratorExamples {
    /// Map, filter, and collect example
    ///
    /// ## Example
    /// ```rust
    /// use rust_202::functional::iterators::IteratorExamples;
    ///
    /// let result = IteratorExamples::map_filter_example(vec![1, 2, 3, 4, 5]);
    /// assert_eq!(result, vec![4, 16]);
    /// ```
    pub fn map_filter_example(numbers: Vec<i32>) -> Vec<i32> {
        numbers
            .into_iter()
            .filter(|&n| n % 2 == 0) // Filter even numbers
            .map(|n| n * n) // Square them
            .collect()
    }

    /// Fold (reduce) example
    ///
    /// ## Example
    /// ```rust
    /// use rust_202::functional::iterators::IteratorExamples;
    ///
    /// let sum = IteratorExamples::fold_example(vec![1, 2, 3, 4, 5]);
    /// assert_eq!(sum, 15);
    /// ```
    pub fn fold_example(numbers: Vec<i32>) -> i32 {
        numbers.into_iter().sum::<i32>()
    }

    /// FlatMap example
    ///
    /// ## Example
    /// ```rust
    /// use rust_202::functional::iterators::IteratorExamples;
    ///
    /// let result = IteratorExamples::flat_map_example(vec![1, 2, 3]);
    /// assert_eq!(result, vec![1, -1, 2, -2, 3, -3]);
    /// ```
    pub fn flat_map_example(numbers: Vec<i32>) -> Vec<i32> {
        numbers.into_iter().flat_map(|n| vec![n, -n]).collect()
    }

    /// Partition example
    ///
    /// ## Example
    /// ```rust
    /// use rust_202::functional::iterators::IteratorExamples;
    ///
    /// let (evens, odds) = IteratorExamples::partition_example(vec![1, 2, 3, 4, 5, 6]);
    /// assert_eq!(evens, vec![2, 4, 6]);
    /// assert_eq!(odds, vec![1, 3, 5]);
    /// ```
    pub fn partition_example(numbers: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
        numbers.into_iter().partition(|&n| n % 2 == 0)
    }

    /// Scan (stateful map) example
    ///
    /// ## Example
    /// ```rust
    /// use rust_202::functional::iterators::IteratorExamples;
    ///
    /// let result = IteratorExamples::scan_example(vec![1, 2, 3, 4]);
    /// assert_eq!(result, vec![1, 3, 6, 10]);
    /// ```
    pub fn scan_example(numbers: Vec<i32>) -> Vec<i32> {
        numbers
            .into_iter()
            .scan(0, |state, x| {
                *state += x;
                Some(*state)
            })
            .collect()
    }
}

/// Infinite iterator example
///
/// ## Example
/// ```rust
/// use rust_202::functional::iterators::InfiniteCounter;
///
/// let first_five: Vec<u64> = InfiniteCounter::new().take(5).collect();
/// assert_eq!(first_five, vec![0, 1, 2, 3, 4]);
/// ```
pub struct InfiniteCounter {
    count: u64,
}

impl InfiniteCounter {
    /// Create a new counter starting at 0
    pub fn new() -> Self {
        Self { count: 0 }
    }

    /// Create a counter starting at a specific value
    pub fn from(start: u64) -> Self {
        Self { count: start }
    }
}

impl Default for InfiniteCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for InfiniteCounter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.count;
        self.count = self.count.saturating_add(1);
        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let fibs: Vec<u64> = fibonacci_sequence().take(10).collect();
        assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_sliding_window() {
        let data = vec![1, 2, 3, 4, 5];
        let windows: Vec<Vec<i32>> = SlidingWindow::new(&data, 3).collect();
        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], vec![1, 2, 3]);
        assert_eq!(windows[2], vec![3, 4, 5]);
    }

    #[test]
    fn test_map_filter() {
        let result = IteratorExamples::map_filter_example(vec![1, 2, 3, 4, 5]);
        assert_eq!(result, vec![4, 16]);
    }

    #[test]
    fn test_fold() {
        let sum = IteratorExamples::fold_example(vec![1, 2, 3, 4, 5]);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_partition() {
        let (evens, odds) = IteratorExamples::partition_example(vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(odds, vec![1, 3, 5]);
    }

    #[test]
    fn test_infinite_counter() {
        let first_five: Vec<u64> = InfiniteCounter::new().take(5).collect();
        assert_eq!(first_five, vec![0, 1, 2, 3, 4]);
    }
}
