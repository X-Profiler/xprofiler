# XProfiler Rust

[![CI](https://github.com/X-Profiler/xprofiler/actions/workflows/ci.yml/badge.svg)](https://github.com/X-Profiler/xprofiler/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/X-Profiler/xprofiler/branch/main/graph/badge.svg)](https://codecov.io/gh/X-Profiler/xprofiler)
[![Crates.io](https://img.shields.io/crates/v/xprofiler_rs.svg)](https://crates.io/crates/xprofiler_rs)
[![Documentation](https://docs.rs/xprofiler_rs/badge.svg)](https://docs.rs/xprofiler_rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A high-performance, cross-platform profiling library for Node.js applications, implemented in Rust. XProfiler Rust provides comprehensive monitoring capabilities for CPU, memory, garbage collection, HTTP requests, and libuv event loop metrics.

## Features

- **🚀 High Performance**: Written in Rust for minimal overhead and maximum performance
- **📊 Comprehensive Monitoring**: CPU, memory, GC, HTTP, and libuv metrics
- **🔄 Real-time Data**: Live monitoring with configurable update intervals
- **🌐 Cross-platform**: Supports Linux, macOS, and Windows
- **🧵 Thread-safe**: Safe concurrent access to monitoring data
- **📈 Low Overhead**: Designed to minimize impact on application performance
- **🔧 Easy Integration**: Simple API for Node.js applications

## Quick Start

### Installation

```bash
npm install xprofiler-rs
```

### Basic Usage

```javascript
const xprofiler = require('xprofiler-rs');

// Start monitoring
xprofiler.start();

// Get current stats
const stats = xprofiler.getStats();
console.log('CPU Usage:', stats.cpu.usage);
console.log('Memory RSS:', stats.memory.rss);
console.log('HTTP Requests:', stats.http.total_requests);

// Stop monitoring
xprofiler.stop();
```

## Monitoring Modules

### CPU Monitoring

Tracks CPU usage, load averages, and system information:

```rust
use xprofiler_rs::monitoring::cpu::CpuMonitor;
use xprofiler_rs::monitoring::Monitor;

let mut cpu_monitor = CpuMonitor::new();
cpu_monitor.start().unwrap();
cpu_monitor.update().unwrap();

let stats = cpu_monitor.get_stats().unwrap();
println!("CPU Usage: {:.2}%", stats.usage_percent);
println!("Load Average: {:?}", stats.load_avg);
```

### Memory Monitoring

Monitors memory usage including RSS, heap, and system memory:

```rust
use xprofiler_rs::monitoring::memory::MemoryMonitor;
use xprofiler_rs::monitoring::Monitor;

let mut memory_monitor = MemoryMonitor::new();
memory_monitor.start().unwrap();
memory_monitor.update().unwrap();

let stats = memory_monitor.get_stats().unwrap();
println!("RSS: {} bytes", stats.rss);
println!("Heap Used: {} bytes", stats.heap_used);
```

### Garbage Collection Monitoring

Tracks GC events, timing, and heap statistics:

```rust
use xprofiler_rs::monitoring::gc::{GcMonitor, GcEvent, GcType};
use xprofiler_rs::monitoring::Monitor;
use std::time::{Duration, Instant};

let mut gc_monitor = GcMonitor::new();
gc_monitor.start().unwrap();

// Record a GC event
let gc_event = GcEvent {
    gc_type: GcType::Scavenge,
    duration: Duration::from_millis(10),
    timestamp: Instant::now(),
    heap_size_before: 1024 * 1024,
    heap_size_after: 512 * 1024,
};
gc_monitor.record_gc_event(gc_event);

let stats = gc_monitor.get_stats().unwrap();
println!("Total GC Count: {}", stats.total_gc_count);
println!("Total GC Time: {:?}", stats.total_gc_time);
```

### HTTP Monitoring

Monitors HTTP requests and responses with detailed metrics:

```rust
use xprofiler_rs::monitoring::http::{HttpMonitor, HttpRequest, HttpResponse};
use xprofiler_rs::monitoring::Monitor;
use std::time::{Duration, Instant};

let mut http_monitor = HttpMonitor::new();
http_monitor.start().unwrap();

// Record HTTP request
let request = HttpRequest {
    method: "GET".to_string(),
    url: "/api/users".to_string(),
    timestamp: Instant::now(),
    headers_size: 1024,
    body_size: 0,
    user_agent: Some("Mozilla/5.0".to_string()),
    remote_ip: Some("192.168.1.100".to_string()),
};

// Record HTTP response
let response = HttpResponse {
    status_code: 200,
    timestamp: Instant::now(),
    headers_size: 512,
    body_size: 2048,
    response_time: Duration::from_millis(50),
};

let request_id = "req_123".to_string();
http_monitor.record_request(request_id.clone(), request);
http_monitor.record_response(request_id, response);

let stats = http_monitor.get_stats().unwrap();
println!("Total Requests: {}", stats.total_requests);
println!("Average Response Time: {:?}", stats.avg_response_time);
```

### Libuv Monitoring

Tracks libuv handles and event loop metrics:

```rust
use xprofiler_rs::monitoring::libuv::{LibuvMonitor, HandleType};
use xprofiler_rs::monitoring::Monitor;
use std::time::Duration;

let mut libuv_monitor = LibuvMonitor::new();
libuv_monitor.start().unwrap();

// Register handles
let timer_handle = libuv_monitor.register_handle(HandleType::Timer, true, false);
let tcp_handle = libuv_monitor.register_handle(HandleType::Tcp, true, true);

// Record event loop iteration
libuv_monitor.record_loop_iteration(
    Duration::from_millis(10), // total time
    Duration::from_millis(2),  // prepare time
    Duration::from_millis(3),  // check time
    Duration::from_millis(1),  // poll time
    Duration::from_millis(4),  // idle time
);

let stats = libuv_monitor.get_stats().unwrap();
println!("Active Handles: {}", stats.active_handles);
println!("Loop Count: {}", stats.loop_metrics.loop_count);
```

## Configuration

### Environment Variables

- `XPROFILER_LOG_LEVEL`: Set logging level (trace, debug, info, warn, error)
- `XPROFILER_LOG_DIR`: Directory for log files
- `XPROFILER_UPDATE_INTERVAL`: Monitoring update interval in milliseconds

### Programmatic Configuration

```javascript
const xprofiler = require('xprofiler-rs');

// Configure monitoring options
xprofiler.configure({
  updateInterval: 5000,  // 5 seconds
  logLevel: 'info',
  enableCpuMonitoring: true,
  enableMemoryMonitoring: true,
  enableGcMonitoring: true,
  enableHttpMonitoring: true,
  enableLibuvMonitoring: true,
});
```

## Performance

XProfiler Rust is designed for minimal performance overhead:

- **CPU Monitoring**: < 0.1% CPU overhead
- **Memory Monitoring**: < 1MB memory overhead
- **HTTP Monitoring**: < 10μs per request
- **GC Monitoring**: < 5μs per GC event
- **Libuv Monitoring**: < 1μs per event loop iteration

Run benchmarks to measure performance on your system:

```bash
cargo bench
```

## Development

### Prerequisites

- Rust 1.70+ (stable, beta, or nightly)
- Node.js 16+ (for Node.js bindings)
- Git

### Building from Source

```bash
# Clone the repository
git clone https://github.com/X-Profiler/xprofiler.git
cd xprofiler/rust/xprofiler-rs

# Build the library
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench

# Generate documentation
cargo doc --open
```

### Testing

The project includes comprehensive test suites:

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test --test unit_tests
cargo test --test integration_tests
cargo test --test compatibility
cargo test --test error_handling_tests

# Run tests with coverage
cargo install cargo-llvm-cov
cargo llvm-cov --html
```

### Code Quality

```bash
# Format code
cargo fmt

# Run clippy lints
cargo clippy -- -D warnings

# Security audit
cargo audit
```

## CI/CD

The project uses GitHub Actions for continuous integration:

- **Multi-platform testing**: Linux, macOS, Windows
- **Multiple Rust versions**: stable, beta, nightly
- **Code coverage**: Automated coverage reporting
- **Security auditing**: Dependency vulnerability scanning
- **Performance benchmarking**: Automated performance regression detection
- **Documentation**: Automatic documentation generation

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and add tests
4. Run the test suite: `cargo test`
5. Run formatting and linting: `cargo fmt && cargo clippy`
6. Commit your changes: `git commit -m 'Add amazing feature'`
7. Push to the branch: `git push origin feature/amazing-feature`
8. Open a Pull Request

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Address all Clippy warnings (`cargo clippy`)
- Write comprehensive tests for new features
- Update documentation for API changes
- Add benchmarks for performance-critical code

## Architecture

### Core Components

```
xprofiler-rs/
├── src/
│   ├── lib.rs              # Main library entry point
│   ├── monitoring/         # Monitoring modules
│   │   ├── mod.rs          # Monitor trait definition
│   │   ├── cpu.rs          # CPU monitoring
│   │   ├── memory.rs       # Memory monitoring
│   │   ├── gc.rs           # Garbage collection monitoring
│   │   ├── http.rs         # HTTP monitoring
│   │   ├── libuv.rs        # Libuv monitoring
│   │   └── error.rs        # Error types
│   ├── napi/               # Node.js bindings
│   └── utils/              # Utility functions
├── tests/                  # Test suites
├── benches/                # Benchmarks
└── examples/               # Usage examples
```

### Design Principles

- **Performance First**: Minimal overhead design
- **Thread Safety**: Safe concurrent access
- **Error Handling**: Comprehensive error management
- **Extensibility**: Modular architecture for easy extension
- **Cross-platform**: Consistent behavior across platforms

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Node.js](https://nodejs.org/) - The runtime environment
- [Rust](https://www.rust-lang.org/) - The systems programming language
- [napi-rs](https://napi.rs/) - Node.js addon framework
- [tokio](https://tokio.rs/) - Asynchronous runtime
- [criterion](https://github.com/bheisler/criterion.rs) - Benchmarking framework

## Support

- 📖 [Documentation](https://docs.rs/xprofiler_rs)
- 🐛 [Issue Tracker](https://github.com/X-Profiler/xprofiler/issues)
- 💬 [Discussions](https://github.com/X-Profiler/xprofiler/discussions)
- 📧 [Email Support](mailto:support@xprofiler.org)

---

**XProfiler Rust** - High-performance profiling for Node.js applications