//! CPU monitoring module
//!
//! This module provides CPU usage monitoring capabilities with support for
//! multiple time period averages, similar to the original C++ implementation.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::{Monitor, TimePeriod};

/// CPU usage statistics
#[derive(Debug, Clone)]
pub struct CpuUsage {
    /// Current CPU usage percentage
    pub current: f64,
    /// Average CPU usage over different time periods
    pub averages: std::collections::HashMap<TimePeriod, f64>,
}

/// CPU monitor implementation
pub struct CpuMonitor {
    /// Current CPU usage
    current_usage: f64,
    /// Storage for different time periods
    period_data: std::collections::HashMap<TimePeriod, PeriodData>,
    /// Last measurement time
    last_time: Option<Instant>,
    /// Last CPU time (platform-specific)
    last_cpu_time: Option<u64>,
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
        let mut period_data = std::collections::HashMap::new();
        
        for period in TimePeriod::all() {
            period_data.insert(period, PeriodData::new(period));
        }
        
        Self {
            current_usage: 0.0,
            period_data,
            last_time: None,
            last_cpu_time: None,
            is_monitoring: false,
        }
    }
    
    /// Update CPU usage measurement
    pub fn update_cpu_usage(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let cpu_usage = self.get_current_cpu_usage()?;
        
        if cpu_usage >= 0.0 {
            self.current_usage = cpu_usage;
            
            // Update all period data
            for period_data in self.period_data.values_mut() {
                period_data.add_value(cpu_usage);
            }
        }
        
        Ok(())
    }
    
    /// Get current CPU usage percentage
    fn get_current_cpu_usage(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let now = Instant::now();
        let current_cpu_time = self.get_process_cpu_time()?;
        
        // First measurement - return -1 to indicate initialization
        if self.last_time.is_none() || self.last_cpu_time.is_none() {
            self.last_time = Some(now);
            self.last_cpu_time = Some(current_cpu_time);
            return Ok(-1.0);
        }
        
        let last_time = self.last_time.unwrap();
        let last_cpu_time = self.last_cpu_time.unwrap();
        
        let time_diff = now.duration_since(last_time).as_secs_f64();
        if time_diff <= 0.0 {
            return Ok(-1.0);
        }
        
        // Calculate CPU usage percentage
        let cpu_time_diff = current_cpu_time.saturating_sub(last_cpu_time) as f64;
        let cpu_usage = (cpu_time_diff / 1_000_000.0) / time_diff * 100.0; // Convert microseconds to seconds
        
        // Update last values
        self.last_time = Some(now);
        self.last_cpu_time = Some(current_cpu_time);
        
        Ok(cpu_usage)
    }
    
    /// Get process CPU time in microseconds (platform-specific implementation)
    #[cfg(unix)]
    fn get_process_cpu_time(&self) -> Result<u64, Box<dyn std::error::Error>> {
        use std::fs;
        
        // Read /proc/self/stat for CPU time information
        let stat_content = fs::read_to_string("/proc/self/stat")?;
        let fields: Vec<&str> = stat_content.split_whitespace().collect();
        
        if fields.len() < 15 {
            return Err("Invalid /proc/self/stat format".into());
        }
        
        // Fields 13 and 14 are utime and stime (user and system CPU time in clock ticks)
        let utime: u64 = fields[13].parse()?;
        let stime: u64 = fields[14].parse()?;
        
        // Convert clock ticks to microseconds
        let clock_ticks_per_sec = unsafe { libc::sysconf(libc::_SC_CLK_TCK) } as u64;
        let total_cpu_time = (utime + stime) * 1_000_000 / clock_ticks_per_sec;
        
        Ok(total_cpu_time)
    }
    
    /// Get process CPU time in microseconds (Windows implementation)
    #[cfg(windows)]
    fn get_process_cpu_time(&self) -> Result<u64, Box<dyn std::error::Error>> {
        use std::mem;
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
        
        // Convert FILETIME to microseconds
        let kernel_us = ((kernel_time.dwHighDateTime as u64) << 32 | kernel_time.dwLowDateTime as u64) / 10;
        let user_us = ((user_time.dwHighDateTime as u64) << 32 | user_time.dwLowDateTime as u64) / 10;
        
        Ok(kernel_us + user_us)
    }
    
    /// Get CPU usage statistics
    pub fn get_cpu_usage(&self) -> CpuUsage {
        let mut averages = std::collections::HashMap::new();
        
        for (period, data) in &self.period_data {
            averages.insert(*period, data.get_average());
        }
        
        CpuUsage {
            current: self.current_usage,
            averages,
        }
    }
    
    /// Format CPU usage for logging (compatible with original format)
    pub fn format_cpu_usage(&self, alinode_format: bool) -> String {
        let usage = self.get_cpu_usage();
        
        if alinode_format {
            format!(
                "cpu_usage(%%) now: {:.2}, cpu_15: {:.2}, cpu_30: {:.2}, cpu_60: {:.2}, cpu_180: {:.2}, cpu_300: {:.2}, cpu_600: {:.2}",
                usage.current,
                usage.averages.get(&TimePeriod::Seconds15).unwrap_or(&0.0),
                usage.averages.get(&TimePeriod::Seconds30).unwrap_or(&0.0),
                usage.averages.get(&TimePeriod::Seconds60).unwrap_or(&0.0),
                usage.averages.get(&TimePeriod::Seconds180).unwrap_or(&0.0),
                usage.averages.get(&TimePeriod::Seconds300).unwrap_or(&0.0),
                usage.averages.get(&TimePeriod::Seconds600).unwrap_or(&0.0),
            )
        } else {
            format!(
                "cpu_usage(%%) cpu_now: {:.2}, cpu_15: {:.2}, cpu_30: {:.2}, cpu_60: {:.2}, cpu_180: {:.2}, cpu_300: {:.2}, cpu_600: {:.2}",
                usage.current,
                usage.averages.get(&TimePeriod::Seconds15).unwrap_or(&0.0),
                usage.averages.get(&TimePeriod::Seconds30).unwrap_or(&0.0),
                usage.averages.get(&TimePeriod::Seconds60).unwrap_or(&0.0),
                usage.averages.get(&TimePeriod::Seconds180).unwrap_or(&0.0),
                usage.averages.get(&TimePeriod::Seconds300).unwrap_or(&0.0),
                usage.averages.get(&TimePeriod::Seconds600).unwrap_or(&0.0),
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
    type Output = CpuUsage;
    
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
    
    fn get_metrics(&self) -> Self::Output {
        self.get_cpu_usage()
    }
    
    fn reset(&mut self) {
        self.current_usage = 0.0;
        self.last_time = None;
        self.last_cpu_time = None;
        
        for data in self.period_data.values_mut() {
            data.values.clear();
            data.is_full = false;
        }
    }
}

/// Global CPU monitor instance
static CPU_MONITOR: Mutex<Option<CpuMonitor>> = Mutex::new(None);

/// Initialize global CPU monitor
pub fn init_cpu_monitor() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = CPU_MONITOR.lock().unwrap();
    *monitor = Some(CpuMonitor::new());
    Ok(())
}

/// Update global CPU usage
pub fn update_cpu_usage() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = CPU_MONITOR.lock().unwrap();
    if let Some(ref mut cpu_monitor) = monitor.as_mut() {
        cpu_monitor.update_cpu_usage()?;
    }
    Ok(())
}

/// Get current CPU usage statistics
pub fn get_cpu_usage() -> Option<CpuUsage> {
    let monitor = CPU_MONITOR.lock().unwrap();
    monitor.as_ref().map(|m| m.get_cpu_usage())
}

/// Format CPU usage for logging
pub fn format_cpu_usage(alinode_format: bool) -> String {
    let monitor = CPU_MONITOR.lock().unwrap();
    monitor
        .as_ref()
        .map(|m| m.format_cpu_usage(alinode_format))
        .unwrap_or_else(|| "CPU monitor not initialized".to_string())
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