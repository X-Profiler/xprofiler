use criterion::{black_box, criterion_group, criterion_main, Criterion};
use xprofiler_rs::utils::{get_process_info, format_bytes};
use xprofiler_rs::platform::{get_platform_capabilities, optimizations};
use xprofiler_rs::monitoring::{
    Monitor,
    cpu::CpuMonitor,
    memory::MemoryMonitor,
    gc::{GcMonitor, GcEvent, GcType},
    http::{HttpMonitor, HttpRequest, HttpResponse},
    libuv::{LibuvMonitor, HandleType}
};
use std::time::{Duration, Instant};

fn platform_detection(c: &mut Criterion) {
    c.bench_function("platform_detection", |b| {
        b.iter(|| {
            black_box(get_platform_capabilities());
        })
    });
}

fn system_info_collection(c: &mut Criterion) {
    c.bench_function("system_info_collection", |b| {
        b.iter(|| {
            let timestamp_fn = optimizations::get_timestamp_fn();
            black_box(timestamp_fn());
            black_box(optimizations::get_memory_alignment());
            black_box(optimizations::get_io_buffer_size());
        })
    });
}

fn process_info_collection(c: &mut Criterion) {
    c.bench_function("process_info_collection", |b| {
        b.iter(|| {
            black_box(get_process_info());
        })
    });
}

fn log_operations(c: &mut Criterion) {
    c.bench_function("log_operations", |b| {
        b.iter(|| {
            let timestamp_fn = optimizations::get_timestamp_fn();
            black_box(timestamp_fn());
            black_box(timestamp_fn());
        })
    });
}

fn memory_formatting(c: &mut Criterion) {
    c.bench_function("memory_formatting", |b| {
        b.iter(|| {
            black_box(format_bytes(black_box(1024 * 1024 * 512))); // 512MB
        })
    });
}

fn cpu_monitor_benchmark(c: &mut Criterion) {
    c.bench_function("cpu_monitor_stats_collection", |b| {
        let mut monitor = CpuMonitor::new();
        monitor.start().unwrap();
        
        b.iter(|| {
            black_box(monitor.get_stats());
        });
        
        monitor.stop().unwrap();
    });
}

fn memory_monitor_benchmark(c: &mut Criterion) {
    c.bench_function("memory_monitor_stats_collection", |b| {
        let mut monitor = MemoryMonitor::new();
        monitor.start().unwrap();
        
        b.iter(|| {
            black_box(monitor.get_stats());
        });
        
        monitor.stop().unwrap();
    });
}

fn gc_monitor_benchmark(c: &mut Criterion) {
    c.bench_function("gc_event_recording", |b| {
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
            monitor.record_gc_event(black_box(gc_event));
        });
        
        monitor.stop().unwrap();
    });
}

fn http_monitor_benchmark(c: &mut Criterion) {
    c.bench_function("http_request_response_recording", |b| {
        let mut monitor = HttpMonitor::new();
        monitor.start().unwrap();
        
        b.iter(|| {
            let request = HttpRequest {
                method: "GET".to_string(),
                url: "/api/test".to_string(),
                headers_size: 256,
                body_size: 0,
                timestamp: Instant::now(),
                user_agent: Some("benchmark".to_string()),
                remote_ip: Some("127.0.0.1".to_string()),
            };
            let request_id = format!("req_{}", black_box(1));
            monitor.record_request(request_id.clone(), black_box(request));
            
            let response = HttpResponse {
                status_code: 200,
                headers_size: 128,
                body_size: 1024,
                response_time: Duration::from_millis(50),
                timestamp: Instant::now(),
            };
            monitor.record_response(request_id, black_box(response));
        });
        
        monitor.stop().unwrap();
    });
}

fn libuv_monitor_benchmark(c: &mut Criterion) {
    c.bench_function("libuv_handle_operations", |b| {
        let mut monitor = LibuvMonitor::new();
        monitor.start().unwrap();
        
        b.iter(|| {
            let handle_id = monitor.register_handle(
                black_box(HandleType::Timer),
                black_box(true),
                black_box(true)
            );
            monitor.unregister_handle(black_box(handle_id));
        });
        
        monitor.stop().unwrap();
    });
}

criterion_group!(
    benches,
    platform_detection,
    system_info_collection,
    process_info_collection,
    log_operations,
    memory_formatting,
    cpu_monitor_benchmark,
    memory_monitor_benchmark,
    gc_monitor_benchmark,
    http_monitor_benchmark,
    libuv_monitor_benchmark
);
criterion_main!(benches);