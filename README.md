# Rust-202: Advanced Rust Concepts Library

[![CI](https://github.com/KrystianMarek/rust-202/workflows/CI/badge.svg)](https://github.com/KrystianMarek/rust-202/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

An interactive, example-driven cheat sheet for advanced Rust concepts. This library provides practical, compilable examples demonstrating:

- 🚀 **Rust 1.75+ Features**: Latest stable Rust capabilities
- 🎯 **OOP Patterns**: Trait-based object-oriented programming
- 🔄 **Functional Programming**: Iterators, closures, and functional patterns
- 🏗️ **Design Patterns**: Gang of Four patterns adapted to Rust
- 💡 **Rust Idioms**: Ownership, error handling, RAII
- ⚡ **Performance**: What sets Rust apart from Python, Go, and C

## Quick Start

This is a **learning and reference repository**. Clone it locally to explore the examples:

```bash
# Clone the repository
git clone https://github.com/KrystianMarek/rust-202.git
cd rust-202

# Explore the code
cargo doc --open         # Browse the documentation
cargo test               # Run all tests
cargo run --example quickstart    # Run examples
```

### Using as a Reference

Browse the source code to learn patterns, or copy specific examples into your own projects. Each module is self-contained and well-documented:

```rust
// Example patterns you can learn from:
// - src/functional/iterators.rs - Zero-cost iterator patterns
// - src/oop/patterns/ - Gang of Four patterns in Rust
// - src/async/ - Modern async/await patterns
// - src/idioms/ - Rust-specific best practices
```

## Examples

Run the included examples:

```bash
# Quickstart tour
cargo run --example quickstart

# Design patterns
cargo run --example patterns

# Async demonstration (requires async-tokio feature, enabled by default)
cargo run --example async_demo
```

## Modules

### 🔧 Rust 1.75+ Features (`rust_191`)

Modern Rust capabilities including:
- Const operations and compile-time evaluation
- Advanced lifetime patterns
- Build optimization features (LLD linker)

```rust
use rust_202::rust_191::const_atomics::ConstMath;

const RESULT: u64 = ConstMath::mul_add(5, 10, 3); // Compile-time!
assert_eq!(RESULT, 53);
```

### 🎯 Object-Oriented Programming (`oop`)

Trait-based OOP patterns including:
- Composition over inheritance
- Gang of Four patterns (Singleton, Factory, Observer, Builder, Strategy, Adapter)

```rust
use rust_202::oop::patterns::{NotificationFactory, NotificationType};

let factory = NotificationFactory;
let email = factory.create(NotificationType::Email);
email.send("Hello, World!");
```

### 🔄 Functional Programming (`functional`)

FP patterns including:
- Zero-cost iterators
- Closures (Fn, FnMut, FnOnce)
- Higher-order functions

```rust
use rust_202::functional::closures::{compose, make_adder};

let add_one = |x: i32| x + 1;
let double = |x: i32| x * 2;
let add_then_double = compose(add_one, double);

assert_eq!(add_then_double(5), 12); // (5 + 1) * 2
```

### 💡 Rust Idioms (`idioms`)

Rust-specific patterns:
- Ownership and borrowing
- RAII with Drop
- Error handling with Result and ?
- Interior mutability

```rust
use rust_202::idioms::ownership::FileHandle;

{
    let handle = FileHandle::open("data.txt");
    handle.write("Hello");
} // Automatically closed via Drop
```

### ⚡ Differentiators (`differentiators`)

What makes Rust unique:
- **Safety**: No null, no data races, no UB (vs C/Python/Go)
- **Performance**: Zero-cost abstractions, no GC (vs Python/Go)
- **Concurrency**: Fearless with Send/Sync (vs all)

### 🔄 Advanced Async (`async`) *NEW!*

Cutting-edge async/await patterns:
- **Async functions & futures**: Zero-cost state machines
- **Streams**: Async iteration with backpressure
- **Async traits**: Native and macro-based
- **Pinning**: Safe self-referential structures
- **Concurrency**: select!, join!, timeout, cancellation
- **Patterns**: Pipelines, state machines, retry logic

```rust
use rust_202::differentiators::safety::ThreadSafeCounter;
use std::sync::Arc;
use std::thread;

let counter = Arc::new(ThreadSafeCounter::new());
let handles: Vec<_> = (0..10)
    .map(|_| {
        let counter = Arc::clone(&counter);
        thread::spawn(move || counter.increment())
    })
    .collect();

handles.into_iter().for_each(|h| h.join().unwrap());
assert_eq!(counter.get(), 10); // No data races!
```

## Benchmarks

Run benchmarks to see performance:

```bash
cargo bench
```

Compare zero-cost abstractions:
- Iterator chains vs manual loops
- Generic monomorphization
- RAII vs manual cleanup

## Documentation

Build and browse the documentation locally:

```bash
cargo doc --open
```

The documentation includes:
- Comprehensive API documentation with examples
- "Why?" sections explaining design decisions
- Comparisons to Python, Go, and C
- Runnable code examples

## Comparison: Rust vs Python/Go/C

| Feature | Rust | Python | Go | C |
|---------|------|--------|-----|---|
| Memory Safety | ✅ Compile-time | ⚠️ Runtime (GC) | ⚠️ Runtime (GC) | ❌ Manual |
| Null Safety | ✅ Option<T> | ❌ None errors | ❌ nil panics | ❌ NULL segfaults |
| Data Race Freedom | ✅ Compile-time | 🔒 GIL (no parallelism) | ⚠️ Runtime detector | ❌ UB |
| Zero-Cost Abstractions | ✅ Yes | ❌ No | ⚠️ Limited | ✅ Yes (manual) |
| Performance | ⚡ Native | 🐌 Interpreted | ⚡ Native (with GC) | ⚡ Native |
| Learning Curve | 📈 Steep | 📉 Gentle | 📊 Moderate | 📈 Steep |

## Contributing

This is a learning resource and reference implementation. Feel free to:
- ⭐ Star the repository if you find it useful
- 🐛 Open issues for corrections or suggestions
- 🔀 Fork and modify for your own learning
- 📖 Use code examples in your projects (MIT/Apache-2.0 licensed)

See [doc/CONTRIBUTING.md](doc/CONTRIBUTING.md) for detailed guidelines if you'd like to contribute improvements.

## License

MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

## Project Goals

This repository serves as:
1. **Educational Resource** - Learn advanced Rust concepts with working examples
2. **Reference Implementation** - See how to implement patterns idiomatically
3. **Comparison Guide** - Understand what sets Rust apart from Python, Go, and C
4. **Code Library** - Copy patterns into your own projects

**Note**: This is not published to crates.io. It's designed for local exploration and learning.

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Playground](https://play.rust-lang.org/)
- [This Week in Rust](https://this-week-in-rust.org/)

## Acknowledgments

Inspired by:
- The Rust community
- Gang of Four Design Patterns
- Functional programming literature
- Real-world production Rust codebases
