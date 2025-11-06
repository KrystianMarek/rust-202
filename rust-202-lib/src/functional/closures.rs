//! # Closures and Higher-Order Functions
//!
//! Demonstrates Rust's closure system with Fn, FnMut, and FnOnce traits.

/// Closure examples demonstrating Fn, FnMut, and FnOnce
///
/// ## Why?
/// Rust's closure traits (Fn, FnMut, FnOnce) provide fine-grained control
/// over how closures capture environment. Unlike Python's nonlocal or
/// Go's implicit capture, Rust's borrow checker enforces safety at compile-time.
pub struct ClosureExamples;

impl ClosureExamples {
    /// Fn trait: borrows captured variables immutably
    ///
    /// ## Example
    /// ```rust
    /// use rust_202::functional::closures::ClosureExamples;
    ///
    /// let result = ClosureExamples::fn_example(vec![1, 2, 3], 10);
    /// assert_eq!(result, vec![11, 12, 13]);
    /// ```
    pub fn fn_example(numbers: Vec<i32>, offset: i32) -> Vec<i32> {
        // This closure borrows `offset` immutably
        let add_offset = |n: i32| n + offset;
        numbers.into_iter().map(add_offset).collect()
    }

    /// FnMut trait: borrows captured variables mutably
    ///
    /// ## Example
    /// ```rust
    /// use rust_202::functional::closures::ClosureExamples;
    ///
    /// let result = ClosureExamples::fn_mut_example(vec![1, 2, 3]);
    /// assert_eq!(result, vec![1, 3, 6]);
    /// ```
    pub fn fn_mut_example(numbers: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        // This closure mutably borrows `sum`
        numbers
            .into_iter()
            .map(|n| {
                sum += n;
                sum
            })
            .collect()
    }

    /// FnOnce trait: takes ownership of captured variables
    ///
    /// ## Example
    /// ```rust
    /// use rust_202::functional::closures::ClosureExamples;
    ///
    /// let result = ClosureExamples::fn_once_example(vec![1, 2, 3]);
    /// assert_eq!(result, 6);
    /// ```
    pub fn fn_once_example(numbers: Vec<i32>) -> i32 {
        // This closure takes ownership of `numbers`
        let consume = move || numbers.into_iter().sum();
        consume()
    }
}

/// Higher-order function that takes a closure
///
/// ## Example
/// ```rust
/// use rust_202::functional::closures::apply_twice;
///
/// let result = apply_twice(5, |x| x * 2);
/// assert_eq!(result, 20);
/// ```
pub fn apply_twice<F>(value: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(f(value))
}

/// Higher-order function returning a closure
///
/// ## Example
/// ```rust
/// use rust_202::functional::closures::make_adder;
///
/// let add_five = make_adder(5);
/// assert_eq!(add_five(10), 15);
/// ```
pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

/// Closure-based filter and map
///
/// ## Example
/// ```rust
/// use rust_202::functional::closures::filter_map;
///
/// let result = filter_map(
///     vec![1, 2, 3, 4, 5],
///     |&x| x % 2 == 0,
///     |x| x * x
/// );
/// assert_eq!(result, vec![4, 16]);
/// ```
pub fn filter_map<T, U, F, G>(items: Vec<T>, predicate: F, mapper: G) -> Vec<U>
where
    F: Fn(&T) -> bool,
    G: Fn(T) -> U,
{
    items.into_iter().filter(predicate).map(mapper).collect()
}

/// Compose two functions
///
/// ## Why?
/// Function composition is a fundamental FP concept. Rust's type system
/// ensures composed functions are type-safe at compile-time.
///
/// ## Example
/// ```rust
/// use rust_202::functional::closures::compose;
///
/// let add_one = |x: i32| x + 1;
/// let double = |x: i32| x * 2;
/// let add_one_then_double = compose(add_one, double);
///
/// assert_eq!(add_one_then_double(5), 12);
/// ```
pub fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

/// Memoization using closures
///
/// ## Example
/// ```rust
/// use rust_202::functional::closures::Memoizer;
///
/// let mut memo = Memoizer::new(|x: i32| x * x);
/// assert_eq!(memo.call(5), 25);
/// assert_eq!(memo.call(5), 25); // Cached result
/// ```
pub struct Memoizer<F, Arg, Ret>
where
    F: FnMut(Arg) -> Ret,
    Arg: Clone + std::hash::Hash + Eq,
    Ret: Clone,
{
    function: F,
    cache: std::collections::HashMap<Arg, Ret>,
}

impl<F, Arg, Ret> Memoizer<F, Arg, Ret>
where
    F: FnMut(Arg) -> Ret,
    Arg: Clone + std::hash::Hash + Eq,
    Ret: Clone,
{
    /// Create a new memoizer
    pub fn new(function: F) -> Self {
        Self {
            function,
            cache: std::collections::HashMap::new(),
        }
    }

    /// Call the function with memoization
    pub fn call(&mut self, arg: Arg) -> Ret {
        if let Some(result) = self.cache.get(&arg) {
            return result.clone();
        }

        let result = (self.function)(arg.clone());
        self.cache.insert(arg, result.clone());
        result
    }
}

/// Lazy evaluation using closures
///
/// ## Example
/// ```rust
/// use rust_202::functional::closures::Lazy;
///
/// let mut lazy = Lazy::new(|| {
///     println!("Computing expensive value...");
///     42
/// });
///
/// // Value is not computed yet
/// let value = lazy.get(); // Now it's computed
/// assert_eq!(*value, 42);
/// ```
pub struct Lazy<T, F>
where
    F: FnOnce() -> T,
{
    init: Option<F>,
    value: Option<T>,
}

impl<T, F> Lazy<T, F>
where
    F: FnOnce() -> T,
{
    /// Create a new lazy value
    pub fn new(init: F) -> Self {
        Self {
            init: Some(init),
            value: None,
        }
    }

    /// Get the value, computing it if necessary
    pub fn get(&mut self) -> &T {
        if self.value.is_none() {
            let init = self.init.take().expect("Lazy already initialized");
            self.value = Some(init());
        }
        self.value.as_ref().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_example() {
        let result = ClosureExamples::fn_example(vec![1, 2, 3], 10);
        assert_eq!(result, vec![11, 12, 13]);
    }

    #[test]
    fn test_fn_mut_example() {
        let result = ClosureExamples::fn_mut_example(vec![1, 2, 3]);
        assert_eq!(result, vec![1, 3, 6]);
    }

    #[test]
    fn test_fn_once_example() {
        let result = ClosureExamples::fn_once_example(vec![1, 2, 3]);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_apply_twice() {
        let result = apply_twice(5, |x| x * 2);
        assert_eq!(result, 20);
    }

    #[test]
    fn test_make_adder() {
        let add_five = make_adder(5);
        assert_eq!(add_five(10), 15);
    }

    #[test]
    fn test_filter_map() {
        let result = filter_map(vec![1, 2, 3, 4, 5], |&x| x % 2 == 0, |x| x * x);
        assert_eq!(result, vec![4, 16]);
    }

    #[test]
    fn test_compose() {
        let add_one = |x: i32| x + 1;
        let double = |x: i32| x * 2;
        let composed = compose(add_one, double);
        assert_eq!(composed(5), 12);
    }

    #[test]
    fn test_memoizer() {
        let mut memo = Memoizer::new(|x: i32| x * x);
        assert_eq!(memo.call(5), 25);
        assert_eq!(memo.call(5), 25);
        assert_eq!(memo.call(3), 9);
    }

    #[test]
    fn test_lazy() {
        let mut lazy = Lazy::new(|| 42);
        assert_eq!(*lazy.get(), 42);
        assert_eq!(*lazy.get(), 42);
    }
}
