//! Quickstart example demonstrating basic usage of rust-202

use rust_202::differentiators::safety::ThreadSafeCounter;
use rust_202::functional::iterators::fibonacci_sequence;
use rust_202::idioms::error_handling::ResultExample;
use rust_202::oop::composition::{Circle, Drawable, HasArea};
use std::sync::Arc;
use std::thread;

fn main() {
    println!("=== Rust-202 Quickstart ===\n");

    // Functional programming: Fibonacci sequence
    println!("1. Functional Programming - Fibonacci:");
    let fibs: Vec<u64> = fibonacci_sequence().take(10).collect();
    println!("   First 10 Fibonacci numbers: {:?}\n", fibs);

    // OOP with traits: Circle
    println!("2. OOP with Traits - Circle:");
    let circle = Circle::new(5.0);
    print!("   ");
    circle.draw();
    println!("   Area: {:.2}\n", circle.area());

    // Error handling with Result
    println!("3. Error Handling - Safe Division:");
    match ResultExample::safe_divide(10, 2) {
        Ok(result) => println!("   10 / 2 = {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    match ResultExample::safe_divide(10, 0) {
        Ok(result) => println!("   10 / 0 = {}", result),
        Err(e) => println!("   Error: {}\n", e),
    }

    // Fearless concurrency
    println!("4. Fearless Concurrency - Thread-Safe Counter:");
    let counter = Arc::new(ThreadSafeCounter::new());
    let mut handles = vec![];

    for i in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            println!("   Thread {} incrementing counter", i);
            counter_clone.increment();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("   Final count: {}\n", counter.get());

    println!("=== End of Quickstart ===");
}
