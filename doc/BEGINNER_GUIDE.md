# Rust-202 Beginner's Guide

Welcome to rust-202! This guide helps you navigate the codebase if you're new to Rust.

---

## 🎯 Start Here

### Recommended Learning Path

1. **Read `rust-202-lib/src/idioms/`** - Core Rust concepts
   - `ownership.rs` - How Rust manages memory without GC
   - `error_handling.rs` - How Rust handles errors without exceptions

2. **Then `rust-202-lib/src/oop/`** - OOP without inheritance
   - `composition.rs` - How traits replace classes
   - `patterns/singleton.rs` - Thread-safe singletons

3. **Next `rust-202-lib/src/functional/`** - Functional programming
   - `iterators.rs` - Zero-cost iteration
   - `closures.rs` - Functions as values

4. **Advanced: `rust-202-lib/src/async/`** - Modern concurrency
   - `basics/async_fn.rs` - Async/await fundamentals

5. **Expert: `rust-202-lib/src/dsl/`** - Language embedding
   - `python/embed.rs` - Running Python from Rust

---

## 📚 Key Rust Concepts Explained

### Ownership (The Big Idea!)

**What**: Each value has exactly one owner. When the owner goes out of scope, the value is dropped (freed).

```rust
{
    let s = String::from("hello");  // s owns the String
}  // s goes out of scope → String is dropped (freed)
   // No garbage collector needed!
```

**Files**: `src/idioms/ownership.rs` (heavily annotated!)

### Borrowing (Sharing Without Owning)

**What**: References let you use values without taking ownership.

```rust
fn print_length(s: &String) {  // &String = borrow (don't take ownership)
    println!("{}", s.len());
}  // Borrow ends here

let my_string = String::from("hello");
print_length(&my_string);  // Lend my_string to function
println!("{}", my_string);  // Still valid! We still own it
```

**Rules**:
- Many `&T` (immutable borrows) OR one `&mut T` (mutable borrow)
- No dangling references (compiler prevents!)

### Result & Option (No Null, No Exceptions!)

**Result<T, E>**: For operations that can fail

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Can't divide by zero".to_string())  // Error case
    } else {
        Ok(a / b)  // Success case
    }
}

// Must handle both cases!
match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}
```

**Option<T>**: For values that might not exist

```rust
fn find_user(id: u32) -> Option<User> {
    if user_exists(id) {
        Some(user)  // Found!
    } else {
        None  // Not found (safe null!)
    }
}
```

**Files**: `src/idioms/error_handling.rs` (annotated!)

### Traits (Interfaces on Steroids)

**What**: Traits define shared behavior. Like interfaces but more powerful.

```rust
// Define a trait
trait Drawable {
    fn draw(&self);  // Method signature
}

// Implement trait for a type
impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle");
    }
}

// Use trait
fn render(item: &dyn Drawable) {  // dyn = dynamic dispatch
    item.draw();
}
```

**Files**: `src/oop/composition.rs`

---

## 🔍 Understanding the Annotations

### Module-Level Comments (//!)

```rust
//! # Module Name
//!
//! Module description
```

These document the entire module/file.

### Item Comments (///)

```rust
/// Function description
///
/// ## Why?
/// Explanation...
///
/// ## For Beginners
/// Simplified explanation with examples
pub fn my_function() { }
```

These document specific functions, structs, etc.

### Inline Comments (//)

```rust
let x = 5;  // This is an inline comment
// This explains the next line
let y = 10;
```

### Special Annotations

```rust
#[cfg(feature = "async-tokio")]  // Conditional compilation
pub mod async;  // Only included if feature enabled

#[derive(Debug, Clone)]  // Auto-implement traits
pub struct MyStruct;

#[allow(dead_code)]  // Suppress specific warning
fn unused_function() {}
```

---

## 🗺️ Code Navigation Guide

### Finding Examples

Each module has a **"Why?"** and **"For Beginners"** section:

```rust
/// ## Why?
/// Explains why this pattern is useful

/// ## For Beginners
/// Simplified explanation for newcomers

/// ## Example
/// ```rust
/// // Working code example
/// ```
```

### Understanding Syntax

#### Type Annotations

```rust
let x: i32 = 5;          // i32 = 32-bit integer
let s: String = ...;     // String = owned string
let slice: &str = ...;   // &str = string slice (borrow)
let vec: Vec<i32> = ...; // Vec = growable array
```

#### Function Signatures

```rust
pub fn example(
    data: Vec<i32>,      // Takes ownership of vector
    flag: &bool,         // Borrows bool immutably
    output: &mut String, // Borrows String mutably
) -> Result<i32, String> // Returns Result (success or error)
{
    // Function body
}
```

#### Common Patterns

```rust
// Match expression (like switch but exhaustive)
match value {
    Some(x) => println!("Got {}", x),
    None => println!("Got nothing"),
}

// If let (match for single pattern)
if let Some(x) = maybe_value {
    println!("Got {}", x);
}

// Iterator chain (functional style)
vec.iter()           // Create iterator
   .filter(|x| x > 0) // Keep positive numbers
   .map(|x| x * 2)    // Double them
   .collect()         // Collect into Vec
```

---

## 📖 Glossary of Rust Terms

| Term | Meaning | Example |
|------|---------|---------|
| **Ownership** | Each value has one owner | `let x = vec![];` |
| **Borrow** | Reference without ownership | `&x` or `&mut x` |
| **Lifetime** | How long a reference is valid | `'a` in `&'a str` |
| **Trait** | Interface/capability | `impl Trait for Type` |
| **Enum** | Tagged union | `Option`, `Result` |
| **Match** | Pattern matching | `match x { ... }` |
| **Closure** | Anonymous function | `|x| x * 2` |
| **Macro** | Code generation | `println!()`, `vec![]` |
| **Crate** | Package/library | `rust_202` |
| **Module** | Code organization | `mod oop` |

---

## 💡 Common Beginner Mistakes

### 1. Trying to Use Moved Values

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 is MOVED to s2
println!("{}", s1);  // ERROR! s1 no longer valid
```

**Fix**: Clone if you need both, or use borrowing

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // Explicit copy
println!("{} {}", s1, s2);  // OK!

// OR borrow
let s1 = String::from("hello");
let s2 = &s1;  // Borrow, don't move
println!("{} {}", s1, s2);  // OK!
```

### 2. Forgetting to Handle Errors

```rust
let result = may_fail();  // ERROR! Result not handled
```

**Fix**: Use match, if let, or ?

```rust
match may_fail() {
    Ok(val) => use_value(val),
    Err(e) => handle_error(e),
}

// OR
let val = may_fail()?;  // Propagate error upward
```

### 3. Trying to Mutate Immutable Values

```rust
let x = 5;
x = 6;  // ERROR! x is immutable by default
```

**Fix**: Use `mut`

```rust
let mut x = 5;  // mut = mutable
x = 6;  // OK!
```

### 4. Borrowing Issues

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];  // Immutable borrow
v.push(4);  // ERROR! Can't mutate while borrowed
```

**Fix**: End borrow before mutation

```rust
let mut v = vec![1, 2, 3];
{
    let first = &v[0];
    println!("{}", first);
}  // Borrow ends here
v.push(4);  // OK now!
```

---

## 🚀 Running Examples

### Library Examples

```bash
# Quickstart (covers basics)
cargo run -p rust-202 --example quickstart

# Async patterns
cargo run -p rust-202 --example async_demo

# Design patterns
cargo run -p rust-202 --example patterns
```

### Interactive Web API

```bash
# Start server
cargo run -p rust-202-web --no-default-features

# Visit Swagger UI
open http://localhost:3000/docs

# Try endpoints interactively!
```

---

## 📝 Where to Find Specific Concepts

| Concept | File | Key Function |
|---------|------|--------------|
| **Ownership** | `idioms/ownership.rs` | `transfer_ownership()` |
| **Borrowing** | `idioms/ownership.rs` | `borrow_immutably()` |
| **Error Handling** | `idioms/error_handling.rs` | `safe_divide()` |
| **Result Type** | `idioms/error_handling.rs` | `parse_and_double()` |
| **Option Type** | `idioms/error_handling.rs` | `safe_get()` |
| **? Operator** | `idioms/error_handling.rs` | Examples throughout |
| **Traits** | `oop/composition.rs` | `Drawable` trait |
| **Iterators** | `functional/iterators.rs` | `fibonacci_sequence()` |
| **Closures** | `functional/closures.rs` | `fn_example()` |
| **Async/Await** | `async/basics/async_fn.rs` | `async_hello()` |

---

## 🎓 Learning Resources

### Official Rust Resources

1. **[The Rust Book](https://doc.rust-lang.org/book/)** - Start here!
2. **[Rust by Example](https://doc.rust-lang.org/rust-by-example/)** - Learn by doing
3. **[Rustlings](https://github.com/rust-lang/rustlings)** - Small exercises

### Using This Repository

1. **Read the code** - Heavily annotated for beginners
2. **Run the examples** - See patterns in action
3. **Try the web API** - Interactive learning via Swagger UI
4. **Modify and experiment** - Break things and learn!

---

## 🔑 Key Takeaways

### What Makes Rust Different

1. **No Garbage Collector**
   - Memory freed immediately when owner goes out of scope
   - Predictable performance, no GC pauses

2. **No Null Pointers**
   - Use `Option<T>` instead of null
   - Compiler forces you to handle the None case

3. **No Data Races**
   - Borrow checker prevents concurrent access
   - Thread safety guaranteed at compile-time

4. **No Exceptions**
   - Use `Result<T, E>` for errors
   - Explicit, compile-time checked error handling

5. **Zero-Cost Abstractions**
   - High-level code compiles to efficient machine code
   - Iterators, closures - all optimized away!

---

## 💬 Getting Help

- **Documentation**: `cargo doc -p rust-202 --open`
- **Tests**: Look at `#[cfg(test)] mod tests` sections
- **Examples**: See `rust-202-lib/examples/`
- **Web API**: Try patterns interactively at `/docs`

---

## ✅ You're Ready When...

- ✅ You understand ownership and borrowing
- ✅ You can use Result and Option confidently
- ✅ You know when to use `&T` vs `&mut T` vs `T`
- ✅ You understand what traits are
- ✅ You can read basic iterator chains

Then you're ready to explore the advanced modules!

---

**Remember**: Rust has a steep learning curve, but the safety and performance are worth it. Take your time, read the annotations, run the examples, and don't hesitate to experiment!

Happy learning! 🦀

