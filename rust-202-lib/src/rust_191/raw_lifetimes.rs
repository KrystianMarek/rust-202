//! # Raw Lifetimes and Complex Borrowing
//!
//! Demonstrates advanced lifetime patterns and the use of raw lifetime identifiers
//! for complex borrow scenarios.

/// A structure demonstrating complex lifetime relationships
///
/// ## Why?
/// Lifetimes are Rust's secret weapon for memory safety without GC.
/// Unlike Go's garbage collector or C's manual memory management,
/// Rust's borrow checker guarantees safety at compile-time.
///
/// ## Example
/// ```rust
/// use rust_202::rust_191::raw_lifetimes::DataHolder;
///
/// let data = vec![1, 2, 3, 4, 5];
/// let holder = DataHolder::new(&data);
/// assert_eq!(holder.first(), Some(&1));
/// ```
pub struct DataHolder<'data> {
    data: &'data [i32],
}

impl<'data> DataHolder<'data> {
    /// Create a new data holder
    pub fn new(data: &'data [i32]) -> Self {
        Self { data }
    }

    /// Get the first element
    pub fn first(&self) -> Option<&'data i32> {
        self.data.first()
    }

    /// Get a slice of the data
    pub fn slice(&self, start: usize, end: usize) -> &'data [i32] {
        &self.data[start..end]
    }
}

/// Iterator with lifetime bounds
///
/// ## Why?
/// Custom iterators with lifetimes enable zero-copy iteration,
/// unlike Python's iterator protocol which often involves copying.
pub struct WindowIterator<'data> {
    data: &'data [i32],
    window_size: usize,
    position: usize,
}

impl<'data> WindowIterator<'data> {
    /// Create a new window iterator
    pub fn new(data: &'data [i32], window_size: usize) -> Self {
        Self {
            data,
            window_size,
            position: 0,
        }
    }
}

impl<'data> Iterator for WindowIterator<'data> {
    type Item = &'data [i32];

    fn next(&mut self) -> Option<Self::Item> {
        if self.position + self.window_size <= self.data.len() {
            let window = &self.data[self.position..self.position + self.window_size];
            self.position += 1;
            Some(window)
        } else {
            None
        }
    }
}

/// Multiple lifetime parameters
///
/// ## Why?
/// Multiple lifetimes allow expressing complex relationships between
/// borrowed data, ensuring references remain valid.
///
/// ## Example
/// ```rust
/// use rust_202::rust_191::raw_lifetimes::DataComparator;
///
/// let data1 = vec![1, 2, 3];
/// let data2 = vec![4, 5, 6];
/// let comparator = DataComparator::new(&data1, &data2);
/// assert_eq!(comparator.first_is_larger(), false);
/// ```
pub struct DataComparator<'a, 'b> {
    left: &'a [i32],
    right: &'b [i32],
}

impl<'a, 'b> DataComparator<'a, 'b> {
    /// Create a new comparator
    pub fn new(left: &'a [i32], right: &'b [i32]) -> Self {
        Self { left, right }
    }

    /// Check if first element of left is larger than first of right
    pub fn first_is_larger(&self) -> bool {
        match (self.left.first(), self.right.first()) {
            (Some(l), Some(r)) => l > r,
            _ => false,
        }
    }

    /// Return the longer slice (demonstrates lifetime subtyping)
    pub fn longer(&self) -> &[i32]
    where
        'a: 'b,
        'b: 'a,
    {
        if self.left.len() > self.right.len() {
            self.left
        } else {
            self.right
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_holder() {
        let data = vec![1, 2, 3, 4, 5];
        let holder = DataHolder::new(&data);
        assert_eq!(holder.first(), Some(&1));
        assert_eq!(holder.slice(1, 3), &[2, 3]);
    }

    #[test]
    fn test_window_iterator() {
        let data = vec![1, 2, 3, 4, 5];
        let windows: Vec<_> = WindowIterator::new(&data, 3).collect();
        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], &[1, 2, 3]);
        assert_eq!(windows[1], &[2, 3, 4]);
        assert_eq!(windows[2], &[3, 4, 5]);
    }

    #[test]
    fn test_data_comparator() {
        let data1 = vec![1, 2, 3];
        let data2 = vec![4, 5, 6];
        let comp = DataComparator::new(&data1, &data2);
        assert!(!comp.first_is_larger());
    }
}
