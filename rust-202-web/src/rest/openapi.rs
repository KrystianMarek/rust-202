//! OpenAPI documentation configuration

use utoipa::OpenApi;

// Import handlers (utoipa generates __path_* symbols)
#[allow(unused_imports)]
use crate::rest::handlers::*;
// Import models for schemas (used by macro)
#[allow(unused_imports)]
use crate::rest::models::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Health & Utility
        health_check,
        execute_pattern,
        // Functional Programming
        get_fibonacci,
        demo_closures,
        // OOP Patterns
        get_singleton,
        create_notification,
        trigger_observer,
        build_user,
        compress_data,
        adapt_temperature,
        // Differentiators
        demo_safety,
        demo_performance,
        demo_concurrency,
        // Idioms
        demo_error_handling,
        // Async
        demo_async_streams,
        // Rust 1.75+
        demo_const_ops,
        demo_atomics,
    ),
    components(
        schemas(
            // Base types
            HealthResponse,
            PatternResponse,
            ErrorResponse,
            // Functional
            FibonacciResponse,
            ClosureRequest,
            ClosureResponse,
            // OOP
            SingletonResponse,
            FactoryRequest,
            FactoryResponse,
            ObserverRequest,
            ObserverResponse,
            BuilderRequest,
            BuilderResponse,
            UserData,
            StrategyRequest,
            StrategyResponse,
            AdapterRequest,
            AdapterResponse,
            // Differentiators
            SafetyResponse,
            PerformanceResponse,
            ConcurrencyRequest,
            ConcurrencyResponse,
            // Idioms
            ErrorHandlingRequest,
            ErrorHandlingResponse,
            // Async
            AsyncStreamRequest,
            AsyncStreamResponse,
            // Rust 1.75+
            ConstOpsRequest,
            ConstOpsResponse,
            AtomicOpsResponse,
        )
    ),
    tags(
        (name = "health", description = "Health check and utility endpoints"),
        (name = "functional", description = "Functional programming patterns: iterators, closures, composition"),
        (name = "oop", description = "Object-oriented patterns: Singleton, Factory, Observer, Builder, Strategy, Adapter"),
        (name = "differentiators", description = "What sets Rust apart: Safety, Performance, Concurrency"),
        (name = "idioms", description = "Rust-specific idioms: Ownership, error handling, RAII"),
        (name = "async", description = "Advanced async/await patterns: streams, traits, concurrency"),
        (name = "rust191", description = "Rust 1.75+ features: const ops, atomics, lifetimes"),
        (name = "patterns", description = "Generic pattern execution"),
    ),
    info(
        title = "rust-202 Web API",
        version = "0.1.0",
        description = "REST API showcasing the rust-202 library patterns",
        contact(
            name = "rust-202",
            url = "https://github.com/KrystianMarek/rust-202"
        ),
        license(
            name = "MIT OR Apache-2.0"
        )
    )
)]
pub struct ApiDoc;
