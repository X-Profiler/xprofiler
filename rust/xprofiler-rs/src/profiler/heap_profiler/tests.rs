//! Unit tests for Heap Profiler

use super::*;
use crate::profiler::Profiler;
use crate::profiler::ProfilerConfig;
// Removed unused StackFrame import
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[cfg(test)]
mod tests {
  use super::*;

  fn create_test_profiler() -> HeapProfiler {
    let config = ProfilerConfig {
      sampling_interval: Duration::from_millis(1),
      collect_stacks: true,
      max_stack_depth: 10,
      max_samples: 1000,
      output_path: None,
    };
    HeapProfiler::new(config).expect("Failed to create heap profiler")
  }

  #[test]
  fn test_heap_profiler_creation() {
    let profiler = create_test_profiler();
    assert!(!profiler.is_running());
    // Cannot access private fields directly
    // assert_eq!(profiler.allocations.len(), 0);
    // assert_eq!(profiler.deallocations.len(), 0);
  }

  #[test]
  fn test_heap_profiler_start_stop() {
    let mut profiler = create_test_profiler();

    // Test start
    profiler.start().expect("Failed to start profiler");
    assert!(profiler.is_running());

    // Test stop
    profiler.stop().expect("Failed to stop profiler");
    assert!(!profiler.is_running());
  }

  #[test]
  fn test_heap_profiler_double_start() {
    let mut profiler = create_test_profiler();

    profiler.start().expect("Failed to start profiler");
    let result = profiler.start();
    assert!(result.is_err());

    profiler.stop().expect("Failed to stop profiler");
  }

  #[test]
  fn test_heap_profiler_stop_without_start() {
    let mut profiler = create_test_profiler();
    let result = profiler.stop();
    assert!(result.is_err());
  }

  #[test]
  fn test_record_allocation() {
    let mut profiler = create_test_profiler();
    profiler.start().expect("Failed to start profiler");

    let address = 0x1000;
    let size = 1024;
    let alloc_type = AllocationType::Malloc;

    profiler
      .record_allocation(address, size, alloc_type.clone())
      .expect("Failed to record allocation");

    // Cannot access private fields directly, use stats instead
    let stats = profiler.get_stats().expect("Failed to get stats");
    assert_eq!(stats.total_allocations, 1);

    profiler.stop().expect("Failed to stop profiler");
  }

  #[test]
  fn test_record_deallocation() {
    let mut profiler = create_test_profiler();
    profiler.start().expect("Failed to start profiler");

    let address = 0x1000;

    // First allocate
    profiler
      .record_allocation(address, 1024, AllocationType::Malloc)
      .expect("Failed to record allocation");

    // Then deallocate
    profiler
      .record_deallocation(address)
      .expect("Failed to record deallocation");

    // Cannot access private fields directly, use stats instead
    let stats = profiler.get_stats().expect("Failed to get stats");
    assert_eq!(stats.total_deallocations, 1);

    profiler.stop().expect("Failed to stop profiler");
  }

  #[test]
  fn test_detect_memory_leaks() {
    let mut profiler = create_test_profiler();
    profiler.start().expect("Failed to start profiler");

    // Record some allocations
    profiler
      .record_allocation(0x1000, 1024, AllocationType::Malloc)
      .expect("Failed to record allocation");
    profiler
      .record_allocation(0x2000, 2048, AllocationType::New)
      .expect("Failed to record allocation");
    profiler
      .record_allocation(0x3000, 512, AllocationType::Calloc)
      .expect("Failed to record allocation");

    // Only deallocate one
    profiler
      .record_deallocation(0x2000)
      .expect("Failed to record deallocation");

    let leaks = profiler
      .detect_memory_leaks()
      .expect("Failed to detect memory leaks");

    // Should have 2 leaks (0x1000 and 0x3000)
    assert_eq!(leaks.len(), 2);

    let leak_addresses: Vec<u64> = leaks.iter().map(|l| l.allocation.address).collect();
    assert!(leak_addresses.contains(&0x1000));
    assert!(leak_addresses.contains(&0x3000));
    assert!(!leak_addresses.contains(&0x2000)); // This was deallocated

    profiler.stop().expect("Failed to stop profiler");
  }

  #[test]
  fn test_get_heap_stats() {
    let mut profiler = create_test_profiler();
    profiler.start().expect("Failed to start profiler");

    // Record some allocations and deallocations
    profiler
      .record_allocation(0x1000, 1024, AllocationType::Malloc)
      .expect("Failed to record allocation");
    profiler
      .record_allocation(0x2000, 2048, AllocationType::New)
      .expect("Failed to record allocation");
    profiler
      .record_deallocation(0x1000)
      .expect("Failed to record deallocation");

    let stats = profiler.get_stats().expect("Failed to get heap stats");

    assert_eq!(stats.total_allocations, 2);
    assert_eq!(stats.total_deallocations, 1);
    assert_eq!(stats.total_bytes_allocated, 1024 + 2048);
    assert_eq!(stats.total_bytes_deallocated, 1024);
    assert_eq!(stats.current_bytes_in_use, 2048);
    assert_eq!(stats.peak_memory_usage, 1024 + 2048);
    assert_eq!(stats.memory_leaks.len(), 1);

    profiler.stop().expect("Failed to stop profiler");
  }

  #[test]
  fn test_heap_profiler_reset() {
    let mut profiler = create_test_profiler();
    profiler.start().expect("Failed to start profiler");

    // Record some data
    profiler
      .record_allocation(0x1000, 1024, AllocationType::Malloc)
      .expect("Failed to record allocation");
    profiler
      .record_deallocation(0x1000)
      .expect("Failed to record deallocation");

    let stats_before = profiler.get_stats().expect("Failed to get stats");
    assert_eq!(stats_before.total_allocations, 1);
    assert_eq!(stats_before.total_deallocations, 1);

    profiler.stop().expect("Failed to stop profiler");
    profiler.reset().expect("Failed to reset profiler");

    let stats_after = profiler.get_stats().expect("Failed to get stats");
    assert_eq!(stats_after.total_allocations, 0);
    assert_eq!(stats_after.total_deallocations, 0);
    assert!(!profiler.is_running());
  }

  #[test]
  fn test_heap_profiler_get_results() {
    let mut profiler = create_test_profiler();
    profiler.start().expect("Failed to start profiler");

    profiler
      .record_allocation(0x1000, 1024, AllocationType::Malloc)
      .expect("Failed to record allocation");

    profiler.stop().expect("Failed to stop profiler");

    let results = profiler.get_results().expect("Failed to get results");
    assert!(!results.is_empty());

    // Verify it's valid JSON
    let _: serde_json::Value =
      serde_json::from_str(&results).expect("Results should be valid JSON");
  }

  #[test]
  fn test_allocation_types() {
    let mut profiler = create_test_profiler();
    profiler.start().expect("Failed to start profiler");

    let types = vec![
      AllocationType::Malloc,
      AllocationType::Calloc,
      AllocationType::Realloc,
      AllocationType::New,
      AllocationType::NewArray,
      AllocationType::Other("custom".to_string()),
    ];

    for (i, alloc_type) in types.iter().enumerate() {
      profiler
        .record_allocation(0x1000 + (i as u64 * 0x1000), 1024, alloc_type.clone())
        .expect("Failed to record allocation");
    }

    let stats = profiler.get_stats().expect("Failed to get stats");
    assert_eq!(stats.total_allocations, types.len());

    profiler.stop().expect("Failed to stop profiler");
  }

  #[test]
  fn test_allocation_event_serialization() {
    let event = AllocationEvent {
      timestamp: std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64,
      address: 0x1000,
      size: 1024,
      allocation_type: AllocationType::Malloc,
      call_stack: vec!["malloc".to_string(), "heap.c:42".to_string()],
      thread_id: "thread-1".to_string(),
    };

    let json = serde_json::to_string(&event).expect("Failed to serialize event");
    let deserialized: AllocationEvent =
      serde_json::from_str(&json).expect("Failed to deserialize event");

    assert_eq!(event.address, deserialized.address);
    assert_eq!(event.size, deserialized.size);
    assert_eq!(event.allocation_type, deserialized.allocation_type);
    assert_eq!(event.call_stack.len(), deserialized.call_stack.len());
  }

  #[test]
  fn test_deallocation_event_serialization() {
    let event = DeallocationEvent {
      timestamp: std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64,
      address: 0x1000,
      thread_id: "thread-1".to_string(),
    };

    let json = serde_json::to_string(&event).expect("Failed to serialize event");
    let deserialized: DeallocationEvent =
      serde_json::from_str(&json).expect("Failed to deserialize event");

    assert_eq!(event.address, deserialized.address);
    assert_eq!(event.thread_id, deserialized.thread_id);
  }

  #[test]
  fn test_heap_profile_stats_serialization() {
    let stats = HeapProfileStats {
      total_allocations: 100,
      total_deallocations: 80,
      active_allocations: 20,
      total_bytes_allocated: 1024 * 100,
      total_bytes_deallocated: 1024 * 80,
      current_bytes_in_use: 1024 * 20,
      peak_memory_usage: 1024 * 150,
      memory_leaks: vec![],
      size_distribution: HashMap::new(),
      top_allocation_sites: HashMap::new(),
      duration_ms: 5000,
    };

    let json = serde_json::to_string(&stats).expect("Failed to serialize stats");
    let deserialized: HeapProfileStats =
      serde_json::from_str(&json).expect("Failed to deserialize stats");

    assert_eq!(stats.total_allocations, deserialized.total_allocations);
    assert_eq!(stats.memory_leaks.len(), deserialized.memory_leaks.len());
    assert_eq!(
      stats.total_bytes_allocated,
      deserialized.total_bytes_allocated
    );
  }

  #[test]
  fn test_profiler_trait_implementation() {
    let mut profiler: Box<dyn Profiler> = Box::new(create_test_profiler());

    // Test trait methods
    profiler.start().expect("Failed to start profiler");
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
  fn test_heap_profiler_realistic_scenario() {
    let mut profiler =
      HeapProfiler::new(ProfilerConfig::default()).expect("Failed to create profiler");

    profiler.start().expect("Failed to start profiler");

    // Simulate realistic allocation patterns
    let mut addresses = Vec::new();

    // Allocate some memory
    for i in 0..100 {
      let address = 0x10000 + (i * 0x1000);
      let size = 1024 + (i % 10) * 512;
      addresses.push(address);

      profiler
        .record_allocation(
          address,
          size as usize,
          if i % 3 == 0 {
            AllocationType::Malloc
          } else {
            AllocationType::New
          },
        )
        .expect("Failed to record allocation");
    }

    // Deallocate some memory (not all - simulate leaks)
    for i in (0..80).step_by(2) {
      profiler
        .record_deallocation(addresses[i])
        .expect("Failed to record deallocation");
    }

    let stats = profiler.get_stats().expect("Failed to get stats");
    assert_eq!(stats.total_allocations, 100);
    assert_eq!(stats.total_deallocations, 40);
    assert!(stats.memory_leaks.len() > 0);
    assert!(stats.current_bytes_in_use > 0);

    let leaks = profiler
      .detect_memory_leaks()
      .expect("Failed to detect leaks");
    assert!(leaks.len() > 0);

    profiler.stop().expect("Failed to stop profiler");
  }

  #[test]
  fn test_heap_profiler_concurrent_access() {
    let profiler = Arc::new(Mutex::new(
      HeapProfiler::new(ProfilerConfig::default()).expect("Failed to create profiler"),
    ));

    {
      let mut p = profiler.lock().unwrap();
      p.start().expect("Failed to start profiler");
    }

    let handles: Vec<_> = (0..4)
      .map(|thread_id| {
        let profiler_clone = profiler.clone();
        thread::spawn(move || {
          for i in 0..25 {
            let address = (thread_id as u64 * 0x100000) + (i * 0x1000);
            let size = 1024 + (i % 10) * 256;

            {
              let p = profiler_clone.lock().unwrap();
              p.record_allocation(address, size as usize, AllocationType::Malloc)
                .expect("Failed to record allocation");
            }

            thread::sleep(Duration::from_millis(1));

            if i % 2 == 0 {
              let p = profiler_clone.lock().unwrap();
              p.record_deallocation(address)
                .expect("Failed to record deallocation");
            }
          }
        })
      })
      .collect();

    // Wait for all threads to complete
    for handle in handles {
      handle.join().expect("Thread panicked");
    }

    {
      let mut p = profiler.lock().unwrap();
      p.stop().expect("Failed to stop profiler");

      let stats = p.get_stats().expect("Failed to get stats");
      assert_eq!(stats.total_allocations, 100); // 4 threads * 25 allocations
      assert!(stats.total_deallocations > 0);
      assert!(stats.memory_leaks.len() > 0);
    }
  }

  #[test]
  fn test_heap_profiler_stress_test() {
    let mut profiler = HeapProfiler::new(ProfilerConfig {
      sampling_interval: Duration::from_millis(1),
      collect_stacks: false, // Disable for performance
      max_stack_depth: 10,
      max_samples: 10000,
      output_path: None,
    })
    .expect("Failed to create profiler");

    profiler.start().expect("Failed to start profiler");

    // Stress test with many allocations/deallocations
    let mut active_addresses = Vec::new();

    for i in 0..1000 {
      let address = 0x100000 + (i * 0x1000);
      let size = 1024 + (i % 100) * 64;

      profiler
        .record_allocation(address, size as usize, AllocationType::Malloc)
        .expect("Failed to record allocation");
      active_addresses.push(address);

      // Randomly deallocate some addresses
      if i > 100 && i % 10 == 0 {
        let dealloc_index = ((i / 10) as usize) % active_addresses.len();
        let dealloc_address = active_addresses.remove(dealloc_index);
        profiler
          .record_deallocation(dealloc_address)
          .expect("Failed to record deallocation");
      }
    }

    let stats = profiler.get_stats().expect("Failed to get stats");
    assert_eq!(stats.total_allocations, 1000);
    assert!(stats.total_deallocations > 0);
    assert!(stats.current_bytes_in_use > 0);

    profiler.stop().expect("Failed to stop profiler");
  }

  #[test]
  fn test_heap_profiler_memory_patterns() {
    let mut profiler =
      HeapProfiler::new(ProfilerConfig::default()).expect("Failed to create profiler");

    profiler.start().expect("Failed to start profiler");

    // Test different allocation patterns

    // Pattern 1: Small frequent allocations
    for i in 0..50 {
      profiler
        .record_allocation(0x10000 + i * 0x100, 64, AllocationType::Malloc)
        .expect("Failed to record allocation");
    }

    // Pattern 2: Large infrequent allocations
    for i in 0..5 {
      profiler
        .record_allocation(
          0x100000 + i * 0x10000,
          1024 * 1024, // 1MB
          AllocationType::New,
        )
        .expect("Failed to record allocation");
    }

    // Pattern 3: Mixed size allocations
    for i in 0..20 {
      let size = if i % 3 == 0 {
        1024
      } else if i % 3 == 1 {
        4096
      } else {
        16384
      };
      profiler
        .record_allocation(0x200000 + i * 0x1000, size, AllocationType::Calloc)
        .expect("Failed to record allocation");
    }

    let stats = profiler.get_stats().expect("Failed to get stats");
    assert_eq!(stats.total_allocations, 75); // 50 + 5 + 20
    assert!(stats.total_bytes_allocated > 5 * 1024 * 1024); // At least 5MB

    profiler.stop().expect("Failed to stop profiler");
  }
}
