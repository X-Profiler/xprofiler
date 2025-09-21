# xprofiler-rs

A high-performance Node.js profiler implemented in Rust using NAPI-RS.

## Overview

xprofiler-rs is a Rust-based implementation of the xprofiler Node.js performance monitoring library. It provides comprehensive profiling capabilities including CPU usage, memory monitoring, environment data collection, and custom metrics tracking.

## Features

- **High Performance**: Implemented in Rust for minimal overhead
- **Thread-Safe**: Designed for multi-threaded Node.js applications
- **Comprehensive Monitoring**: CPU, memory, environment, and custom metrics
- **Flexible Configuration**: Runtime configuration management
- **Structured Logging**: Built-in logging system with multiple formatters
- **NAPI Integration**: Seamless integration with Node.js via NAPI-RS
- **Cross-Platform**: Support for Linux, macOS, and Windows

## Architecture

The library is organized into several core modules:

### Core Modules

- **Config Management** (`src/config/`): Runtime configuration system
- **Environment Monitoring** (`src/environment/`): System and process environment data
- **Logging System** (`src/logger/`): Structured logging with multiple formatters
- **Utilities** (`src/utils/`): Common utility functions and helpers
- **NAPI Bindings** (`src/bindings/`): JavaScript interface layer

### Key Components

```
src/
├── config/           # Configuration management
│   ├── mod.rs        # Main configuration interface
│   └── manager.rs    # Configuration manager implementation
├── environment/      # Environment data collection
│   ├── mod.rs        # Environment monitoring interface
│   ├── collector.rs  # Data collection implementation
│   └── metrics.rs    # Environment metrics definitions
├── logger/           # Logging system
│   ├── mod.rs        # Logger interface
│   ├── config.rs     # Logger configuration
│   ├── formatter.rs  # Log formatters
│   └── writer.rs     # Log writers
├── utils/            # Utility functions
│   ├── mod.rs        # Common utilities
│   ├── time.rs       # Time-related utilities
│   ├── string.rs     # String manipulation
│   └── fs.rs         # File system utilities
├── bindings/         # NAPI bindings
│   ├── mod.rs        # Main NAPI interface
│   ├── config.rs     # Configuration bindings
│   ├── environment.rs # Environment bindings
│   ├── logger.rs     # Logger bindings
│   └── monitoring.rs # Monitoring bindings
└── lib.rs            # Library entry point
```

## Installation

### Prerequisites

- Node.js 16+ 
- Rust 1.70+
- Python 3.7+ (for node-gyp)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/X-Profiler/xprofiler.git
cd xprofiler/rust/xprofiler-rs

# Install dependencies
npm install

# Build the native module
npm run build

# Run tests
npm test
```

### Development Setup

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install required targets
rustup target add x86_64-apple-darwin  # macOS
rustup target add x86_64-pc-windows-msvc  # Windows
rustup target add x86_64-unknown-linux-gnu  # Linux

# Install development tools
cargo install cargo-watch
cargo install criterion
```

## Usage

### Basic Usage

```javascript
const xprofiler = require('xprofiler-rs');

// Initialize the profiler
xprofiler.initialize({
  enabled: true,
  sampleRate: 1000,
  logLevel: 'info'
});

// Start profiling
xprofiler.startProfiling();

// Your application code here

// Get statistics
const stats = xprofiler.getStatistics();
console.log('Profiling stats:', stats);

// Stop profiling
xprofiler.stopProfiling();

// Cleanup
xprofiler.shutdown();
```

### Configuration

```javascript
// Set configuration options
xprofiler.setConfig('sampleRate', 500);
xprofiler.setConfig('logLevel', 'debug');
xprofiler.setConfig('outputDir', '/tmp/xprofiler');

// Get configuration
const config = xprofiler.getConfig();
console.log('Current config:', config);

// Validate configuration
const isValid = xprofiler.validateConfig();
if (!isValid) {
  console.error('Invalid configuration');
}
```

### Monitoring

```javascript
// Record custom metrics
xprofiler.recordCounter('http_requests', 1);
xprofiler.recordGauge('memory_usage', process.memoryUsage().heapUsed);
xprofiler.recordHistogram('response_time', 150);

// Timer operations
const timerId = xprofiler.startTimer('database_query');
// ... perform database operation
xprofiler.endTimer(timerId);

// Get metrics
const metrics = xprofiler.getMetrics();
console.log('Metrics:', metrics);
```

### Logging

```javascript
// Log messages with different levels
xprofiler.log('info', 'Application started');
xprofiler.log('debug', 'Processing request', { userId: 123 });
xprofiler.log('warn', 'High memory usage detected');
xprofiler.log('error', 'Database connection failed');

// Set log level
xprofiler.setLogLevel('debug');

// Flush logs
xprofiler.flushLogs();
```

### Environment Data

```javascript
// Get environment data
const envData = xprofiler.getEnvironmentData();
console.log('Environment:', envData);

// Register current thread for monitoring
xprofiler.registerThread();

// Get thread-specific data
const threadData = xprofiler.getCurrentThreadData();
console.log('Thread data:', threadData);

// Unregister thread
xprofiler.unregisterThread();
```

## Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `enabled` | boolean | `true` | Enable/disable profiling |
| `sampleRate` | number | `1000` | Sampling rate in milliseconds |
| `logLevel` | string | `'info'` | Log level (trace, debug, info, warn, error) |
| `outputDir` | string | `'/tmp/xprofiler'` | Output directory for logs and data |
| `maxMemory` | number | `104857600` | Maximum memory usage (100MB) |
| `bufferSize` | number | `8192` | Buffer size for data collection |
| `flushInterval` | number | `5000` | Flush interval in milliseconds |

## Performance

xprofiler-rs is designed for high performance with minimal overhead:

- **Low Latency**: Sub-microsecond metric recording
- **High Throughput**: >100k operations per second
- **Memory Efficient**: Minimal memory footprint
- **Thread Safe**: Lock-free data structures where possible

### Benchmarks

Run benchmarks to measure performance:

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench metric_recording

# Generate benchmark report
cargo bench -- --output-format html
```

## Testing

### Unit Tests

```bash
# Run all tests
cargo test

# Run specific test module
cargo test config_tests

# Run tests with output
cargo test -- --nocapture
```

### Integration Tests

```bash
# Run integration tests
cargo test --test integration_tests

# Run compatibility tests
cargo test --test compatibility
```

### Test Coverage

```bash
# Install coverage tool
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

## Development

### Code Style

This project follows Rust standard formatting:

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Lint code
cargo clippy
```

### Documentation

```bash
# Generate documentation
cargo doc --open

# Check documentation
cargo doc --no-deps
```

### Debugging

```bash
# Build with debug symbols
cargo build

# Run with debug logging
RUST_LOG=debug cargo test

# Use debugger
rust-gdb target/debug/xprofiler_rs
```

## Platform Support

| Platform | Architecture | Status |
|----------|--------------|--------|
| Linux | x86_64 | ✅ Supported |
| Linux | aarch64 | ✅ Supported |
| macOS | x86_64 | ✅ Supported |
| macOS | aarch64 (M1) | ✅ Supported |
| Windows | x86_64 | ✅ Supported |
| Windows | aarch64 | 🚧 Experimental |

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

### Development Guidelines

- Follow Rust naming conventions
- Write comprehensive tests
- Document public APIs
- Use meaningful commit messages
- Keep PRs focused and small

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history and changes.

## Support

- 📖 [Documentation](https://docs.rs/xprofiler-rs)
- 🐛 [Issue Tracker](https://github.com/X-Profiler/xprofiler/issues)
- 💬 [Discussions](https://github.com/X-Profiler/xprofiler/discussions)
- 📧 [Email Support](mailto:support@xprofiler.com)

## Related Projects

- [xprofiler](https://github.com/X-Profiler/xprofiler) - Original JavaScript implementation
- [napi-rs](https://github.com/napi-rs/napi-rs) - NAPI bindings for Rust
- [Node.js](https://nodejs.org/) - JavaScript runtime

---

**Note**: This is a Rust implementation of xprofiler designed for high-performance scenarios. For the original JavaScript implementation, see the main [xprofiler repository](https://github.com/X-Profiler/xprofiler).