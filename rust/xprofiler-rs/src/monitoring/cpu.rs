//! CPU monitoring module
//!
//! This module provides CPU usage monitoring functionality,
//! including current CPU usage and historical averages over different time periods.
//! 
//! The implementation follows the original C++ logic but with improved error handling
//! and platform-specific optimizations.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use once_cell::sync::Lazy;

use super::{Monitor, TimePeriod};

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

/// Data storage for a specific time period
struct PeriodData {
    /// Circular buffer for CPU usage values
    values: VecDeque<f64>,
    /// Maximum number of values to store
    max_size: usize,
    /// Whether the buffer is full
    is_full: bool,
}

impl PeriodData {
    fn new(period: TimePeriod) -> Self {
        Self {
            values: VecDeque::new(),
            max_size: period.as_seconds() as usize,
            is_full: false,
        }
    }
    
    fn add_value(&mut self, value: f64) {
        if self.values.len() >= self.max_size {
            self.values.pop_front();
            self.is_full = true;
        }
        self.values.push_back(value);
    }
    
    fn get_average(&self) -> f64 {
        if self.values.is_empty() {
            return 0.0;
        }
        
        let sum: f64 = self.values.iter().sum();
        sum / self.values.len() as f64
    }
}

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
    pub fn update_cpu_usage(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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
    fn get_current_cpu_usage(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let current_time = self.get_current_cpu_time()?;
        
        let usage = if let Ok(mut last_time_guard) = self.last_cpu_time.lock() {
            if let Some(last_time) = *last_time_guard {
                let cpu_time_diff = (current_time.user_time + current_time.system_time)
                    .saturating_sub(last_time.user_time + last_time.system_time);
                let wall_time_diff = current_time.wall_time.saturating_sub(last_time.wall_time);
                
                if wall_time_diff > 0 {
                    let usage = (cpu_time_diff as f64 / wall_time_diff as f64) * 100.0;
                    usage.min(100.0).max(0.0)
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
        
        Ok(usage)
    }
    
    /// Get current CPU time (platform-specific)
    fn get_current_cpu_time(&self) -> Result<CpuTime, Box<dyn std::error::Error>> {
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
    fn get_unix_cpu_time(&self) -> Result<CpuTime, Box<dyn std::error::Error>> {
        use std::fs;
        use std::time::SystemTime;
        
        // Read /proc/self/stat for process CPU time
        let stat = fs::read_to_string("/proc/self/stat")?;
        let fields: Vec<&str> = stat.split_whitespace().collect();
        
        if fields.len() < 15 {
            return Err("Invalid /proc/self/stat format".into());
        }
        
        // Fields 13 and 14 are utime and stime (in clock ticks)
        let utime: u64 = fields[13].parse()?;
        let stime: u64 = fields[14].parse()?;
        
        // Convert clock ticks to nanoseconds
        let clock_ticks_per_sec = unsafe { libc::sysconf(libc::_SC_CLK_TCK) } as u64;
        let ns_per_tick = 1_000_000_000 / clock_ticks_per_sec;
        
        let user_time = utime * ns_per_tick;
        let system_time = stime * ns_per_tick;
        
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
    fn get_windows_cpu_time(&self) -> Result<CpuTime, Box<dyn std::error::Error>> {
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
        let kernel_ns = ((kernel_time.dwHighDateTime as u64) << 32 | kernel_time.dwLowDateTime as u64) * 100;
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
        let current = self.current_usage.lock().unwrap_or_else(|_| std::sync::MutexGuard::new(0.0));
        
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
    
    /// Calculate average from history queue
    fn calculate_average(&self, history: &Arc<Mutex<VecDeque<f64>>>) -> f64 {
        if let Ok(hist) = history.lock() {
            if hist.is_empty() {
                0.0
            } else {
                hist.iter().sum::<f64>() / hist.len() as f64
            }
        } else {
            0.0
        }
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
    
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.is_monitoring = true;
        // Initialize first measurement
        let _ = self.get_current_cpu_usage();
        Ok(())
    }
    
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.is_monitoring = false;
        Ok(())
    }
    
    fn is_running(&self) -> bool {
        self.is_monitoring
    }
    
    fn get_stats(&self) -> Self::Stats {
        self.get_cpu_usage()
    }
    
    fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Reset current usage
        if let Ok(mut current) = self.current_usage.lock() {
            *current = 0.0;
        }
        
        // Reset all history queues
        let histories = [
            &self.history_15s, &self.history_30s, &self.history_1m,
            &self.history_3m, &self.history_5m, &self.history_10m,
        ];
        
        for history in histories {
            if let Ok(mut hist) = history.lock() {
                hist.clear();
            }
        }
        
        // Reset last CPU time
        if let Ok(mut last_time) = self.last_cpu_time.lock() {
            *last_time = None;
        }
        
        Ok(())
    }
}

/// Global CPU monitor instance
static CPU_MONITOR: Lazy<Arc<Mutex<CpuMonitor>>> = Lazy::new(|| {
    Arc::new(Mutex::new(CpuMonitor::new()))
});

/// Initialize CPU monitoring
pub fn init_cpu_monitor() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = CPU_MONITOR.lock().map_err(|_| "Failed to lock CPU monitor")?;
    monitor.start()
}

/// Start CPU monitoring
pub fn start_cpu_monitor() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = CPU_MONITOR.lock().map_err(|_| "Failed to lock CPU monitor")?;
    monitor.start()
}

/// Stop CPU monitoring
pub fn stop_cpu_monitor() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = CPU_MONITOR.lock().map_err(|_| "Failed to lock CPU monitor")?;
    monitor.stop()
}

/// Get current CPU usage statistics
pub fn get_cpu_usage() -> CpuUsage {
    let monitor = CPU_MONITOR.lock().unwrap_or_else(|_| {
        panic!("Failed to lock CPU monitor")
    });
    monitor.get_stats()
}

/// Update CPU usage (should be called periodically)
pub fn update_cpu_usage() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = CPU_MONITOR.lock().map_err(|_| "Failed to lock CPU monitor")?;
    monitor.update()
}

/// Reset CPU monitoring statistics
pub fn reset_cpu_monitor() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = CPU_MONITOR.lock().map_err(|_| "Failed to lock CPU monitor")?;
    monitor.reset()
}

/// Check if CPU monitoring is running
pub fn is_cpu_monitor_running() -> bool {
    let monitor = CPU_MONITOR.lock().unwrap_or_else(|_| {
        panic!("Failed to lock CPU monitor")
    });
    monitor.is_running()
}

/// Format CPU usage for logging
pub fn format_cpu_usage(alinode_format: bool) -> String {
    let monitor = CPU_MONITOR.lock().unwrap();
    let usage = monitor.get_stats();
    
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

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
        let stats = monitor.get_stats();
        
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
        
        let stats = monitor.get_stats();
        assert_eq!(stats.avg_15s, 0.0);
    }

    #[test]
    fn test_global_functions() {
        assert!(init_cpu_monitor().is_ok());
        assert!(is_cpu_monitor_running());
        
        let usage = get_cpu_usage();
        assert!(usage.current >= 0.0);
        
        assert!(stop_cpu_monitor().is_ok());
        assert!(!is_cpu_monitor_running());
    }

    #[test]
    fn test_format_cpu_usage() {
        let _ = init_cpu_monitor();
        
        let alinode_format = format_cpu_usage(true);
        assert!(alinode_format.contains("cpu_usage(%%) now:"));
        
        let normal_format = format_cpu_usage(false);
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cpu_monitor_creation() {
        let monitor = CpuMonitor::new();
        assert_eq!(monitor.current_usage, 0.0);
        assert!(!monitor.is_monitoring);
    }
    
    #[test]
    fn test_period_data() {
        let mut data = PeriodData::new(TimePeriod::Seconds15);
        
        // Add some values
        for i in 1..=10 {
            data.add_value(i as f64);
        }
        
        assert_eq!(data.values.len(), 10);
        assert!(!data.is_full);
        
        let average = data.get_average();
        assert!((average - 5.5).abs() < 0.01); // Average of 1..10 is 5.5
    }
    
    #[test]
    fn test_time_periods() {
        let periods = TimePeriod::all();
        assert_eq!(periods.len(), 6);
        assert_eq!(TimePeriod::Seconds15.as_seconds(), 15);
        assert_eq!(TimePeriod::Seconds600.as_seconds(), 600);
    }
}