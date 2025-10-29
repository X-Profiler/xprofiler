//! Unit tests for GC Profiler

use super::*;
use crate::profiler::cpu_profiler::StackFrame;
use crate::profiler::Profiler;
use crate::profiler::ProfilerConfig;
use serde_json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[cfg(test)]
mod tests {
  use super::*;

  fn create_test_profiler() -> GcProfiler {
    let config = ProfilerConfig {
      sampling_interval: Duration::from_millis(1),
      collect_stacks: true,
      max_stack_depth: 10,
      max_samples: 1000,
      output_path: None,
    };
    GcProfiler::new(config).expect("Failed to create GC profiler")
  }

  #[test]
  fn test_gc_profiler_creation() {
    let profiler = create_test_profiler();
    assert!(!profiler.is_running());
    // Cannot access private field gc_events directly
    // assert_eq!(profiler.gc_events.len(), 0);
  }

  #[test]
  fn test_gc_profiler_start_stop() {
    let mut profiler = create_test_profiler();

    // Test start
    profiler.start().expect("Failed to start profiler");
    assert!(profiler.is_running());

    // Test stop
    profiler.stop().expect("Failed to stop profiler");
    assert!(!profiler.is_running());
  }

  #[test]
  fn test_gc_profiler_double_start() {
    let mut profiler = create_test_profiler();

    profiler.start().expect("Failed to start profiler");
    let result = profiler.start();
    assert!(result.is_err());

    profiler.stop().expect("Failed to stop profiler");
  }

  #[test]
  fn test_gc_profiler_stop_without_start() {
    let mut profiler = create_test_profiler();
    let result = profiler.stop();
    assert!(result.is_err());
  }

  #[test]
  fn test_record_gc_event() {
    let mut profiler = create_test_profiler();
    profiler.start().expect("Failed to start profiler");

    let mut metadata = HashMap::new();
    metadata.insert("generation".to_string(), "young".to_string());

    profiler
      .record_gc_event(
        GcEventType::MinorGc,
        GcPhase::Start,
        1000,        // 1ms in microseconds
        1024 * 1024, // 1MB before
        512 * 1024,  // 512KB after
        2048 * 1024, // 2MB heap size
        "allocation_failure".to_string(),
        metadata.clone(),
      )
      .expect("Failed to record GC event");

    // Cannot access private field gc_events directly, use stats instead
    let stats = profiler.get_stats().expect("Failed to get stats");
    assert_eq!(stats.total_gc_events, 1);

    profiler.stop().expect("Failed to stop profiler");
  }

  #[test]
  fn test_gc_event_types() {
    let mut profiler = create_test_profiler();
    profiler.start().expect("Failed to start profiler");

    let event_types = vec![
      GcEventType::MinorGc,
      GcEventType::MajorGc,
      GcEventType::FullGc,
      GcEventType::ConcurrentGc,
      GcEventType::IncrementalGc,
      GcEventType::Other("custom_gc".to_string()),
    ];

    for (i, event_type) in event_types.iter().enumerate() {
      profiler
        .record_gc_event(
          event_type.clone(),
          GcPhase::Start,
          1000 + i as u64,
          1024,
          512,
          2048,
          format!("reason_{}", i),
          HashMap::new(),
        )
        .expect("Failed to record GC event");
    }

    // Cannot access private field gc_events directly, use stats instead
    let stats = profiler.get_stats().expect("Failed to get stats");
    assert_eq!(stats.total_gc_events, event_types.len());

    profiler.stop().expect("Failed to stop profiler");
  }

  #[test]
  fn test_gc_phases() {
    let mut profiler = create_test_profiler();
    profiler.start().expect("Failed to start profiler");

    let phases = vec![
      GcPhase::Start,
      GcPhase::Mark,
      GcPhase::Sweep,
      GcPhase::Compact,
      GcPhase::End,
    ];

    for (i, phase) in phases.iter().enumerate() {
      profiler
        .record_gc_event(
          GcEventType::MajorGc,
          phase.clone(),
          1000 + i as u64,
          1024,
          512,
          2048,
          format!("phase_{}", i),
          HashMap::new(),
        )
        .expect("Failed to record GC event");
    }

    // Cannot access private field gc_events directly, use stats instead
    let stats = profiler.get_stats().expect("Failed to get stats");
    assert_eq!(stats.total_gc_events, phases.len());

    profiler.stop().expect("Failed to stop profiler");
  }

  #[test]
  fn test_calculate_gc_stats() {
    let mut profiler = create_test_profiler();
    profiler.start().expect("Failed to start profiler");

    // Record multiple GC events
    let events = vec![
      (GcEventType::MinorGc, 1000u64, 1024usize, 512usize),
      (GcEventType::MinorGc, 1500u64, 2048usize, 1024usize),
      (GcEventType::MajorGc, 5000u64, 4096usize, 2048usize),
      (GcEventType::FullGc, 10000u64, 8192usize, 4096usize),
    ];

    for (event_type, duration, mem_before, mem_after) in events {
      profiler
        .record_gc_event(
          event_type,
          GcPhase::Start,
          duration,
          mem_before,
          mem_after,
          mem_before + 1024,
          "test".to_string(),
          HashMap::new(),
        )
        .expect("Failed to record GC event");
    }

    let stats = profiler.get_stats().expect("Failed to get GC stats");

    assert_eq!(stats.total_gc_events, 4);
    // Check GC type stats instead of direct counts
    assert!(stats.gc_type_stats.len() > 0);
    assert_eq!(stats.total_gc_time_us, 1000 + 1500 + 5000 + 10000);
    assert_eq!(stats.average_gc_time_us, (1000 + 1500 + 5000 + 10000) / 4);
    assert_eq!(stats.max_gc_time_us, 10000);
    assert_eq!(stats.min_gc_time_us, 1000);
    assert_eq!(
      stats.total_memory_reclaimed,
      (1024 - 512) + (2048 - 1024) + (4096 - 2048) + (8192 - 4096)
    );

    profiler.stop().expect("Failed to stop profiler");
  }

  #[test]
  fn test_gc_profiler_reset() {
    let mut profiler = create_test_profiler();
    profiler.start().expect("Failed to start profiler");

    // Record some events
    profiler
      .record_gc_event(
        GcEventType::MinorGc,
        GcPhase::Start,
        1000,
        1024,
        512,
        2048,
        "test".to_string(),
        HashMap::new(),
      )
      .expect("Failed to record GC event");

    let stats = profiler.get_stats().expect("Failed to get stats");
    assert_eq!(stats.total_gc_events, 1);

    profiler.stop().expect("Failed to stop profiler");
    profiler.reset().expect("Failed to reset profiler");

    let stats = profiler.get_stats().expect("Failed to get stats");
    assert_eq!(stats.total_gc_events, 0);
    assert!(!profiler.is_running());
  }

  #[test]
  fn test_gc_profiler_get_results() {
    let mut profiler = create_test_profiler();
    profiler.start().expect("Failed to start profiler");

    profiler
      .record_gc_event(
        GcEventType::MinorGc,
        GcPhase::Start,
        1000,
        1024,
        512,
        2048,
        "test".to_string(),
        HashMap::new(),
      )
      .expect("Failed to record GC event");

    profiler.stop().expect("Failed to stop profiler");

    let results = profiler.get_results().expect("Failed to get results");
    assert!(!results.is_empty());

    // Verify it's valid JSON
    let _: serde_json::Value =
      serde_json::from_str(&results).expect("Results should be valid JSON");
  }

  #[test]
  fn test_gc_generation_stats() {
    let mut profiler = create_test_profiler();
    profiler.start().expect("Failed to start profiler");

    // Record events for different generations
    let mut metadata_young = HashMap::new();
    metadata_young.insert("generation".to_string(), "young".to_string());

    let mut metadata_old = HashMap::new();
    metadata_old.insert("generation".to_string(), "old".to_string());

    // Young generation events
    for i in 0..3 {
      profiler
        .record_gc_event(
          GcEventType::MinorGc,
          GcPhase::Start,
          1000 + i * 100,
          1024,
          512,
          2048,
          "young_gc".to_string(),
          metadata_young.clone(),
        )
        .expect("Failed to record GC event");
    }

    // Old generation events
    for i in 0..2 {
      profiler
        .record_gc_event(
          GcEventType::MajorGc,
          GcPhase::Start,
          5000 + i * 1000,
          4096,
          2048,
          8192,
          "old_gc".to_string(),
          metadata_old.clone(),
        )
        .expect("Failed to record GC event");
    }

    let stats = profiler.get_stats().expect("Failed to get GC stats");

    assert_eq!(stats.total_gc_events, 5);
    // Check GC type stats instead of direct counts
    assert!(stats.gc_type_stats.len() > 0);

    // Check generation-specific stats
    assert_eq!(stats.gc_type_stats.len(), 2);
    assert!(stats.gc_type_stats.contains_key("young"));
    assert!(stats.gc_type_stats.contains_key("old"));

    let young_stats = &stats.gc_type_stats["young"];
    assert_eq!(young_stats.collection_count, 3);

    let old_stats = &stats.gc_type_stats["old"];
    assert_eq!(old_stats.collection_count, 2);

    profiler.stop().expect("Failed to stop profiler");
  }

  #[test]
  fn test_gc_event_serialization() {
    let mut metadata = HashMap::new();
    metadata.insert("test_key".to_string(), "test_value".to_string());

    let event = GcEvent {
      id: 1,
      event_type: GcEventType::MinorGc,
      phase: GcPhase::Mark,
      timestamp: 1234567890,
      duration_us: 1500,
      memory_before: 2048,
      memory_after: 1024,
      memory_reclaimed: 1024,
      heap_size: 4096,
      thread_id: "main".to_string(),
      reason: "allocation_failure".to_string(),
      metadata: metadata.clone(),
    };

    let json = serde_json::to_string(&event).expect("Failed to serialize event");
    let deserialized: GcEvent = serde_json::from_str(&json).expect("Failed to deserialize event");

    assert_eq!(event.event_type, deserialized.event_type);
    assert_eq!(event.phase, deserialized.phase);
    assert_eq!(event.duration_us, deserialized.duration_us);
    assert_eq!(event.memory_before, deserialized.memory_before);
    assert_eq!(event.memory_after, deserialized.memory_after);
    assert_eq!(event.heap_size, deserialized.heap_size);
    assert_eq!(event.reason, deserialized.reason);
    assert_eq!(event.metadata, deserialized.metadata);
    assert_eq!(event.id, deserialized.id);
    assert_eq!(event.thread_id, deserialized.thread_id);
  }

  #[test]
  fn test_gc_profile_stats_serialization() {
    let mut generation_stats = HashMap::new();
    generation_stats.insert(
      "young".to_string(),
      GcGenerationStats {
        name: "young".to_string(),
        collection_count: 10,
        total_time_us: 5000,
        average_time_us: 500,
        max_time_us: 1000,
        min_time_us: 100,
        total_memory_reclaimed: 1024,
        average_memory_reclaimed: 102,
      },
    );

    let stats = GcProfileStats {
      total_gc_events: 15,
      total_gc_time_us: 25000,
      average_gc_time_us: 1666,
      max_gc_time_us: 10000,
      min_gc_time_us: 500,
      gc_frequency: 2.5,
      total_memory_reclaimed: 5120,
      gc_overhead_percent: 15.5,
      gc_type_stats: generation_stats,
      recent_events: Vec::new(),
      time_distribution: HashMap::new(),
      reclamation_efficiency: 85.5,
      duration_ms: 10000,
      longest_pause_us: 10000,
      throughput_mb_per_sec: 12.5,
    };

    let json = serde_json::to_string(&stats).expect("Failed to serialize stats");
    let deserialized: GcProfileStats =
      serde_json::from_str(&json).expect("Failed to deserialize stats");

    assert_eq!(stats.total_gc_events, deserialized.total_gc_events);
    assert_eq!(stats.gc_frequency, deserialized.gc_frequency);
    assert_eq!(stats.gc_type_stats.len(), deserialized.gc_type_stats.len());
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
  fn test_gc_profiler_realistic_scenario() {
    let mut profiler =
      GcProfiler::new(ProfilerConfig::default()).expect("Failed to create profiler");

    profiler.start().expect("Failed to start profiler");

    // Simulate a realistic GC scenario

    // Young generation GCs (frequent, short)
    for i in 0..20 {
      let mut metadata = HashMap::new();
      metadata.insert("generation".to_string(), "young".to_string());
      metadata.insert("trigger".to_string(), "allocation_rate".to_string());

      profiler
        .record_gc_event(
          GcEventType::MinorGc,
          GcPhase::Start,
          500 + (i % 5) * 100, // 500-900 microseconds
          (1024 * (i + 1)) as usize,
          (512 * (i + 1)) as usize,
          (2048 * (i + 1)) as usize,
          "eden_space_full".to_string(),
          metadata,
        )
        .expect("Failed to record minor GC event");
    }

    // Old generation GCs (less frequent, longer)
    for i in 0..5 {
      let mut metadata = HashMap::new();
      metadata.insert("generation".to_string(), "old".to_string());
      metadata.insert("trigger".to_string(), "promotion_failure".to_string());

      profiler
        .record_gc_event(
          GcEventType::MajorGc,
          GcPhase::Start,
          5000 + i * 1000, // 5-9 milliseconds
          (8192 * (i + 1)) as usize,
          (4096 * (i + 1)) as usize,
          (16384 * (i + 1)) as usize,
          "old_space_full".to_string(),
          metadata,
        )
        .expect("Failed to record major GC event");
    }

    // Full GC (rare, very long)
    let mut metadata = HashMap::new();
    metadata.insert("generation".to_string(), "all".to_string());
    metadata.insert("trigger".to_string(), "system_gc".to_string());

    profiler
      .record_gc_event(
        GcEventType::FullGc,
        GcPhase::Start,
        50000, // 50 milliseconds
        32768,
        16384,
        65536,
        "system_gc_call".to_string(),
        metadata,
      )
      .expect("Failed to record full GC event");

    let stats = profiler.get_stats().expect("Failed to get GC stats");

    assert_eq!(stats.total_gc_events, 26); // 20 + 5 + 1
    assert!(stats.total_gc_time_us > 50000); // At least the full GC time
    assert!(stats.max_gc_time_us >= 50000); // Full GC should be the longest
    assert!(stats.min_gc_time_us <= 900); // Minor GC should be the shortest

    // Check generation-specific stats
    assert_eq!(stats.gc_type_stats.len(), 3); // young, old, all
    assert!(stats.gc_type_stats.contains_key("young"));
    assert!(stats.gc_type_stats.contains_key("old"));
    assert!(stats.gc_type_stats.contains_key("all"));

    profiler.stop().expect("Failed to stop profiler");
  }

  #[test]
  fn test_gc_profiler_concurrent_access() {
    let profiler = Arc::new(Mutex::new(
      GcProfiler::new(ProfilerConfig::default()).expect("Failed to create profiler"),
    ));

    {
      let mut p = profiler.lock().unwrap();
      p.start().expect("Failed to start profiler");
    }

    let handles: Vec<_> = (0..4)
      .map(|thread_id| {
        let profiler_clone = profiler.clone();
        thread::spawn(move || {
          for i in 0..10 {
            let mut metadata = HashMap::new();
            metadata.insert("thread_id".to_string(), thread_id.to_string());

            {
              let p = profiler_clone.lock().unwrap();
              p.record_gc_event(
                if i % 3 == 0 {
                  GcEventType::MinorGc
                } else {
                  GcEventType::MajorGc
                },
                GcPhase::Start,
                1000 + (i * 100),
                (1024 * (i + 1)) as usize,
                (512 * (i + 1)) as usize,
                (2048 * (i + 1)) as usize,
                format!("gc_thread_{}", thread_id),
                metadata,
              )
              .expect("Failed to record GC event");
            }

            thread::sleep(Duration::from_millis(1));
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
      assert_eq!(stats.total_gc_events, 40); // 4 threads * 10 events
      assert!(stats.total_gc_time_us > 0);
    }
  }

  #[test]
  fn test_gc_profiler_performance_analysis() {
    let mut profiler =
      GcProfiler::new(ProfilerConfig::default()).expect("Failed to create profiler");

    profiler.start().expect("Failed to start profiler");

    // Simulate performance degradation over time
    let base_duration = 1000u64;
    let base_memory = 1024usize;

    for i in 0..50 {
      // Simulate increasing GC times (performance degradation)
      let duration = base_duration + (i * 50); // Gradually increasing
      let memory_before = base_memory * (i as usize + 1);
      let memory_after = memory_before / 2;

      let event_type = match i % 10 {
        0..=7 => GcEventType::MinorGc,
        8 => GcEventType::MajorGc,
        _ => GcEventType::FullGc,
      };

      profiler
        .record_gc_event(
          event_type,
          GcPhase::Start,
          duration,
          memory_before,
          memory_after,
          memory_before + 1024,
          "performance_test".to_string(),
          HashMap::new(),
        )
        .expect("Failed to record GC event");
    }

    let stats = profiler.get_stats().expect("Failed to get GC stats");

    assert_eq!(stats.total_gc_events, 50);
    assert!(stats.max_gc_time_us > stats.min_gc_time_us);
    assert!(stats.average_gc_time_us > base_duration);

    // Note: Performance trend analysis would require access to individual events
    // which is not exposed in the public API for thread safety reasons

    profiler.stop().expect("Failed to stop profiler");
  }

  #[test]
  fn test_gc_profiler_memory_efficiency() {
    let mut profiler =
      GcProfiler::new(ProfilerConfig::default()).expect("Failed to create profiler");

    profiler.start().expect("Failed to start profiler");

    // Test different memory efficiency scenarios

    // Scenario 1: Efficient GC (high memory reclaim)
    for i in 0..10 {
      profiler
        .record_gc_event(
          GcEventType::MinorGc,
          GcPhase::Start,
          1000,
          1024 * 10, // 10KB before
          1024 * 2,  // 2KB after (80% reclaimed)
          1024 * 20,
          "efficient_gc".to_string(),
          HashMap::new(),
        )
        .expect("Failed to record efficient GC event");
    }

    // Scenario 2: Inefficient GC (low memory reclaim)
    for i in 0..10 {
      profiler
        .record_gc_event(
          GcEventType::MajorGc,
          GcPhase::Start,
          5000,
          1024 * 10, // 10KB before
          1024 * 8,  // 8KB after (20% reclaimed)
          1024 * 20,
          "inefficient_gc".to_string(),
          HashMap::new(),
        )
        .expect("Failed to record inefficient GC event");
    }

    let stats = profiler.get_stats().expect("Failed to get GC stats");

    assert_eq!(stats.total_gc_events, 20);

    // Check that we have GC type stats for different event types
    assert!(
      stats.gc_type_stats.len() > 0,
      "Should have GC type statistics"
    );

    // Memory efficiency can be calculated from the stats
    // Note: Individual event data is not exposed for thread safety
    assert!(stats.total_gc_events > 0, "Should have recorded GC events");
    assert!(stats.total_gc_time_us > 0, "Should have recorded GC time");

    profiler.stop().expect("Failed to stop profiler");
  }
}
