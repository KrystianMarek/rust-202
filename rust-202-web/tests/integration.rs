//! Integration tests for rust-202-web API

#[cfg(test)]
mod rest_tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        Router,
    };
    use serde_json::Value;
    use tower::ServiceExt;

    // Helper to create test app
    fn create_test_app() -> Router {
        use axum::routing::{get, post};
        use rust_202_web::rest::handlers::*;

        Router::new()
            .route("/health", get(health_check))
            .route("/functional/fibonacci", get(get_fibonacci))
            .route("/oop/singleton", get(get_singleton))
            .route("/patterns/:pattern_name", post(execute_pattern))
    }

    #[tokio::test]
    async fn test_health_check() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "healthy");
        assert!(json["version"].is_string());
    }

    #[tokio::test]
    async fn test_fibonacci_endpoint() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/functional/fibonacci?count=5")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["count"], 5);
        assert_eq!(json["numbers"].as_array().unwrap().len(), 5);
        assert_eq!(json["numbers"][0], 0);
        assert_eq!(json["numbers"][1], 1);
        assert_eq!(json["numbers"][4], 3);
    }

    #[tokio::test]
    async fn test_singleton_endpoint() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/oop/singleton")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert!(json["instance_id"].is_string());
        assert!(json["config"].is_object());
    }

    #[tokio::test]
    async fn test_pattern_execution_fibonacci() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/patterns/fibonacci")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["pattern"], "fibonacci");
        assert!(json["execution_time_us"].is_number());
    }

    #[tokio::test]
    async fn test_pattern_execution_not_found() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/patterns/nonexistent")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["code"], 404);
        assert!(json["error"].as_str().unwrap().contains("not found"));
    }
}
