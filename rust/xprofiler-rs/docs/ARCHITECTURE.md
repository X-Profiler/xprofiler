# XProfiler Rust Architecture Documentation

This document describes the architecture and design principles of the XProfiler Rust library.

## Table of Contents

- [Overview](#overview)
- [Core Architecture](#core-architecture)
- [Module Structure](#module-structure)
- [Design Patterns](#design-patterns)
- [Threading Model](#threading-model)
- [Memory Management](#memory-management)
- [Error Handling Strategy](#error-handling-strategy)
- [Performance Considerations](#performance-considerations)
- [Platform Abstraction](#platform-abstraction)
- [Integration Points](#integration-points)

## Overview

XProfiler Rust is a high-performance profiling library designed for production environments. It provides comprehensive monitoring and profiling capabilities with minimal overhead.

### Key Design Goals

1. **Low Overhead**: Minimal impact on application performance
2. **Thread Safety**: Safe concurrent access across multiple threads
3. **Modular Design**: Pluggable components for different profiling needs
4. **Cross-Platform**: Support for major operating systems
5. **Real-time Monitoring**: Live performance metrics collection
6. **Memory Efficiency**: Bounded memory usage with configurable limits

## Core Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                        │
├─────────────────────────────────────────────────────────────┤
│                    Public API Layer                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │  Profiler   │  │  Monitor    │  │   Config    │        │
│  │   Traits    │  │   Traits    │  │  Management │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
├─────────────────────────────────────────────────────────────┤
│                  Implementation Layer                      │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │     CPU     │  │    Heap     │  │     GC      │        │
│  │  Profiler   │  │  Profiler   │  │  Profiler   │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │     CPU     │  │   Memory    │  │    HTTP     │        │
│  │   Monitor   │  │   Monitor   │  │   Monitor   │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
├─────────────────────────────────────────────────────────────┤
│                    Core Services                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │   Thread    │  │   Memory    │  │    Error    │        │
│  │ Management  │  │ Management  │  │  Handling   │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
├─────────────────────────────────────────────────────────────┤
│                  Platform Layer                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │   System    │  │   Stack     │  │    Time     │        │
│  │    APIs     │  │   Walking   │  │   Services  │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────┘
```

## Module Structure

### Core Modules

#### `lib.rs`
- Public API exports
- Library initialization
- Feature flag management

#### `error.rs`
- Centralized error handling
- Error type definitions
- Result type aliases

#### `config.rs`
- Configuration management
- Default settings
- Environment variable parsing

### Profiler Modules

#### `profiler/mod.rs`
- Profiler trait definitions
- Common profiler utilities
- Profiler factory functions

#### `profiler/cpu_profiler.rs`
- CPU profiling implementation
- Call stack sampling
- Performance metrics collection

#### `profiler/heap_profiler.rs`
- Memory allocation tracking
- Leak detection algorithms
- Heap statistics collection

#### `profiler/gc_profiler.rs`
- Garbage collection monitoring
- GC event tracking
- Performance impact analysis

### Monitoring Modules

#### `monitoring/mod.rs`
- Monitor trait definitions
- Common monitoring utilities
- Monitor registry

#### `monitoring/cpu.rs`
- System CPU monitoring
- Load average tracking
- Process-specific CPU metrics

#### `monitoring/memory.rs`
- Memory usage monitoring
- Heap statistics
- Virtual memory tracking

#### `monitoring/http.rs`
- HTTP request/response monitoring
- Performance metrics
- Request pattern analysis

#### `monitoring/libuv.rs`
- Event loop monitoring
- Handle tracking
- Asynchronous operation metrics

## Design Patterns

### Trait-Based Architecture

The library uses traits to define common interfaces:

```rust
pub trait Profiler {
    fn start(&mut self) -> XProfilerResult<()>;
    fn stop(&mut self) -> XProfilerResult<()>;
    fn is_running(&self) -> bool;
    fn get_results(&self) -> XProfilerResult<String>;
    fn reset(&mut self) -> XProfilerResult<()>;
}

pub trait Monitor {
    fn start(&mut self) -> XProfilerResult<()>;
    fn stop(&mut self) -> XProfilerResult<()>;
    fn update(&mut self) -> XProfilerResult<()>;
    fn is_running(&self) -> bool;
    fn get_stats(&self) -> XProfilerResult<String>;
}
```

### Builder Pattern

Configuration uses the builder pattern for flexibility:

```rust
let config = ProfilerConfig::builder()
    .sampling_interval(Duration::from_millis(50))
    .max_samples(5000)
    .enable_stack_traces(true)
    .max_stack_depth(32)
    .build();
```

### Observer Pattern

Event-driven architecture for real-time monitoring:

```rust
pub trait ProfilerObserver {
    fn on_sample_collected(&self, sample: &CpuSample);
    fn on_allocation(&self, event: &AllocationEvent);
    fn on_gc_event(&self, event: &GcEvent);
}
```

### Factory Pattern

Profiler creation through factory methods:

```rust
pub struct ProfilerFactory;

impl ProfilerFactory {
    pub fn create_cpu_profiler(config: ProfilerConfig) -> XProfilerResult<CpuProfiler> {
        CpuProfiler::new(config)
    }
    
    pub fn create_heap_profiler(config: ProfilerConfig) -> XProfilerResult<HeapProfiler> {
        HeapProfiler::new(config)
    }
}
```

## Threading Model

### Thread Safety

All profiler components are designed to be thread-safe:

- **Arc<Mutex<T>>**: For shared mutable state
- **Arc<RwLock<T>>**: For read-heavy shared state
- **Atomic types**: For simple counters and flags
- **mpsc channels**: For inter-thread communication

### Sampling Threads

Each profiler runs its own sampling thread:

```rust
struct CpuProfiler {
    config: ProfilerConfig,
    samples: Arc<Mutex<Vec<CpuSample>>>,
    running: Arc<AtomicBool>,
    sampling_thread: Option<JoinHandle<()>>,
    stop_signal: Arc<AtomicBool>,
}
```

### Thread Pool

For CPU-intensive operations, a shared thread pool is used:

```rust
lazy_static! {
    static ref THREAD_POOL: ThreadPool = ThreadPool::new(num_cpus::get());
}
```

## Memory Management

### Bounded Collections

All data structures use bounded collections to prevent memory leaks:

```rust
struct BoundedVec<T> {
    data: VecDeque<T>,
    max_size: usize,
}

impl<T> BoundedVec<T> {
    fn push(&mut self, item: T) {
        if self.data.len() >= self.max_size {
            self.data.pop_front();
        }
        self.data.push_back(item);
    }
}
```

### Memory Pools

Object pools for frequently allocated objects:

```rust
struct ObjectPool<T> {
    pool: Mutex<Vec<T>>,
    factory: Box<dyn Fn() -> T + Send + Sync>,
}

impl<T> ObjectPool<T> {
    fn acquire(&self) -> PooledObject<T> {
        let mut pool = self.pool.lock().unwrap();
        let object = pool.pop().unwrap_or_else(|| (self.factory)());
        PooledObject::new(object, &self.pool)
    }
}
```

### RAII Pattern

Automatic resource cleanup using RAII:

```rust
struct ProfilerGuard {
    profiler: Arc<Mutex<dyn Profiler>>,
}

impl Drop for ProfilerGuard {
    fn drop(&mut self) {
        let _ = self.profiler.lock().unwrap().stop();
    }
}
```

## Error Handling Strategy

### Error Types

Structured error handling with specific error types:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum XProfilerError {
    Configuration { message: String },
    Monitoring { message: String, monitor_type: MonitorType },
    Platform { message: String, platform: String },
    Io { message: String, source: String },
    Serialization { message: String },
    Synchronization { message: String },
}
```

### Error Recovery

Graceful degradation and recovery mechanisms:

```rust
impl CpuProfiler {
    fn handle_sampling_error(&mut self, error: XProfilerError) {
        match error {
            XProfilerError::Platform { .. } => {
                // Fallback to alternative sampling method
                self.use_fallback_sampling();
            }
            XProfilerError::Synchronization { .. } => {
                // Reset synchronization primitives
                self.reset_sync_state();
            }
            _ => {
                // Log error and continue
                log::warn!("Sampling error: {:?}", error);
            }
        }
    }
}
```

## Performance Considerations

### Sampling Strategy

- **Adaptive Sampling**: Adjust sampling rate based on system load
- **Statistical Sampling**: Use statistical methods to reduce overhead
- **Lazy Evaluation**: Defer expensive operations until needed

### Memory Optimization

- **Zero-Copy**: Minimize data copying where possible
- **Compression**: Compress historical data
- **Circular Buffers**: Use fixed-size buffers for continuous data

### CPU Optimization

- **Lock-Free Algorithms**: Use atomic operations where possible
- **SIMD Instructions**: Leverage vectorization for data processing
- **Branch Prediction**: Optimize hot paths for better prediction

## Platform Abstraction

### Operating System Support

```rust
#[cfg(target_os = "linux")]
mod linux {
    pub fn get_cpu_usage() -> Result<f64, PlatformError> {
        // Linux-specific implementation
    }
}

#[cfg(target_os = "macos")]
mod macos {
    pub fn get_cpu_usage() -> Result<f64, PlatformError> {
        // macOS-specific implementation
    }
}

#[cfg(target_os = "windows")]
mod windows {
    pub fn get_cpu_usage() -> Result<f64, PlatformError> {
        // Windows-specific implementation
    }
}
```

### Architecture Support

```rust
#[cfg(target_arch = "x86_64")]
mod x86_64 {
    pub fn get_stack_trace() -> Vec<StackFrame> {
        // x86_64-specific stack walking
    }
}

#[cfg(target_arch = "aarch64")]
mod aarch64 {
    pub fn get_stack_trace() -> Vec<StackFrame> {
        // ARM64-specific stack walking
    }
}
```

## Integration Points

### Node.js Integration

```rust
#[napi]
pub struct NodeProfiler {
    inner: Arc<Mutex<CpuProfiler>>,
}

#[napi]
impl NodeProfiler {
    #[napi(constructor)]
    pub fn new(config: Option<String>) -> napi::Result<Self> {
        // Create profiler from Node.js
    }
    
    #[napi]
    pub fn start(&self) -> napi::Result<()> {
        // Start profiling from Node.js
    }
}
```

### C FFI

```rust
#[no_mangle]
pub extern "C" fn xprofiler_create() -> *mut c_void {
    let profiler = Box::new(CpuProfiler::new(ProfilerConfig::default()).unwrap());
    Box::into_raw(profiler) as *mut c_void
}

#[no_mangle]
pub extern "C" fn xprofiler_start(profiler: *mut c_void) -> i32 {
    let profiler = unsafe { &mut *(profiler as *mut CpuProfiler) };
    match profiler.start() {
        Ok(()) => 0,
        Err(_) => -1,
    }
}
```

### WebAssembly Support

```rust
#[wasm_bindgen]
pub struct WasmProfiler {
    inner: CpuProfiler,
}

#[wasm_bindgen]
impl WasmProfiler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: CpuProfiler::new(ProfilerConfig::default()).unwrap(),
        }
    }
    
    #[wasm_bindgen]
    pub fn start(&mut self) -> Result<(), JsValue> {
        self.inner.start().map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
```

## Future Considerations

### Extensibility

- Plugin architecture for custom profilers
- Dynamic configuration updates
- Runtime feature toggling

### Scalability

- Distributed profiling support
- Cloud-native integration
- Kubernetes operator

### Observability

- OpenTelemetry integration
- Prometheus metrics export
- Jaeger tracing support

This architecture provides a solid foundation for high-performance profiling while maintaining flexibility and extensibility for future enhancements.