//! Memory usage tests for xprofiler-rs
//!
//! This module contains tests to verify that the monitoring modules
//! have minimal memory overhead and don't cause memory leaks.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use xprofiler_rs::monitoring::cpu::CpuMonitor;
use xprofiler_rs::monitoring::gc::{GcEvent, GcMonitor, GcType};
use xprofiler_rs::monitoring::http::{HttpMonitor, HttpRequest, HttpResponse};
use xprofiler_rs::monitoring::libuv::{HandleType, LibuvMonitor};
use xprofiler_rs::monitoring::memory::MemoryMonitor;
use xprofiler_rs::monitoring::Monitor;

/// Helper function to get current memory usage
fn get_memory_usage() -> usize {
  // Use a simple approximation of memory usage
  // In a real scenario, you might use a more sophisticated method
  let mut memory_monitor = MemoryMonitor::new();
  memory_monitor.start().unwrap();
  memory_monitor.update().unwrap();
  let stats = memory_monitor.get_stats().unwrap();
  memory_monitor.stop().unwrap();
  stats.rss as usize
}

/// Helper function to measure memory usage of a closure
fn measure_memory_usage<F>(f: F) -> (usize, usize)
where
  F: FnOnce(),
{
  // Force garbage collection before measurement
  std::hint::black_box(vec![0u8; 1024]); // Small allocation to trigger any pending cleanup

  let memory_before = get_memory_usage();
  f();
  let memory_after = get_memory_usage();

  (memory_before, memory_after)
}

#[cfg(test)]
mod memory_overhead_tests {
  use super::*;

  #[test]
  fn test_cpu_monitor_memory_overhead() {
    let (memory_before, memory_after) = measure_memory_usage(|| {
      let mut monitor = CpuMonitor::new();
      monitor.start().unwrap();

      // Perform multiple updates to ensure steady state
      for _ in 0..100 {
        monitor.update().unwrap();
        thread::sleep(Duration::from_millis(1));
      }

      monitor.stop().unwrap();
    });

    let memory_overhead = memory_after.saturating_sub(memory_before);
    println!("CPU Monitor memory overhead: {} bytes", memory_overhead);

    // CPU monitor should use less than 6MB of additional memory
    assert!(
      memory_overhead < 6 * 1024 * 1024,
      "CPU monitor memory overhead too high: {} bytes",
      memory_overhead
    );
  }

  #[test]
  fn test_memory_monitor_memory_overhead() {
    let (memory_before, memory_after) = measure_memory_usage(|| {
      let mut monitor = MemoryMonitor::new();
      monitor.start().unwrap();

      // Perform multiple updates
      for _ in 0..100 {
        monitor.update().unwrap();
        thread::sleep(Duration::from_millis(1));
      }

      monitor.stop().unwrap();
    });

    let memory_overhead = memory_after.saturating_sub(memory_before);
    println!("Memory Monitor memory overhead: {} bytes", memory_overhead);

    // Memory monitor should use less than 8MB of additional memory
    assert!(
      memory_overhead < 8 * 1024 * 1024,
      "Memory monitor memory overhead too high: {} bytes",
      memory_overhead
    );
  }

  #[test]
  fn test_gc_monitor_memory_overhead() {
    let (memory_before, memory_after) = measure_memory_usage(|| {
      let mut monitor = GcMonitor::new();
      monitor.start().unwrap();

      // Record multiple GC events
      for i in 0..1000 {
        let gc_event = GcEvent {
          gc_type: if i % 2 == 0 {
            GcType::Scavenge
          } else {
            GcType::MarkSweepCompact
          },
          duration: Duration::from_millis(i % 50),
          timestamp: Instant::now(),
          heap_size_before: (i * 1024) as u64,
          heap_size_after: ((i + 1) * 1024) as u64,
        };
        monitor.record_gc_event(gc_event);
      }

      monitor.stop().unwrap();
    });

    let memory_overhead = memory_after.saturating_sub(memory_before);
    println!("GC Monitor memory overhead: {} bytes", memory_overhead);

    // GC monitor should use less than 6MB even with 1000 events
    assert!(
      memory_overhead < 6 * 1024 * 1024,
      "GC monitor memory overhead too high: {} bytes",
      memory_overhead
    );
  }

  #[test]
  fn test_http_monitor_memory_overhead() {
    let (memory_before, memory_after) = measure_memory_usage(|| {
      let mut monitor = HttpMonitor::new();
      monitor.start().unwrap();

      // Record multiple HTTP transactions
      for i in 0..1000 {
        let request_id = format!("req_{}", i);

        let request = HttpRequest {
          method: "GET".to_string(),
          url: format!("/api/test/{}", i),
          timestamp: Instant::now(),
          headers_size: 1024,
          body_size: 0,
          user_agent: Some("test-agent".to_string()),
          remote_ip: Some("127.0.0.1".to_string()),
        };

        let response = HttpResponse {
          status_code: 200,
          timestamp: Instant::now(),
          headers_size: 512,
          body_size: 1024,
          response_time: Duration::from_millis(i % 100),
        };

        monitor.record_request(request_id.clone(), request);
        monitor.record_response(request_id, response);
      }

      monitor.stop().unwrap();
    });

    let memory_overhead = memory_after.saturating_sub(memory_before);
    println!("HTTP Monitor memory overhead: {} bytes", memory_overhead);

    // HTTP monitor should use less than 5MB even with 1000 transactions
    assert!(
      memory_overhead < 5 * 1024 * 1024,
      "HTTP monitor memory overhead too high: {} bytes",
      memory_overhead
    );
  }

  #[test]
  fn test_libuv_monitor_memory_overhead() {
    let (memory_before, memory_after) = measure_memory_usage(|| {
      let mut monitor = LibuvMonitor::new();
      monitor.start().unwrap();

      // Register multiple handles
      let mut handles = Vec::new();
      for i in 0..100 {
        let handle_type = match i % 4 {
          0 => HandleType::Timer,
          1 => HandleType::Tcp,
          2 => HandleType::FsEvent,
          _ => HandleType::Process,
        };
        let handle_id = format!("handle_{}", i);
        monitor
          .register_handle(handle_id.clone(), handle_type)
          .unwrap();
        handles.push(handle_id);
      }

      // Record multiple loop iterations
      for _i in 0..1000 {
        monitor.record_loop_iteration().unwrap();
      }

      // Unregister handles
      for handle in handles {
        monitor.unregister_handle(&handle).unwrap();
      }

      monitor.stop().unwrap();
    });

    let memory_overhead = memory_after.saturating_sub(memory_before);
    println!("Libuv Monitor memory overhead: {} bytes", memory_overhead);

    // Libuv monitor should use less than 6MB
    assert!(
      memory_overhead < 6 * 1024 * 1024,
      "Libuv monitor memory overhead too high: {} bytes",
      memory_overhead
    );
  }

  #[test]
  fn test_all_monitors_combined_memory_overhead() {
    let (memory_before, memory_after) = measure_memory_usage(|| {
      let mut cpu_monitor = CpuMonitor::new();
      let mut memory_monitor = MemoryMonitor::new();
      let mut gc_monitor = GcMonitor::new();
      let mut http_monitor = HttpMonitor::new();
      let mut libuv_monitor = LibuvMonitor::new();

      // Start all monitors
      cpu_monitor.start().unwrap();
      memory_monitor.start().unwrap();
      gc_monitor.start().unwrap();
      http_monitor.start().unwrap();
      libuv_monitor.start().unwrap();

      // Simulate moderate activity
      for i in 0..100 {
        // Update system monitors
        cpu_monitor.update().unwrap();
        memory_monitor.update().unwrap();

        // Record GC event
        let gc_event = GcEvent {
          gc_type: GcType::Scavenge,
          duration: Duration::from_millis(i % 10),
          timestamp: Instant::now(),
          heap_size_before: (i * 1024) as u64,
          heap_size_after: ((i + 1) * 1024) as u64,
        };
        gc_monitor.record_gc_event(gc_event);

        // Record HTTP transaction
        let request_id = format!("req_{}", i);
        let request = HttpRequest {
          method: "GET".to_string(),
          url: format!("/api/{}", i),
          timestamp: Instant::now(),
          headers_size: 1024,
          body_size: 0,
          user_agent: Some("test".to_string()),
          remote_ip: Some("127.0.0.1".to_string()),
        };
        let response = HttpResponse {
          status_code: 200,
          timestamp: Instant::now(),
          headers_size: 512,
          body_size: 1024,
          response_time: Duration::from_millis(i % 50),
        };
        http_monitor.record_request(request_id.clone(), request);
        http_monitor.record_response(request_id, response);

        // Record libuv activity
        libuv_monitor.record_loop_iteration().unwrap();

        thread::sleep(Duration::from_millis(1));
      }

      // Stop all monitors
      cpu_monitor.stop().unwrap();
      memory_monitor.stop().unwrap();
      gc_monitor.stop().unwrap();
      http_monitor.stop().unwrap();
      libuv_monitor.stop().unwrap();
    });

    let memory_overhead = memory_after.saturating_sub(memory_before);
    println!(
      "Combined monitors memory overhead: {} bytes",
      memory_overhead
    );

    // All monitors combined should use less than 15MB
    assert!(
      memory_overhead < 15 * 1024 * 1024,
      "Combined monitors memory overhead too high: {} bytes",
      memory_overhead
    );
  }
}

#[cfg(test)]
mod memory_leak_tests {
  use super::*;

  #[test]
  fn test_monitor_lifecycle_no_memory_leak() {
    let initial_memory = get_memory_usage();

    // Create and destroy monitors multiple times
    for cycle in 0..10 {
      let mut cpu_monitor = CpuMonitor::new();
      let mut memory_monitor = MemoryMonitor::new();
      let mut gc_monitor = GcMonitor::new();
      let mut http_monitor = HttpMonitor::new();
      let mut libuv_monitor = LibuvMonitor::new();

      // Start all monitors
      cpu_monitor.start().unwrap();
      memory_monitor.start().unwrap();
      gc_monitor.start().unwrap();
      http_monitor.start().unwrap();
      libuv_monitor.start().unwrap();

      // Simulate some activity
      for _ in 0..10 {
        cpu_monitor.update().unwrap();
        memory_monitor.update().unwrap();
        gc_monitor.update().unwrap();
        http_monitor.update().unwrap();
        libuv_monitor.update().unwrap();
        thread::sleep(Duration::from_millis(1));
      }

      // Stop all monitors
      cpu_monitor.stop().unwrap();
      memory_monitor.stop().unwrap();
      gc_monitor.stop().unwrap();
      http_monitor.stop().unwrap();
      libuv_monitor.stop().unwrap();

      // Force cleanup
      std::hint::black_box(vec![0u8; 1024]);

      if cycle % 3 == 0 {
        let current_memory = get_memory_usage();
        let memory_growth = current_memory.saturating_sub(initial_memory);
        println!(
          "Memory growth after {} cycles: {} bytes",
          cycle + 1,
          memory_growth
        );

        // Memory growth should be minimal (less than 8MB after multiple cycles)
        assert!(
          memory_growth < 8 * 1024 * 1024,
          "Potential memory leak detected: {} bytes growth after {} cycles",
          memory_growth,
          cycle + 1
        );
      }
    }

    let final_memory = get_memory_usage();
    let total_growth = final_memory.saturating_sub(initial_memory);
    println!("Total memory growth: {} bytes", total_growth);

    // Total memory growth should be minimal
    assert!(
      total_growth < 8 * 1024 * 1024,
      "Memory leak detected: {} bytes total growth",
      total_growth
    );
  }

  #[test]
  fn test_long_running_monitor_memory_stability() {
    let mut memory_monitor = MemoryMonitor::new();
    let mut http_monitor = HttpMonitor::new();

    memory_monitor.start().unwrap();
    http_monitor.start().unwrap();

    let initial_memory = get_memory_usage();
    let mut memory_samples = Vec::new();

    // Run for a longer period with continuous activity
    for i in 0..500 {
      memory_monitor.update().unwrap();

      // Record HTTP activity
      let request_id = format!("long_req_{}", i);
      let request = HttpRequest {
        method: "POST".to_string(),
        url: format!("/api/long/{}", i),
        timestamp: Instant::now(),
        headers_size: 1024,
        body_size: 2048,
        user_agent: Some("long-test".to_string()),
        remote_ip: Some("127.0.0.1".to_string()),
      };
      let response = HttpResponse {
        status_code: 200,
        timestamp: Instant::now(),
        headers_size: 512,
        body_size: 4096,
        response_time: Duration::from_millis(i % 100),
      };
      http_monitor.record_request(request_id.clone(), request);
      http_monitor.record_response(request_id, response);

      // Sample memory usage periodically
      if i % 50 == 0 {
        let current_memory = get_memory_usage();
        memory_samples.push(current_memory);
        println!("Memory at iteration {}: {} bytes", i, current_memory);
      }

      thread::sleep(Duration::from_millis(2));
    }

    memory_monitor.stop().unwrap();
    http_monitor.stop().unwrap();

    // Analyze memory stability
    let final_memory = get_memory_usage();
    let total_growth = final_memory.saturating_sub(initial_memory);

    // Check for memory growth trend
    if memory_samples.len() >= 3 {
      let early_avg = memory_samples[0..2].iter().sum::<usize>() / 2;
      let late_avg = memory_samples[memory_samples.len() - 2..]
        .iter()
        .sum::<usize>()
        / 2;
      let growth_trend = late_avg.saturating_sub(early_avg);

      println!("Memory growth trend: {} bytes", growth_trend);

      // Memory should not grow significantly over time
      assert!(
        growth_trend < 3 * 1024 * 1024,
        "Memory growth trend too high: {} bytes",
        growth_trend
      );
    }

    println!("Total memory growth in long run: {} bytes", total_growth);
    assert!(
      total_growth < 5 * 1024 * 1024,
      "Long-running memory growth too high: {} bytes",
      total_growth
    );
  }
}
