# rust-202-web Quick Start

## Prerequisites

### 1. Install Protocol Buffer Compiler (for gRPC)

```bash
# macOS
brew install protobuf

# Linux (Debian/Ubuntu)
sudo apt-get install protobuf-compiler

# Linux (Fedora/RHEL)
sudo dnf install protobuf-compiler

# Verify installation
protoc --version  # Should show libprotoc 3.x or higher
```

### 2. Python 3.8+ (for Python DSL feature)

```bash
# Check Python version
python3 --version  # Should be 3.8 or higher
```

---

## Building

### Option 1: Full Build (All Features)

Includes: REST API + gRPC + Async + Python DSL

```bash
cd rust-202-web

# With Python 3.14+ (requires forward compatibility flag)
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo build

# Or set environment variable permanently
export PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1
cargo build
```

**Enables**: All 21 REST endpoints + 3 gRPC methods

### Option 2: REST + Async Only (No gRPC, No Python)

```bash
cd rust-202-web
cargo build --no-default-features --features async-tokio
```

**Enables**: 17 REST endpoints (no gRPC, no Python DSL)

### Option 3: REST Only (Minimal)

```bash
cd rust-202-web
cargo build --no-default-features
```

**Enables**: 13 REST endpoints (core patterns only)

---

## Feature Flags

| Feature | Description | Requires |
|---------|-------------|----------|
| `grpc` | gRPC server on port 50051 | protoc compiler |
| `async-tokio` | Async stream endpoints | - |
| `python-dsl` | Python embedding endpoints | Python 3.8+ |
| **default** | All features enabled | protoc + Python |

### Build with Specific Features

```bash
# REST + gRPC only
cargo build --no-default-features --features grpc

# REST + Python DSL only
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 \
  cargo build --no-default-features --features python-dsl

# REST + Async + Python (no gRPC)
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 \
  cargo build --no-default-features --features async-tokio,python-dsl
```

---

## Running

### Full Server (All Features)

```bash
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo run
```

Output:
```
🚀 rust-202 Web API Server
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📡 REST API:    http://127.0.0.1:3000
📚 Swagger UI:  http://127.0.0.1:3000/docs
❤️  Health:      http://127.0.0.1:3000/health
🔧 gRPC:        grpc://127.0.0.1:50051
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### REST-Only Server

```bash
cargo run --no-default-features
```

---

## Testing the API

### Health Check

```bash
curl http://localhost:3000/health
```

### Functional Programming

```bash
# Fibonacci sequence
curl "http://localhost:3000/functional/fibonacci?count=10"

# Closures
curl -X POST http://localhost:3000/functional/closures \
  -H "Content-Type: application/json" \
  -d '{"numbers": [1, 2, 3], "offset": 10}'
```

### OOP Design Patterns (All 6!)

```bash
# Singleton
curl http://localhost:3000/oop/singleton

# Factory
curl -X POST http://localhost:3000/oop/factory \
  -H "Content-Type: application/json" \
  -d '{"notification_type": "Email"}'

# Observer
curl -X POST http://localhost:3000/oop/observer \
  -H "Content-Type: application/json" \
  -d '{"event_type": "UserLoggedIn", "data": "Alice"}'

# Builder
curl -X POST http://localhost:3000/oop/builder \
  -H "Content-Type: application/json" \
  -d '{"username": "alice", "email": "alice@example.com", "age": 30}'

# Strategy
curl -X POST http://localhost:3000/oop/strategy \
  -H "Content-Type: application/json" \
  -d '{"data": "Hello World", "strategy": "Gzip"}'

# Adapter
curl -X POST http://localhost:3000/oop/adapter \
  -H "Content-Type: application/json" \
  -d '{"fahrenheit": 68.0}'
```

### Differentiators (Why Rust?)

```bash
# Memory & Thread Safety
curl http://localhost:3000/differentiators/safety

# Zero-Cost Abstractions
curl http://localhost:3000/differentiators/performance

# Lock-Free Concurrency
curl -X POST http://localhost:3000/differentiators/concurrency \
  -H "Content-Type: application/json" \
  -d '{"thread_count": 4, "increments": 1000}'
```

### Python DSL 🆕

```bash
# Evaluate expression
curl -X POST http://localhost:3000/dsl/eval \
  -H "Content-Type: application/json" \
  -d '{"expression": "2 ** 10"}'

# Execute Python script
curl -X POST http://localhost:3000/dsl/execute \
  -H "Content-Type: application/json" \
  -d '{
    "source": "def main():\n    return \"Hello from Python!\"",
    "module_name": "test"
  }'

# Sandboxed execution (safe!)
curl -X POST http://localhost:3000/dsl/sandbox \
  -H "Content-Type: application/json" \
  -d '{"expression": "sum([1, 2, 3, 4, 5])"}'

# Data transformation
curl -X POST http://localhost:3000/dsl/transform \
  -H "Content-Type: application/json" \
  -d '{
    "script": "def process(items):\n    return [x * 2 for x in items]",
    "data": [1, 2, 3, 4, 5]
  }'
```

### Async Patterns

```bash
curl -X POST http://localhost:3000/async/streams \
  -H "Content-Type: application/json" \
  -d '{"stream_type": "fibonacci", "count": 10}'
```

### Rust 1.75+ Features

```bash
# Const operations
curl -X POST http://localhost:3000/rust191/const-ops \
  -H "Content-Type: application/json" \
  -d '{"a": 5, "b": 10, "c": 3}'

# Atomic operations
curl http://localhost:3000/rust191/atomics
```

---

## gRPC Examples

Install grpcurl:
```bash
# macOS
brew install grpcurl

# Linux
go install github.com/fullstorydev/grpcurl/cmd/grpcurl@latest
```

Test gRPC:
```bash
# List services
grpcurl -plaintext localhost:50051 list

# Get Fibonacci
grpcurl -plaintext -d '{"count": 10}' \
  localhost:50051 patterns.PatternsService/GetFibonacci

# Stream events
grpcurl -plaintext -d '{"pattern_type": "observer", "count": 5}' \
  localhost:50051 patterns.PatternsService/StreamEvents
```

---

## Interactive Documentation

Visit **http://localhost:3000/docs** for Swagger UI

Features:
- ✅ Browse all 21 REST endpoints
- ✅ Try endpoints with "Execute" button
- ✅ View request/response schemas
- ✅ See example payloads
- ✅ Organized by 8 categories

---

## Troubleshooting

### Error: "Could not find protoc"

**Solution 1**: Install protoc (see Prerequisites)

**Solution 2**: Build without gRPC:
```bash
cargo build --no-default-features
```

### Error: "Python interpreter version newer than PyO3"

**Solution**: Use forward compatibility flag:
```bash
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo build
```

### Error: "Address already in use"

**Solution**: Kill existing process:
```bash
lsof -ti:3000 | xargs kill
```

### Python DSL Endpoints Return 501

**Cause**: Built without `python-dsl` feature

**Solution**: Build with Python DSL:
```bash
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo build
```

---

## Development Workflow

### Hot Reload

```bash
# Install cargo-watch
cargo install cargo-watch

# Run with auto-reload
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo watch -x run
```

### Testing

```bash
# Run integration tests
cargo test

# With all features
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo test --all-features
```

---

## Environment Variables

```bash
# Logging level
RUST_LOG=debug cargo run

# Ports (modify in src/main.rs)
# REST: 3000 (default)
# gRPC: 50051 (default)

# Python compatibility
export PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1
```

---

## What's Next

- ✅ Explore all 21 REST endpoints in Swagger UI
- ✅ Try Python DSL with custom scripts
- ✅ Test gRPC with grpcurl
- ✅ Experiment with all 6 OOP patterns
- ✅ See differentiators (why Rust?)

See the main [README.md](README.md) for complete documentation.
