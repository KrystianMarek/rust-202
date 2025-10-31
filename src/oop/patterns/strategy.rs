//! # Strategy Pattern
//!
//! Strategy pattern using traits for interchangeable algorithms.

/// Compression strategy trait
///
/// ## Why?
/// Strategy pattern with traits allows swapping algorithms at runtime
/// while maintaining type safety. Unlike Python's function passing or
/// Go's interface{}, Rust ensures all strategies implement the contract.
///
/// ## Example
/// ```rust
/// use rust_202::oop::patterns::strategy::{CompressionContext, GzipCompression};
///
/// let mut context = CompressionContext::new(Box::new(GzipCompression));
/// let compressed = context.compress("Hello, World!");
/// ```
pub trait CompressionStrategy {
    /// Compress data
    fn compress(&self, data: &str) -> Vec<u8>;

    /// Decompress data
    fn decompress(&self, data: &[u8]) -> String;

    /// Get strategy name
    fn name(&self) -> &str;
}

/// Gzip compression strategy
pub struct GzipCompression;

impl CompressionStrategy for GzipCompression {
    fn compress(&self, data: &str) -> Vec<u8> {
        // Simulated compression
        format!("GZIP:{}", data).into_bytes()
    }

    fn decompress(&self, data: &[u8]) -> String {
        String::from_utf8_lossy(data).replace("GZIP:", "")
    }

    fn name(&self) -> &str {
        "Gzip"
    }
}

/// Zip compression strategy
pub struct ZipCompression;

impl CompressionStrategy for ZipCompression {
    fn compress(&self, data: &str) -> Vec<u8> {
        format!("ZIP:{}", data).into_bytes()
    }

    fn decompress(&self, data: &[u8]) -> String {
        String::from_utf8_lossy(data).replace("ZIP:", "")
    }

    fn name(&self) -> &str {
        "Zip"
    }
}

/// No compression strategy
pub struct NoCompression;

impl CompressionStrategy for NoCompression {
    fn compress(&self, data: &str) -> Vec<u8> {
        data.as_bytes().to_vec()
    }

    fn decompress(&self, data: &[u8]) -> String {
        String::from_utf8_lossy(data).to_string()
    }

    fn name(&self) -> &str {
        "None"
    }
}

/// Context that uses a compression strategy
pub struct CompressionContext {
    strategy: Box<dyn CompressionStrategy>,
}

impl CompressionContext {
    /// Create a new context with a strategy
    pub fn new(strategy: Box<dyn CompressionStrategy>) -> Self {
        Self { strategy }
    }

    /// Change the strategy
    pub fn set_strategy(&mut self, strategy: Box<dyn CompressionStrategy>) {
        self.strategy = strategy;
    }

    /// Compress using the current strategy
    pub fn compress(&self, data: &str) -> Vec<u8> {
        println!("Using {} compression", self.strategy.name());
        self.strategy.compress(data)
    }

    /// Decompress using the current strategy
    pub fn decompress(&self, data: &[u8]) -> String {
        self.strategy.decompress(data)
    }
}

/// Sorting strategy example
pub trait SortStrategy<T> {
    /// Sort the data
    fn sort(&self, data: &mut [T]);

    /// Get strategy name
    fn name(&self) -> &str;
}

/// Quick sort strategy
pub struct QuickSort;

impl<T: Ord> SortStrategy<T> for QuickSort {
    fn sort(&self, data: &mut [T]) {
        data.sort();
    }

    fn name(&self) -> &str {
        "QuickSort"
    }
}

/// Bubble sort strategy
pub struct BubbleSort;

impl<T: Ord> SortStrategy<T> for BubbleSort {
    fn sort(&self, data: &mut [T]) {
        let len = data.len();
        for i in 0..len {
            for j in 0..len - i - 1 {
                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                }
            }
        }
    }

    fn name(&self) -> &str {
        "BubbleSort"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_strategies() {
        let mut context = CompressionContext::new(Box::new(GzipCompression));
        let compressed = context.compress("Hello");
        let decompressed = context.decompress(&compressed);
        assert_eq!(decompressed, "Hello");

        context.set_strategy(Box::new(ZipCompression));
        let compressed = context.compress("World");
        let decompressed = context.decompress(&compressed);
        assert_eq!(decompressed, "World");
    }

    #[test]
    fn test_sort_strategies() {
        let mut data = vec![5, 2, 8, 1, 9];
        let strategy = QuickSort;
        strategy.sort(&mut data);
        assert_eq!(data, vec![1, 2, 5, 8, 9]);

        let mut data = vec![5, 2, 8, 1, 9];
        let strategy = BubbleSort;
        strategy.sort(&mut data);
        assert_eq!(data, vec![1, 2, 5, 8, 9]);
    }
}
