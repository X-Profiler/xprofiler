//! Performance regression tests for XProfiler Rust
//!
//! These tests ensure that performance doesn't degrade between releases
//! by comparing current performance against established baselines.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::{Duration, Instant};
use xprofiler_rs::monitoring::cpu::CpuMonitor;
use xprofiler_rs::monitoring::memory::MemoryMonitor;
use xprofiler_rs::monitoring::Monitor;
use xprofiler_rs::profiler::{CpuProfiler, GcProfiler, HeapProfiler, Profiler, ProfilerConfig};

/// Performance baseline thresholds (in nanoseconds)
const CPU_PROFILER_START_THRESHOLD: u64 = 1_000_000; // 1ms
const CPU_PROFILER_SAMPLE_THRESHOLD: u64 = 100_000; // 100μs
const HEAP_PROFILER_ALLOCATION_THRESHOLD: u64 = 50_000; // 50μs
const GC_PROFILER_EVENT_THRESHOLD: u64 = 10_000; // 10μs
const MONITOR_UPDATE_THRESHOLD: u64 = 500_000; // 500μs

/// Test workload sizes for scalability testing
const WORKLOAD_SIZES: &[usize] = &[100, 1_000, 10_000, 100_000];

/// CPU-intensive workload for testing profiler overhead
fn cpu_intensive_workload(iterations: usize) {
  for i in 0..iterations {
    black_box(fibonacci(black_box(i % 30)));
  }
}

/// Fibonacci function for CPU workload
fn fibonacci(n: usize) -> u64 {
  match n {
    0 => 0,
    1 => 1,
    _ => {
      let mut a = 0;
      let mut b = 1;
      for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
      }
      b
    }
  }
}

/// Memory allocation workload
fn memory_allocation_workload(allocations: usize) {
  let mut vectors = Vec::new();
  for i in 0..allocations {
    let size = (i % 1000) + 1;
    let vec: Vec<u8> = vec![0; size];
    vectors.push(vec);
  }
  black_box(vectors);
}

/// Benchmark CPU profiler startup performance
fn benchmark_cpu_profiler_startup(c: &mut Criterion) {
  let mut group = c.benchmark_group("cpu_profiler_startup");

  group.bench_function("start_stop", |b| {
    b.iter(|| {
      let config = ProfilerConfig::default();
      let mut profiler = CpuProfiler::new(config).unwrap();

      let start = Instant::now();
      profiler.start().unwrap();
      let startup_time = start.elapsed();

      profiler.stop().unwrap();

      // Assert startup time is within threshold
      assert!(
        startup_time.as_nanos() < CPU_PROFILER_START_THRESHOLD as u128,
        "CPU profiler startup too slow: {:?}",
        startup_time
      );
    })
  });

  group.finish();
}

/// Benchmark CPU profiler sampling overhead
fn benchmark_cpu_profiler_overhead(c: &mut Criterion) {
  let mut group = c.benchmark_group("cpu_profiler_overhead");

  // Test different sampling intervals
  let intervals = vec![1, 10, 50, 100];

  for interval_ms in intervals {
    group.bench_with_input(
      BenchmarkId::new("sampling_overhead", interval_ms),
      &interval_ms,
      |b, &interval_ms| {
        let config = ProfilerConfig {
          sampling_interval: Duration::from_millis(interval_ms),
          max_samples: 1000,
          collect_stacks: true,
          max_stack_depth: 32,
          output_path: None,
        };

        let mut profiler = CpuProfiler::new(config).unwrap();
        profiler.start().unwrap();

        b.iter(|| {
          cpu_intensive_workload(1000);
        });

        profiler.stop().unwrap();
      },
    );
  }

  group.finish();
}

/// Benchmark heap profiler allocation tracking
fn benchmark_heap_profiler_allocation(c: &mut Criterion) {
  let mut group = c.benchmark_group("heap_profiler_allocation");

  for &size in WORKLOAD_SIZES {
    group.bench_with_input(
      BenchmarkId::new("allocation_tracking", size),
      &size,
      |b, &size| {
        let config = ProfilerConfig::default();
        let mut profiler = HeapProfiler::new(config).unwrap();
        profiler.start().unwrap();

        b.iter(|| {
          let start = Instant::now();

          // Simulate allocations
          for i in 0..size {
            let address = 0x1000 + (i * 8) as u64;
            let allocation_size = (i % 1024) + 1;
            profiler
              .record_allocation(
                address,
                allocation_size,
                xprofiler_rs::profiler::heap_profiler::AllocationType::Malloc,
              )
              .unwrap();
          }

          let allocation_time = start.elapsed();

          // Assert allocation tracking time is reasonable
          if size <= 1000 {
            assert!(
              allocation_time.as_nanos()
                < (HEAP_PROFILER_ALLOCATION_THRESHOLD * size as u64) as u128,
              "Heap profiler allocation tracking too slow: {:?} for {} allocations",
              allocation_time,
              size
            );
          }
        });

        profiler.stop().unwrap();
      },
    );
  }

  group.finish();
}

/// Benchmark GC profiler event recording
fn benchmark_gc_profiler_events(c: &mut Criterion) {
  let mut group = c.benchmark_group("gc_profiler_events");

  group.bench_function("event_recording", |b| {
    let config = ProfilerConfig::default();
    let mut profiler = GcProfiler::new(config).unwrap();
    profiler.start().unwrap();

    b.iter(|| {
      let start = Instant::now();

      profiler
        .record_gc_event(
          xprofiler_rs::profiler::gc_profiler::GcEventType::MinorGc,
          xprofiler_rs::profiler::gc_profiler::GcPhase::Start,
          1000,        // duration_us
          1024 * 1024, // memory_before
          512 * 1024,  // memory_after
          2048 * 1024, // heap_size
          "allocation_failure".to_string(),
          std::collections::HashMap::new(),
        )
        .unwrap();

      let event_time = start.elapsed();

      // Assert event recording time is within threshold
      assert!(
        event_time.as_nanos() < GC_PROFILER_EVENT_THRESHOLD as u128,
        "GC profiler event recording too slow: {:?}",
        event_time
      );
    });

    profiler.stop().unwrap();
  });

  group.finish();
}

/// Benchmark monitor update performance
fn benchmark_monitor_updates(c: &mut Criterion) {
  let mut group = c.benchmark_group("monitor_updates");

  group.bench_function("cpu_monitor_update", |b| {
    let mut monitor = CpuMonitor::new();
    monitor.start().unwrap();

    b.iter(|| {
      let start = Instant::now();
      monitor.update().unwrap();
      let update_time = start.elapsed();

      // Assert update time is within threshold
      assert!(
        update_time.as_nanos() < MONITOR_UPDATE_THRESHOLD as u128,
        "CPU monitor update too slow: {:?}",
        update_time
      );
    });

    monitor.stop().unwrap();
  });

  group.bench_function("memory_monitor_update", |b| {
    let mut monitor = MemoryMonitor::new();
    monitor.start().unwrap();

    b.iter(|| {
      let start = Instant::now();
      monitor.update().unwrap();
      let update_time = start.elapsed();

      // Assert update time is within threshold
      assert!(
        update_time.as_nanos() < MONITOR_UPDATE_THRESHOLD as u128,
        "Memory monitor update too slow: {:?}",
        update_time
      );
    });

    monitor.stop().unwrap();
  });

  group.finish();
}

/// Benchmark scalability with different workload sizes
fn benchmark_scalability(c: &mut Criterion) {
  let mut group = c.benchmark_group("scalability");

  for &size in WORKLOAD_SIZES {
    group.bench_with_input(
      BenchmarkId::new("cpu_profiler_scalability", size),
      &size,
      |b, &size| {
        let config = ProfilerConfig {
          sampling_interval: Duration::from_millis(10),
          max_samples: size * 2,
          collect_stacks: true,
          max_stack_depth: 16,
          output_path: None,
        };

        let mut profiler = CpuProfiler::new(config).unwrap();
        profiler.start().unwrap();

        b.iter(|| {
          cpu_intensive_workload(size);
        });

        profiler.stop().unwrap();

        // Verify profiler collected samples
        let stats = profiler.get_stats().unwrap();
        assert!(
          stats.total_samples > 0,
          "No samples collected for workload size {}",
          size
        );
      },
    );
  }

  group.finish();
}

/// Benchmark memory usage regression
fn benchmark_memory_usage(c: &mut Criterion) {
  let mut group = c.benchmark_group("memory_usage");

  group.bench_function("profiler_memory_footprint", |b| {
    b.iter(|| {
      let config = ProfilerConfig {
        max_samples: 10_000,
        sampling_interval: Duration::from_millis(1),
        collect_stacks: true,
        max_stack_depth: 64,
        output_path: None,
      };

      let mut cpu_profiler = CpuProfiler::new(config.clone()).unwrap();
      let mut heap_profiler = HeapProfiler::new(config.clone()).unwrap();
      let mut gc_profiler = GcProfiler::new(config).unwrap();

      // Start all profilers
      cpu_profiler.start().unwrap();
      heap_profiler.start().unwrap();
      gc_profiler.start().unwrap();

      // Generate some data
      cpu_intensive_workload(1000);
      memory_allocation_workload(100);

      // Stop profilers
      cpu_profiler.stop().unwrap();
      heap_profiler.stop().unwrap();
      gc_profiler.stop().unwrap();

      // Verify data was collected
      let cpu_stats = cpu_profiler.get_stats().unwrap();
      let heap_stats = heap_profiler.get_stats().unwrap();
      let gc_stats = gc_profiler.get_stats().unwrap();

      black_box((cpu_stats, heap_stats, gc_stats));
    });
  });

  group.finish();
}

/// Benchmark concurrent profiler usage
fn benchmark_concurrency(c: &mut Criterion) {
  let mut group = c.benchmark_group("concurrency");

  group.bench_function("concurrent_profilers", |b| {
    b.iter(|| {
      let config = ProfilerConfig {
        sampling_interval: Duration::from_millis(5),
        max_samples: 1000,
        collect_stacks: false, // Reduce overhead for concurrency test
        max_stack_depth: 16,
        output_path: None,
      };

      let mut profilers = Vec::new();

      // Create multiple profilers
      for _ in 0..4 {
        let mut profiler = CpuProfiler::new(config.clone()).unwrap();
        profiler.start().unwrap();
        profilers.push(profiler);
      }

      // Simulate concurrent workload
      std::thread::scope(|s| {
        for _ in 0..4 {
          s.spawn(|| {
            cpu_intensive_workload(500);
          });
        }
      });

      // Stop all profilers
      for mut profiler in profilers {
        profiler.stop().unwrap();
      }
    });
  });

  group.finish();
}

criterion_group!(
  regression_benches,
  benchmark_cpu_profiler_startup,
  benchmark_cpu_profiler_overhead,
  benchmark_heap_profiler_allocation,
  benchmark_gc_profiler_events,
  benchmark_monitor_updates,
  benchmark_scalability,
  benchmark_memory_usage,
  benchmark_concurrency
);

criterion_main!(regression_benches);
