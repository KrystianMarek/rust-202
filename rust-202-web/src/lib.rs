//! rust-202-web library
//!
//! Exports handlers and utilities for testing

pub mod rest;

#[cfg(feature = "grpc")]
pub mod grpc;
