//! # Adapter Pattern
//!
//! Adapter pattern using newtype wrappers and Deref coercion.

use std::ops::Deref;

/// Legacy temperature API (returns Fahrenheit)
pub struct LegacyThermometer {
    fahrenheit: f64,
}

impl LegacyThermometer {
    /// Create a new thermometer
    pub fn new(fahrenheit: f64) -> Self {
        Self { fahrenheit }
    }

    /// Get temperature in Fahrenheit
    pub fn get_temperature(&self) -> f64 {
        self.fahrenheit
    }
}

/// Modern temperature interface (expects Celsius)
pub trait TemperatureSensor {
    /// Get temperature in Celsius
    fn celsius(&self) -> f64;

    /// Get temperature in Kelvin
    fn kelvin(&self) -> f64 {
        self.celsius() + 273.15
    }
}

/// Adapter from legacy thermometer to modern interface
///
/// ## Why?
/// The adapter pattern in Rust uses newtype wrappers to provide
/// a new interface to existing code. Unlike Python's monkey patching
/// or Go's embedding, Rust adapters are explicit and type-safe.
///
/// ## Example
/// ```rust
/// use rust_202::oop::patterns::adapter::{
///     LegacyThermometer,
///     ThermometerAdapter,
///     TemperatureSensor
/// };
///
/// let legacy = LegacyThermometer::new(68.0);
/// let adapter = ThermometerAdapter::new(legacy);
/// let celsius = adapter.celsius();
/// assert!((celsius - 20.0).abs() < 0.1);
/// ```
pub struct ThermometerAdapter {
    thermometer: LegacyThermometer,
}

impl ThermometerAdapter {
    /// Create a new adapter
    pub fn new(thermometer: LegacyThermometer) -> Self {
        Self { thermometer }
    }
}

impl TemperatureSensor for ThermometerAdapter {
    fn celsius(&self) -> f64 {
        (self.thermometer.get_temperature() - 32.0) * 5.0 / 9.0
    }
}

impl Deref for ThermometerAdapter {
    type Target = LegacyThermometer;

    fn deref(&self) -> &Self::Target {
        &self.thermometer
    }
}

/// String adapter for case-insensitive comparisons
///
/// ## Example
/// ```rust
/// use rust_202::oop::patterns::adapter::CaseInsensitiveString;
///
/// let s1 = CaseInsensitiveString::new("Hello");
/// let s2 = CaseInsensitiveString::new("HELLO");
/// assert_eq!(s1, s2);
/// ```
#[derive(Debug, Clone)]
pub struct CaseInsensitiveString(String);

impl CaseInsensitiveString {
    /// Create a new case-insensitive string
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }

    /// Get the inner string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl PartialEq for CaseInsensitiveString {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl Eq for CaseInsensitiveString {}

impl Deref for CaseInsensitiveString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Collection adapter for legacy Vec API
pub struct SortedVec<T> {
    inner: Vec<T>,
}

impl<T: Ord> SortedVec<T> {
    /// Create a new sorted vector
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Insert an element (maintains sorted order)
    pub fn insert(&mut self, item: T) {
        let pos = self.inner.binary_search(&item).unwrap_or_else(|e| e);
        self.inner.insert(pos, item);
    }

    /// Get the length
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T: Ord> Default for SortedVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Deref for SortedVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermometer_adapter() {
        let legacy = LegacyThermometer::new(68.0);
        let adapter = ThermometerAdapter::new(legacy);

        let celsius = adapter.celsius();
        assert!((celsius - 20.0).abs() < 0.1);

        let kelvin = adapter.kelvin();
        assert!((kelvin - 293.15).abs() < 0.1);
    }

    #[test]
    fn test_case_insensitive_string() {
        let s1 = CaseInsensitiveString::new("Hello");
        let s2 = CaseInsensitiveString::new("HELLO");
        let s3 = CaseInsensitiveString::new("hello");

        assert_eq!(s1, s2);
        assert_eq!(s2, s3);
        assert_eq!(s1, s3);
    }

    #[test]
    fn test_sorted_vec() {
        let mut vec = SortedVec::new();
        vec.insert(5);
        vec.insert(2);
        vec.insert(8);
        vec.insert(1);

        assert_eq!(vec.len(), 4);
        assert_eq!(&vec[..], &[1, 2, 5, 8]);
    }
}
