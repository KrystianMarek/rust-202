//! # Python Script Execution
//!
//! Demonstrates running Python code from Rust.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule};

/// Execute a simple Python expression
///
/// ## Why?
/// Allows runtime evaluation of Python expressions while maintaining Rust's
/// safety guarantees. The GIL (Global Interpreter Lock) is automatically managed.
///
/// ## Comparison
/// - **Rust + Python**: Best of both worlds - safety + flexibility
/// - **Pure Python**: No compile-time safety
/// - **Go**: No mature Python embedding
/// - **C + libpython**: Manual, error-prone
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "python-dsl")]
/// # fn example() -> pyo3::PyResult<()> {
/// use rust_202::dsl::python::eval_simple;
///
/// let result = eval_simple("2 + 2")?;
/// assert_eq!(result, 4);
/// # Ok(())
/// # }
/// ```
pub fn eval_simple(expression: &str) -> PyResult<i32> {
    Python::with_gil(|py| {
        let result = py.eval_bound(expression, None, None)?;
        result.extract()
    })
}

/// Execute Python code and return a string result
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "python-dsl")]
/// # fn example() -> pyo3::PyResult<()> {
/// use rust_202::dsl::python::eval_string;
///
/// let result = eval_string("'Hello, ' + 'World!'")?;
/// assert_eq!(result, "Hello, World!");
/// # Ok(())
/// # }
/// ```
pub fn eval_string(expression: &str) -> PyResult<String> {
    Python::with_gil(|py| {
        let result = py.eval_bound(expression, None, None)?;
        result.extract()
    })
}

/// Execute a Python script with variables
///
/// ## Why?
/// Demonstrates passing Rust data into Python and getting results back.
/// Type conversion is automatic for supported types.
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "python-dsl")]
/// # fn example() -> pyo3::PyResult<()> {
/// use rust_202::dsl::python::eval_with_context;
/// use std::collections::HashMap;
///
/// let mut vars = HashMap::new();
/// vars.insert("x", 10);
/// vars.insert("y", 32);
///
/// let result = eval_with_context("x + y", vars)?;
/// assert_eq!(result, 42);
/// # Ok(())
/// # }
/// ```
pub fn eval_with_context(
    expression: &str,
    variables: std::collections::HashMap<&str, i32>,
) -> PyResult<i32> {
    Python::with_gil(|py| {
        let locals = PyDict::new_bound(py);

        for (key, value) in variables {
            locals.set_item(key, value)?;
        }

        let result = py.eval_bound(expression, None, Some(&locals))?;
        result.extract()
    })
}

/// Run a Python module and call its main function
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "python-dsl")]
/// # fn example() -> pyo3::PyResult<()> {
/// use rust_202::dsl::python::run_module;
///
/// let source = r#"
/// def main():
///     return "Hello from Python!"
/// "#;
///
/// let result = run_module(source, "test")?;
/// assert_eq!(result, "Hello from Python!");
/// # Ok(())
/// # }
/// ```
pub fn run_module(source: &str, module_name: &str) -> PyResult<String> {
    Python::with_gil(|py| {
        let module = PyModule::from_code_bound(py, source, &format!("{}.py", module_name), module_name)?;
        let main = module.getattr("main")?;
        let result = main.call0()?;
        result.extract()
    })
}

/// Execute Python code from a file
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "python-dsl")]
/// # fn example() -> pyo3::PyResult<()> {
/// use rust_202::dsl::python::eval_file;
///
/// // Assuming config.py exists with: result = {"key": "value"}
/// let config = eval_file("result", include_str!("../py/config.py"))?;
/// # Ok(())
/// # }
/// ```
pub fn eval_file(var_name: &str, source: &str) -> PyResult<String> {
    Python::with_gil(|py| {
        let globals = PyDict::new_bound(py);
        py.run_bound(source, Some(&globals), None)?;

        if let Some(result) = globals.get_item(var_name)? {
            result.extract()
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                format!("Variable '{}' not found", var_name),
            ))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_simple() {
        let result = eval_simple("5 + 3").unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    fn test_eval_string() {
        let result = eval_string("'Rust' + ' and ' + 'Python'").unwrap();
        assert_eq!(result, "Rust and Python");
    }

    #[test]
    fn test_eval_with_context() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("a", 10);
        vars.insert("b", 20);

        let result = eval_with_context("a * b", vars).unwrap();
        assert_eq!(result, 200);
    }

    #[test]
    fn test_run_module() {
        let source = r#"
def main():
    return "Module executed"
"#;

        let result = run_module(source, "test_module").unwrap();
        assert_eq!(result, "Module executed");
    }
}

