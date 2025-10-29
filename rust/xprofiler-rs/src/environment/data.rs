//! Environment data structures and implementations

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Thread-specific environment data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentData {
  pub thread_id: u32,
  pub is_main_thread: bool,
  pub node_version: String,
  pub created_at: u64,
  pub last_updated: u64,
  pub thread_name: Option<String>,
  pub stack_size: Option<usize>,
  pub heap_statistics: HeapStatistics,
  pub gc_statistics: GcStatistics,
  pub uv_statistics: UvStatistics,
}

/// Heap memory statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HeapStatistics {
  pub total_heap_size: u64,
  pub total_heap_size_executable: u64,
  pub total_physical_size: u64,
  pub total_available_size: u64,
  pub used_heap_size: u64,
  pub heap_size_limit: u64,
  pub malloced_memory: u64,
  pub peak_malloced_memory: u64,
  pub does_zap_garbage: bool,
  pub number_of_native_contexts: u32,
  pub number_of_detached_contexts: u32,
}

/// Garbage collection statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GcStatistics {
  pub gc_count_full: u32,
  pub gc_count_incremental: u32,
  pub gc_time_full: u64,
  pub gc_time_incremental: u64,
  pub gc_time_total: u64,
  pub last_gc_type: String,
  pub last_gc_reason: String,
  pub last_gc_duration: u64,
}

/// libuv event loop statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UvStatistics {
  pub active_handles: u32,
  pub active_requests: u32,
  pub loop_count: u64,
  pub events: u64,
  pub loop_idle_time: u64,
  pub loop_prepare_time: u64,
  pub loop_check_time: u64,
  pub loop_poll_time: u64,
}

impl EnvironmentData {
  /// Create new environment data for a thread
  pub fn new(
    thread_id: u32,
    is_main_thread: bool,
    node_version: String,
  ) -> Result<Self, Box<dyn std::error::Error>> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;

    Ok(Self {
      thread_id,
      is_main_thread,
      node_version,
      created_at: now,
      last_updated: now,
      thread_name: std::thread::current().name().map(|s| s.to_string()),
      stack_size: None, // Would be obtained from platform-specific APIs
      heap_statistics: HeapStatistics::default(),
      gc_statistics: GcStatistics::default(),
      uv_statistics: UvStatistics::default(),
    })
  }

  /// Update the last updated timestamp
  pub fn touch(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    self.last_updated = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;
    Ok(())
  }

  /// Update heap statistics
  pub fn update_heap_statistics(&mut self, stats: HeapStatistics) {
    self.heap_statistics = stats;
    let _ = self.touch();
  }

  /// Update GC statistics
  pub fn update_gc_statistics(&mut self, stats: GcStatistics) {
    self.gc_statistics = stats;
    let _ = self.touch();
  }

  /// Update UV statistics
  pub fn update_uv_statistics(&mut self, stats: UvStatistics) {
    self.uv_statistics = stats;
    let _ = self.touch();
  }

  /// Get the age of this environment data in milliseconds
  pub fn age_ms(&self) -> Result<u64, Box<dyn std::error::Error>> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;
    Ok(now - self.created_at)
  }

  /// Get the time since last update in milliseconds
  pub fn time_since_update_ms(&self) -> Result<u64, Box<dyn std::error::Error>> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;
    Ok(now - self.last_updated)
  }

  /// Check if this environment data is stale (not updated recently)
  pub fn is_stale(&self, threshold_ms: u64) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(self.time_since_update_ms()? > threshold_ms)
  }

  /// Get a summary of the environment data
  pub fn summary(&self) -> String {
    format!(
      "Thread {} ({}): heap={:.2}MB, gc_count={}, uv_handles={}",
      self.thread_id,
      if self.is_main_thread {
        "main"
      } else {
        "worker"
      },
      self.heap_statistics.used_heap_size as f64 / 1024.0 / 1024.0,
      self.gc_statistics.gc_count_full + self.gc_statistics.gc_count_incremental,
      self.uv_statistics.active_handles
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_environment_data_creation() {
    let env_data = EnvironmentData::new(1, true, "v18.0.0".to_string()).unwrap();
    assert_eq!(env_data.thread_id, 1);
    assert!(env_data.is_main_thread);
    assert_eq!(env_data.node_version, "v18.0.0");
    assert!(env_data.created_at > 0);
  }

  #[test]
  fn test_environment_data_touch() {
    let mut env_data = EnvironmentData::new(1, true, "v18.0.0".to_string()).unwrap();
    let initial_time = env_data.last_updated;

    std::thread::sleep(std::time::Duration::from_millis(1));
    env_data.touch().unwrap();

    assert!(env_data.last_updated > initial_time);
  }

  #[test]
  fn test_environment_data_age() {
    let env_data = EnvironmentData::new(1, true, "v18.0.0".to_string()).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(1));

    let age = env_data.age_ms().unwrap();
    assert!(age > 0);
  }

  #[test]
  fn test_environment_data_summary() {
    let env_data = EnvironmentData::new(1, true, "v18.0.0".to_string()).unwrap();
    let summary = env_data.summary();
    assert!(summary.contains("Thread 1"));
    assert!(summary.contains("main"));
  }
}
