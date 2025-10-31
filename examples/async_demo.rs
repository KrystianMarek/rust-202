//! Async Rust demonstration example

#[cfg(feature = "async-tokio")]
use rust_202::r#async::{
    basics::{async_hello, chain_async_ops},
    concurrency::{spawn_tasks, timeout_example},
    patterns::{simple_pipeline, ConnectionStateMachine},
    streams::{CounterStream, FibonacciStream},
    traits::{Repository, InMemoryRepo},
};

#[cfg(feature = "async-tokio")]
use futures::StreamExt;

#[cfg(feature = "async-tokio")]
#[tokio::main]
async fn main() {
    println!("=== Async Rust Demo ===\n");

    // 1. Basic async functions
    println!("1. Basic Async:");
    let greeting = async_hello("Async Rust").await;
    println!("   {}", greeting);

    let result = chain_async_ops(10).await;
    println!("   {}\n", result);

    // 2. Streams
    println!("2. Async Streams:");
    let counter = CounterStream::new(5);
    let values: Vec<u32> = counter.collect().await;
    println!("   Counter: {:?}", values);

    let fib = FibonacciStream::new();
    let fibs: Vec<u64> = fib.take(8).collect().await;
    println!("   Fibonacci: {:?}\n", fibs);

    // 3. Concurrency
    println!("3. Concurrent Tasks:");
    let sum = spawn_tasks(vec![1, 2, 3, 4, 5]).await;
    println!("   Sum of spawned tasks: {}", sum);

    match timeout_example(30).await {
        Ok(msg) => println!("   Timeout success: {}", msg),
        Err(e) => println!("   Timeout failed: {}", e),
    }
    println!();

    // 4. Async traits
    println!("4. Async Traits:");
    let mut repo = InMemoryRepo::new();
    repo.save(1, "Alice".to_string()).await.unwrap();
    repo.save(2, "Bob".to_string()).await.unwrap();

    if let Some(user) = repo.find_by_id(1).await {
        println!("   Found user: {}", user);
    }
    println!();

    // 5. Async patterns
    println!("5. Async Patterns:");
    let pipeline_result = simple_pipeline(vec![1, 2, 3, 4, 5, 6]).await;
    println!("   Pipeline result: {:?}", pipeline_result);

    let mut state_machine = ConnectionStateMachine::new();
    println!("   Initial state: {:?}", state_machine.current_state());
    state_machine.connect().await;
    println!("   After connect: {:?}", state_machine.current_state());
    println!();

    println!("=== End of Async Demo ===");
}

#[cfg(not(feature = "async-tokio"))]
fn main() {
    println!("This example requires the 'async-tokio' feature.");
    println!("Run with: cargo run --example async_demo --features async-tokio");
}

