//! Benchmark tests for xprofiler-rs
//!
//! This module contains performance benchmarks to ensure
//! the xprofiler-rs library meets performance requirements.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use xprofiler_rs::bindings::*;

/// Benchmark metric recording performance
fn benchmark_metric_recording(c: &mut Criterion) {
    let mut group = c.benchmark_group("metric_recording");
    
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("counter", size), size, |b, &size| {
            let mut xprofiler = XProfiler::new();
            xprofiler.initialize(None).unwrap();
            xprofiler.start_profiling().unwrap();
            
            let monitoring_manager = xprofiler.get_monitoring_manager().unwrap();
            monitoring_manager.start().unwrap();
            
            b.iter(|| {
                for i in 0..size {
                    let metric_name = format!("benchmark_counter_{}", i % 10);
                    monitoring_manager.record_counter(
                        black_box(metric_name),
                        black_box(1.0),
                        None
                    ).unwrap();
                }
            });
            
            monitoring_manager.stop().unwrap();
            xprofiler.stop_profiling().unwrap();
            xprofiler.shutdown().unwrap();
        });
        
        group.bench_with_input(BenchmarkId::new("gauge", size), size, |b, &size| {
            let mut xprofiler = XProfiler::new();
            xprofiler.initialize(None).unwrap();
            xprofiler.start_profiling().unwrap();
            
            let monitoring_manager = xprofiler.get_monitoring_manager().unwrap();
            monitoring_manager.start().unwrap();
            
            b.iter(|| {
                for i in 0..size {
                    let metric_name = format!("benchmark_gauge_{}", i % 10);
                    monitoring_manager.record_gauge(
                        black_box(metric_name),
                        black_box(i as f64),
                        None
                    ).unwrap();
                }
            });
            
            monitoring_manager.stop().unwrap();
            xprofiler.stop_profiling().unwrap();
            xprofiler.shutdown().unwrap();
        });
        
        group.bench_with_input(BenchmarkId::new("histogram", size), size, |b, &size| {
            let mut xprofiler = XProfiler::new();
            xprofiler.initialize(None).unwrap();
            xprofiler.start_profiling().unwrap();
            
            let monitoring_manager = xprofiler.get_monitoring_manager().unwrap();
            monitoring_manager.start().unwrap();
            
            b.iter(|| {
                for i in 0..size {
                    let metric_name = format!("benchmark_histogram_{}", i % 10);
                    monitoring_manager.record_histogram(
                        black_box(metric_name),
                        black_box((i % 1000) as f64),
                        None
                    ).unwrap();
                }
            });
            
            monitoring_manager.stop().unwrap();
            xprofiler.stop_profiling().unwrap();
            xprofiler.shutdown().unwrap();
        });
    }
    
    group.finish();
}

/// Benchmark timer operations
fn benchmark_timer_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("timer_operations");
    
    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("start_end_timer", size), size, |b, &size| {
            let mut xprofiler = XProfiler::new();
            xprofiler.initialize(None).unwrap();
            xprofiler.start_profiling().unwrap();
            
            let monitoring_manager = xprofiler.get_monitoring_manager().unwrap();
            monitoring_manager.start().unwrap();
            
            b.iter(|| {
                for i in 0..size {
                    let timer_name = format!("benchmark_timer_{}", i % 10);
                    let timer_id = monitoring_manager.start_timer(
                        black_box(timer_name),
                        None
                    ).unwrap();
                    
                    // Simulate very short operation
                    black_box(i * 2);
                    
                    monitoring_manager.end_timer(black_box(timer_id)).unwrap();
                }
            });
            
            monitoring_manager.stop().unwrap();
            xprofiler.stop_profiling().unwrap();
            xprofiler.shutdown().unwrap();
        });
    }
    
    group.finish();
}

/// Benchmark logging operations
fn benchmark_logging_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("logging_operations");
    
    for size in [100, 1000, 5000].iter() {
        group.bench_with_input(BenchmarkId::new("log_messages", size), size, |b, &size| {
            let mut xprofiler = XProfiler::new();
            xprofiler.initialize(None).unwrap();
            
            let logger_manager = xprofiler.get_logger_manager().unwrap();
            
            b.iter(|| {
                for i in 0..size {
                    let message = format!("Benchmark log message {}", i);
                    logger_manager.log(
                        black_box("info".to_string()),
                        black_box(message),
                        None
                    ).unwrap();
                }
            });
            
            xprofiler.shutdown().unwrap();
        });
    }
    
    group.finish();
}

/// Benchmark configuration operations
fn benchmark_config_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("config_operations");
    
    group.bench_function("set_config", |b| {
        let mut xprofiler = XProfiler::new();
        xprofiler.initialize(None).unwrap();
        
        let config_manager = xprofiler.get_config_manager().unwrap();
        
        b.iter(|| {
            for i in 0..100 {
                let key = format!("benchmark_key_{}", i % 10);
                let value = format!("benchmark_value_{}", i);
                config_manager.set(
                    black_box(key),
                    black_box(napi::JsUnknown::from(value))
                ).unwrap();
            }
        });
        
        xprofiler.shutdown().unwrap();
    });
    
    group.bench_function("get_config", |b| {
        let mut xprofiler = XProfiler::new();
        xprofiler.initialize(None).unwrap();
        
        let config_manager = xprofiler.get_config_manager().unwrap();
        
        // Pre-populate configuration
        for i in 0..100 {
            let key = format!("benchmark_key_{}", i);
            let value = format!("benchmark_value_{}", i);
            config_manager.set(key, napi::JsUnknown::from(value)).unwrap();
        }
        
        b.iter(|| {
            for i in 0..100 {
                let key = format!("benchmark_key_{}", i % 100);
                let _ = config_manager.get(black_box(key));
            }
        });
        
        xprofiler.shutdown().unwrap();
    });
    
    group.finish();
}

/// Benchmark environment operations
fn benchmark_environment_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("environment_operations");
    
    group.bench_function("thread_registration", |b| {
        let mut xprofiler = XProfiler::new();
        xprofiler.initialize(None).unwrap();
        xprofiler.start_profiling().unwrap();
        
        let env_manager = xprofiler.get_environment_manager().unwrap();
        
        b.iter(|| {
            env_manager.register_thread().unwrap();
            let _ = env_manager.get_current_thread_data();
            env_manager.unregister_thread().unwrap();
        });
        
        xprofiler.stop_profiling().unwrap();
        xprofiler.shutdown().unwrap();
    });
    
    group.bench_function("get_thread_data", |b| {
        let mut xprofiler = XProfiler::new();
        xprofiler.initialize(None).unwrap();
        xprofiler.start_profiling().unwrap();
        
        let env_manager = xprofiler.get_environment_manager().unwrap();
        env_manager.register_thread().unwrap();
        
        b.iter(|| {
            let _ = env_manager.get_current_thread_data();
        });
        
        env_manager.unregister_thread().unwrap();
        xprofiler.stop_profiling().unwrap();
        xprofiler.shutdown().unwrap();
    });
    
    group.finish();
}

/// Benchmark concurrent operations
fn benchmark_concurrent_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_operations");
    
    for thread_count in [2, 4, 8].iter() {
        group.bench_with_input(
            BenchmarkId::new("concurrent_metrics", thread_count),
            thread_count,
            |b, &thread_count| {
                b.iter(|| {
                    let mut xprofiler = XProfiler::new();
                    xprofiler.initialize(None).unwrap();
                    xprofiler.start_profiling().unwrap();
                    
                    let monitoring_manager = Arc::new(xprofiler.get_monitoring_manager().unwrap());
                    monitoring_manager.start().unwrap();
                    
                    let handles: Vec<_> = (0..thread_count)
                        .map(|thread_id| {
                            let mon_mgr = Arc::clone(&monitoring_manager);
                            
                            thread::spawn(move || {
                                for i in 0..100 {
                                    let counter_name = format!("thread_{}_counter", thread_id);
                                    mon_mgr.record_counter(counter_name, 1.0, None).unwrap();
                                    
                                    if i % 10 == 0 {
                                        let gauge_name = format!("thread_{}_gauge", thread_id);
                                        mon_mgr.record_gauge(gauge_name, i as f64, None).unwrap();
                                    }
                                }
                            })
                        })
                        .collect();
                    
                    for handle in handles {
                        handle.join().unwrap();
                    }
                    
                    monitoring_manager.stop().unwrap();
                    xprofiler.stop_profiling().unwrap();
                    xprofiler.shutdown().unwrap();
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark memory usage patterns
fn benchmark_memory_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_operations");
    
    group.bench_function("initialization_shutdown_cycle", |b| {
        b.iter(|| {
            let mut xprofiler = XProfiler::new();
            xprofiler.initialize(None).unwrap();
            xprofiler.start_profiling().unwrap();
            
            let monitoring_manager = xprofiler.get_monitoring_manager().unwrap();
            monitoring_manager.start().unwrap();
            
            // Generate some data
            for i in 0..50 {
                let metric_name = format!("memory_test_metric_{}", i);
                monitoring_manager.record_counter(metric_name, 1.0, None).unwrap();
            }
            
            monitoring_manager.stop().unwrap();
            xprofiler.stop_profiling().unwrap();
            xprofiler.flush().unwrap();
            xprofiler.shutdown().unwrap();
        });
    });
    
    group.bench_function("large_metric_batch", |b| {
        let mut xprofiler = XProfiler::new();
        xprofiler.initialize(None).unwrap();
        xprofiler.start_profiling().unwrap();
        
        let monitoring_manager = xprofiler.get_monitoring_manager().unwrap();
        monitoring_manager.start().unwrap();
        
        b.iter(|| {
            // Record a large batch of metrics
            for i in 0..1000 {
                let counter_name = format!("large_batch_counter_{}", i % 100);
                monitoring_manager.record_counter(counter_name, 1.0, None).unwrap();
                
                if i % 10 == 0 {
                    let gauge_name = format!("large_batch_gauge_{}", i % 50);
                    monitoring_manager.record_gauge(gauge_name, i as f64, None).unwrap();
                }
                
                if i % 25 == 0 {
                    let histogram_name = format!("large_batch_histogram_{}", i % 20);
                    monitoring_manager.record_histogram(histogram_name, (i % 1000) as f64, None).unwrap();
                }
            }
        });
        
        monitoring_manager.stop().unwrap();
        xprofiler.stop_profiling().unwrap();
        xprofiler.shutdown().unwrap();
    });
    
    group.finish();
}

/// Benchmark data retrieval operations
fn benchmark_data_retrieval(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_retrieval");
    
    group.bench_function("get_metrics", |b| {
        let mut xprofiler = XProfiler::new();
        xprofiler.initialize(None).unwrap();
        xprofiler.start_profiling().unwrap();
        
        let monitoring_manager = xprofiler.get_monitoring_manager().unwrap();
        monitoring_manager.start().unwrap();
        
        // Pre-populate with metrics
        for i in 0..500 {
            let counter_name = format!("retrieval_counter_{}", i % 50);
            monitoring_manager.record_counter(counter_name, 1.0, None).unwrap();
            
            let gauge_name = format!("retrieval_gauge_{}", i % 30);
            monitoring_manager.record_gauge(gauge_name, i as f64, None).unwrap();
        }
        
        b.iter(|| {
            let _ = monitoring_manager.get_metrics();
        });
        
        monitoring_manager.stop().unwrap();
        xprofiler.stop_profiling().unwrap();
        xprofiler.shutdown().unwrap();
    });
    
    group.bench_function("get_statistics", |b| {
        let mut xprofiler = XProfiler::new();
        xprofiler.initialize(None).unwrap();
        xprofiler.start_profiling().unwrap();
        
        let monitoring_manager = xprofiler.get_monitoring_manager().unwrap();
        monitoring_manager.start().unwrap();
        
        // Pre-populate with data
        for i in 0..200 {
            let counter_name = format!("stats_counter_{}", i % 20);
            monitoring_manager.record_counter(counter_name, 1.0, None).unwrap();
        }
        
        b.iter(|| {
            let _ = xprofiler.get_statistics();
        });
        
        monitoring_manager.stop().unwrap();
        xprofiler.stop_profiling().unwrap();
        xprofiler.shutdown().unwrap();
    });
    
    group.bench_function("get_environment_data", |b| {
        let mut xprofiler = XProfiler::new();
        xprofiler.initialize(None).unwrap();
        xprofiler.start_profiling().unwrap();
        
        let env_manager = xprofiler.get_environment_manager().unwrap();
        env_manager.register_thread().unwrap();
        
        b.iter(|| {
            let _ = xprofiler.get_environment_data();
        });
        
        env_manager.unregister_thread().unwrap();
        xprofiler.stop_profiling().unwrap();
        xprofiler.shutdown().unwrap();
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_metric_recording,
    benchmark_timer_operations,
    benchmark_logging_operations,
    benchmark_config_operations,
    benchmark_environment_operations,
    benchmark_concurrent_operations,
    benchmark_memory_operations,
    benchmark_data_retrieval
);

criterion_main!(benches);