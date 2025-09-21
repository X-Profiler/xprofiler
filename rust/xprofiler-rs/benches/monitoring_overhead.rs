//! Performance benchmarks for monitoring overhead
//!
//! This module contains benchmarks to measure the performance impact
//! of various monitoring operations to ensure minimal overhead.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use xprofiler_rs::monitoring::{
    cpu::{CpuMonitor, init_cpu_monitor, update_cpu_usage},
    memory::{MemoryMonitor, init_memory_monitor, update_memory_usage},
    MonitoringResult,
};

/// Benchmark CPU monitoring overhead
fn bench_cpu_monitoring(c: &mut Criterion) {
    // Initialize CPU monitor
    let _ = init_cpu_monitor();
    
    c.bench_function("cpu_monitor_update", |b| {
        b.iter(|| {
            black_box(update_cpu_usage()).unwrap();
        })
    });
    
    c.bench_function("cpu_monitor_single_update", |b| {
        let mut monitor = CpuMonitor::new();
        b.iter(|| {
            black_box(monitor.update()).unwrap();
        })
    });
}

/// Benchmark memory monitoring overhead
fn bench_memory_monitoring(c: &mut Criterion) {
    // Initialize memory monitor
    let _ = init_memory_monitor();
    
    c.bench_function("memory_monitor_update", |b| {
        b.iter(|| {
            black_box(update_memory_usage()).unwrap();
        })
    });
    
    c.bench_function("memory_monitor_single_update", |b| {
        let mut monitor = MemoryMonitor::new();
        b.iter(|| {
            black_box(monitor.update()).unwrap();
        })
    });
}

/// Benchmark concurrent monitoring overhead
fn bench_concurrent_monitoring(c: &mut Criterion) {
    let _ = init_cpu_monitor();
    let _ = init_memory_monitor();
    
    c.bench_function("concurrent_cpu_memory_updates", |b| {
        b.iter(|| {
            let cpu_handle = thread::spawn(|| {
                black_box(update_cpu_usage()).unwrap();
            });
            
            let memory_handle = thread::spawn(|| {
                black_box(update_memory_usage()).unwrap();
            });
            
            cpu_handle.join().unwrap();
            memory_handle.join().unwrap();
        })
    });
}

/// Benchmark monitoring data retrieval overhead
fn bench_data_retrieval(c: &mut Criterion) {
    let _ = init_cpu_monitor();
    let _ = init_memory_monitor();
    
    // Populate some data first
    for _ in 0..10 {
        update_cpu_usage().unwrap();
        update_memory_usage().unwrap();
        thread::sleep(Duration::from_millis(10));
    }
    
    c.bench_function("cpu_data_retrieval", |b| {
        let monitor = CpuMonitor::new();
        b.iter(|| {
            black_box(monitor.get_cpu_usage());
        })
    });
    
    c.bench_function("memory_data_retrieval", |b| {
        let monitor = MemoryMonitor::new();
        b.iter(|| {
            let _ = black_box(monitor.get_memory_usage());
        })
    });
}

/// Benchmark monitoring with different update frequencies
fn bench_update_frequencies(c: &mut Criterion) {
    let mut group = c.benchmark_group("update_frequencies");
    
    for freq_ms in [1, 10, 100, 1000].iter() {
        group.bench_with_input(
            format!("cpu_updates_{}ms", freq_ms),
            freq_ms,
            |b, &freq_ms| {
                let mut monitor = CpuMonitor::new();
                b.iter(|| {
                    for _ in 0..10 {
                        black_box(monitor.update()).unwrap();
                        thread::sleep(Duration::from_millis(freq_ms));
                    }
                })
            },
        );
    }
    
    group.finish();
}

/// Benchmark memory allocation patterns during monitoring
fn bench_memory_allocation_patterns(c: &mut Criterion) {
    c.bench_function("monitor_with_allocations", |b| {
        let mut monitor = MemoryMonitor::new();
        b.iter(|| {
            // Simulate some memory allocations
            let _data: Vec<u8> = vec![0; 1024];
            let _more_data: Vec<String> = (0..100)
                .map(|i| format!("test_string_{}", i))
                .collect();
            
            black_box(monitor.update()).unwrap();
        })
    });
}

/// Benchmark error handling overhead
fn bench_error_handling(c: &mut Criterion) {
    c.bench_function("error_creation_overhead", |b| {
        b.iter(|| {
            let result: MonitoringResult<()> = Err(
                xprofiler_rs::monitoring::error::MonitoringError::SystemCall {
                    operation: "test_operation".to_string(),
                    source: Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Test error")),
                }
            );
            black_box(result).unwrap_err();
        })
    });
}

criterion_group!(
    benches,
    bench_cpu_monitoring,
    bench_memory_monitoring,
    bench_concurrent_monitoring,
    bench_data_retrieval,
    bench_update_frequencies,
    bench_memory_allocation_patterns,
    bench_error_handling
);

criterion_main!(benches);