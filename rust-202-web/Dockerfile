# Multi-stage build for rust-202-web

# Build stage
FROM rust:1.75-slim as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    libprotobuf-dev \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the entire workspace
COPY ../Cargo.toml ./Cargo.toml
COPY ../src ./src
COPY ./rust-202-web/Cargo.toml ./rust-202-web/Cargo.toml
COPY ./rust-202-web/build.rs ./rust-202-web/build.rs
COPY ./rust-202-web/proto ./rust-202-web/proto
COPY ./rust-202-web/src ./rust-202-web/src

# Build the application
WORKDIR /app/rust-202-web
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/rust-202-web/target/release/rust-202-web /app/rust-202-web

# Expose ports
EXPOSE 3000 50051

# Set environment variables
ENV RUST_LOG=info

# Run the application
CMD ["/app/rust-202-web"]

