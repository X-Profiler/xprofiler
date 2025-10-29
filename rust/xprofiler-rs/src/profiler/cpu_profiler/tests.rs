//! Unit tests for CPU Profiler

use super::*;
use crate::profiler::Profiler;
use crate::profiler::ProfilerConfig;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[cfg(test)]
mod tests {
  use super::*;

  fn create_test_profiler() -> CpuProfiler {
    let config = ProfilerConfig {
      sampling_interval: Duration::from_millis(1),
      collect_stacks: true,
      max_stack_depth: 10,
      max_samples: 1000,
      output_path: None,
    };
    CpuProfiler::new(config).expect("Failed to create CPU profiler")
  }

  #[test]
  fn test_cpu_profiler_creation() {
    let profiler = create_test_profiler();
    assert!(!profiler.is_running());
    // Cannot access private field samples directly
    // assert_eq!(profiler.samples.len(), 0);
  }

  #[test]
  fn test_cpu_profiler_start_stop() {
    let mut profiler = create_test_profiler();

    // Test start
    profiler.start().expect("Failed to start profiler");
    assert!(profiler.is_running());

    // Test stop
    profiler.stop().expect("Failed to stop profiler");
    assert!(!profiler.is_running());
  }

  #[test]
  fn test_cpu_profiler_double_start() {
    let mut profiler = create_test_profiler();

    profiler.start().expect("Failed to start profiler");
    let result = profiler.start();
    assert!(result.is_err());

    profiler.stop().expect("Failed to stop profiler");
  }

  #[test]
  fn test_cpu_profiler_stop_without_start() {
    let mut profiler = create_test_profiler();
    let result = profiler.stop();
    assert!(result.is_err());
  }

  #[test]
  fn test_cpu_profiler_sampling() {
    let mut profiler = create_test_profiler();

    profiler.start().expect("Failed to start profiler");

    // Simulate some CPU work
    thread::sleep(Duration::from_millis(10));

    profiler.stop().expect("Failed to stop profiler");

    // Check that samples were collected through stats
    let stats = profiler.get_stats().expect("Failed to get stats");
    assert!(stats.total_samples > 0);
  }

  #[test]
  fn test_cpu_profiler_reset() {
    let mut profiler = create_test_profiler();

    profiler.start().expect("Failed to start profiler");
    thread::sleep(Duration::from_millis(5));
    profiler.stop().expect("Failed to stop profiler");

    let stats_before = profiler.get_stats().expect("Failed to get stats");
    assert!(stats_before.total_samples > 0);

    profiler.reset().expect("Failed to reset profiler");
    let stats_after = profiler.get_stats().expect("Failed to get stats");
    assert_eq!(stats_after.total_samples, 0);
    assert!(!profiler.is_running());
  }

  #[test]
  fn test_cpu_profiler_get_results() {
    let mut profiler = create_test_profiler();

    profiler.start().expect("Failed to start profiler");
    thread::sleep(Duration::from_millis(5));
    profiler.stop().expect("Failed to stop profiler");

    let results = profiler.get_results().expect("Failed to get results");
    assert!(!results.is_empty());

    // Verify it's valid JSON
    let _: serde_json::Value =
      serde_json::from_str(&results).expect("Results should be valid JSON");
  }

  #[test]
  fn test_cpu_profiler_get_stats() {
    let mut profiler = create_test_profiler();

    profiler.start().expect("Failed to start profiler");
    thread::sleep(Duration::from_millis(5));
    profiler.stop().expect("Failed to stop profiler");

    let stats = profiler.get_stats().expect("Failed to get stats");
    assert!(stats.total_samples > 0);
    assert!(stats.duration_ms > 0);
    assert!(stats.avg_cpu_usage >= 0.0);
    // Memory usage is not directly available in stats
  }

  #[test]
  fn test_cpu_profiler_max_samples() {
    let config = ProfilerConfig {
      sampling_interval: Duration::from_millis(1),
      collect_stacks: true,
      max_stack_depth: 5,
      max_samples: 5, // Very small limit
      output_path: None,
    };
    let mut profiler = CpuProfiler::new(config).expect("Failed to create profiler");

    profiler.start().expect("Failed to start profiler");
    thread::sleep(Duration::from_millis(20)); // Should exceed max samples
    profiler.stop().expect("Failed to stop profiler");

    // Should not exceed max samples
    let stats = profiler.get_stats().expect("Failed to get stats");
    assert!(stats.total_samples <= 5);
  }

  #[test]
  fn test_cpu_profiler_call_stack_collection() {
    let mut profiler = create_test_profiler();

    profiler.start().expect("Failed to start profiler");

    // Call a function to generate call stack
    fn test_function() {
      thread::sleep(Duration::from_millis(2));
    }
    test_function();

    profiler.stop().expect("Failed to stop profiler");

    // Check that call stacks were collected through results
    let results = profiler.get_results().expect("Failed to get results");
    // Results should contain stack information when collect_stacks is true
    assert!(results.contains("call_stack") || results.contains("stack"));
  }

  #[test]
  fn test_cpu_profiler_disabled_call_stack() {
    let config = ProfilerConfig {
      sampling_interval: Duration::from_millis(1),
      collect_stacks: false, // Disable call stack collection
      max_stack_depth: 10,
      max_samples: 1000,
      output_path: None,
    };
    let mut profiler = CpuProfiler::new(config).expect("Failed to create profiler");

    profiler.start().expect("Failed to start profiler");
    thread::sleep(Duration::from_millis(5));
    profiler.stop().expect("Failed to stop profiler");

    // When collect_stacks is false, results should not contain detailed stack info
    let results = profiler.get_results().expect("Failed to get results");
    // This is a basic check - in practice, you might want more sophisticated validation
    assert!(!results.is_empty());
  }

  #[test]
  fn test_cpu_profiler_concurrent_access() {
    use std::sync::{Arc, Mutex};

    let profiler = Arc::new(Mutex::new(create_test_profiler()));
    let profiler_clone = profiler.clone();

    let handle = thread::spawn(move || {
      let mut p = profiler_clone.lock().unwrap();
      p.start().expect("Failed to start profiler");
      thread::sleep(Duration::from_millis(5));
      p.stop().expect("Failed to stop profiler");
    });

    handle.join().expect("Thread panicked");

    let p = profiler.lock().unwrap();
    let stats = p.get_stats().expect("Failed to get stats");
    assert!(stats.total_samples > 0);
  }

  #[test]
  fn test_stack_frame_creation() {
    let frame = StackFrame {
      function_name: "test_function".to_string(),
      file_name: Some("test.rs".to_string()),
      line_number: Some(42),
      column_number: Some(10),
    };

    assert_eq!(frame.function_name, "test_function");
    assert_eq!(frame.file_name, Some("test.rs".to_string()));
    assert_eq!(frame.line_number, Some(42));
    assert_eq!(frame.column_number, Some(10));
  }

  #[test]
  fn test_cpu_sample_creation() {
    let frames = vec![StackFrame {
      function_name: "main".to_string(),
      file_name: Some("main.rs".to_string()),
      line_number: Some(1),
      column_number: Some(1),
    }];
    let call_stack = CallStack {
      frames,
      timestamp: 0,
      thread_id: 1,
    };
    let sample = CpuSample {
      call_stack,
      cpu_usage: 50.0,
      memory_usage: 1024 * 1024, // 1MB
    };

    assert_eq!(sample.cpu_usage, 50.0);
    assert_eq!(sample.memory_usage, 1024 * 1024);
    assert_eq!(sample.call_stack.frames.len(), 1);
    assert_eq!(sample.call_stack.frames[0].function_name, "main");
  }

  #[test]
  fn test_cpu_profile_stats_serialization() {
    let stats = CpuProfileStats {
      total_samples: 100,
      duration_ms: 1000,
      avg_cpu_usage: 45.5,
      peak_cpu_usage: 95.0,
      function_frequency: HashMap::new(),
      hot_functions: vec![("function1".to_string(), 10), ("function2".to_string(), 5)],
    };

    let json = serde_json::to_string(&stats).expect("Failed to serialize stats");
    let deserialized: CpuProfileStats =
      serde_json::from_str(&json).expect("Failed to deserialize stats");

    assert_eq!(stats.total_samples, deserialized.total_samples);
    assert_eq!(stats.duration_ms, deserialized.duration_ms);
    assert_eq!(stats.hot_functions, deserialized.hot_functions);
  }

  #[test]
  fn test_profiler_trait_implementation() {
    let mut profiler: Box<dyn Profiler> = Box::new(create_test_profiler());

    // Test trait methods
    profiler.start().expect("Failed to start profiler");
    thread::sleep(Duration::from_millis(2));
    profiler.stop().expect("Failed to stop profiler");

    let results = profiler.get_results().expect("Failed to get results");
    assert!(!results.is_empty());

    profiler.reset().expect("Failed to reset profiler");
  }
}

#[cfg(test)]
mod integration_tests {
  use super::*;
  use std::sync::{Arc, Mutex};
  use std::thread;
  use std::time::Duration;

  #[test]
  fn test_cpu_profiler_real_workload() {
    let mut profiler =
      CpuProfiler::new(ProfilerConfig::default()).expect("Failed to create profiler");

    profiler.start().expect("Failed to start profiler");

    // Simulate real CPU work
    let mut sum = 0u64;
    for i in 0..1000000 {
      sum = sum.wrapping_add(i);
    }

    profiler.stop().expect("Failed to stop profiler");

    let stats = profiler.get_stats().expect("Failed to get stats");
    assert!(stats.total_samples > 0);
    assert!(stats.avg_cpu_usage > 0.0);

    // Prevent optimization
    assert!(sum > 0);
  }

  #[test]
  fn test_cpu_profiler_multiple_sessions() {
    let mut profiler =
      CpuProfiler::new(ProfilerConfig::default()).expect("Failed to create profiler");

    // First session
    profiler.start().expect("Failed to start profiler");
    thread::sleep(Duration::from_millis(5));
    profiler.stop().expect("Failed to stop profiler");

    let first_stats = profiler.get_stats().expect("Failed to get stats");
    let first_samples = first_stats.total_samples;
    assert!(first_samples > 0);

    // Second session (should accumulate)
    profiler.start().expect("Failed to start profiler");
    thread::sleep(Duration::from_millis(5));
    profiler.stop().expect("Failed to stop profiler");

    let second_stats = profiler.get_stats().expect("Failed to get stats");
    assert!(second_stats.total_samples > first_samples);

    // Reset and third session
    profiler.reset().expect("Failed to reset profiler");
    profiler.start().expect("Failed to start profiler");
    thread::sleep(Duration::from_millis(5));
    profiler.stop().expect("Failed to stop profiler");

    let third_stats = profiler.get_stats().expect("Failed to get stats");
    let third_samples = third_stats.total_samples;
    assert!(third_samples > 0);
    assert!(third_samples < second_stats.total_samples); // Should be less after reset
  }

  #[test]
  fn test_cpu_profiler_stress_test() {
    let profiler = Arc::new(Mutex::new(
      CpuProfiler::new(ProfilerConfig {
        sampling_interval: Duration::from_millis(1),
        max_samples: 1000,
        collect_stacks: true,
        max_stack_depth: 64,
        output_path: None,
      })
      .expect("Failed to create profiler"),
    ));

    let handles: Vec<_> = (0..4)
      .map(|_| {
        let profiler_clone = profiler.clone();
        thread::spawn(move || {
          for _ in 0..10 {
            // Simulate work
            let mut sum = 0u64;
            for i in 0..10000 {
              sum = sum.wrapping_add(i);
            }
            thread::sleep(Duration::from_millis(1));
          }
        })
      })
      .collect();

    // Start profiling
    {
      let mut p = profiler.lock().unwrap();
      p.start().expect("Failed to start profiler");
    }

    // Wait for all threads to complete
    for handle in handles {
      handle.join().expect("Thread panicked");
    }

    // Stop profiling
    {
      let mut p = profiler.lock().unwrap();
      p.stop().expect("Failed to stop profiler");

      let stats = p.get_stats().expect("Failed to get stats");
      assert!(stats.total_samples > 0);
      assert!(stats.duration_ms > 0);
    }
  }
}
