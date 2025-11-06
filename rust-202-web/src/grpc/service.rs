//! gRPC service implementation showcasing rust-202 library

use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use rust_202::functional::iterators::fibonacci_sequence;

use super::pb::{
    patterns_service_server::PatternsService, EventRequest, EventResponse, FibonacciRequest,
    FibonacciResponse, PatternRequest, PatternResponse,
};

/// Implementation of the Patterns gRPC service
pub struct MyPatternsService;

#[tonic::async_trait]
impl PatternsService for MyPatternsService {
    async fn get_fibonacci(
        &self,
        request: Request<FibonacciRequest>,
    ) -> Result<Response<FibonacciResponse>, Status> {
        let count = request.into_inner().count as usize;

        if count > 100 {
            return Err(Status::invalid_argument("Count must be <= 100"));
        }

        let numbers: Vec<u64> = fibonacci_sequence().take(count).collect();

        Ok(Response::new(FibonacciResponse { numbers }))
    }

    type StreamEventsStream = ReceiverStream<Result<EventResponse, Status>>;

    async fn stream_events(
        &self,
        request: Request<EventRequest>,
    ) -> Result<Response<Self::StreamEventsStream>, Status> {
        let req = request.into_inner();
        let count = req.count as usize;

        let (tx, rx) = tokio::sync::mpsc::channel(128);

        tokio::spawn(async move {
            for i in 0..count {
                let event = EventResponse {
                    event_type: req.pattern_type.clone(),
                    data: format!("Event {}", i),
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i64,
                };

                if tx.send(Ok(event)).await.is_err() {
                    break;
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn execute_pattern(
        &self,
        request: Request<PatternRequest>,
    ) -> Result<Response<PatternResponse>, Status> {
        let req = request.into_inner();

        let (success, message) = match req.pattern_name.as_str() {
            "fibonacci" => {
                let fibs: Vec<u64> = fibonacci_sequence().take(5).collect();
                (true, format!("Fibonacci: {:?}", fibs))
            }
            "singleton" => (true, "Singleton pattern executed".to_string()),
            _ => (false, format!("Unknown pattern: {}", req.pattern_name)),
        };

        Ok(Response::new(PatternResponse {
            result: message.clone(),
            success,
            message,
        }))
    }
}
