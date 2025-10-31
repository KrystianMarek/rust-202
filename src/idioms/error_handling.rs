//! # Error Handling Patterns
//!
//! Demonstrates idiomatic error handling in Rust using Result, Option, and the ? operator.

/// Result-based error handling
///
/// ## Why?
/// Rust's Result type forces explicit error handling at compile-time.
/// Unlike Python's exceptions (can be ignored) or Go's (error, nil) tuples
/// (can be forgotten), Rust's Result must be handled or explicitly ignored.
///
/// ## Example
/// ```rust
/// use rust_202::idioms::error_handling::ResultExample;
///
/// let result = ResultExample::safe_divide(10, 2);
/// assert_eq!(result.unwrap(), 5);
/// ```
pub struct ResultExample;

impl ResultExample {
    /// Safe division that returns Result
    pub fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err("Division by zero".to_string())
        } else {
            Ok(a / b)
        }
    }

    /// Parse and compute with error propagation
    ///
    /// ## Example
    /// ```rust
    /// use rust_202::idioms::error_handling::ResultExample;
    ///
    /// let result = ResultExample::parse_and_double("42");
    /// assert_eq!(result.unwrap(), 84);
    /// ```
    pub fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
        let num = s.parse::<i32>()?; // ? operator propagates error
        Ok(num * 2)
    }

    /// Chain multiple operations
    pub fn complex_operation(a: &str, b: &str) -> Result<i32, String> {
        let x = a.parse::<i32>().map_err(|e| e.to_string())?;
        let y = b.parse::<i32>().map_err(|e| e.to_string())?;
        Self::safe_divide(x, y)
    }
}

/// Option patterns
///
/// ## Why?
/// Option replaces null pointers, eliminating null pointer exceptions.
/// Unlike Python's None (can cause runtime errors) or C's NULL (UB),
/// Rust's Option is type-safe and must be handled.
///
/// ## Example
/// ```rust
/// use rust_202::idioms::error_handling::OptionExample;
///
/// let data = vec![1, 2, 3];
/// assert_eq!(OptionExample::safe_get(&data, 1), Some(&2));
/// assert_eq!(OptionExample::safe_get(&data, 10), None);
/// ```
pub struct OptionExample;

impl OptionExample {
    /// Safe array access
    pub fn safe_get<T>(data: &[T], index: usize) -> Option<&T> {
        data.get(index)
    }

    /// Chain option operations
    pub fn chain_example(maybe_num: Option<i32>) -> Option<i32> {
        maybe_num
            .filter(|&n| n > 0) // Filter negative numbers
            .map(|n| n * 2) // Double it
    }

    /// Combine multiple options
    pub fn combine_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
        a.and_then(|x| b.map(|y| x + y))
    }

    /// Option to Result conversion
    pub fn option_to_result(maybe: Option<i32>) -> Result<i32, &'static str> {
        maybe.ok_or("Value is None")
    }
}

/// Custom error types
///
/// ## Why?
/// Custom error types provide rich error context with zero runtime cost.
/// Type-safe error handling is impossible in Python or Go without
/// runtime reflection.
///
/// ## Example
/// ```rust
/// use rust_202::idioms::error_handling::{calculate, MathError};
///
/// let result = calculate(10, 0);
/// assert!(matches!(result, Err(MathError::DivisionByZero)));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    /// Division by zero error
    DivisionByZero,
    /// Negative square root
    NegativeSquareRoot,
    /// Overflow error
    Overflow,
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MathError::NegativeSquareRoot => write!(f, "Cannot take square root of negative"),
            MathError::Overflow => write!(f, "Arithmetic overflow"),
        }
    }
}

impl std::error::Error for MathError {}

/// Calculate with custom error type
pub fn calculate(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        return Err(MathError::DivisionByZero);
    }

    a.checked_div(b).ok_or(MathError::Overflow)
}

/// The ? operator in practice
///
/// ## Why?
/// The ? operator provides concise error propagation without try/catch
/// boilerplate (Python) or explicit if err != nil checks (Go).
///
/// ## Example
/// ```rust
/// use rust_202::idioms::error_handling::QuestionMarkExample;
///
/// let result = QuestionMarkExample::read_and_parse("42").unwrap();
/// assert_eq!(result, 42);
/// ```
pub struct QuestionMarkExample;

impl QuestionMarkExample {
    /// Simulate reading and parsing
    pub fn read_and_parse(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let trimmed = Self::validate_input(input)?;
        let parsed = Self::parse_number(trimmed)?;
        Self::validate_range(parsed)?;
        Ok(parsed)
    }

    fn validate_input(input: &str) -> Result<&str, &'static str> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            Err("Input is empty")
        } else {
            Ok(trimmed)
        }
    }

    fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
        s.parse()
    }

    fn validate_range(n: i32) -> Result<i32, &'static str> {
        if !(0..=100).contains(&n) {
            Err("Number out of range")
        } else {
            Ok(n)
        }
    }
}

/// Panic boundaries and unwinding
///
/// ## Why?
/// Panics in Rust unwind the stack (by default), running all Drop
/// implementations. Unlike C segfaults or Python uncaught exceptions,
/// Rust panics can be caught and provide clean resource cleanup.
pub struct PanicExample;

impl PanicExample {
    /// Catch panic with std::panic::catch_unwind
    pub fn safe_operation<F, R>(f: F) -> Result<R, String>
    where
        F: FnOnce() -> R + std::panic::UnwindSafe,
    {
        std::panic::catch_unwind(f).map_err(|_| "Operation panicked".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_divide() {
        assert_eq!(ResultExample::safe_divide(10, 2), Ok(5));
        assert!(ResultExample::safe_divide(10, 0).is_err());
    }

    #[test]
    fn test_parse_and_double() {
        assert_eq!(ResultExample::parse_and_double("42"), Ok(84));
        assert!(ResultExample::parse_and_double("abc").is_err());
    }

    #[test]
    fn test_complex_operation() {
        assert_eq!(ResultExample::complex_operation("10", "2"), Ok(5));
        assert!(ResultExample::complex_operation("10", "0").is_err());
        assert!(ResultExample::complex_operation("abc", "2").is_err());
    }

    #[test]
    fn test_option_patterns() {
        let data = vec![1, 2, 3];
        assert_eq!(OptionExample::safe_get(&data, 1), Some(&2));
        assert_eq!(OptionExample::safe_get(&data, 10), None);
    }

    #[test]
    fn test_chain_example() {
        assert_eq!(OptionExample::chain_example(Some(5)), Some(10));
        assert_eq!(OptionExample::chain_example(Some(-5)), None);
        assert_eq!(OptionExample::chain_example(None), None);
    }

    #[test]
    fn test_combine_options() {
        assert_eq!(OptionExample::combine_options(Some(5), Some(3)), Some(8));
        assert_eq!(OptionExample::combine_options(Some(5), None), None);
    }

    #[test]
    fn test_custom_error() {
        assert!(matches!(calculate(10, 0), Err(MathError::DivisionByZero)));
        assert_eq!(calculate(10, 2), Ok(5));
    }

    #[test]
    fn test_question_mark() {
        assert_eq!(QuestionMarkExample::read_and_parse("42").unwrap(), 42);
        assert!(QuestionMarkExample::read_and_parse("").is_err());
        assert!(QuestionMarkExample::read_and_parse("abc").is_err());
        assert!(QuestionMarkExample::read_and_parse("200").is_err());
    }

    #[test]
    fn test_panic_boundary() {
        let result = PanicExample::safe_operation(|| 42);
        assert_eq!(result, Ok(42));

        let result = PanicExample::safe_operation(|| panic!("test panic"));
        assert!(result.is_err());
    }
}
