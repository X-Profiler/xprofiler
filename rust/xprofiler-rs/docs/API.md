# XProfiler Rust API Documentation

This document provides comprehensive API documentation for the XProfiler Rust library.

## Table of Contents

- [Core Traits](#core-traits)
- [Profiler Module](#profiler-module)
- [Monitoring Module](#monitoring-module)
- [Error Handling](#error-handling)
- [Configuration](#configuration)
- [Examples](#examples)

## Core Traits

### Profiler Trait

The `Profiler` trait defines the interface for all profiling components.

```rust
pub trait Profiler {
    /// Start the profiler
    fn start(&mut self) -> XProfilerResult<()>;
    
    /// Stop the profiler
    fn stop(&mut self) -> XProfilerResult<()>;
    
    /// Check if the profiler is running
    fn is_running(&self) -> bool;
    
    /// Get profiling results as JSON string
    fn get_results(&self) -> XProfilerResult<String>;
    
    /// Reset profiler state
    fn reset(&mut self) -> XProfilerResult<()>;
}
```

### Monitor Trait

The `Monitor` trait defines the interface for monitoring components.

```rust
pub trait Monitor {
    /// Start monitoring
    fn start(&mut self) -> XProfilerResult<()>;
    
    /// Stop monitoring
    fn stop(&mut self) -> XProfilerResult<()>;
    
    /// Update monitoring data
    fn update(&mut self) -> XProfilerResult<()>;
    
    /// Check if monitoring is active
    fn is_running(&self) -> bool;
    
    /// Get monitoring statistics
    fn get_stats(&self) -> XProfilerResult<String>;
}
```

## Profiler Module

### CPU Profiler

Provides CPU profiling capabilities with call stack sampling.

#### CpuProfiler

```rust
pub struct CpuProfiler {
    // Internal fields...
}

impl CpuProfiler {
    /// Create a new CPU profiler
    pub fn new(config: ProfilerConfig) -> XProfilerResult<Self>
    
    /// Get current call stack
    pub fn get_current_call_stack(&self) -> XProfilerResult<CallStack>
    
    /// Get profiling statistics
    pub fn get_stats(&self) -> XProfilerResult<CpuProfileStats>
    
    /// Get all collected samples
    pub fn get_samples(&self) -> XProfilerResult<Vec<CpuSample>>
}
```

#### Data Structures

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    pub function_name: String,
    pub file_name: Option<String>,
    pub line_number: Option<u32>,
    pub column_number: Option<u32>,
    pub address: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallStack {
    pub frames: Vec<StackFrame>,
    pub thread_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuSample {
    pub call_stack: CallStack,
    pub cpu_usage: f64,
    pub memory_usage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuProfileStats {
    pub total_samples: usize,
    pub duration_ms: u64,
    pub avg_cpu_usage: f64,
    pub peak_cpu_usage: f64,
    pub function_frequency: HashMap<String, usize>,
    pub hot_functions: Vec<(String, usize)>,
}
```

### Heap Profiler

Provides heap memory profiling with allocation tracking.

#### HeapProfiler

```rust
pub struct HeapProfiler {
    // Internal fields...
}

impl HeapProfiler {
    /// Create a new heap profiler
    pub fn new(config: ProfilerConfig) -> XProfilerResult<Self>
    
    /// Record a memory allocation
    pub fn record_allocation(
        &self,
        address: u64,
        size: usize,
        allocation_type: AllocationType,
    ) -> XProfilerResult<()>
    
    /// Record a memory deallocation
    pub fn record_deallocation(&self, address: u64) -> XProfilerResult<()>
    
    /// Detect potential memory leaks
    pub fn detect_memory_leaks(&self) -> XProfilerResult<Vec<MemoryLeak>>
    
    /// Get profiling statistics
    pub fn get_stats(&self) -> XProfilerResult<HeapProfileStats>
}
```

#### Data Structures

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AllocationType {
    Malloc,
    Calloc,
    Realloc,
    New,
    NewArray,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationEvent {
    pub timestamp: u64,
    pub size: usize,
    pub address: u64,
    pub call_stack: Vec<String>,
    pub thread_id: String,
    pub allocation_type: AllocationType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLeak {
    pub allocation: AllocationEvent,
    pub leak_duration_ms: u64,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeapProfileStats {
    pub total_allocations: usize,
    pub total_deallocations: usize,
    pub active_allocations: usize,
    pub total_bytes_allocated: usize,
    pub total_bytes_deallocated: usize,
    pub current_bytes_in_use: usize,
    pub peak_memory_usage: usize,
    pub memory_leaks: Vec<MemoryLeak>,
    pub size_distribution: HashMap<String, usize>,
    pub top_allocation_sites: HashMap<String, usize>,
    pub duration_ms: u64,
}
```

### GC Profiler

Provides garbage collection profiling capabilities.

#### GcProfiler

```rust
pub struct GcProfiler {
    // Internal fields...
}

impl GcProfiler {
    /// Create a new GC profiler
    pub fn new(config: ProfilerConfig) -> XProfilerResult<Self>
    
    /// Record a GC event
    pub fn record_gc_event(
        &self,
        event_type: GcEventType,
        phase: GcPhase,
        duration_us: u64,
        memory_before: usize,
        memory_after: usize,
        heap_size: usize,
        reason: String,
        metadata: HashMap<String, String>,
    ) -> XProfilerResult<()>
    
    /// Get profiling statistics
    pub fn get_stats(&self) -> XProfilerResult<GcProfileStats>
}
```

#### Data Structures

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GcEventType {
    MinorGc,
    MajorGc,
    FullGc,
    ConcurrentGc,
    IncrementalGc,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GcPhase {
    Start,
    Mark,
    Sweep,
    Compact,
    End,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcEvent {
    pub id: u64,
    pub event_type: GcEventType,
    pub phase: GcPhase,
    pub timestamp: u64,
    pub duration_us: u64,
    pub memory_before: usize,
    pub memory_after: usize,
    pub memory_reclaimed: usize,
    pub heap_size: usize,
    pub thread_id: String,
    pub reason: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcProfileStats {
    pub total_gc_events: usize,
    pub total_gc_time_us: u64,
    pub average_gc_time_us: u64,
    pub max_gc_time_us: u64,
    pub min_gc_time_us: u64,
    pub gc_frequency: f64,
    pub total_memory_reclaimed: usize,
    pub gc_overhead_percent: f64,
    pub gc_type_stats: HashMap<String, GcGenerationStats>,
    pub recent_events: Vec<GcEvent>,
    pub time_distribution: HashMap<String, usize>,
    pub reclamation_efficiency: f64,
    pub duration_ms: u64,
    pub longest_pause_us: u64,
    pub throughput_mb_per_sec: f64,
}
```

## Monitoring Module

### CPU Monitor

Monitors CPU usage and system load.

```rust
pub struct CpuMonitor {
    // Internal fields...
}

impl CpuMonitor {
    /// Create a new CPU monitor
    pub fn new() -> Self
    
    /// Get CPU statistics
    pub fn get_stats(&self) -> XProfilerResult<CpuStats>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuStats {
    pub usage_percent: f64,
    pub load_avg: [f64; 3],
    pub cpu_count: usize,
    pub uptime_seconds: u64,
}
```

### Memory Monitor

Monitors memory usage and heap statistics.

```rust
pub struct MemoryMonitor {
    // Internal fields...
}

impl MemoryMonitor {
    /// Create a new memory monitor
    pub fn new() -> Self
    
    /// Get memory statistics
    pub fn get_stats(&self) -> XProfilerResult<MemoryStats>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub rss: usize,
    pub heap_total: usize,
    pub heap_used: usize,
    pub external: usize,
    pub array_buffers: usize,
}
```

### HTTP Monitor

Monitors HTTP requests and responses.

```rust
pub struct HttpMonitor {
    // Internal fields...
}

impl HttpMonitor {
    /// Create a new HTTP monitor
    pub fn new() -> Self
    
    /// Record HTTP request
    pub fn record_request(&mut self, request_id: String, request: HttpRequest)
    
    /// Record HTTP response
    pub fn record_response(&mut self, request_id: String, response: HttpResponse)
    
    /// Get HTTP statistics
    pub fn get_stats(&self) -> XProfilerResult<HttpStats>
}
```

### Libuv Monitor

Monitors libuv handles and event loop metrics.

```rust
pub struct LibuvMonitor {
    // Internal fields...
}

impl LibuvMonitor {
    /// Create a new libuv monitor
    pub fn new() -> Self
    
    /// Register a handle
    pub fn register_handle(&mut self, handle_id: String, handle_type: HandleType) -> XProfilerResult<()>
    
    /// Unregister a handle
    pub fn unregister_handle(&mut self, handle_id: &str) -> XProfilerResult<()>
    
    /// Record event loop iteration
    pub fn record_loop_iteration(&mut self) -> XProfilerResult<()>
    
    /// Get libuv statistics
    pub fn get_stats(&self) -> XProfilerResult<LibuvStats>
}
```

## Error Handling

### XProfilerError

The main error type for the library.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum XProfilerError {
    /// Configuration errors
    Configuration {
        message: String,
    },
    /// Monitoring errors
    Monitoring {
        message: String,
        monitor_type: MonitorType,
    },
    /// Platform-specific errors
    Platform {
        message: String,
        platform: String,
    },
    /// I/O errors
    Io {
        message: String,
        source: String,
    },
    /// Serialization errors
    Serialization {
        message: String,
    },
    /// Thread synchronization errors
    Synchronization {
        message: String,
    },
}

pub type XProfilerResult<T> = Result<T, XProfilerError>;
```

## Configuration

### ProfilerConfig

Configuration for profiler components.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilerConfig {
    /// Sampling interval for profilers
    pub sampling_interval: Duration,
    /// Maximum number of samples to collect
    pub max_samples: usize,
    /// Enable stack trace collection
    pub enable_stack_traces: bool,
    /// Maximum stack depth to capture
    pub max_stack_depth: usize,
    /// Enable detailed timing information
    pub enable_timing: bool,
}

impl Default for ProfilerConfig {
    fn default() -> Self {
        Self {
            sampling_interval: Duration::from_millis(100),
            max_samples: 10000,
            enable_stack_traces: true,
            max_stack_depth: 64,
            enable_timing: true,
        }
    }
}
```

## Examples

### Basic CPU Profiling

```rust
use xprofiler_rs::profiler::{CpuProfiler, Profiler, ProfilerConfig};
use std::time::Duration;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ProfilerConfig::default();
    let mut profiler = CpuProfiler::new(config)?;
    
    // Start profiling
    profiler.start()?;
    
    // Simulate some work
    thread::sleep(Duration::from_secs(2));
    
    // Stop profiling and get results
    profiler.stop()?;
    let results = profiler.get_results()?;
    println!("CPU Profile Results: {}", results);
    
    Ok(())
}
```

### Memory Leak Detection

```rust
use xprofiler_rs::profiler::{HeapProfiler, Profiler, ProfilerConfig};
use xprofiler_rs::profiler::heap_profiler::AllocationType;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ProfilerConfig::default();
    let mut profiler = HeapProfiler::new(config)?;
    
    profiler.start()?;
    
    // Simulate memory allocations
    profiler.record_allocation(0x1000, 1024, AllocationType::Malloc)?;
    profiler.record_allocation(0x2000, 2048, AllocationType::New)?;
    
    // Only deallocate one allocation (simulate leak)
    profiler.record_deallocation(0x1000)?;
    
    // Check for leaks
    let leaks = profiler.detect_memory_leaks()?;
    println!("Detected {} potential memory leaks", leaks.len());
    
    profiler.stop()?;
    Ok(())
}
```

### Comprehensive Monitoring

```rust
use xprofiler_rs::monitoring::*;
use std::time::Duration;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cpu_monitor = cpu::CpuMonitor::new();
    let mut memory_monitor = memory::MemoryMonitor::new();
    let mut http_monitor = http::HttpMonitor::new();
    
    // Start all monitors
    cpu_monitor.start()?;
    memory_monitor.start()?;
    http_monitor.start()?;
    
    // Simulate application activity
    for i in 0..10 {
        thread::sleep(Duration::from_millis(100));
        
        // Update monitoring data
        cpu_monitor.update()?;
        memory_monitor.update()?;
        
        if i % 3 == 0 {
            // Simulate HTTP request every 300ms
            let request = http::HttpRequest {
                method: "GET".to_string(),
                url: format!("/api/data/{}", i),
                timestamp: std::time::Instant::now(),
                headers_size: 512,
                body_size: 0,
                user_agent: Some("XProfiler/1.0".to_string()),
                remote_ip: Some("127.0.0.1".to_string()),
            };
            
            let response = http::HttpResponse {
                status_code: 200,
                timestamp: std::time::Instant::now(),
                headers_size: 256,
                body_size: 1024,
                response_time: Duration::from_millis(50),
            };
            
            let request_id = format!("req_{}", i);
            http_monitor.record_request(request_id.clone(), request);
            http_monitor.record_response(request_id, response);
        }
    }
    
    // Get final statistics
    let cpu_stats = cpu_monitor.get_stats()?;
    let memory_stats = memory_monitor.get_stats()?;
    let http_stats = http_monitor.get_stats()?;
    
    println!("CPU Stats: {}", cpu_stats);
    println!("Memory Stats: {}", memory_stats);
    println!("HTTP Stats: {}", http_stats);
    
    Ok(())
}
```

For more examples, see the [examples directory](../examples/).