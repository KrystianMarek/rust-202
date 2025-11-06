//! rust-202 Web Application
//!
//! A REST + gRPC server showcasing the rust-202 library patterns

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod rest;

#[cfg(feature = "grpc")]
mod grpc;

use rest::handlers::*;
use rest::openapi::ApiDoc;

#[cfg(feature = "grpc")]
use grpc::pb::patterns_service_server::PatternsServiceServer;
#[cfg(feature = "grpc")]
use grpc::MyPatternsService;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_202_web=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build the application router
    let app = Router::new()
        // Health & Utility
        .route("/health", get(health_check))
        .route("/patterns/:pattern_name", post(execute_pattern))
        // Functional Programming
        .route("/functional/fibonacci", get(get_fibonacci))
        .route("/functional/closures", post(demo_closures))
        // OOP Patterns (all 6!)
        .route("/oop/singleton", get(get_singleton))
        .route("/oop/factory", post(create_notification))
        .route("/oop/observer", post(trigger_observer))
        .route("/oop/builder", post(build_user))
        .route("/oop/strategy", post(compress_data))
        .route("/oop/adapter", post(adapt_temperature))
        // Differentiators (Why Rust?)
        .route("/differentiators/safety", get(demo_safety))
        .route("/differentiators/performance", get(demo_performance))
        .route("/differentiators/concurrency", post(demo_concurrency))
        // Idioms
        .route("/idioms/error-handling", post(demo_error_handling))
        // Async Patterns
        .route("/async/streams", post(demo_async_streams))
        // Rust 1.75+ Features
        .route("/rust191/const-ops", post(demo_const_ops))
        .route("/rust191/atomics", get(demo_atomics))
        // Python DSL
        .route("/dsl/eval", post(python_eval))
        .route("/dsl/execute", post(python_execute))
        .route("/dsl/sandbox", post(python_sandbox))
        .route("/dsl/transform", post(python_transform))
        // Swagger UI
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // CORS layer
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // Server addresses
    let rest_addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("\n🚀 rust-202 Web API Server");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📡 REST API:    http://{}", rest_addr);
    println!("📚 Swagger UI:  http://{}/docs", rest_addr);
    println!("❤️  Health:      http://{}/health", rest_addr);

    #[cfg(feature = "grpc")]
    {
        let grpc_addr = SocketAddr::from(([127, 0, 0, 1], 50051));
        println!("🔧 gRPC:        grpc://{}", grpc_addr);
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    println!("💡 Example requests:");
    println!("   curl http://{}/health", rest_addr);
    println!("   curl http://{}/functional/fibonacci?count=10", rest_addr);
    println!("   curl http://{}/oop/singleton", rest_addr);
    println!("   curl -X POST http://{}/patterns/fibonacci", rest_addr);

    #[cfg(feature = "grpc")]
    {
        let grpc_addr = SocketAddr::from(([127, 0, 0, 1], 50051));
        println!("\n🔧 gRPC:");
        println!(
            "   grpcurl -plaintext -d '{{\"count\": 10}}' {} patterns.PatternsService/GetFibonacci",
            grpc_addr
        );
    }

    println!();

    // Start REST server
    let listener = tokio::net::TcpListener::bind(&rest_addr).await.unwrap();
    tracing::info!("REST server listening on {}", rest_addr);

    #[cfg(feature = "grpc")]
    {
        // Start both REST and gRPC servers concurrently
        let grpc_addr = SocketAddr::from(([127, 0, 0, 1], 50051));
        let grpc_server = async move {
            let service = MyPatternsService;
            tracing::info!("gRPC server listening on {}", grpc_addr);

            tonic::transport::Server::builder()
                .add_service(PatternsServiceServer::new(service))
                .serve(grpc_addr)
                .await
                .unwrap();
        };

        let rest_server = axum::serve(listener, app);

        tokio::select! {
            _ = rest_server => {},
            _ = grpc_server => {},
        }
    }

    #[cfg(not(feature = "grpc"))]
    {
        // REST-only mode
        tracing::info!("Running in REST-only mode (gRPC feature disabled)");
        axum::serve(listener, app).await.unwrap();
    }
}
