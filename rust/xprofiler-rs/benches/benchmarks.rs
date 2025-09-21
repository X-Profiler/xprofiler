use criterion::{black_box, criterion_group, criterion_main, Criterion};
use xprofiler_rs::monitoring::*;
use xprofiler_rs::logger::*;
use xprofiler_rs::config::*;
use xprofiler_rs::environment::*;

fn cpu_monitoring(c: &mut Criterion) {
    c.bench_function("cpu_monitoring", |b| {
        b.iter(|| {
            let mut monitor = cpu::CpuMonitor::new();
            monitor.start().unwrap();
            monitor.update_cpu_usage().unwrap();
        })
    });
}

fn memory_monitoring(c: &mut Criterion) {
    c.bench_function("memory_monitoring", |b| {
        b.iter(|| {
            let mut monitor = memory::MemoryMonitor::new();
            monitor.start().unwrap();
            monitor.update().unwrap();
        })
    });
}

fn logging_operations(c: &mut Criterion) {
    c.bench_function("logging_operations", |b| {
        b.iter(|| {
            let _ = info("Benchmark log message");
        })
    });
}

fn config_operations(c: &mut Criterion) {
    c.bench_function("config_operations", |b| {
        b.iter(|| {
            let _ = set_config("test_key", ConfigValue::String("test_value".to_string()));
            let _ = get_config("test_key");
        })
    });
}

fn environment_operations(c: &mut Criterion) {
    c.bench_function("environment_operations", |b| {
        b.iter(|| {
            let _ = setup_environment_data();
            let _ = get_current_environment_data();
        })
    });
}

criterion_group!(
    benches,
    cpu_monitoring,
    memory_monitoring,
    logging_operations,
    config_operations,
    environment_operations
);
criterion_main!(benches);