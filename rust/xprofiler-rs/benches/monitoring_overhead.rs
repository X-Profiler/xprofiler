//! Monitoring overhead benchmarks for the xprofiler-rs library.
//!
//! This module measures the performance overhead introduced by the monitoring system.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::{Duration, Instant};
use std::thread;
use xprofiler_rs::monitoring::Monitor;
use xprofiler_rs::monitoring::cpu::CpuMonitor;
use xprofiler_rs::monitoring::memory::MemoryMonitor;
use xprofiler_rs::monitoring::gc::{GcMonitor, GcType, GcEvent};
use xprofiler_rs::monitoring::http::{HttpMonitor, HttpRequest, HttpResponse};
use xprofiler_rs::monitoring::libuv::{LibuvMonitor, HandleType};

/// Simulates a CPU-intensive workload
fn cpu_intensive_work(iterations: usize) -> u64 {
    let mut result = 0u64;
    for i in 0..iterations {
        result = result.wrapping_add(i as u64);
        result = result.wrapping_mul(17);
        result = result.wrapping_add(result >> 8);
    }
    result
}

/// Simulates memory allocation patterns
fn memory_intensive_work(allocations: usize, size: usize) -> Vec<Vec<u8>> {
    let mut data = Vec::new();
    for i in 0..allocations {
        let mut allocation = vec![0u8; size];
        // Touch the memory to ensure it's actually allocated
        for j in 0..size.min(1000) {
            allocation[j] = (i + j) as u8;
        }
        data.push(allocation);
    }
    data
}

/// Simulates HTTP request processing
fn simulate_http_request_processing(request_count: usize) {
    for i in 0..request_count {
        // Simulate request parsing
        let _url = format!("/api/endpoint/{}", i);
        let _headers = vec![("Content-Type", "application/json"), ("User-Agent", "test")];
        
        // Simulate some processing work
        let _result = cpu_intensive_work(100);
        
        // Simulate response generation
        let _response_body = format!("{{\"id\": {}, \"status\": \"ok\"}}", i);
        
        // Small delay to simulate I/O
        thread::sleep(Duration::from_micros(10));
    }
}

fn bench_cpu_monitoring_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("cpu_monitoring_overhead");
    
    // Baseline: CPU work without monitoring
    group.bench_function("baseline_cpu_work", |b| {
        b.iter(|| {
            let result = cpu_intensive_work(10000);
            black_box(result);
        })
    });
    
    // With CPU monitoring enabled
    group.bench_function("with_cpu_monitoring", |b| {
        let mut monitor = CpuMonitor::new();
        monitor.start().unwrap();
        
        b.iter(|| {
            let result = cpu_intensive_work(10000);
            monitor.update().unwrap();
            black_box(result);
        })
    });
    
    // Overhead calculation
    group.bench_function("monitoring_only", |b| {
        let mut monitor = CpuMonitor::new();
        monitor.start().unwrap();
        
        b.iter(|| {
            monitor.update().unwrap();
        })
    });
    
    group.finish();
}

fn bench_memory_monitoring_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_monitoring_overhead");
    
    // Baseline: Memory work without monitoring
    group.bench_function("baseline_memory_work", |b| {
        b.iter(|| {
            let data = memory_intensive_work(100, 1024);
            black_box(data);
        })
    });
    
    // With memory monitoring enabled
    group.bench_function("with_memory_monitoring", |b| {
        let mut monitor = MemoryMonitor::new();
        monitor.start().unwrap();
        
        b.iter(|| {
            let data = memory_intensive_work(100, 1024);
            monitor.update().unwrap();
            black_box(data);
        })
    });
    
    // Overhead calculation
    group.bench_function("monitoring_only", |b| {
        let mut monitor = MemoryMonitor::new();
        monitor.start().unwrap();
        
        b.iter(|| {
            monitor.update().unwrap();
        })
    });
    
    group.finish();
}

fn bench_http_monitoring_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("http_monitoring_overhead");
    
    // Baseline: HTTP processing without monitoring
    group.bench_function("baseline_http_processing", |b| {
        b.iter(|| {
            simulate_http_request_processing(10);
        })
    });
    
    // With HTTP monitoring enabled
    group.bench_function("with_http_monitoring", |b| {
        let mut monitor = HttpMonitor::new();
        monitor.start().unwrap();
        
        b.iter(|| {
            let start_time = Instant::now();
            simulate_http_request_processing(10);
            let end_time = Instant::now();
            
            // Record monitoring data
            for i in 0..10 {
                let request = HttpRequest {
                    method: "GET".to_string(),
                    url: format!("/api/endpoint/{}", i),
                    timestamp: start_time,
                    headers_size: 512,
                    body_size: 256,
                    user_agent: Some("test".to_string()),
                    remote_ip: Some("127.0.0.1".to_string()),
                };
                
                let response = HttpResponse {
                    status_code: 200,
                    timestamp: end_time,
                    headers_size: 256,
                    body_size: 128,
                    response_time: end_time.duration_since(start_time) / 10,
                };
                
                let request_id = format!("req_{}", i);
                monitor.record_request(request_id.clone(), request);
                monitor.record_response(request_id, response);
            }
        })
    });
    
    // Overhead calculation
    group.bench_function("monitoring_only", |b| {
        let mut monitor = HttpMonitor::new();
        monitor.start().unwrap();
        
        b.iter(|| {
            for i in 0..10 {
                let request = HttpRequest {
                    method: "GET".to_string(),
                    url: format!("/api/endpoint/{}", i),
                    timestamp: Instant::now(),
                    headers_size: 512,
                    body_size: 256,
                    user_agent: Some("test".to_string()),
                    remote_ip: Some("127.0.0.1".to_string()),
                };
                
                let response = HttpResponse {
                    status_code: 200,
                    timestamp: Instant::now(),
                    headers_size: 256,
                    body_size: 128,
                    response_time: Duration::from_millis(10),
                };
                
                let request_id = format!("req_{}", i);
                monitor.record_request(request_id.clone(), request);
                monitor.record_response(request_id, response);
            }
        })
    });
    
    group.finish();
}

fn bench_gc_monitoring_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("gc_monitoring_overhead");
    
    // Baseline: Memory allocation without GC monitoring
    group.bench_function("baseline_allocation_work", |b| {
        b.iter(|| {
            let data = memory_intensive_work(50, 2048);
            // Simulate GC by dropping data
            drop(data);
        })
    });
    
    // With GC monitoring enabled
    group.bench_function("with_gc_monitoring", |b| {
        let mut monitor = GcMonitor::new();
        monitor.start().unwrap();
        
        b.iter(|| {
            let gc_start = Instant::now();
            let data = memory_intensive_work(50, 2048);
            let gc_end = Instant::now();
            
            // Simulate GC event recording
            let gc_event = GcEvent {
                gc_type: GcType::Scavenge,
                duration: gc_end.duration_since(gc_start),
                timestamp: gc_start,
                heap_size_before: 50 * 2048,
                heap_size_after: 25 * 2048,
            };
            monitor.record_gc_event(gc_event);
            
            drop(data);
        })
    });
    
    // Overhead calculation
    group.bench_function("monitoring_only", |b| {
        let mut monitor = GcMonitor::new();
        monitor.start().unwrap();
        
        b.iter(|| {
            let gc_event = GcEvent {
                gc_type: GcType::Scavenge,
                duration: Duration::from_millis(5),
                timestamp: Instant::now(),
                heap_size_before: 50 * 2048,
                heap_size_after: 25 * 2048,
            };
            monitor.record_gc_event(gc_event);
        })
    });
    
    group.finish();
}

fn bench_libuv_monitoring_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("libuv_monitoring_overhead");
    
    // Baseline: Event loop simulation without monitoring
    group.bench_function("baseline_event_loop", |b| {
        b.iter(|| {
            // Simulate event loop work
            for i in 0..100 {
                let _work = cpu_intensive_work(50);
                thread::sleep(Duration::from_micros(i % 10));
            }
        })
    });
    
    // With libuv monitoring enabled
    group.bench_function("with_libuv_monitoring", |b| {
        let mut monitor = LibuvMonitor::new();
        monitor.start().unwrap();
        
        // Pre-register some handles
        let mut handles = Vec::new();
        for i in 0..10 {
            let handle_type = match i % 4 {
                0 => HandleType::Timer,
                1 => HandleType::Tcp,
                2 => HandleType::Udp,
                _ => HandleType::FsEvent,
            };
            let handle_id = monitor.register_handle(handle_type, true, i % 2 == 0);
            handles.push(handle_id);
        }
        
        b.iter(|| {
            // Simulate event loop work with monitoring
            for i in 0..100 {
                let loop_start = Instant::now();
                let _work = cpu_intensive_work(50);
                let loop_end = Instant::now();
                
                // Record loop iteration
                monitor.record_loop_iteration(
                    loop_end.duration_since(loop_start),
                    Duration::from_micros(i % 10),
                    Duration::from_micros(i % 5),
                    Duration::from_micros(i % 3),
                    Duration::from_micros(i % 7),
                );
                
                // Update handle status occasionally
                if i % 10 == 0 {
                    let handle_id = handles[i as usize % handles.len()];
                    monitor.update_handle_status(handle_id, i % 2 == 0, i % 3 == 0);
                }
                
                thread::sleep(Duration::from_micros(i % 10));
            }
        })
    });
    
    // Overhead calculation
    group.bench_function("monitoring_only", |b| {
        let mut monitor = LibuvMonitor::new();
        monitor.start().unwrap();
        
        // Pre-register some handles
        let mut handles = Vec::new();
        for i in 0..10 {
            let handle_type = match i % 4 {
                0 => HandleType::Timer,
                1 => HandleType::Tcp,
                2 => HandleType::Udp,
                _ => HandleType::FsEvent,
            };
            let handle_id = monitor.register_handle(handle_type, true, i % 2 == 0);
            handles.push(handle_id);
        }
        
        b.iter(|| {
            for i in 0..100 {
                // Record loop iteration
                monitor.record_loop_iteration(
                    Duration::from_micros(i % 20),
                    Duration::from_micros(i % 10),
                    Duration::from_micros(i % 5),
                    Duration::from_micros(i % 3),
                    Duration::from_micros(i % 7),
                );
                
                // Update handle status occasionally
                if i % 10 == 0 {
                    let handle_id = handles[i as usize % handles.len()];
                    monitor.update_handle_status(handle_id, i % 2 == 0, i % 3 == 0);
                }
            }
        })
    });
    
    group.finish();
}

fn bench_all_monitors_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("all_monitors_overhead");
    
    // Baseline: Mixed workload without any monitoring
    group.bench_function("baseline_mixed_workload", |b| {
        b.iter(|| {
            // CPU work
            let _cpu_result = cpu_intensive_work(1000);
            
            // Memory work
            let _memory_data = memory_intensive_work(20, 1024);
            
            // HTTP simulation
            simulate_http_request_processing(5);
            
            // Event loop simulation
            for i in 0..20 {
                let _work = cpu_intensive_work(25);
                thread::sleep(Duration::from_micros(i % 5));
            }
        })
    });
    
    // With all monitors enabled
    group.bench_function("with_all_monitoring", |b| {
        let mut cpu_monitor = CpuMonitor::new();
        let mut memory_monitor = MemoryMonitor::new();
        let mut gc_monitor = GcMonitor::new();
        let mut http_monitor = HttpMonitor::new();
        let mut libuv_monitor = LibuvMonitor::new();
        
        cpu_monitor.start().unwrap();
        memory_monitor.start().unwrap();
        gc_monitor.start().unwrap();
        http_monitor.start().unwrap();
        libuv_monitor.start().unwrap();
        
        // Pre-register libuv handles
        let handle_id = libuv_monitor.register_handle(HandleType::Timer, true, false);
        
        b.iter(|| {
            // CPU work with monitoring
            let _cpu_result = cpu_intensive_work(1000);
            cpu_monitor.update().unwrap();
            
            // Memory work with monitoring
            let gc_start = Instant::now();
            let _memory_data = memory_intensive_work(20, 1024);
            let gc_end = Instant::now();
            memory_monitor.update().unwrap();
            
            // Record GC event
            let gc_event = GcEvent {
                gc_type: GcType::Scavenge,
                duration: gc_end.duration_since(gc_start),
                timestamp: gc_start,
                heap_size_before: 20 * 1024,
                heap_size_after: 10 * 1024,
            };
            gc_monitor.record_gc_event(gc_event);
            
            // HTTP simulation with monitoring
            let http_start = Instant::now();
            simulate_http_request_processing(5);
            let http_end = Instant::now();
            
            for i in 0..5 {
                let request = HttpRequest {
                    method: "GET".to_string(),
                    url: format!("/api/endpoint/{}", i),
                    timestamp: http_start,
                    headers_size: 512,
                    body_size: 256,
                    user_agent: Some("test".to_string()),
                    remote_ip: Some("127.0.0.1".to_string()),
                };
                
                let response = HttpResponse {
                    status_code: 200,
                    timestamp: http_end,
                    headers_size: 256,
                    body_size: 128,
                    response_time: http_end.duration_since(http_start) / 5,
                };
                
                let request_id = format!("req_{}", i);
                http_monitor.record_request(request_id.clone(), request);
                http_monitor.record_response(request_id, response);
            }
            
            // Event loop simulation with monitoring
            for i in 0..20 {
                let loop_start = Instant::now();
                let _work = cpu_intensive_work(25);
                let loop_end = Instant::now();
                
                libuv_monitor.record_loop_iteration(
                    loop_end.duration_since(loop_start),
                    Duration::from_micros(i % 5),
                    Duration::from_micros(i % 3),
                    Duration::from_micros(i % 2),
                    Duration::from_micros(i % 4),
                );
                
                if i % 5 == 0 {
                    libuv_monitor.update_handle_status(handle_id, true, i % 10 == 0);
                }
                
                thread::sleep(Duration::from_micros(i % 5));
            }
        })
    });
    
    group.finish();
}

criterion_group!(
    overhead_benches,
    bench_cpu_monitoring_overhead,
    bench_memory_monitoring_overhead,
    bench_http_monitoring_overhead,
    bench_gc_monitoring_overhead,
    bench_libuv_monitoring_overhead,
    bench_all_monitors_overhead
);
criterion_main!(overhead_benches);