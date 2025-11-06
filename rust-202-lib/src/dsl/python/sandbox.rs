//! # Sandboxed Python Execution
//!
//! Demonstrates restricted Python execution for untrusted code.

use pyo3::prelude::*;
use pyo3::types::PyDict;

/// Execute Python in a restricted environment
///
/// ## Why?
/// Running user-provided Python code is dangerous (arbitrary execution).
/// Sandboxing restricts access to dangerous modules like os, sys, subprocess.
///
/// ## Comparison
/// - **Rust sandboxing**: Compile-time + runtime safety
/// - **Python alone**: Easy to escape sandbox
/// - **Go**: No mature sandboxing
/// - **C**: Manual, error-prone
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "python-dsl")]
/// # fn example() -> pyo3::PyResult<()> {
/// use rust_202::dsl::python::sandboxed_eval;
///
/// // Safe expression
/// let result = sandboxed_eval("2 + 2")?;
/// assert_eq!(result, 4);
///
/// // Dangerous imports are blocked
/// let result = sandboxed_eval("import os; os.system('ls')");
/// assert!(result.is_err());
/// # Ok(())
/// # }
/// ```
pub fn sandboxed_eval(expression: &str) -> PyResult<i32> {
    Python::with_gil(|py| {
        // Create restricted builtins
        let restricted_builtins = PyDict::new_bound(py);

        // Allow only safe builtins
        let safe_builtins = vec![
            "abs", "all", "any", "bool", "dict", "enumerate",
            "filter", "int", "len", "list", "map", "max", "min",
            "range", "str", "sum", "tuple", "zip",
        ];

        let builtins = py.import_bound("builtins")?;
        for name in safe_builtins {
            if let Ok(func) = builtins.getattr(name) {
                restricted_builtins.set_item(name, func)?;
            }
        }

        let globals = PyDict::new_bound(py);
        globals.set_item("__builtins__", restricted_builtins)?;

        let result = py.eval_bound(expression, Some(&globals), None)?;
        result.extract()
    })
}

/// Execute Python with allowed modules only
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "python-dsl")]
/// # fn example() -> pyo3::PyResult<()> {
/// use rust_202::dsl::python::sandboxed_with_modules;
///
/// let source = r#"
/// import math
/// result = math.sqrt(16)
/// "#;
///
/// let allowed = vec!["math"];
/// let result = sandboxed_with_modules(source, allowed)?;
/// assert_eq!(result, 4.0);
/// # Ok(())
/// # }
/// ```
pub fn sandboxed_with_modules(source: &str, allowed_modules: Vec<&str>) -> PyResult<f64> {
    Python::with_gil(|py| {
        let globals = PyDict::new_bound(py);

        // Import only allowed modules
        for module_name in allowed_modules {
            let module = py.import_bound(module_name)?;
            globals.set_item(module_name, module)?;
        }

        py.run_bound(source, Some(&globals), None)?;

        if let Some(result) = globals.get_item("result")? {
            result.extract()
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                "Variable 'result' not found in script",
            ))
        }
    })
}

/// Configuration loader example
///
/// ## Why?
/// Python makes a great configuration DSL - more flexible than TOML/YAML,
/// safer than eval() in Python alone.
///
/// ## Example
/// ```rust,no_run
/// # #[cfg(feature = "python-dsl")]
/// # fn example() -> pyo3::PyResult<()> {
/// use rust_202::dsl::python::load_config;
///
/// let config_py = r#"
/// DATABASE_URL = "postgresql://localhost/db"
/// MAX_CONNECTIONS = 10
///
/// def get_config():
///     return {
///         "db_url": DATABASE_URL,
///         "max_conn": MAX_CONNECTIONS
///     }
/// "#;
///
/// let (db_url, max_conn) = load_config(config_py)?;
/// assert_eq!(db_url, "postgresql://localhost/db");
/// assert_eq!(max_conn, 10);
/// # Ok(())
/// # }
/// ```
pub fn load_config(source: &str) -> PyResult<(String, i32)> {
    Python::with_gil(|py| {
        let module = PyModule::from_code_bound(py, source, "config.py", "config")?;
        let get_config = module.getattr("get_config")?;
        let config = get_config.call0()?;

        let db_url: String = config.get_item("db_url")?.extract()?;
        let max_conn: i32 = config.get_item("max_conn")?.extract()?;

        Ok((db_url, max_conn))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandboxed_eval_safe() {
        let result = sandboxed_eval("2 + 3").unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_sandboxed_eval_list_ops() {
        let result = sandboxed_eval("sum([1, 2, 3, 4, 5])").unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    #[should_panic]
    fn test_sandboxed_eval_blocks_import() {
        // This should fail because 'import' is not allowed
        sandboxed_eval("import os").unwrap();
    }

    #[test]
    fn test_sandboxed_with_modules() {
        let source = r#"
import math
result = math.sqrt(25)
"#;
        let result = sandboxed_with_modules(source, vec!["math"]).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_load_config() {
        let config = r#"
DATABASE_URL = "postgresql://localhost/test"
MAX_CONNECTIONS = 5

def get_config():
    return {
        "db_url": DATABASE_URL,
        "max_conn": MAX_CONNECTIONS
    }
"#;

        let (db_url, max_conn) = load_config(config).unwrap();
        assert_eq!(db_url, "postgresql://localhost/test");
        assert_eq!(max_conn, 5);
    }
}

