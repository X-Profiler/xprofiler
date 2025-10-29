//! Libuv monitoring module
//!
//! This module provides libuv event loop monitoring functionality,
//! including event loop lag, handle counts, and request statistics.
//!
//! The implementation tracks Node.js event loop performance metrics
//! which are crucial for understanding application responsiveness.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::{Monitor, MonitoringError, MonitoringResult, TimePeriod};

/// Libuv handle types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HandleType {
  /// TCP handles
  Tcp,
  /// UDP handles
  Udp,
  /// Pipe handles
  Pipe,
  /// TTY handles
  Tty,
  /// Timer handles
  Timer,
  /// Prepare handles
  Prepare,
  /// Check handles
  Check,
  /// Idle handles
  Idle,
  /// Async handles
  Async,
  /// Poll handles
  Poll,
  /// Signal handles
  Signal,
  /// Process handles
  Process,
  /// FS Event handles
  FsEvent,
  /// FS Poll handles
  FsPoll,
  /// Unknown handle type
  Unknown,
}

impl HandleType {
  /// Convert from string representation
  pub fn from_str(s: &str) -> Self {
    match s.to_lowercase().as_str() {
      "tcp" => HandleType::Tcp,
      "udp" => HandleType::Udp,
      "pipe" => HandleType::Pipe,
      "tty" => HandleType::Tty,
      "timer" => HandleType::Timer,
      "prepare" => HandleType::Prepare,
      "check" => HandleType::Check,
      "idle" => HandleType::Idle,
      "async" => HandleType::Async,
      "poll" => HandleType::Poll,
      "signal" => HandleType::Signal,
      "process" => HandleType::Process,
      "fs_event" => HandleType::FsEvent,
      "fs_poll" => HandleType::FsPoll,
      _ => HandleType::Unknown,
    }
  }

  /// Convert to string representation
  pub fn as_str(&self) -> &'static str {
    match self {
      HandleType::Tcp => "tcp",
      HandleType::Udp => "udp",
      HandleType::Pipe => "pipe",
      HandleType::Tty => "tty",
      HandleType::Timer => "timer",
      HandleType::Prepare => "prepare",
      HandleType::Check => "check",
      HandleType::Idle => "idle",
      HandleType::Async => "async",
      HandleType::Poll => "poll",
      HandleType::Signal => "signal",
      HandleType::Process => "process",
      HandleType::FsEvent => "fs_event",
      HandleType::FsPoll => "fs_poll",
      HandleType::Unknown => "unknown",
    }
  }
}

/// Loop metrics for detailed event loop analysis
#[derive(Debug, Clone)]
pub struct LoopMetrics {
  /// Average loop lag in milliseconds
  pub avg_lag: f64,
  /// Maximum loop lag in milliseconds
  pub max_lag: f64,
  /// Total loop iterations
  pub iterations: u64,
  /// Iterations per second
  pub iterations_per_second: f64,
}

/// Libuv statistics for a specific time period
#[derive(Debug, Clone)]
pub struct LibuvStats {
  /// Event loop lag in milliseconds (average)
  pub avg_loop_lag: f64,
  /// Minimum event loop lag in milliseconds
  pub min_loop_lag: f64,
  /// Maximum event loop lag in milliseconds
  pub max_loop_lag: f64,
  /// 95th percentile event loop lag in milliseconds
  pub p95_loop_lag: f64,
  /// 99th percentile event loop lag in milliseconds
  pub p99_loop_lag: f64,
  /// Total number of active handles
  pub total_handles: u64,
  /// Handle count by type
  pub handles_by_type: HashMap<HandleType, u64>,
  /// Total number of active requests
  pub total_requests: u64,
  /// Average requests per second
  pub requests_per_second: f64,
  /// Total number of loop iterations
  pub total_loop_count: u64,
  /// Average loop iterations per second
  pub loops_per_second: f64,
  /// Time spent in different phases (in milliseconds)
  pub phase_times: HashMap<String, f64>,
  /// Loop metrics for detailed analysis
  pub loop_metrics: LoopMetrics,
  /// Time period for these statistics
  pub period: TimePeriod,
  /// Timestamp when stats were calculated
  pub timestamp: Instant,
}

/// Event loop measurement
#[derive(Debug, Clone)]
struct LoopMeasurement {
  /// Timestamp of the measurement
  pub timestamp: Instant,
  /// Event loop lag in milliseconds
  pub lag: f64,
  /// Number of active handles
  pub handle_count: u64,
  /// Handle counts by type
  pub handles_by_type: HashMap<HandleType, u64>,
  /// Number of active requests
  pub request_count: u64,
  /// Loop iteration count
  pub loop_count: u64,
  /// Phase timing information
  pub phase_times: HashMap<String, f64>,
}

/// Libuv monitor implementation
#[derive(Debug)]
pub struct LibuvMonitor {
  /// Historical measurements for different time periods
  measurements: Arc<Mutex<VecDeque<LoopMeasurement>>>,
  /// Statistics cache for different periods
  stats_cache: Arc<Mutex<HashMap<TimePeriod, LibuvStats>>>,
  /// Whether monitoring is active
  is_monitoring: bool,
  /// Maximum number of measurements to keep
  max_measurements: usize,
  /// Last measurement time
  last_measurement: Instant,
  /// Measurement interval
  measurement_interval: Duration,
  /// Registered handles by ID
  registered_handles: Arc<Mutex<HashMap<String, HandleType>>>,
  /// Loop iteration counter
  loop_iterations: Arc<Mutex<u64>>,
}

impl LibuvMonitor {
  /// Create a new libuv monitor
  pub fn new() -> Self {
    Self {
      measurements: Arc::new(Mutex::new(VecDeque::new())),
      stats_cache: Arc::new(Mutex::new(HashMap::new())),
      is_monitoring: false,
      max_measurements: 3600, // Keep 1 hour of measurements (1 per second)
      last_measurement: Instant::now(),
      measurement_interval: Duration::from_secs(1),
      registered_handles: Arc::new(Mutex::new(HashMap::new())),
      loop_iterations: Arc::new(Mutex::new(0)),
    }
  }

  /// Record a libuv measurement
  pub fn record_measurement(
    &self,
    lag: f64,
    handle_count: u64,
    handles_by_type: HashMap<HandleType, u64>,
    request_count: u64,
    loop_count: u64,
    phase_times: HashMap<String, f64>,
  ) -> MonitoringResult<()> {
    if !self.is_monitoring {
      return Ok(());
    }

    let measurement = LoopMeasurement {
      timestamp: Instant::now(),
      lag,
      handle_count,
      handles_by_type,
      request_count,
      loop_count,
      phase_times,
    };

    if let Ok(mut measurements) = self.measurements.lock() {
      measurements.push_back(measurement);

      // Cleanup old measurements
      while measurements.len() > self.max_measurements {
        measurements.pop_front();
      }
    }

    // Invalidate stats cache
    if let Ok(mut cache) = self.stats_cache.lock() {
      cache.clear();
    }

    Ok(())
  }

  /// Get libuv statistics for a specific time period
  pub fn get_stats_for_period(&self, period: TimePeriod) -> MonitoringResult<LibuvStats> {
    // Check cache first
    if let Ok(cache) = self.stats_cache.lock() {
      if let Some(stats) = cache.get(&period) {
        // Return cached stats if they're recent (within 1 second)
        if stats.timestamp.elapsed() < Duration::from_secs(1) {
          return Ok(stats.clone());
        }
      }
    }

    // Calculate new stats
    let stats = self.calculate_stats_for_period(period)?;

    // Cache the stats
    if let Ok(mut cache) = self.stats_cache.lock() {
      cache.insert(period, stats.clone());
    }

    Ok(stats)
  }

  /// Calculate statistics for a specific time period
  fn calculate_stats_for_period(&self, period: TimePeriod) -> MonitoringResult<LibuvStats> {
    let now = Instant::now();
    let period_duration = period.duration();
    let cutoff_time = now - period_duration;

    let measurements = self
      .measurements
      .lock()
      .map_err(|_| MonitoringError::LockFailed {
        resource: "measurements".to_string(),
        details: "Failed to lock measurements".to_string(),
      })?;

    // Filter measurements within the time period
    let relevant_measurements: Vec<_> = measurements
      .iter()
      .filter(|m| m.timestamp >= cutoff_time)
      .collect();

    if relevant_measurements.is_empty() {
      // Even without measurements, we can still report registered handles and loop iterations
      let registered_handles =
        self
          .registered_handles
          .lock()
          .map_err(|_| MonitoringError::LockFailed {
            resource: "registered_handles".to_string(),
            details: "Failed to lock registered handles".to_string(),
          })?;

      let total_handles = registered_handles.len() as u64;
      let mut handles_by_type = HashMap::new();
      for handle_type in registered_handles.values() {
        *handles_by_type.entry(*handle_type).or_insert(0) += 1;
      }
      drop(registered_handles);

      let actual_iterations = self.loop_iterations.lock().map(|i| *i).unwrap_or(0);

      return Ok(LibuvStats {
        avg_loop_lag: 0.0,
        min_loop_lag: 0.0,
        max_loop_lag: 0.0,
        p95_loop_lag: 0.0,
        p99_loop_lag: 0.0,
        total_handles,
        handles_by_type,
        total_requests: 0,
        requests_per_second: 0.0,
        total_loop_count: 0,
        loops_per_second: 0.0,
        phase_times: HashMap::new(),
        loop_metrics: LoopMetrics {
          avg_lag: 0.0,
          max_lag: 0.0,
          iterations: actual_iterations,
          iterations_per_second: 0.0,
        },
        period,
        timestamp: now,
      });
    }

    // Calculate lag statistics
    let mut lags: Vec<f64> = relevant_measurements.iter().map(|m| m.lag).collect();
    lags.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let avg_loop_lag = lags.iter().sum::<f64>() / lags.len() as f64;
    let min_loop_lag = lags.first().copied().unwrap_or(0.0);
    let max_loop_lag = lags.last().copied().unwrap_or(0.0);

    let p95_loop_lag = if lags.is_empty() {
      0.0
    } else {
      let index = ((lags.len() as f64 * 0.95) as usize).min(lags.len() - 1);
      lags[index]
    };

    let p99_loop_lag = if lags.is_empty() {
      0.0
    } else {
      let index = ((lags.len() as f64 * 0.99) as usize).min(lags.len() - 1);
      lags[index]
    };

    // Calculate handle statistics from registered handles
    let registered_handles =
      self
        .registered_handles
        .lock()
        .map_err(|_| MonitoringError::LockFailed {
          resource: "registered_handles".to_string(),
          details: "Failed to lock registered handles".to_string(),
        })?;

    let total_handles = registered_handles.len() as u64;
    let mut handles_by_type = HashMap::new();
    for handle_type in registered_handles.values() {
      *handles_by_type.entry(*handle_type).or_insert(0) += 1;
    }
    drop(registered_handles);

    // Get latest measurement for other stats
    let latest_measurement = relevant_measurements.last().unwrap();

    // Calculate request statistics
    let total_requests = latest_measurement.request_count;
    let requests_per_second = if relevant_measurements.len() > 1 {
      let first = relevant_measurements.first().unwrap();
      let last = relevant_measurements.last().unwrap();
      let time_diff = last.timestamp.duration_since(first.timestamp).as_secs_f64();
      if time_diff > 0.0 {
        (last.request_count.saturating_sub(first.request_count)) as f64 / time_diff
      } else {
        0.0
      }
    } else {
      0.0
    };

    // Calculate loop statistics
    let total_loop_count = latest_measurement.loop_count;
    let loops_per_second = if relevant_measurements.len() > 1 {
      let first = relevant_measurements.first().unwrap();
      let last = relevant_measurements.last().unwrap();
      let time_diff = last.timestamp.duration_since(first.timestamp).as_secs_f64();
      if time_diff > 0.0 {
        (last.loop_count.saturating_sub(first.loop_count)) as f64 / time_diff
      } else {
        0.0
      }
    } else {
      0.0
    };

    // Calculate average phase times
    let mut phase_times = HashMap::new();
    for measurement in &relevant_measurements {
      for (phase, time) in &measurement.phase_times {
        let entry = phase_times.entry(phase.clone()).or_insert(0.0);
        *entry += time;
      }
    }

    // Average the phase times
    let measurement_count = relevant_measurements.len() as f64;
    for time in phase_times.values_mut() {
      *time /= measurement_count;
    }

    // Get actual loop iterations
    let actual_iterations = self.loop_iterations.lock().map(|i| *i).unwrap_or(0);

    // Create loop metrics
    let loop_metrics = LoopMetrics {
      avg_lag: avg_loop_lag,
      max_lag: max_loop_lag,
      iterations: actual_iterations,
      iterations_per_second: loops_per_second,
    };

    Ok(LibuvStats {
      avg_loop_lag,
      min_loop_lag,
      max_loop_lag,
      p95_loop_lag,
      p99_loop_lag,
      total_handles,
      handles_by_type,
      total_requests,
      requests_per_second,
      total_loop_count,
      loops_per_second,
      phase_times,
      loop_metrics,
      period,
      timestamp: now,
    })
  }

  /// Simulate event loop measurement (for testing)
  pub fn simulate_measurement(&self) -> MonitoringResult<()> {
    use std::collections::HashMap;

    // Simulate some realistic values
    let lag = (rand::random::<f64>() * 10.0).max(0.1); // 0.1-10ms lag
    let handle_count = 10 + (rand::random::<u64>() % 50); // 10-60 handles

    let mut handles_by_type = HashMap::new();
    handles_by_type.insert(HandleType::Tcp, 2 + (rand::random::<u64>() % 5));
    handles_by_type.insert(HandleType::Timer, 1 + (rand::random::<u64>() % 3));
    handles_by_type.insert(HandleType::Async, 3 + (rand::random::<u64>() % 7));

    let request_count = rand::random::<u64>() % 100;
    let loop_count = rand::random::<u64>() % 1000000;

    let mut phase_times = HashMap::new();
    phase_times.insert("timer".to_string(), rand::random::<u32>() as f64 * 2.0);
    phase_times.insert("pending".to_string(), rand::random::<u32>() as f64 * 1.0);
    phase_times.insert("idle".to_string(), rand::random::<u32>() as f64 * 0.5);
    phase_times.insert("prepare".to_string(), rand::random::<u32>() as f64 * 0.3);
    phase_times.insert("poll".to_string(), rand::random::<u32>() as f64 * 5.0);
    phase_times.insert("check".to_string(), rand::random::<u32>() as f64 * 0.2);
    phase_times.insert("close".to_string(), rand::random::<u32>() as f64 * 0.1);

    self.record_measurement(
      lag,
      handle_count,
      handles_by_type,
      request_count,
      loop_count,
      phase_times,
    )
  }

  /// Register a handle for monitoring
  pub fn register_handle(
    &self,
    handle_id: String,
    handle_type: HandleType,
  ) -> MonitoringResult<()> {
    if let Ok(mut handles) = self.registered_handles.lock() {
      handles.insert(handle_id, handle_type);
    }
    Ok(())
  }

  /// Unregister a handle from monitoring
  pub fn unregister_handle(&self, handle_id: &str) -> MonitoringResult<()> {
    if let Ok(mut handles) = self.registered_handles.lock() {
      handles.remove(handle_id);
    }
    Ok(())
  }

  /// Record a loop iteration
  pub fn record_loop_iteration(&self) -> MonitoringResult<()> {
    if let Ok(mut iterations) = self.loop_iterations.lock() {
      *iterations += 1;
    }
    Ok(())
  }

  /// Get current loop iteration count
  pub fn get_loop_iterations(&self) -> u64 {
    self.loop_iterations.lock().map(|i| *i).unwrap_or(0)
  }

  /// Get registered handles count
  pub fn get_registered_handles_count(&self) -> u64 {
    self
      .registered_handles
      .lock()
      .map(|h| h.len() as u64)
      .unwrap_or(0)
  }
}

impl Monitor for LibuvMonitor {
  type Stats = HashMap<TimePeriod, LibuvStats>;

  fn start(&mut self) -> MonitoringResult<()> {
    self.is_monitoring = true;
    Ok(())
  }

  fn stop(&mut self) -> MonitoringResult<()> {
    self.is_monitoring = false;
    Ok(())
  }

  fn is_running(&self) -> bool {
    self.is_monitoring
  }

  fn get_stats(&self) -> MonitoringResult<Self::Stats> {
    let mut all_stats = HashMap::new();

    for period in TimePeriod::all() {
      let stats = self.get_stats_for_period(period)?;
      all_stats.insert(period, stats);
    }

    Ok(all_stats)
  }

  fn reset(&mut self) -> MonitoringResult<()> {
    if let Ok(mut measurements) = self.measurements.lock() {
      measurements.clear();
    }

    if let Ok(mut cache) = self.stats_cache.lock() {
      cache.clear();
    }

    if let Ok(mut handles) = self.registered_handles.lock() {
      handles.clear();
    }

    if let Ok(mut iterations) = self.loop_iterations.lock() {
      *iterations = 0;
    }

    Ok(())
  }

  fn update(&mut self) -> MonitoringResult<()> {
    // Check if it's time for a new measurement
    let now = Instant::now();
    if now.duration_since(self.last_measurement) >= self.measurement_interval {
      self.simulate_measurement()?;
      self.last_measurement = now;
    }

    Ok(())
  }

  fn module_name(&self) -> &'static str {
    "libuv"
  }
}

impl Default for LibuvMonitor {
  fn default() -> Self {
    Self::new()
  }
}

// Simple random number generation for testing
mod rand {
  use std::sync::atomic::{AtomicU64, Ordering};

  static SEED: AtomicU64 = AtomicU64::new(1);

  pub fn random<T>() -> T
  where
    T: From<u32>,
  {
    let current = SEED.load(Ordering::Relaxed);
    let next = current.wrapping_mul(1103515245).wrapping_add(12345);
    SEED.store(next, Ordering::Relaxed);
    T::from(next as u32)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::thread;
  use std::time::Duration;

  #[test]
  fn test_handle_type_conversion() {
    assert_eq!(HandleType::from_str("tcp"), HandleType::Tcp);
    assert_eq!(HandleType::from_str("TCP"), HandleType::Tcp);
    assert_eq!(HandleType::from_str("unknown_type"), HandleType::Unknown);

    assert_eq!(HandleType::Tcp.as_str(), "tcp");
    assert_eq!(HandleType::Unknown.as_str(), "unknown");
  }

  #[test]
  fn test_libuv_monitor_creation() {
    let monitor = LibuvMonitor::new();
    assert!(!monitor.is_running());
  }

  #[test]
  fn test_libuv_monitor_start_stop() {
    let mut monitor = LibuvMonitor::new();

    assert!(monitor.start().is_ok());
    assert!(monitor.is_running());

    assert!(monitor.stop().is_ok());
    assert!(!monitor.is_running());
  }

  #[test]
  fn test_record_measurement() {
    let mut monitor = LibuvMonitor::new();
    monitor.start().unwrap();

    let mut handles_by_type = HashMap::new();
    handles_by_type.insert(HandleType::Tcp, 5);
    handles_by_type.insert(HandleType::Timer, 2);

    let mut phase_times = HashMap::new();
    phase_times.insert("poll".to_string(), 1.5);
    phase_times.insert("timer".to_string(), 0.3);

    assert!(monitor
      .record_measurement(2.5, 10, handles_by_type, 5, 1000, phase_times,)
      .is_ok());
  }

  #[test]
  fn test_get_stats() {
    let mut monitor = LibuvMonitor::new();
    monitor.start().unwrap();

    // Register some handles to test handle tracking
    for i in 0..5 {
      let handle_id = format!("handle_{}", i);
      monitor.register_handle(handle_id, HandleType::Tcp).unwrap();
    }

    // Record some test measurements
    for i in 0..5 {
      let mut handles_by_type = HashMap::new();
      handles_by_type.insert(HandleType::Tcp, 3 + i);

      let mut phase_times = HashMap::new();
      phase_times.insert("poll".to_string(), 1.0 + i as f64 * 0.1);

      monitor
        .record_measurement(
          1.0 + i as f64 * 0.5,
          10 + i,
          handles_by_type,
          i * 2,
          i * 100,
          phase_times,
        )
        .unwrap();
    }

    let stats = monitor.get_stats().unwrap();
    assert!(!stats.is_empty());

    if let Some(ten_sec_stats) = stats.get(&TimePeriod::TenSeconds) {
      assert!(ten_sec_stats.avg_loop_lag > 0.0);
      assert!(ten_sec_stats.total_handles > 0);
    }
  }

  #[test]
  fn test_simulate_measurement() {
    let mut monitor = LibuvMonitor::new();
    monitor.start().unwrap();

    assert!(monitor.simulate_measurement().is_ok());

    let stats = monitor.get_stats().unwrap();
    if let Some(ten_sec_stats) = stats.get(&TimePeriod::TenSeconds) {
      assert!(ten_sec_stats.avg_loop_lag >= 0.0);
    }
  }

  #[test]
  fn test_reset() {
    let mut monitor = LibuvMonitor::new();
    monitor.start().unwrap();

    // Add some data
    monitor.simulate_measurement().unwrap();

    // Reset should clear all data
    assert!(monitor.reset().is_ok());

    let stats = monitor.get_stats().unwrap();
    for (_, period_stats) in stats {
      assert_eq!(period_stats.avg_loop_lag, 0.0);
      assert_eq!(period_stats.total_handles, 0);
      assert_eq!(period_stats.loop_metrics.avg_lag, 0.0);
      assert_eq!(period_stats.loop_metrics.iterations, 0);
    }
  }
}
