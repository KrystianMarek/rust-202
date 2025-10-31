# Rust-202: Advanced Rust Concepts Library

[![CI](https://github.com/KrystianMarek/rust-202/workflows/CI/badge.svg)](https://github.com/KrystianMarek/rust-202/actions)
[![Crates.io](https://img.shields.io/crates/v/rust-202.svg)](https://crates.io/crates/rust-202)
[![Documentation](https://docs.rs/rust-202/badge.svg)](https://docs.rs/rust-202)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

An interactive, example-driven cheat sheet for advanced Rust concepts. This library provides practical, compilable examples demonstrating:

- 🚀 **Rust 1.75+ Features**: Latest stable Rust capabilities
- 🎯 **OOP Patterns**: Trait-based object-oriented programming
- 🔄 **Functional Programming**: Iterators, closures, and functional patterns
- 🏗️ **Design Patterns**: Gang of Four patterns adapted to Rust
- 💡 **Rust Idioms**: Ownership, error handling, RAII
- ⚡ **Performance**: What sets Rust apart from Python, Go, and C

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-202 = "0.1"
```

Then use in your code:

```rust
use rust_202::functional::iterators::fibonacci_sequence;
use rust_202::oop::composition::{Circle, Drawable, HasArea};
use rust_202::idioms::error_handling::ResultExample;

fn main() {
    // Functional programming
    let fibs: Vec<u64> = fibonacci_sequence().take(10).collect();
    println!("First 10 Fibonacci numbers: {:?}", fibs);

    // OOP with traits
    let circle = Circle::new(5.0);
    circle.draw();
    println!("Circle area: {}", circle.area());

    // Error handling
    match ResultExample::safe_divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
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

Full documentation available at [docs.rs/rust-202](https://docs.rs/rust-202).

Build locally:

```bash
cargo doc --open
```

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

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

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

