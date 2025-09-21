use criterion::{black_box, criterion_group, criterion_main, Criterion};
use xprofiler_rs::utils::{get_process_info, format_bytes};
use xprofiler_rs::platform::{get_platform_capabilities, optimizations};

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

criterion_group!(
    benches,
    platform_detection,
    system_info_collection,
    process_info_collection,
    log_operations,
    memory_formatting
);
criterion_main!(benches);