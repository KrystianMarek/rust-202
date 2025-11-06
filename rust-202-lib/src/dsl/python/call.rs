//! # Python Function Calls
//!
//! Call Python functions from Rust with type-safe conversions.

use pyo3::prelude::*;
use pyo3::types::PyModule;

/// Call Python function and extract result
///
/// ## Why?
/// Enables leveraging Python's vast ecosystem (data science, ML, etc.)
/// from safe, performant Rust code.
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "python-dsl")]
/// # fn example() -> pyo3::PyResult<()> {
/// use rust_202::dsl::python::call_py_function;
///
/// let script = r#"
/// def fibonacci(n):
///     if n <= 1:
///         return n
///     return fibonacci(n-1) + fibonacci(n-2)
/// "#;
///
/// let result = call_py_function::<i32>(script, "fibonacci", 10)?;
/// assert_eq!(result, 55);
/// # Ok(())
/// # }
/// ```
pub fn call_py_function<T>(source: &str, function_name: &str, arg: i32) -> PyResult<T>
where
    T: for<'py> FromPyObject<'py>,
{
    Python::with_gil(|py| {
        let module = PyModule::from_code_bound(py, source, "script.py", "script")?;
        let function = module.getattr(function_name)?;
        let result = function.call1((arg,))?;
        result.extract()
    })
}

/// Data transformation pipeline
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "python-dsl")]
/// # fn example() -> pyo3::PyResult<()> {
/// use rust_202::dsl::python::transform_pipeline;
///
/// let script = r#"
/// def process(data):
///     # Python's list comprehensions
///     return [x * 2 for x in data if x % 2 == 0]
/// "#;
///
/// let input = vec![1, 2, 3, 4, 5, 6];
/// let result = transform_pipeline(script, input)?;
/// assert_eq!(result, vec![4, 8, 12]);
/// # Ok(())
/// # }
/// ```
pub fn transform_pipeline(source: &str, input: Vec<i32>) -> PyResult<Vec<i32>> {
    Python::with_gil(|py| {
        let module = PyModule::from_code_bound(py, source, "pipeline.py", "pipeline")?;
        let process = module.getattr("process")?;

        let py_input = pyo3::types::PyList::new_bound(py, input);
        let result = process.call1((py_input,))?;
        result.extract()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_py_function() {
        let source = r#"
def square(x):
    return x * x
"#;

        let result: i32 = call_py_function(source, "square", 7).unwrap();
        assert_eq!(result, 49);
    }

    #[test]
    fn test_transform_pipeline() {
        let source = r#"
def process(data):
    return [x + 100 for x in data]
"#;

        let result = transform_pipeline(source, vec![1, 2, 3]).unwrap();
        assert_eq!(result, vec![101, 102, 103]);
    }
}
