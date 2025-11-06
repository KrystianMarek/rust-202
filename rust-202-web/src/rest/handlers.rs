//! REST API handlers

use axum::{extract::Query, http::StatusCode, response::Json};
use serde::Deserialize;
use serde_json::json;
use std::time::Instant;
use utoipa::IntoParams;

use rust_202::differentiators::concurrency::LockFreeCounter;
use rust_202::differentiators::performance::ZeroCostIterators;
use rust_202::differentiators::safety::{BorrowCheckerExample, ThreadSafeCounter};
use rust_202::functional::closures::ClosureExamples;
use rust_202::functional::iterators::fibonacci_sequence;
use rust_202::idioms::error_handling::ResultExample;
use rust_202::oop::patterns::adapter::{LegacyThermometer, TemperatureSensor, ThermometerAdapter};
use rust_202::oop::patterns::builder::UserBuilder;
use rust_202::oop::patterns::factory::{NotificationFactory, NotificationType};
use rust_202::oop::patterns::observer::{ConsoleLogger, Event, EventSubject};
use rust_202::oop::patterns::singleton::Config;
use rust_202::oop::patterns::strategy::{
    CompressionContext, GzipCompression, NoCompression, ZipCompression,
};
use rust_202::rust_191::const_atomics::{AtomicCounter, ConstMath};

#[cfg(feature = "python-dsl")]
use rust_202::dsl::python::{eval_string, run_module, sandboxed_eval, transform_pipeline};

#[cfg(feature = "async-tokio")]
use futures::StreamExt;
#[cfg(feature = "async-tokio")]
use rust_202::r#async::streams::{CounterStream, FibonacciStream};

use std::sync::Arc;
use std::thread;

pub use super::models::*;

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    ),
    tag = "health"
)]
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Fibonacci sequence parameters
#[derive(Debug, Deserialize, IntoParams)]
pub struct FibonacciParams {
    /// Number of Fibonacci numbers to generate
    #[param(example = 10, minimum = 1, maximum = 100)]
    count: Option<usize>,
}

/// Get Fibonacci sequence
///
/// Demonstrates rust-202's functional iterators with zero-cost abstractions
#[utoipa::path(
    get,
    path = "/functional/fibonacci",
    params(FibonacciParams),
    responses(
        (status = 200, description = "Fibonacci sequence generated", body = FibonacciResponse),
        (status = 400, description = "Invalid parameters", body = ErrorResponse)
    ),
    tag = "functional"
)]
pub async fn get_fibonacci(Query(params): Query<FibonacciParams>) -> Json<FibonacciResponse> {
    let count = params.count.unwrap_or(10).min(100);
    let numbers: Vec<u64> = fibonacci_sequence().take(count).collect();

    Json(FibonacciResponse { numbers, count })
}

/// Get singleton instance
///
/// Demonstrates rust-202's thread-safe Singleton pattern
#[utoipa::path(
    get,
    path = "/oop/singleton",
    responses(
        (status = 200, description = "Singleton instance retrieved", body = SingletonResponse)
    ),
    tag = "oop"
)]
pub async fn get_singleton() -> Json<SingletonResponse> {
    // Initialize some config
    Config::set("api_version", "1.0");
    Config::set("environment", "production");

    let mut config = std::collections::HashMap::new();
    if let Some(ver) = Config::get("api_version") {
        config.insert("api_version".to_string(), ver);
    }
    if let Some(env) = Config::get("environment") {
        config.insert("environment".to_string(), env);
    }

    Json(SingletonResponse {
        instance_id: "singleton-001".to_string(),
        config,
    })
}

/// Execute a design pattern
#[utoipa::path(
    post,
    path = "/patterns/{pattern_name}",
    params(
        ("pattern_name" = String, Path, description = "Pattern name (singleton, factory, observer)")
    ),
    responses(
        (status = 200, description = "Pattern executed", body = PatternResponse),
        (status = 404, description = "Pattern not found", body = ErrorResponse)
    ),
    tag = "patterns"
)]
pub async fn execute_pattern(
    axum::extract::Path(pattern_name): axum::extract::Path<String>,
) -> Result<Json<PatternResponse>, (StatusCode, Json<ErrorResponse>)> {
    let start = Instant::now();

    let result = match pattern_name.as_str() {
        "singleton" => {
            Config::set("test", "value");
            json!({
                "pattern": "Singleton",
                "instance": "configured"
            })
        }
        "fibonacci" => {
            let fibs: Vec<u64> = fibonacci_sequence().take(5).collect();
            json!({
                "pattern": "Fibonacci Iterator",
                "values": fibs
            })
        }
        _ => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: format!("Pattern '{}' not found", pattern_name),
                    code: 404,
                }),
            ));
        }
    };

    let execution_time = start.elapsed();

    Ok(Json(PatternResponse {
        pattern: pattern_name,
        result,
        execution_time_us: execution_time.as_micros(),
    }))
}

/// Create notification using Factory pattern
#[utoipa::path(
    post,
    path = "/oop/factory",
    request_body = FactoryRequest,
    responses(
        (status = 200, description = "Notification created", body = FactoryResponse),
        (status = 400, description = "Invalid type", body = ErrorResponse)
    ),
    tag = "oop"
)]
pub async fn create_notification(
    Json(req): Json<FactoryRequest>,
) -> Result<Json<FactoryResponse>, (StatusCode, Json<ErrorResponse>)> {
    let factory = NotificationFactory;

    let notification_type = match req.notification_type.as_str() {
        "Email" => NotificationType::Email,
        "Sms" => NotificationType::Sms,
        "Push" => NotificationType::Push,
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("Unknown notification type: {}", req.notification_type),
                    code: 400,
                }),
            ));
        }
    };

    let notification = factory.create(notification_type);

    Ok(Json(FactoryResponse {
        object_type: notification.notification_type().to_string(),
        description: format!("Created {} notification", notification.notification_type()),
    }))
}

/// Trigger Observer pattern event
#[utoipa::path(
    post,
    path = "/oop/observer",
    request_body = ObserverRequest,
    responses(
        (status = 200, description = "Event triggered", body = ObserverResponse)
    ),
    tag = "oop"
)]
pub async fn trigger_observer(Json(req): Json<ObserverRequest>) -> Json<ObserverResponse> {
    let mut subject = EventSubject::new();
    subject.attach(Box::new(ConsoleLogger));

    let event = match req.event_type.as_str() {
        "UserLoggedIn" => Event::UserLoggedIn(req.data),
        "UserLoggedOut" => Event::UserLoggedOut(req.data),
        _ => Event::DataUpdated(req.data),
    };

    subject.notify(event.clone());
    let count = subject.observer_count();

    Json(ObserverResponse {
        observers_notified: count,
        event: format!("{:?}", event),
    })
}

/// Build user using Builder pattern
#[utoipa::path(
    post,
    path = "/oop/builder",
    request_body = BuilderRequest,
    responses(
        (status = 200, description = "User built", body = BuilderResponse)
    ),
    tag = "oop"
)]
pub async fn build_user(Json(req): Json<BuilderRequest>) -> Json<BuilderResponse> {
    let mut builder = UserBuilder::new(&req.username);

    if let Some(email) = &req.email {
        builder = builder.email(email);
    }

    if let Some(age) = req.age {
        builder = builder.age(age);
    }

    let user = builder.build();

    Json(BuilderResponse {
        user: UserData {
            username: user.username().to_string(),
            email: user.email().map(|s| s.to_string()),
            age: user.age(),
            is_active: user.is_active(),
        },
    })
}

/// Compress data using Strategy pattern
#[utoipa::path(
    post,
    path = "/oop/strategy",
    request_body = StrategyRequest,
    responses(
        (status = 200, description = "Data compressed", body = StrategyResponse)
    ),
    tag = "oop"
)]
pub async fn compress_data(Json(req): Json<StrategyRequest>) -> Json<StrategyResponse> {
    let strategy: Box<dyn rust_202::oop::patterns::strategy::CompressionStrategy> =
        match req.strategy.as_str() {
            "Gzip" => Box::new(GzipCompression),
            "Zip" => Box::new(ZipCompression),
            _ => Box::new(NoCompression),
        };

    let context = CompressionContext::new(strategy);
    let compressed = context.compress(&req.data);

    let ratio = compressed.len() as f64 / req.data.len() as f64;
    let compressed_b64 = base64_encode(&compressed);

    Json(StrategyResponse {
        strategy: req.strategy,
        compressed: compressed_b64,
        ratio,
    })
}

/// Adapt temperature using Adapter pattern
#[utoipa::path(
    post,
    path = "/oop/adapter",
    request_body = AdapterRequest,
    responses(
        (status = 200, description = "Temperature converted", body = AdapterResponse)
    ),
    tag = "oop"
)]
pub async fn adapt_temperature(Json(req): Json<AdapterRequest>) -> Json<AdapterResponse> {
    let legacy = LegacyThermometer::new(req.fahrenheit);
    let adapter = ThermometerAdapter::new(legacy);

    let celsius = adapter.celsius();
    let kelvin = adapter.kelvin();

    Json(AdapterResponse {
        celsius,
        kelvin,
        fahrenheit: req.fahrenheit,
    })
}

/// Demonstrate safety features
#[utoipa::path(
    get,
    path = "/differentiators/safety",
    responses(
        (status = 200, description = "Safety examples", body = SafetyResponse)
    ),
    tag = "differentiators"
)]
pub async fn demo_safety() -> Json<SafetyResponse> {
    let borrow_result = BorrowCheckerExample::safe_reference_example();

    let counter = Arc::new(ThreadSafeCounter::new());
    let mut handles = vec![];
    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            counter_clone.increment();
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }

    Json(SafetyResponse {
        borrow_check: format!("Safe reference returned: {}", borrow_result),
        null_safety: "Option<T> prevents null pointer dereferences".to_string(),
        thread_safety: format!(
            "Thread-safe counter result: {} (no data races!)",
            counter.get()
        ),
    })
}

/// Demonstrate performance features
#[utoipa::path(
    get,
    path = "/differentiators/performance",
    responses(
        (status = 200, description = "Performance examples", body = PerformanceResponse)
    ),
    tag = "differentiators"
)]
pub async fn demo_performance() -> Json<PerformanceResponse> {
    let data = vec![1, 2, 3, 4, 5];
    let sum1 = ZeroCostIterators::sum_squares(&data);
    let sum2 = ZeroCostIterators::sum_squares_manual(&data);

    Json(PerformanceResponse {
        zero_cost: format!(
            "Iterator ({}) == Manual loop ({}) - Same performance!",
            sum1, sum2
        ),
        no_gc: "Memory freed deterministically, no GC pauses".to_string(),
        inline_opt: "Functions inlined at compile-time for zero overhead".to_string(),
    })
}

/// Demonstrate concurrency
#[utoipa::path(
    post,
    path = "/differentiators/concurrency",
    request_body = ConcurrencyRequest,
    responses(
        (status = 200, description = "Concurrency demo", body = ConcurrencyResponse)
    ),
    tag = "differentiators"
)]
pub async fn demo_concurrency(Json(req): Json<ConcurrencyRequest>) -> Json<ConcurrencyResponse> {
    let start = Instant::now();
    let counter = Arc::new(LockFreeCounter::new());
    let mut handles = vec![];

    for _ in 0..req.thread_count {
        let counter_clone = Arc::clone(&counter);
        let increments = req.increments;
        handles.push(thread::spawn(move || {
            for _ in 0..increments {
                counter_clone.increment();
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let execution_time = start.elapsed();

    Json(ConcurrencyResponse {
        final_count: counter.get() as u32,
        threads: req.thread_count,
        execution_time_us: execution_time.as_micros(),
    })
}

/// Demonstrate closures
#[utoipa::path(
    post,
    path = "/functional/closures",
    request_body = ClosureRequest,
    responses(
        (status = 200, description = "Closure applied", body = ClosureResponse)
    ),
    tag = "functional"
)]
pub async fn demo_closures(Json(req): Json<ClosureRequest>) -> Json<ClosureResponse> {
    let result = ClosureExamples::fn_example(req.numbers.clone(), req.offset);

    Json(ClosureResponse {
        input: req.numbers,
        output: result,
        closure_type: "Fn (immutable borrow)".to_string(),
    })
}

/// Demonstrate error handling
#[utoipa::path(
    post,
    path = "/idioms/error-handling",
    request_body = ErrorHandlingRequest,
    responses(
        (status = 200, description = "Division result", body = ErrorHandlingResponse)
    ),
    tag = "idioms"
)]
pub async fn demo_error_handling(
    Json(req): Json<ErrorHandlingRequest>,
) -> Json<ErrorHandlingResponse> {
    match ResultExample::safe_divide(req.a, req.b) {
        Ok(result) => Json(ErrorHandlingResponse {
            result: Some(result),
            error: None,
            pattern: "Result<T, E> with ? operator".to_string(),
        }),
        Err(e) => Json(ErrorHandlingResponse {
            result: None,
            error: Some(e),
            pattern: "Result<T, E> with ? operator".to_string(),
        }),
    }
}

/// Demonstrate async streams
#[cfg(feature = "async-tokio")]
#[utoipa::path(
    post,
    path = "/async/streams",
    request_body = AsyncStreamRequest,
    responses(
        (status = 200, description = "Async stream generated", body = AsyncStreamResponse)
    ),
    tag = "async"
)]
pub async fn demo_async_streams(Json(req): Json<AsyncStreamRequest>) -> Json<AsyncStreamResponse> {
    let values: Vec<u64> = match req.stream_type.as_str() {
        "fibonacci" => FibonacciStream::new().take(req.count).collect().await,
        "counter" => CounterStream::new(req.count as u32)
            .collect::<Vec<u32>>()
            .await
            .into_iter()
            .map(|x| x as u64)
            .collect(),
        _ => (0..req.count as u64).collect(),
    };

    Json(AsyncStreamResponse {
        stream_type: req.stream_type,
        values,
        count: req.count,
    })
}

/// Dummy async handler for non-async-tokio builds
#[cfg(not(feature = "async-tokio"))]
#[utoipa::path(
    post,
    path = "/async/streams",
    request_body = AsyncStreamRequest,
    responses(
        (status = 501, description = "Async features not enabled", body = ErrorResponse)
    ),
    tag = "async"
)]
pub async fn demo_async_streams(
    _req: Json<AsyncStreamRequest>,
) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ErrorResponse {
            error: "Async features not enabled. Build with --features async-tokio".to_string(),
            code: 501,
        }),
    )
}

/// Demonstrate const operations
#[utoipa::path(
    post,
    path = "/rust191/const-ops",
    request_body = ConstOpsRequest,
    responses(
        (status = 200, description = "Const operation result", body = ConstOpsResponse)
    ),
    tag = "rust191"
)]
pub async fn demo_const_ops(Json(req): Json<ConstOpsRequest>) -> Json<ConstOpsResponse> {
    let result = ConstMath::mul_add(req.a, req.b, req.c);

    Json(ConstOpsResponse {
        result,
        formula: format!("({} * {}) + {} = {}", req.a, req.b, req.c, result),
        compile_time: true,
    })
}

/// Demonstrate atomic operations
#[utoipa::path(
    get,
    path = "/rust191/atomics",
    responses(
        (status = 200, description = "Atomic operations demo", body = AtomicOpsResponse)
    ),
    tag = "rust191"
)]
pub async fn demo_atomics() -> Json<AtomicOpsResponse> {
    let counter = AtomicCounter::new();

    counter.increment();
    counter.increment();
    counter.increment();

    let value = counter.get();

    Json(AtomicOpsResponse {
        value,
        operations: "increment() called 3 times atomically".to_string(),
        thread_safe: true,
    })
}

/// Evaluate Python expression
#[cfg(feature = "python-dsl")]
#[utoipa::path(
    post,
    path = "/dsl/eval",
    request_body = PythonEvalRequest,
    responses(
        (status = 200, description = "Python expression evaluated", body = PythonEvalResponse),
        (status = 400, description = "Python error", body = ErrorResponse)
    ),
    tag = "dsl"
)]
pub async fn python_eval(
    Json(req): Json<PythonEvalRequest>,
) -> Result<Json<PythonEvalResponse>, (StatusCode, Json<ErrorResponse>)> {
    match eval_string(&req.expression) {
        Ok(result) => Ok(Json(PythonEvalResponse {
            result,
            expression: req.expression,
        })),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Python error: {}", e),
                code: 400,
            }),
        )),
    }
}

/// Execute Python script (file upload simulation)
#[cfg(feature = "python-dsl")]
#[utoipa::path(
    post,
    path = "/dsl/execute",
    request_body = PythonScriptRequest,
    responses(
        (status = 200, description = "Python script executed", body = PythonScriptResponse),
        (status = 400, description = "Execution error", body = ErrorResponse)
    ),
    tag = "dsl"
)]
pub async fn python_execute(Json(req): Json<PythonScriptRequest>) -> Json<PythonScriptResponse> {
    let module_name = req.module_name.unwrap_or_else(|| "uploaded".to_string());

    match run_module(&req.source, &module_name) {
        Ok(result) => Json(PythonScriptResponse {
            result,
            success: true,
            error: None,
        }),
        Err(e) => Json(PythonScriptResponse {
            result: String::new(),
            success: false,
            error: Some(format!("{}", e)),
        }),
    }
}

/// Execute Python in sandbox (restricted environment)
#[cfg(feature = "python-dsl")]
#[utoipa::path(
    post,
    path = "/dsl/sandbox",
    request_body = PythonSandboxRequest,
    responses(
        (status = 200, description = "Sandboxed evaluation successful", body = PythonSandboxResponse),
        (status = 400, description = "Sandbox violation or error", body = ErrorResponse)
    ),
    tag = "dsl"
)]
pub async fn python_sandbox(
    Json(req): Json<PythonSandboxRequest>,
) -> Result<Json<PythonSandboxResponse>, (StatusCode, Json<ErrorResponse>)> {
    match sandboxed_eval(&req.expression) {
        Ok(result) => Ok(Json(PythonSandboxResponse {
            result,
            expression: req.expression,
            sandbox_note: "Executed in restricted environment (no imports, limited builtins)".to_string(),
        })),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Sandbox error: {}", e),
                code: 400,
            }),
        )),
    }
}

/// Transform data using Python
#[cfg(feature = "python-dsl")]
#[utoipa::path(
    post,
    path = "/dsl/transform",
    request_body = PythonTransformRequest,
    responses(
        (status = 200, description = "Data transformed", body = PythonTransformResponse),
        (status = 400, description = "Transform error", body = ErrorResponse)
    ),
    tag = "dsl"
)]
pub async fn python_transform(
    Json(req): Json<PythonTransformRequest>,
) -> Result<Json<PythonTransformResponse>, (StatusCode, Json<ErrorResponse>)> {
    match transform_pipeline(&req.script, req.data.clone()) {
        Ok(output) => Ok(Json(PythonTransformResponse {
            input: req.data,
            output,
            description: "Data transformed using Python pipeline".to_string(),
        })),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Transform error: {}", e),
                code: 400,
            }),
        )),
    }
}

// Fallback handlers for when python-dsl feature is disabled
#[cfg(not(feature = "python-dsl"))]
#[utoipa::path(
    post,
    path = "/dsl/eval",
    request_body = PythonEvalRequest,
    responses(
        (status = 501, description = "Python DSL not enabled", body = ErrorResponse)
    ),
    tag = "dsl"
)]
pub async fn python_eval(
    _req: Json<PythonEvalRequest>,
) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ErrorResponse {
            error: "Python DSL feature not enabled. Build with --features python-dsl".to_string(),
            code: 501,
        }),
    )
}

#[cfg(not(feature = "python-dsl"))]
pub async fn python_execute(_req: Json<PythonScriptRequest>) -> Json<PythonScriptResponse> {
    Json(PythonScriptResponse {
        result: String::new(),
        success: false,
        error: Some("Python DSL not enabled".to_string()),
    })
}

#[cfg(not(feature = "python-dsl"))]
pub async fn python_sandbox(
    _req: Json<PythonSandboxRequest>,
) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ErrorResponse {
            error: "Python DSL not enabled".to_string(),
            code: 501,
        }),
    )
}

#[cfg(not(feature = "python-dsl"))]
pub async fn python_transform(
    _req: Json<PythonTransformRequest>,
) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ErrorResponse {
            error: "Python DSL not enabled".to_string(),
            code: 501,
        }),
    )
}

// Helper function for base64 encoding
fn base64_encode(data: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::new();
    for byte in data {
        write!(&mut s, "{:02x}", byte).unwrap();
    }
    s
}
