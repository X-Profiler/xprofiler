//! Benchmarks for the xprofiler-rs library.
//!
//! This module contains performance benchmarks for all monitoring components.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::time::{Duration, Instant};
use xprofiler_rs::monitoring::Monitor;
use xprofiler_rs::monitoring::cpu::CpuMonitor;
use xprofiler_rs::monitoring::memory::MemoryMonitor;
use xprofiler_rs::monitoring::gc::{GcMonitor, GcType, GcEvent};
use xprofiler_rs::monitoring::http::{HttpMonitor, HttpRequest, HttpResponse};
use xprofiler_rs::monitoring::libuv::{LibuvMonitor, HandleType};

fn bench_cpu_monitor(c: &mut Criterion) {
    let mut group = c.benchmark_group("cpu_monitor");
    
    group.bench_function("create_and_start", |b| {
        b.iter(|| {
            let mut monitor = CpuMonitor::new();
            monitor.start().unwrap();
            black_box(monitor);
        })
    });
    
    group.bench_function("update_stats", |b| {
        let mut monitor = CpuMonitor::new();
        monitor.start().unwrap();
        
        b.iter(|| {
            monitor.update().unwrap();
        })
    });
    
    group.bench_function("get_stats", |b| {
        let mut monitor = CpuMonitor::new();
        monitor.start().unwrap();
        monitor.update().unwrap();
        
        b.iter(|| {
            let stats = monitor.get_stats().unwrap();
            black_box(stats);
        })
    });
    
    group.finish();
}

fn bench_memory_monitor(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_monitor");
    
    group.bench_function("create_and_start", |b| {
        b.iter(|| {
            let mut monitor = MemoryMonitor::new();
            monitor.start().unwrap();
            black_box(monitor);
        })
    });
    
    group.bench_function("update_stats", |b| {
        let mut monitor = MemoryMonitor::new();
        monitor.start().unwrap();
        
        b.iter(|| {
            monitor.update().unwrap();
        })
    });
    
    group.bench_function("get_stats", |b| {
        let mut monitor = MemoryMonitor::new();
        monitor.start().unwrap();
        monitor.update().unwrap();
        
        b.iter(|| {
            let stats = monitor.get_stats().unwrap();
            black_box(stats);
        })
    });
    
    group.finish();
}

fn bench_gc_monitor(c: &mut Criterion) {
    let mut group = c.benchmark_group("gc_monitor");
    
    group.bench_function("create_and_start", |b| {
        b.iter(|| {
            let mut monitor = GcMonitor::new();
            monitor.start().unwrap();
            black_box(monitor);
        })
    });
    
    group.bench_function("record_gc_event", |b| {
        let mut monitor = GcMonitor::new();
        monitor.start().unwrap();
        
        b.iter(|| {
            let gc_event = GcEvent {
                gc_type: GcType::Scavenge,
                duration: Duration::from_millis(10),
                timestamp: Instant::now(),
                heap_size_before: 1024 * 1024,
                heap_size_after: 512 * 1024,
            };
            monitor.record_gc_event(gc_event);
        })
    });
    
    group.bench_function("get_stats", |b| {
        let mut monitor = GcMonitor::new();
        monitor.start().unwrap();
        
        // Add some events first
        for i in 0..100 {
            let gc_event = GcEvent {
                gc_type: if i % 2 == 0 { GcType::Scavenge } else { GcType::MarkSweepCompact },
                duration: Duration::from_millis(i % 20),
                timestamp: Instant::now(),
                heap_size_before: (1024 + i) * 1024,
                heap_size_after: (512 + i) * 1024,
            };
            monitor.record_gc_event(gc_event);
        }
        
        b.iter(|| {
            let stats = monitor.get_stats().unwrap();
            black_box(stats);
        })
    });
    
    group.finish();
}

fn bench_http_monitor(c: &mut Criterion) {
    let mut group = c.benchmark_group("http_monitor");
    
    group.bench_function("create_and_start", |b| {
        b.iter(|| {
            let mut monitor = HttpMonitor::new();
            monitor.start().unwrap();
            black_box(monitor);
        })
    });
    
    group.bench_function("record_request", |b| {
        let mut monitor = HttpMonitor::new();
        monitor.start().unwrap();
        let mut counter = 0;
        
        b.iter(|| {
            let request = HttpRequest {
                method: "GET".to_string(),
                url: format!("/api/test/{}", counter),
                timestamp: Instant::now(),
                headers_size: 1024,
                body_size: 512,
                user_agent: Some("benchmark-agent".to_string()),
                remote_ip: Some("127.0.0.1".to_string()),
            };
            
            monitor.record_request(format!("req_{}", counter), request);
            counter += 1;
        })
    });
    
    group.bench_function("record_response", |b| {
        let mut monitor = HttpMonitor::new();
        monitor.start().unwrap();
        
        // Pre-populate with requests
        for i in 0..1000 {
            let request = HttpRequest {
                method: "GET".to_string(),
                url: format!("/api/test/{}", i),
                timestamp: Instant::now(),
                headers_size: 1024,
                body_size: 512,
                user_agent: Some("benchmark-agent".to_string()),
                remote_ip: Some("127.0.0.1".to_string()),
            };
            monitor.record_request(format!("req_{}", i), request);
        }
        
        let mut counter = 0;
        b.iter(|| {
            let response = HttpResponse {
                status_code: 200,
                timestamp: Instant::now(),
                headers_size: 512,
                body_size: 1024,
                response_time: Duration::from_millis(50),
            };
            
            monitor.record_response(format!("req_{}", counter % 1000), response);
            counter += 1;
        })
    });
    
    group.bench_function("get_stats", |b| {
        let mut monitor = HttpMonitor::new();
        monitor.start().unwrap();
        
        // Add some transactions first
        for i in 0..100 {
            let request = HttpRequest {
                method: "GET".to_string(),
                url: format!("/api/test/{}", i),
                timestamp: Instant::now(),
                headers_size: 1024,
                body_size: 512,
                user_agent: Some("benchmark-agent".to_string()),
                remote_ip: Some("127.0.0.1".to_string()),
            };
            
            let response = HttpResponse {
                status_code: 200,
                timestamp: Instant::now(),
                headers_size: 512,
                body_size: 1024,
                response_time: Duration::from_millis(i % 100),
            };
            
            let request_id = format!("req_{}", i);
            monitor.record_request(request_id.clone(), request);
            monitor.record_response(request_id, response);
        }
        
        b.iter(|| {
            let stats = monitor.get_stats().unwrap();
            black_box(stats);
        })
    });
    
    group.finish();
}

fn bench_libuv_monitor(c: &mut Criterion) {
    let mut group = c.benchmark_group("libuv_monitor");
    
    group.bench_function("create_and_start", |b| {
        b.iter(|| {
            let mut monitor = LibuvMonitor::new();
            monitor.start().unwrap();
            black_box(monitor);
        })
    });
    
    group.bench_function("register_handle", |b| {
        let mut monitor = LibuvMonitor::new();
        monitor.start().unwrap();
        
        b.iter(|| {
            let handle_id = monitor.register_handle(HandleType::Timer, true, false);
            black_box(handle_id);
        })
    });
    
    group.bench_function("update_handle_status", |b| {
        let mut monitor = LibuvMonitor::new();
        monitor.start().unwrap();
        
        // Pre-register some handles
        let mut handles = Vec::new();
        for _ in 0..100 {
            let handle_id = monitor.register_handle(HandleType::Tcp, true, false);
            handles.push(handle_id);
        }
        
        let mut counter = 0;
        b.iter(|| {
            let handle_id = handles[counter % handles.len()];
            monitor.update_handle_status(handle_id, true, counter % 2 == 0);
            counter += 1;
        })
    });
    
    group.bench_function("record_loop_iteration", |b| {
        let mut monitor = LibuvMonitor::new();
        monitor.start().unwrap();
        
        b.iter(|| {
            monitor.record_loop_iteration(
                Duration::from_millis(10),
                Duration::from_millis(2),
                Duration::from_millis(3),
                Duration::from_millis(1),
                Duration::from_millis(4),
            );
        })
    });
    
    group.bench_function("get_stats", |b| {
        let mut monitor = LibuvMonitor::new();
        monitor.start().unwrap();
        
        // Add some handles and loop iterations
        for i in 0..50 {
            monitor.register_handle(HandleType::Timer, true, i % 2 == 0);
            monitor.record_loop_iteration(
                Duration::from_millis(i % 20),
                Duration::from_millis(i % 5),
                Duration::from_millis(i % 3),
                Duration::from_millis(i % 2),
                Duration::from_millis(i % 7),
            );
        }
        
        b.iter(|| {
            let stats = monitor.get_stats().unwrap();
            black_box(stats);
        })
    });
    
    group.finish();
}

fn bench_concurrent_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_access");
    
    for thread_count in [1, 2, 4, 8].iter() {
        group.bench_with_input(
            BenchmarkId::new("http_monitor_concurrent", thread_count),
            thread_count,
            |b, &thread_count| {
                b.iter(|| {
                    use std::sync::{Arc, Mutex};
                    use std::thread;
                    
                    let monitor = Arc::new(Mutex::new(HttpMonitor::new()));
                    monitor.lock().unwrap().start().unwrap();
                    
                    let mut handles = vec![];
                    
                    for i in 0..thread_count {
                        let monitor_clone = Arc::clone(&monitor);
                        let handle = thread::spawn(move || {
                            for j in 0..10 {
                                let request = HttpRequest {
                                    method: "GET".to_string(),
                                    url: format!("/api/thread/{}/req/{}", i, j),
                                    timestamp: Instant::now(),
                                    headers_size: 1024,
                                    body_size: 512,
                                    user_agent: Some("concurrent-test".to_string()),
                                    remote_ip: Some("127.0.0.1".to_string()),
                                };
                                
                                let response = HttpResponse {
                                    status_code: 200,
                                    timestamp: Instant::now(),
                                    headers_size: 512,
                                    body_size: 1024,
                                    response_time: Duration::from_millis(25),
                                };
                                
                                let request_id = format!("thread_{}_{}", i, j);
                                {
                                    let mut mon = monitor_clone.lock().unwrap();
                                    mon.record_request(request_id.clone(), request);
                                    mon.record_response(request_id, response);
                                }
                            }
                        });
                        handles.push(handle);
                    }
                    
                    for handle in handles {
                        handle.join().unwrap();
                    }
                    
                    black_box(monitor);
                })
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_cpu_monitor,
    bench_memory_monitor,
    bench_gc_monitor,
    bench_http_monitor,
    bench_libuv_monitor,
    bench_concurrent_access
);
criterion_main!(benches);