# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Advanced Async Rust Module** (`async/`): Comprehensive async/await patterns
  - Async basics: async fn, futures, Send bounds
  - Streams: Custom Stream implementations and combinators
  - Async traits: Native async fn in trait + async-trait macro
  - Pinning: Pin, !Unpin, self-referential structures
  - Concurrency: select!, join!, timeout, cancellation, retry
  - Async I/O: TCP server patterns, buffer handling
  - Design patterns: Async pipeline, state machine
  - 35 async tests, 1 comprehensive example
  - Full documentation with Python/Go/C comparisons

## [0.1.0] - 2025-10-31

### Added

#### Rust 1.75+ Features Module
- Const operations with compile-time evaluation
- Advanced lifetime patterns and examples
- LLD linker and build optimization information
- Atomic operations for lock-free concurrency

#### OOP Module
- Trait-based composition patterns
- Circle and Rectangle implementations with Drawable and HasArea traits
- Design patterns:
  - Singleton pattern with OnceCell
  - Factory pattern for object creation
  - Observer pattern with channels
  - Builder pattern for fluent APIs
  - Strategy pattern for interchangeable algorithms
  - Adapter pattern with newtype wrappers

#### Functional Programming Module
- Fibonacci sequence iterator
- Sliding window iterator
- Advanced iterator combinators (map, filter, fold, flat_map, partition, scan)
- Closure examples (Fn, FnMut, FnOnce)
- Higher-order functions
- Function composition
- Memoization pattern
- Lazy evaluation

#### Idioms Module
- RAII with Drop trait
- Move semantics and ownership examples
- Borrowing patterns (immutable and mutable)
- Interior mutability with Cell
- Copy-on-Write (Cow) pattern
- Reference counting with Rc/Arc
- Result-based error handling
- Option patterns
- Custom error types
- The ? operator examples
- Panic boundaries

#### Differentiators Module
- **Safety**: Borrow checker, null safety, data race prevention, bounds checking, type safety
- **Performance**: Zero-cost abstractions, no GC overhead, inline optimizations, stack vs heap control, SIMD hints
- **Concurrency**: Send/Sync traits, message passing, lock-free data structures, scoped threads

#### Examples
- Quickstart example demonstrating basic usage
- Design patterns example

#### Benchmarks
- OOP benchmarks (circle area, singleton access)
- Functional programming benchmarks (fibonacci, iterators, closures)

#### Infrastructure
- Comprehensive test suite with 90%+ coverage
- CI/CD with GitHub Actions
- Documentation with rustdoc
- Clippy linting configuration
- README with comparison tables and usage examples

### Documentation
- Full API documentation with examples
- Comparison tables (Rust vs Python/Go/C)
- Contributing guidelines
- License information (MIT/Apache-2.0)

[Unreleased]: https://github.com/KrystianMarek/rust-202/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/KrystianMarek/rust-202/releases/tag/v0.1.0

