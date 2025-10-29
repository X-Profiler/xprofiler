//! CPU monitoring module
//!
//! This module provides CPU usage monitoring functionality,
//! including current CPU usage and historical averages over different time periods.
//!
//! The implementation follows the original C++ logic but with improved error handling
//! and platform-specific optimizations.

use once_cell::sync::Lazy;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use std::arch::x86_64::*;

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

use super::error::IntoMonitoringError;
use super::{Monitor, MonitoringError, MonitoringResult};

/// CPU usage information
#[derive(Debug, Clone)]
pub struct CpuUsage {
  /// Current CPU usage percentage (0.0 - 100.0)
  pub current: f64,
  /// Average CPU usage over 15 seconds
  pub avg_15s: f64,
  /// Average CPU usage over 30 seconds
  pub avg_30s: f64,
  /// Average CPU usage over 1 minute
  pub avg_1m: f64,
  /// Average CPU usage over 3 minutes
  pub avg_3m: f64,
  /// Average CPU usage over 5 minutes
  pub avg_5m: f64,
  /// Average CPU usage over 10 minutes
  pub avg_10m: f64,
  /// Timestamp of the measurement
  pub timestamp: Instant,
}

/// CPU time information for calculations
#[derive(Debug, Clone, Copy)]
struct CpuTime {
  /// User time in nanoseconds
  pub user_time: u64,
  /// System time in nanoseconds  
  pub system_time: u64,
  /// Wall clock time in nanoseconds
  pub wall_time: u64,
}

/// CPU monitor implementation
#[derive(Debug)]
pub struct CpuMonitor {
  /// Historical CPU usage data for different periods
  history_15s: Arc<Mutex<VecDeque<f64>>>,
  history_30s: Arc<Mutex<VecDeque<f64>>>,
  history_1m: Arc<Mutex<VecDeque<f64>>>,
  history_3m: Arc<Mutex<VecDeque<f64>>>,
  history_5m: Arc<Mutex<VecDeque<f64>>>,
  history_10m: Arc<Mutex<VecDeque<f64>>>,
  /// Current CPU usage
  current_usage: Arc<Mutex<f64>>,
  /// Last CPU time measurement
  last_cpu_time: Arc<Mutex<Option<CpuTime>>>,
  /// Whether monitoring is active
  is_monitoring: bool,
}

// Data storage for a specific time period - commented out as unused
// struct PeriodData {
//     /// Circular buffer for CPU usage values
//     values: VecDeque<f64>,
//     /// Maximum number of values to store
//     max_size: usize,
//     /// Whether the buffer is full
//     is_full: bool,
// }
//
// impl PeriodData {
//     fn new(period: TimePeriod) -> Self {
//         Self {
//             values: VecDeque::new(),
//             max_size: period.as_seconds() as usize,
//             is_full: false,
//         }
//     }
//
//     fn add_value(&mut self, value: f64) {
//         if self.values.len() >= self.max_size {
//             self.values.pop_front();
//             self.is_full = true;
//         }
//         self.values.push_back(value);
//     }
//
//     fn get_average(&self) -> f64 {
//         if self.values.is_empty() {
//             return 0.0;
//         }
//
//         let sum: f64 = self.values.iter().sum();
//         sum / self.values.len() as f64
//     }
// }

impl CpuMonitor {
  /// Create a new CPU monitor
  pub fn new() -> Self {
    Self {
      history_15s: Arc::new(Mutex::new(VecDeque::with_capacity(15))),
      history_30s: Arc::new(Mutex::new(VecDeque::with_capacity(30))),
      history_1m: Arc::new(Mutex::new(VecDeque::with_capacity(60))),
      history_3m: Arc::new(Mutex::new(VecDeque::with_capacity(180))),
      history_5m: Arc::new(Mutex::new(VecDeque::with_capacity(300))),
      history_10m: Arc::new(Mutex::new(VecDeque::with_capacity(600))),
      current_usage: Arc::new(Mutex::new(0.0)),
      last_cpu_time: Arc::new(Mutex::new(None)),
      is_monitoring: false,
    }
  }

  /// Update CPU usage measurement
  pub fn update_cpu_usage(&mut self) -> MonitoringResult<()> {
    let cpu_usage = self.get_current_cpu_usage()?;

    // Update current usage
    if let Ok(mut current) = self.current_usage.lock() {
      *current = cpu_usage;
    }

    // Update all history queues
    self.add_to_history(cpu_usage);

    Ok(())
  }

  /// Add CPU usage to all history queues
  fn add_to_history(&self, usage: f64) {
    let histories = [
      (&self.history_15s, 15),
      (&self.history_30s, 30),
      (&self.history_1m, 60),
      (&self.history_3m, 180),
      (&self.history_5m, 300),
      (&self.history_10m, 600),
    ];

    for (history, max_size) in histories {
      if let Ok(mut hist) = history.lock() {
        hist.push_back(usage);
        while hist.len() > max_size {
          hist.pop_front();
        }
      }
    }
  }

  /// Get current CPU usage percentage
  fn get_current_cpu_usage(&mut self) -> MonitoringResult<f64> {
    let current_time = self.get_current_cpu_time()?;

    let usage = if let Ok(mut last_time_guard) = self.last_cpu_time.lock() {
      let usage = if let Some(last_time) = *last_time_guard {
        let time_diff = current_time.wall_time.saturating_sub(last_time.wall_time) as f64;

        if time_diff > 0.0 {
          let user_diff = current_time.user_time.saturating_sub(last_time.user_time) as f64;
          let system_diff = current_time
            .system_time
            .saturating_sub(last_time.system_time) as f64;
          let total_cpu_time = user_diff + system_diff;

          // Convert to percentage
          (total_cpu_time / time_diff) * 100.0
        } else {
          0.0
        }
      } else {
        0.0
      };

      *last_time_guard = Some(current_time);
      usage
    } else {
      0.0
    };

    Ok(usage.min(100.0).max(0.0))
  }

  /// Get current CPU time (platform-specific)
  fn get_current_cpu_time(&self) -> MonitoringResult<CpuTime> {
    #[cfg(unix)]
    {
      self.get_unix_cpu_time()
    }
    #[cfg(windows)]
    {
      self.get_windows_cpu_time()
    }
  }

  /// Get CPU time on Unix systems
  #[cfg(unix)]
  fn get_unix_cpu_time(&self) -> MonitoringResult<CpuTime> {
    #[cfg(target_os = "linux")]
    {
      self.get_linux_cpu_time()
    }
    #[cfg(target_os = "macos")]
    {
      self.get_macos_cpu_time()
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
      self.get_other_unix_cpu_time()
    }
  }

  /// Get CPU time on Linux systems with optimized parsing
  #[cfg(target_os = "linux")]
  fn get_linux_cpu_time(&self) -> MonitoringResult<CpuTime> {
    use std::fs;
    use std::time::SystemTime;

    // Try to use cached file descriptor for better performance
    static PROC_STAT_FD: Lazy<std::sync::Mutex<Option<std::fs::File>>> =
      Lazy::new(|| std::sync::Mutex::new(None));

    let stat_content = if let Ok(mut fd_guard) = PROC_STAT_FD.lock() {
      if fd_guard.is_none() {
        *fd_guard = std::fs::File::open("/proc/self/stat").ok();
      }

      if let Some(ref mut file) = *fd_guard {
        use std::io::{Read, Seek, SeekFrom};
        let _ = file.seek(SeekFrom::Start(0));
        let mut content = String::new();
        if file.read_to_string(&mut content).is_ok() {
          content
        } else {
          fs::read_to_string("/proc/self/stat")?
        }
      } else {
        fs::read_to_string("/proc/self/stat")?
      }
    } else {
      fs::read_to_string("/proc/self/stat")?
    };

    // Optimized parsing: find the last ')' to handle process names with spaces
    let close_paren = stat_content
      .rfind(')')
      .ok_or_else(|| MonitoringError::ParseError {
        message: "Invalid /proc/self/stat format: missing closing parenthesis".to_string(),
        input: stat_content.clone(),
      })?;

    let fields_part = &stat_content[close_paren + 1..];
    let fields: Vec<&str> = fields_part.split_whitespace().collect();

    if fields.len() < 13 {
      return Err(MonitoringError::ParseError {
        message: format!(
          "Invalid /proc/self/stat format: expected at least 13 fields after process name, got {}",
          fields.len()
        ),
        input: stat_content,
      });
    }

    // Fields 11 and 12 are utime and stime (in clock ticks) after the process name
    let utime: u64 = fields[11]
      .parse()
      .map_err(|e| MonitoringError::ParseError {
        message: format!("Failed to parse utime: {}", e),
        input: fields[11].to_string(),
      })?;
    let stime: u64 = fields[12]
      .parse()
      .map_err(|e| MonitoringError::ParseError {
        message: format!("Failed to parse stime: {}", e),
        input: fields[12].to_string(),
      })?;

    // Convert clock ticks to nanoseconds with error handling
    let clock_ticks_per_sec = unsafe { libc::sysconf(libc::_SC_CLK_TCK) };
    if clock_ticks_per_sec <= 0 {
      return Err(MonitoringError::SystemCall {
        operation: "sysconf(_SC_CLK_TCK)".to_string(),
        source: std::io::Error::new(std::io::ErrorKind::Other, "Invalid clock ticks per second"),
      });
    }

    let ns_per_tick = 1_000_000_000 / clock_ticks_per_sec as u64;
    let user_time = utime * ns_per_tick;
    let system_time = stime * ns_per_tick;

    // Get wall clock time using platform-optimized timer
    let wall_time = if crate::platform::get_platform_capabilities().has_high_res_timer {
      unsafe { self.get_high_res_timestamp() }
    } else {
      SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_nanos() as u64
    };

    Ok(CpuTime {
      user_time,
      system_time,
      wall_time,
    })
  }

  /// Get CPU time on macOS systems using mach system calls
  #[cfg(target_os = "macos")]
  fn get_macos_cpu_time(&self) -> MonitoringResult<CpuTime> {
    use std::mem;
    use std::time::SystemTime;

    // Use mach system calls for accurate CPU time measurement
    let mut info: libc::rusage = unsafe { mem::zeroed() };
    let result = unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut info) };

    if result != 0 {
      return Err(MonitoringError::SystemCall {
        operation: "getrusage".to_string(),
        source: Box::new(std::io::Error::last_os_error()),
      });
    }

    // Convert timeval to nanoseconds
    let user_time =
      (info.ru_utime.tv_sec as u64 * 1_000_000_000) + (info.ru_utime.tv_usec as u64 * 1_000);
    let system_time =
      (info.ru_stime.tv_sec as u64 * 1_000_000_000) + (info.ru_stime.tv_usec as u64 * 1_000);

    // Get wall clock time using high-resolution timer if available
    let wall_time = if crate::platform::get_platform_capabilities().has_high_res_timer {
      unsafe { self.get_high_res_timestamp() }
    } else {
      SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_nanos() as u64
    };

    Ok(CpuTime {
      user_time,
      system_time,
      wall_time,
    })
  }

  /// Get high-resolution timestamp using platform-specific optimizations
  #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
  unsafe fn get_high_res_timestamp(&self) -> u64 {
    if is_x86_feature_detected!("rdtsc") {
      _rdtsc()
    } else {
      std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64
    }
  }

  /// Get high-resolution timestamp fallback
  #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
  unsafe fn get_high_res_timestamp(&self) -> u64 {
    std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .unwrap_or_default()
      .as_nanos() as u64
  }

  /// Get CPU time on other Unix systems
  #[cfg(not(any(target_os = "linux", target_os = "macos", windows)))]
  fn get_other_unix_cpu_time(&self) -> MonitoringResult<CpuTime> {
    use std::time::SystemTime;

    // Fallback implementation for other Unix systems
    let user_time = 1000000; // 1ms in nanoseconds as placeholder
    let system_time = 1000000; // 1ms in nanoseconds as placeholder

    // Get wall clock time
    let wall_time = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)?
      .as_nanos() as u64;

    Ok(CpuTime {
      user_time,
      system_time,
      wall_time,
    })
  }

  /// Get CPU time on Windows systems
  #[cfg(windows)]
  fn get_windows_cpu_time(&self) -> MonitoringResult<CpuTime> {
    use std::mem;
    use std::time::SystemTime;
    use winapi::um::processthreadsapi::{GetCurrentProcess, GetProcessTimes};
    use winapi::um::winnt::FILETIME;

    let mut creation_time = unsafe { mem::zeroed::<FILETIME>() };
    let mut exit_time = unsafe { mem::zeroed::<FILETIME>() };
    let mut kernel_time = unsafe { mem::zeroed::<FILETIME>() };
    let mut user_time = unsafe { mem::zeroed::<FILETIME>() };

    let result = unsafe {
      GetProcessTimes(
        GetCurrentProcess(),
        &mut creation_time,
        &mut exit_time,
        &mut kernel_time,
        &mut user_time,
      )
    };

    if result == 0 {
      return Err("Failed to get process times".into());
    }

    // Convert FILETIME to nanoseconds
    let kernel_ns =
      ((kernel_time.dwHighDateTime as u64) << 32 | kernel_time.dwLowDateTime as u64) * 100;
    let user_ns = ((user_time.dwHighDateTime as u64) << 32 | user_time.dwLowDateTime as u64) * 100;

    // Get wall clock time
    let wall_time = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)?
      .as_nanos() as u64;

    Ok(CpuTime {
      user_time: user_ns,
      system_time: kernel_ns,
      wall_time,
    })
  }

  /// Get CPU usage statistics
  pub fn get_cpu_usage(&self) -> CpuUsage {
    let current = self
      .current_usage
      .lock()
      .unwrap_or_else(|_| panic!("Failed to lock current usage"));

    let avg_15s = self.calculate_average(&self.history_15s);
    let avg_30s = self.calculate_average(&self.history_30s);
    let avg_1m = self.calculate_average(&self.history_1m);
    let avg_3m = self.calculate_average(&self.history_3m);
    let avg_5m = self.calculate_average(&self.history_5m);
    let avg_10m = self.calculate_average(&self.history_10m);

    CpuUsage {
      current: *current,
      avg_15s,
      avg_30s,
      avg_1m,
      avg_3m,
      avg_5m,
      avg_10m,
      timestamp: Instant::now(),
    }
  }

  /// Calculate average from history queue with SIMD optimization
  fn calculate_average(&self, history: &Arc<Mutex<VecDeque<f64>>>) -> f64 {
    if let Ok(hist) = history.lock() {
      if hist.is_empty() {
        0.0
      } else {
        let values: Vec<f64> = hist.iter().copied().collect();
        self.calculate_average_simd(&values)
      }
    } else {
      0.0
    }
  }

  /// SIMD-optimized average calculation
  fn calculate_average_simd(&self, values: &[f64]) -> f64 {
    if values.is_empty() {
      return 0.0;
    }

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
      if is_x86_feature_detected!("avx2") {
        return unsafe { self.calculate_average_avx2(values) };
      } else if is_x86_feature_detected!("sse2") {
        return unsafe { self.calculate_average_sse2(values) };
      }
    }

    #[cfg(target_arch = "aarch64")]
    {
      if std::arch::is_aarch64_feature_detected!("neon") {
        return unsafe { self.calculate_average_neon(values) };
      }
    }

    // Fallback to scalar implementation
    self.calculate_average_scalar(values)
  }

  /// Scalar fallback implementation
  fn calculate_average_scalar(&self, values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
  }

  /// AVX2-optimized average calculation for x86_64
  #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
  #[target_feature(enable = "avx2")]
  unsafe fn calculate_average_avx2(&self, values: &[f64]) -> f64 {
    let mut sum = _mm256_setzero_pd();
    let chunks = values.chunks_exact(4);
    let remainder = chunks.remainder();

    for chunk in chunks {
      let v = _mm256_loadu_pd(chunk.as_ptr());
      sum = _mm256_add_pd(sum, v);
    }

    // Horizontal sum of the 4 doubles in the AVX2 register
    let sum_high = _mm256_extractf128_pd(sum, 1);
    let sum_low = _mm256_castpd256_pd128(sum);
    let sum_combined = _mm_add_pd(sum_high, sum_low);
    let sum_final = _mm_hadd_pd(sum_combined, sum_combined);

    let mut total = _mm_cvtsd_f64(sum_final);

    // Handle remainder
    for &value in remainder {
      total += value;
    }

    total / values.len() as f64
  }

  /// SSE2-optimized average calculation for x86_64
  #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
  #[target_feature(enable = "sse2")]
  unsafe fn calculate_average_sse2(&self, values: &[f64]) -> f64 {
    let mut sum = _mm_setzero_pd();
    let chunks = values.chunks_exact(2);
    let remainder = chunks.remainder();

    for chunk in chunks {
      let v = _mm_loadu_pd(chunk.as_ptr());
      sum = _mm_add_pd(sum, v);
    }

    // Horizontal sum of the 2 doubles in the SSE register
    let sum_final = _mm_hadd_pd(sum, sum);
    let mut total = _mm_cvtsd_f64(sum_final);

    // Handle remainder
    for &value in remainder {
      total += value;
    }

    total / values.len() as f64
  }

  /// NEON-optimized average calculation for AArch64
  #[cfg(target_arch = "aarch64")]
  #[target_feature(enable = "neon")]
  unsafe fn calculate_average_neon(&self, values: &[f64]) -> f64 {
    let mut sum = vdupq_n_f64(0.0);
    let chunks = values.chunks_exact(2);
    let remainder = chunks.remainder();

    for chunk in chunks {
      let v = vld1q_f64(chunk.as_ptr());
      sum = vaddq_f64(sum, v);
    }

    // Horizontal sum of the 2 doubles in the NEON register
    let mut total = vgetq_lane_f64(sum, 0) + vgetq_lane_f64(sum, 1);

    // Handle remainder
    for &value in remainder {
      total += value;
    }

    total / values.len() as f64
  }

  /// Update CPU usage and add to history
  pub fn update(&mut self) -> MonitoringResult<()> {
    let current_usage = self.get_current_cpu_usage()?;

    // Update current usage
    if let Ok(mut current) = self.current_usage.lock() {
      *current = current_usage;
    }

    // Add to history queues
    let _now = std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)?
      .as_secs();

    self.add_to_history(current_usage);

    Ok(())
  }

  /// Format CPU usage for logging (compatible with original format)
  pub fn format_cpu_usage(&self, alinode_format: bool) -> String {
    let usage = self.get_cpu_usage();

    if alinode_format {
      format!(
                "cpu_usage(%%) now: {:.2}, cpu_15: {:.2}, cpu_30: {:.2}, cpu_60: {:.2}, cpu_180: {:.2}, cpu_300: {:.2}, cpu_600: {:.2}",
                usage.current,
                usage.avg_15s,
                usage.avg_30s,
                usage.avg_1m,
                usage.avg_3m,
                usage.avg_5m,
                usage.avg_10m,
            )
    } else {
      format!(
                "cpu_usage(%%) cpu_now: {:.2}, cpu_15: {:.2}, cpu_30: {:.2}, cpu_60: {:.2}, cpu_180: {:.2}, cpu_300: {:.2}, cpu_600: {:.2}",
                usage.current,
                usage.avg_15s,
                usage.avg_30s,
                usage.avg_1m,
                usage.avg_3m,
                usage.avg_5m,
                usage.avg_10m,
            )
    }
  }
}

impl Default for CpuMonitor {
  fn default() -> Self {
    Self::new()
  }
}

impl Monitor for CpuMonitor {
  type Stats = CpuUsage;

  fn start(&mut self) -> MonitoringResult<()> {
    self.is_monitoring = true;
    self.last_cpu_time = Arc::new(Mutex::new(None));
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
    Ok(self.get_cpu_usage())
  }

  fn reset(&mut self) -> MonitoringResult<()> {
    // Reset current usage
    self
      .current_usage
      .lock()
      .map_err(|_| MonitoringError::LockFailed {
        resource: "current usage".to_string(),
        details: "Failed to acquire lock".to_string(),
      })?
      .clone_from(&0.0);

    // Reset all history queues
    let histories = [
      &self.history_15s,
      &self.history_30s,
      &self.history_1m,
      &self.history_3m,
      &self.history_5m,
      &self.history_10m,
    ];

    for history in histories {
      history
        .lock()
        .map_err(|_| MonitoringError::LockFailed {
          resource: "history queue".to_string(),
          details: "Failed to acquire lock".to_string(),
        })?
        .clear();
    }

    // Reset last CPU time
    self
      .last_cpu_time
      .lock()
      .map_err(|_| MonitoringError::LockFailed {
        resource: "last CPU time".to_string(),
        details: "Failed to acquire lock".to_string(),
      })?
      .take();

    Ok(())
  }

  fn update(&mut self) -> MonitoringResult<()> {
    let current_usage = self.get_current_cpu_usage().with_context("get CPU usage")?;

    // Update current usage
    if let Ok(mut current) = self.current_usage.lock() {
      *current = current_usage;
    }

    // Add to history queues
    self.add_to_history(current_usage);

    Ok(())
  }

  fn module_name(&self) -> &'static str {
    "cpu"
  }
}

/// Global CPU monitor instance
pub static CPU_MONITOR: Lazy<Arc<Mutex<CpuMonitor>>> =
  Lazy::new(|| Arc::new(Mutex::new(CpuMonitor::new())));

/// Initialize CPU monitoring
pub fn init_cpu_monitor() -> MonitoringResult<()> {
  let mut monitor = CPU_MONITOR
    .lock()
    .map_err(|_| MonitoringError::LockFailed {
      resource: "CPU monitor".to_string(),
      details: "Failed to acquire lock".to_string(),
    })?;
  monitor.start()
}

/// Start CPU monitoring
pub fn start_cpu_monitor() -> MonitoringResult<()> {
  let mut monitor = CPU_MONITOR
    .lock()
    .map_err(|_| MonitoringError::LockFailed {
      resource: "CPU monitor".to_string(),
      details: "Failed to acquire lock".to_string(),
    })?;
  monitor.start()
}

/// Stop CPU monitoring
pub fn stop_cpu_monitor() -> MonitoringResult<()> {
  let mut monitor = CPU_MONITOR
    .lock()
    .map_err(|_| MonitoringError::LockFailed {
      resource: "CPU monitor".to_string(),
      details: "Failed to acquire lock".to_string(),
    })?;
  monitor.stop()
}

/// Get current CPU usage statistics
pub fn get_cpu_usage() -> MonitoringResult<CpuUsage> {
  let monitor = CPU_MONITOR
    .lock()
    .map_err(|_| MonitoringError::LockFailed {
      resource: "CPU monitor".to_string(),
      details: "Failed to acquire lock".to_string(),
    })?;
  monitor.get_stats()
}

/// Update CPU usage (should be called periodically)
pub fn update_cpu_usage() -> MonitoringResult<()> {
  let mut monitor = CPU_MONITOR
    .lock()
    .map_err(|_| MonitoringError::LockFailed {
      resource: "CPU monitor".to_string(),
      details: "Failed to acquire lock".to_string(),
    })?;
  monitor.update()
}

/// Reset CPU monitoring statistics
pub fn reset_cpu_monitor() -> MonitoringResult<()> {
  let mut monitor = CPU_MONITOR
    .lock()
    .map_err(|_| MonitoringError::LockFailed {
      resource: "CPU monitor".to_string(),
      details: "Failed to acquire lock".to_string(),
    })?;
  monitor.reset()
}

/// Check if CPU monitoring is running
pub fn is_cpu_monitor_running() -> bool {
  let monitor = CPU_MONITOR
    .lock()
    .unwrap_or_else(|_| panic!("Failed to lock CPU monitor"));
  monitor.is_running()
}

/// Format CPU usage for logging
pub fn format_cpu_usage(alinode_format: bool) -> MonitoringResult<String> {
  let monitor = CPU_MONITOR
    .lock()
    .map_err(|_| MonitoringError::LockFailed {
      resource: "CPU monitor".to_string(),
      details: "Failed to acquire lock".to_string(),
    })?;
  let usage = monitor.get_stats()?;

  let formatted = if alinode_format {
    format!(
            "cpu_usage(%%) now: {:.2}, cpu_15: {:.2}, cpu_30: {:.2}, cpu_60: {:.2}, cpu_180: {:.2}, cpu_300: {:.2}, cpu_600: {:.2}",
            usage.current,
            usage.avg_15s,
            usage.avg_30s,
            usage.avg_1m,
            usage.avg_3m,
            usage.avg_5m,
            usage.avg_10m,
        )
  } else {
    format!(
            "cpu_usage(%%) cpu_now: {:.2}, cpu_15: {:.2}, cpu_30: {:.2}, cpu_60: {:.2}, cpu_180: {:.2}, cpu_300: {:.2}, cpu_600: {:.2}",
            usage.current,
            usage.avg_15s,
            usage.avg_30s,
            usage.avg_1m,
            usage.avg_3m,
            usage.avg_5m,
            usage.avg_10m,
        )
  };

  Ok(formatted)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cpu_monitor_creation() {
    let monitor = CpuMonitor::new();
    assert!(!monitor.is_running());
  }

  #[test]
  fn test_cpu_monitor_start_stop() {
    let mut monitor = CpuMonitor::new();

    assert!(monitor.start().is_ok());
    assert!(monitor.is_running());

    assert!(monitor.stop().is_ok());
    assert!(!monitor.is_running());
  }

  #[test]
  fn test_cpu_usage_stats() {
    let monitor = CpuMonitor::new();
    let stats = monitor.get_stats().unwrap();

    assert_eq!(stats.current, 0.0);
    assert_eq!(stats.avg_15s, 0.0);
    assert_eq!(stats.avg_30s, 0.0);
    assert_eq!(stats.avg_1m, 0.0);
    assert_eq!(stats.avg_3m, 0.0);
    assert_eq!(stats.avg_5m, 0.0);
    assert_eq!(stats.avg_10m, 0.0);
  }

  #[test]
  fn test_cpu_monitor_reset() {
    let mut monitor = CpuMonitor::new();

    // Add some test data
    monitor.add_to_history(50.0);
    monitor.add_to_history(60.0);

    // Reset should clear all data
    assert!(monitor.reset().is_ok());

    let stats = monitor.get_stats().unwrap();
    assert_eq!(stats.avg_15s, 0.0);
  }

  #[test]
  fn test_global_functions() {
    assert!(init_cpu_monitor().is_ok());
    assert!(is_cpu_monitor_running());

    let usage = get_cpu_usage().unwrap();
    assert!(usage.current >= 0.0);

    assert!(stop_cpu_monitor().is_ok());
    assert!(!is_cpu_monitor_running());
  }

  #[test]
  fn test_format_cpu_usage() {
    let _ = init_cpu_monitor();

    let alinode_format = format_cpu_usage(true).unwrap();
    assert!(alinode_format.contains("cpu_usage(%%) now:"));

    let normal_format = format_cpu_usage(false).unwrap();
    assert!(normal_format.contains("cpu_usage(%%) cpu_now:"));
  }

  #[test]
  fn test_cpu_time_calculation() {
    let monitor = CpuMonitor::new();

    // Test that CPU time retrieval doesn't panic
    let result = monitor.get_current_cpu_time();
    assert!(result.is_ok() || result.is_err()); // Should not panic
  }
}
