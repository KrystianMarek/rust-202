# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-10-31

### Project Structure
- **Cargo Workspace**: Migrated to workspace with 2 members
  - `rust-202-lib`: Pure Rust patterns library
  - `rust-202-web`: REST + gRPC web API server
- Shared dependency management
- Workspace-aware CI/CD

### Library Modules

#### Rust 1.75+ Features Module (`rust_191`)
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

#### Advanced Async Module (`async`)
- Async basics: async fn, futures, Send bounds
- Streams: Custom Stream implementations and combinators (Counter, Fibonacci, Range)
- Async traits: Native async fn in trait + async-trait macro
- Pinning: Pin, !Unpin, self-referential structures
- Concurrency: select!, join!, timeout, cancellation, retry with backoff
- Async I/O: TCP server patterns, buffer handling
- Design patterns: Async pipeline, state machine
- 35 async tests with full documentation

#### Python DSL Module (`dsl`) 🆕
- PyO3 integration for Python embedding
- Basic execution: eval_simple, eval_string, run_module
- Function calling: call_py_function, transform_pipeline
- Sandboxed execution: restricted builtins, no dangerous imports
- Configuration DSL: load_config from Python
- Example scripts: config.py, transform.py
- 11 DSL tests
- Type-safe Rust ↔ Python conversions
- Python 3.8-3.14+ support with ABI3

### Web Application (`rust-202-web`)

#### REST API (21 endpoints)
- Health & utility endpoints (2)
- Functional programming: fibonacci, closures (2)
- OOP patterns: ALL 6 GoF patterns exposed (6)
  - Singleton, Factory, Observer, Builder, Strategy, Adapter
- Differentiators: safety, performance, concurrency (3)
- Idioms: error handling (1)
- Async patterns: streams (1)
- Rust 1.75+ features: const ops, atomics (2)
- Python DSL: eval, execute, sandbox, transform (4)

#### gRPC API (3 methods)
- GetFibonacci: Unary RPC
- StreamEvents: Server-side streaming
- ExecutePattern: Pattern execution

#### Features
- Axum 0.7 framework
- Tonic 0.12 for gRPC
- Auto-generated OpenAPI 3.0 with Utoipa
- Interactive Swagger UI at `/docs`
- CORS middleware
- Structured logging with tracing
- Type-safe request/response handling
- 5 integration tests

### Infrastructure
- Workspace CI/CD with GitHub Actions (5 jobs)
  - Multi-platform library testing (Ubuntu, macOS, Windows)
  - Web app testing (REST-only, no protoc required)
  - Benchmarks and code coverage
  - Workspace integration verification
- Comprehensive test suite: 132 tests (127 library + 5 web)
- Documentation with rustdoc (100% public API)
- Clippy linting configuration
- Docker support for deployment

### Documentation
- 6 consolidated markdown guides in `doc/`
- Full API documentation with examples
- Comparison tables (Rust vs Python/Go/C)
- OpenAPI 3.0 specification
- Contributing guidelines
- Dual license (MIT/Apache-2.0)

### Coverage
- Library modules: 7/7 exposed via web API (100%)
- Overall endpoint coverage: ~55% of library capabilities
- All design patterns accessible interactively

[Unreleased]: https://github.com/KrystianMarek/rust-202/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/KrystianMarek/rust-202/releases/tag/v0.1.0

