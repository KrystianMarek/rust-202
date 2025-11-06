//! API request/response models

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Response for Fibonacci endpoint
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FibonacciResponse {
    /// The Fibonacci sequence numbers
    #[schema(example = json!([0, 1, 1, 2, 3, 5, 8, 13]))]
    pub numbers: Vec<u64>,
    /// Number of elements returned
    pub count: usize,
}

/// Response for pattern execution
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PatternResponse {
    /// Name of the pattern executed
    pub pattern: String,
    /// Result data
    pub result: serde_json::Value,
    /// Execution time in microseconds
    pub execution_time_us: u128,
}

/// Health check response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    /// Service status
    pub status: String,
    /// Service version
    pub version: String,
}

/// Error response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
    /// HTTP status code
    pub code: u16,
}

/// Singleton pattern response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SingletonResponse {
    /// Singleton instance data
    pub instance_id: String,
    /// Configuration values
    pub config: std::collections::HashMap<String, String>,
}

/// Factory pattern request
#[derive(Debug, Deserialize, ToSchema)]
#[allow(dead_code)]
pub struct FactoryRequest {
    /// Type of notification to create
    #[schema(example = "Email")]
    pub notification_type: String,
}

/// Factory pattern response
#[derive(Debug, Serialize, ToSchema)]
pub struct FactoryResponse {
    /// Created object type
    pub object_type: String,
    /// Object description
    pub description: String,
}

/// Observer pattern request
#[derive(Debug, Deserialize, ToSchema)]
pub struct ObserverRequest {
    /// Event type to trigger
    #[schema(example = "UserLoggedIn")]
    pub event_type: String,
    /// Event data
    pub data: String,
}

/// Observer pattern response
#[derive(Debug, Serialize, ToSchema)]
pub struct ObserverResponse {
    /// Number of observers notified
    pub observers_notified: usize,
    /// Event that was triggered
    pub event: String,
}

/// Builder pattern request
#[derive(Debug, Deserialize, ToSchema)]
pub struct BuilderRequest {
    /// Username (required)
    pub username: String,
    /// Email (optional)
    pub email: Option<String>,
    /// Age (optional)
    pub age: Option<u32>,
}

/// Builder pattern response
#[derive(Debug, Serialize, ToSchema)]
pub struct BuilderResponse {
    /// Built user data
    pub user: UserData,
}

/// User data
#[derive(Debug, Serialize, ToSchema)]
pub struct UserData {
    /// Username
    pub username: String,
    /// Email
    pub email: Option<String>,
    /// Age
    pub age: Option<u32>,
    /// Active status
    pub is_active: bool,
}

/// Strategy pattern request
#[derive(Debug, Deserialize, ToSchema)]
pub struct StrategyRequest {
    /// Data to compress
    pub data: String,
    /// Strategy to use (Gzip, Zip, None)
    #[schema(example = "Gzip")]
    pub strategy: String,
}

/// Strategy pattern response
#[derive(Debug, Serialize, ToSchema)]
pub struct StrategyResponse {
    /// Strategy used
    pub strategy: String,
    /// Compressed data (base64)
    pub compressed: String,
    /// Compression ratio
    pub ratio: f64,
}

/// Adapter pattern request
#[derive(Debug, Deserialize, ToSchema)]
pub struct AdapterRequest {
    /// Temperature in Fahrenheit
    pub fahrenheit: f64,
}

/// Adapter pattern response
#[derive(Debug, Serialize, ToSchema)]
pub struct AdapterResponse {
    /// Temperature in Celsius
    pub celsius: f64,
    /// Temperature in Kelvin
    pub kelvin: f64,
    /// Original Fahrenheit
    pub fahrenheit: f64,
}

/// Differentiator safety response
#[derive(Debug, Serialize, ToSchema)]
pub struct SafetyResponse {
    /// Borrow checker example result
    pub borrow_check: String,
    /// Null safety example
    pub null_safety: String,
    /// Thread safety example
    pub thread_safety: String,
}

/// Differentiator performance response
#[derive(Debug, Serialize, ToSchema)]
pub struct PerformanceResponse {
    /// Zero-cost abstraction demo
    pub zero_cost: String,
    /// No GC example
    pub no_gc: String,
    /// Inline optimization
    pub inline_opt: String,
}

/// Concurrency request
#[derive(Debug, Deserialize, ToSchema)]
pub struct ConcurrencyRequest {
    /// Number of threads
    #[schema(example = 4, minimum = 1, maximum = 100)]
    pub thread_count: usize,
    /// Increments per thread
    #[schema(example = 1000)]
    pub increments: usize,
}

/// Concurrency response
#[derive(Debug, Serialize, ToSchema)]
pub struct ConcurrencyResponse {
    /// Final counter value
    pub final_count: u32,
    /// Number of threads used
    pub threads: usize,
    /// Execution time in microseconds
    pub execution_time_us: u128,
}

/// Closure example request
#[derive(Debug, Deserialize, ToSchema)]
pub struct ClosureRequest {
    /// Numbers to process
    pub numbers: Vec<i32>,
    /// Offset to add
    pub offset: i32,
}

/// Closure example response
#[derive(Debug, Serialize, ToSchema)]
pub struct ClosureResponse {
    /// Original numbers
    pub input: Vec<i32>,
    /// Processed numbers
    pub output: Vec<i32>,
    /// Closure type used
    pub closure_type: String,
}

/// Error handling request
#[derive(Debug, Deserialize, ToSchema)]
pub struct ErrorHandlingRequest {
    /// Numerator
    pub a: i32,
    /// Denominator
    pub b: i32,
}

/// Error handling response
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorHandlingResponse {
    /// Result of division (if successful)
    pub result: Option<i32>,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Demonstrates Result type
    pub pattern: String,
}

/// Async stream request
#[derive(Debug, Deserialize, ToSchema)]
#[allow(dead_code)]  // Used only when async-tokio feature is enabled
pub struct AsyncStreamRequest {
    /// Stream type (counter, fibonacci, range)
    #[schema(example = "fibonacci")]
    pub stream_type: String,
    /// Number of items
    #[schema(example = 10)]
    pub count: usize,
}

/// Async stream response
#[derive(Debug, Serialize, ToSchema)]
pub struct AsyncStreamResponse {
    /// Stream type
    pub stream_type: String,
    /// Generated values
    pub values: Vec<u64>,
    /// Count
    pub count: usize,
}

/// Const operations request
#[derive(Debug, Deserialize, ToSchema)]
pub struct ConstOpsRequest {
    /// Base value
    pub a: u64,
    /// Multiplier
    pub b: u64,
    /// Addend
    pub c: u64,
}

/// Const operations response
#[derive(Debug, Serialize, ToSchema)]
pub struct ConstOpsResponse {
    /// Result of (a * b) + c
    pub result: u64,
    /// Formula used
    pub formula: String,
    /// Computed at compile-time
    pub compile_time: bool,
}

/// Atomic operations response
#[derive(Debug, Serialize, ToSchema)]
pub struct AtomicOpsResponse {
    /// Counter value
    pub value: usize,
    /// Operations performed
    pub operations: String,
    /// Thread-safe
    pub thread_safe: bool,
}

/// Python eval request
#[derive(Debug, Deserialize, ToSchema)]
#[allow(dead_code)]  // Used only when python-dsl feature is enabled
pub struct PythonEvalRequest {
    /// Python expression to evaluate
    #[schema(example = "2 + 2")]
    pub expression: String,
}

/// Python eval response
#[derive(Debug, Serialize, ToSchema)]
#[allow(dead_code)]
pub struct PythonEvalResponse {
    /// Result of evaluation
    pub result: String,
    /// Expression that was evaluated
    pub expression: String,
}

/// Python script execution request
#[derive(Debug, Deserialize, ToSchema)]
#[allow(dead_code)]
pub struct PythonScriptRequest {
    /// Python source code
    #[schema(example = "def main():\n    return 'Hello from Python!'")]
    pub source: String,
    /// Module name (optional)
    pub module_name: Option<String>,
}

/// Python script execution response
#[derive(Debug, Serialize, ToSchema)]
pub struct PythonScriptResponse {
    /// Execution result
    pub result: String,
    /// Success status
    pub success: bool,
    /// Error message (if failed)
    pub error: Option<String>,
}

/// Python sandbox eval request
#[derive(Debug, Deserialize, ToSchema)]
#[allow(dead_code)]
pub struct PythonSandboxRequest {
    /// Python expression (restricted builtins only)
    #[schema(example = "sum([1, 2, 3, 4, 5])")]
    pub expression: String,
}

/// Python sandbox response
#[derive(Debug, Serialize, ToSchema)]
#[allow(dead_code)]
pub struct PythonSandboxResponse {
    /// Result of sandboxed evaluation
    pub result: i32,
    /// Expression evaluated
    pub expression: String,
    /// Safety note
    pub sandbox_note: String,
}

/// Python transform request
#[derive(Debug, Deserialize, ToSchema)]
#[allow(dead_code)]
pub struct PythonTransformRequest {
    /// Python transformation function
    #[schema(example = "def process(items):\n    return [x * 2 for x in items]")]
    pub script: String,
    /// Input data
    pub data: Vec<i32>,
}

/// Python transform response
#[derive(Debug, Serialize, ToSchema)]
#[allow(dead_code)]
pub struct PythonTransformResponse {
    /// Original input
    pub input: Vec<i32>,
    /// Transformed output
    pub output: Vec<i32>,
    /// Transformation description
    pub description: String,
}
