# Contributing to Rust-202

Thank you for your interest in contributing to rust-202! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Please be respectful and constructive in all interactions.

## How to Contribute

### Reporting Issues

- Check if the issue already exists
- Use a clear and descriptive title
- Provide:
  - Rust version (`rustc --version`)
  - Operating system
  - Steps to reproduce
  - Expected vs actual behavior

### Suggesting Enhancements

- Use a clear and descriptive title
- Explain the use case and benefits
- Provide examples if possible

### Pull Requests

1. **Fork and Clone**
   ```bash
   git clone https://github.com/YOUR_USERNAME/rust-202.git
   cd rust-202
   ```

2. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make Changes**
   - Follow the coding style (see below)
   - Add tests for new functionality
   - Update documentation
   - Ensure all tests pass

4. **Commit**
   ```bash
   git commit -m "Add: Brief description of changes"
   ```

   Use conventional commit messages:
   - `Add:` for new features
   - `Fix:` for bug fixes
   - `Docs:` for documentation
   - `Refactor:` for code restructuring
   - `Test:` for test additions/changes

5. **Push and Create PR**
   ```bash
   git push origin feature/your-feature-name
   ```
   Then create a pull request on GitHub.

## Coding Standards

### Rust Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting:
  ```bash
  cargo fmt --all
  ```
- Use `cargo clippy` for linting:
  ```bash
  cargo clippy --all-targets --all-features -- -D warnings
  ```

### Documentation

- Add rustdoc comments for all public items
- Include examples in documentation:
  ```rust
  /// Brief description
  ///
  /// ## Why?
  /// Explain the purpose and benefits
  ///
  /// ## Example
  /// ```rust
  /// use rust_202::module::Item;
  ///
  /// let item = Item::new();
  /// assert_eq!(item.value(), 42);
  /// ```
  pub struct Item;
  ```

- Include comparison notes where relevant:
  ```rust
  /// ## Comparison
  /// - **Rust**: Type-safe at compile-time
  /// - **Python**: Runtime type errors
  /// - **Go**: Interface{} allows any type
  /// - **C**: Void pointers, no safety
  ```

### Testing

- Add unit tests for all functionality:
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn test_feature() {
          let result = feature();
          assert_eq!(result, expected);
      }
  }
  ```

- Run tests before submitting:
  ```bash
  cargo test --all-features
  cargo test --doc
  ```

### Performance

- Add benchmarks for performance-critical code:
  ```rust
  // In benches/your_bench.rs
  use criterion::{black_box, criterion_group, criterion_main, Criterion};

  fn bench_feature(c: &mut Criterion) {
      c.bench_function("feature", |b| {
          b.iter(|| black_box(feature()))
      });
  }

  criterion_group!(benches, bench_feature);
  criterion_main!(benches);
  ```

## Adding New Content

### Module Structure

When adding new modules, follow this structure:

```
src/
├── your_module/
│   ├── mod.rs          # Public API and re-exports
│   ├── feature1.rs     # Feature implementation
│   └── feature2.rs     # Another feature
```

### Example Template

```rust
//! # Module Title
//!
//! Brief description of what this module provides.

/// Item description
///
/// ## Why?
/// Explain why this is useful and how it compares to other languages.
///
/// ## Example
/// ```rust
/// use rust_202::module::Item;
///
/// let item = Item::new();
/// // Use the item
/// ```
pub struct Item;

impl Item {
    /// Create a new item
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let item = Item::new();
        // Assertions
    }
}
```

## Design Pattern Contributions

When adding new design patterns:

1. Create a new file in `src/oop/patterns/`
2. Implement the pattern using Rust idioms
3. Include comparison notes explaining how it differs from traditional OOP
4. Add tests and examples
5. Update `src/oop/patterns/mod.rs` to export the new pattern

## Documentation Contributions

- Update README.md for new features
- Add entries to CHANGELOG.md
- Update comparison tables if relevant
- Add examples to the `examples/` directory

## Review Process

1. All PRs must pass CI checks:
   - Formatting (`cargo fmt`)
   - Linting (`cargo clippy`)
   - Tests (`cargo test`)
   - Documentation (`cargo doc`)

2. Maintainers will review code for:
   - Correctness
   - Style consistency
   - Documentation quality
   - Test coverage

3. Address review comments and update PR

4. Once approved, maintainers will merge

## Questions?

Feel free to:
- Open an issue for discussion
- Ask questions in pull requests
- Reach out to maintainers

## License

By contributing, you agree that your contributions will be licensed under the same terms as the project (MIT/Apache-2.0).

