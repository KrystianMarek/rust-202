//! gRPC service implementations

pub mod service;

pub use service::*;

// Include generated protobuf code
pub mod pb {
    tonic::include_proto!("patterns");
}
