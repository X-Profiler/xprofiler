//! Memory monitoring module
//!
//! This module provides memory usage monitoring capabilities including
//! RSS, heap usage, and other memory-related metrics with historical tracking
//! and time period averages following the original C++ implementation.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use once_cell::sync::Lazy;
use super::{Monitor, TimePeriod};

/// Memory usage statistics
#[derive(Debug, Clone)]
pub struct MemoryUsage {
    /// Resident Set Size (RSS) in bytes
    pub rss: u64,
    /// Virtual memory size in bytes
    pub vms: u64,
    /// Heap used in bytes
    pub heap_used: u64,
    /// Heap total in bytes
    pub heap_total: u64,
    /// External memory in bytes
    pub external: u64,
    /// Array buffers in bytes
    pub array_buffers: u64,
    /// Average RSS over 15 seconds
    pub avg_rss_15s: f64,
    /// Average RSS over 30 seconds
    pub avg_rss_30s: f64,
    /// Average RSS over 1 minute
    pub avg_rss_1m: f64,
    /// Average RSS over 3 minutes
    pub avg_rss_3m: f64,
    /// Average RSS over 5 minutes
    pub avg_rss_5m: f64,
    /// Timestamp when this measurement was taken
    pub timestamp: u64,
}

/// Memory monitor implementation
#[derive(Debug)]
pub struct MemoryMonitor {
    /// Whether monitoring is active
    is_monitoring: bool,
    /// Historical RSS data for 15 seconds (capacity: 15)
    history_rss_15s: VecDeque<u64>,
    /// Historical RSS data for 30 seconds (capacity: 30)
    history_rss_30s: VecDeque<u64>,
    /// Historical RSS data for 1 minute (capacity: 60)
    history_rss_1m: VecDeque<u64>,
    /// Historical RSS data for 3 minutes (capacity: 180)
    history_rss_3m: VecDeque<u64>,
    /// Historical RSS data for 5 minutes (capacity: 300)
    history_rss_5m: VecDeque<u64>,
    /// Current RSS usage in bytes
    current_rss: Arc<Mutex<u64>>,
    /// Last update timestamp
    last_update: Arc<Mutex<Option<Instant>>>,
}

impl MemoryMonitor {
    /// Create a new memory monitor
    pub fn new() -> Self {
        Self {
            is_monitoring: false,
            history_rss_15s: VecDeque::with_capacity(15),
            history_rss_30s: VecDeque::with_capacity(30),
            history_rss_1m: VecDeque::with_capacity(60),
            history_rss_3m: VecDeque::with_capacity(180),
            history_rss_5m: VecDeque::with_capacity(300),
            current_rss: Arc::new(Mutex::new(0)),
            last_update: Arc::new(Mutex::new(None)),
        }
    }
    
    /// Update memory usage and add to history
    pub fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let process_memory = self.get_process_memory()?;
        
        // Update current RSS
        {
            let mut current_rss = self.current_rss.lock().map_err(|_| "Failed to lock current_rss")?;
            *current_rss = process_memory.rss;
        }
        
        // Add to history
        self.add_to_history_all(process_memory.rss);
        
        // Update timestamp
        {
            let mut last_update = self.last_update.lock().map_err(|_| "Failed to lock last_update")?;
            *last_update = Some(Instant::now());
        }
        
        Ok(())
    }
    
    /// Add RSS value to all history queues
    fn add_to_history_all(&mut self, rss: u64) {
        // Add to 15s history
        if self.history_rss_15s.len() >= 15 {
            self.history_rss_15s.pop_front();
        }
        self.history_rss_15s.push_back(rss);
        
        // Add to 30s history
        if self.history_rss_30s.len() >= 30 {
            self.history_rss_30s.pop_front();
        }
        self.history_rss_30s.push_back(rss);
        
        // Add to 1m history
        if self.history_rss_1m.len() >= 60 {
            self.history_rss_1m.pop_front();
        }
        self.history_rss_1m.push_back(rss);
        
        // Add to 3m history
        if self.history_rss_3m.len() >= 180 {
            self.history_rss_3m.pop_front();
        }
        self.history_rss_3m.push_back(rss);
        
        // Add to 5m history
        if self.history_rss_5m.len() >= 300 {
            self.history_rss_5m.pop_front();
        }
        self.history_rss_5m.push_back(rss);
    }
    
    /// Get current memory usage with averages
    pub fn get_memory_usage(&self) -> Result<MemoryUsage, Box<dyn std::error::Error>> {
        let process_memory = self.get_process_memory()?;
        let heap_stats = self.get_heap_statistics();
        
        let current_rss = self.current_rss.lock().map_err(|_| "Failed to lock current_rss")?;
        
        let avg_rss_15s = self.calculate_average(&self.history_rss_15s);
        let avg_rss_30s = self.calculate_average(&self.history_rss_30s);
        let avg_rss_1m = self.calculate_average(&self.history_rss_1m);
        let avg_rss_3m = self.calculate_average(&self.history_rss_3m);
        let avg_rss_5m = self.calculate_average(&self.history_rss_5m);
        
        Ok(MemoryUsage {
            rss: *current_rss,
            vms: process_memory.vms,
            heap_used: heap_stats.heap_used,
            heap_total: heap_stats.heap_total,
            external: heap_stats.external,
            array_buffers: heap_stats.array_buffers,
            avg_rss_15s,
            avg_rss_30s,
            avg_rss_1m,
            avg_rss_3m,
            avg_rss_5m,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        })
    }
    
    /// Calculate average from history queue
    fn calculate_average(&self, history: &VecDeque<u64>) -> f64 {
        if history.is_empty() {
            0.0
        } else {
            let sum: u64 = history.iter().sum();
            sum as f64 / history.len() as f64
        }
    }
    
    /// Get process memory information
    #[cfg(unix)]
    fn get_process_memory(&self) -> Result<ProcessMemory, Box<dyn std::error::Error>> {
        use std::fs;
        
        // Read /proc/self/status for memory information
        let status_content = fs::read_to_string("/proc/self/status")?;
        
        let mut rss = 0;
        let mut vms = 0;
        
        for line in status_content.lines() {
            if line.starts_with("VmRSS:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    rss = value.parse::<u64>().unwrap_or(0) * 1024; // Convert KB to bytes
                }
            } else if line.starts_with("VmSize:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    vms = value.parse::<u64>().unwrap_or(0) * 1024; // Convert KB to bytes
                }
            }
        }
        
        Ok(ProcessMemory { rss, vms })
    }
    
    /// Get process memory information (Windows implementation)
    #[cfg(windows)]
    fn get_process_memory(&self) -> Result<ProcessMemory, Box<dyn std::error::Error>> {
        use std::mem;
        use winapi::um::processthreadsapi::GetCurrentProcess;
        use winapi::um::psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
        
        let mut pmc = unsafe { mem::zeroed::<PROCESS_MEMORY_COUNTERS>() };
        pmc.cb = mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
        
        let result = unsafe {
            GetProcessMemoryInfo(
                GetCurrentProcess(),
                &mut pmc,
                mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
            )
        };
        
        if result == 0 {
            return Err("Failed to get process memory info".into());
        }
        
        Ok(ProcessMemory {
            rss: pmc.WorkingSetSize as u64,
            vms: pmc.PagefileUsage as u64,
        })
    }
    
    /// Get heap statistics (placeholder - would integrate with V8 in real implementation)
    fn get_heap_statistics(&self) -> HeapStats {
        // In a real implementation, this would integrate with V8's heap statistics
        // For now, return placeholder values
        HeapStats {
            heap_used: 0,
            heap_total: 0,
            external: 0,
            array_buffers: 0,
            total_heap_size: 0,
            total_heap_size_executable: 0,
            total_physical_size: 0,
            total_available_size: 0,
            used_heap_size: 0,
            heap_size_limit: 0,
            malloced_memory: 0,
            peak_malloced_memory: 0,
            external_memory: 0,
        }
    }
    
    /// Format memory usage for logging
    pub fn format_memory_usage(&self) -> Result<String, Box<dyn std::error::Error>> {
        let usage = self.get_memory_usage()?;
        
        Ok(format!(
            "memory rss: {} MB, vms: {} MB, heap_used: {} MB, heap_total: {} MB, external: {} MB, array_buffers: {} MB",
            usage.rss / 1024 / 1024,
            usage.vms / 1024 / 1024,
            usage.heap_used / 1024 / 1024,
            usage.heap_total / 1024 / 1024,
            usage.external / 1024 / 1024,
            usage.array_buffers / 1024 / 1024,
        ))
    }
}

/// Process memory information
struct ProcessMemory {
    rss: u64,
    vms: u64,
}

/// Heap statistics
#[derive(Debug, Clone)]
pub struct HeapStats {
    pub heap_used: u64,
    pub heap_total: u64,
    pub external: u64,
    pub array_buffers: u64,
    pub total_heap_size: u64,
    pub total_heap_size_executable: u64,
    pub total_physical_size: u64,
    pub total_available_size: u64,
    pub used_heap_size: u64,
    pub heap_size_limit: u64,
    pub malloced_memory: u64,
    pub peak_malloced_memory: u64,
    pub external_memory: u64,
}

impl Default for MemoryMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Monitor for MemoryMonitor {
    type Stats = MemoryUsage;
    
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.is_monitoring = true;
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
        self.get_memory_usage().unwrap_or(MemoryUsage {
            rss: 0,
            vms: 0,
            heap_used: 0,
            heap_total: 0,
            external: 0,
            array_buffers: 0,
            avg_rss_15s: 0.0,
            avg_rss_30s: 0.0,
            avg_rss_1m: 0.0,
            avg_rss_3m: 0.0,
            avg_rss_5m: 0.0,
            timestamp: 0,
        })
    }
    
    fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.history_rss_15s.clear();
        self.history_rss_30s.clear();
        self.history_rss_1m.clear();
        self.history_rss_3m.clear();
        self.history_rss_5m.clear();
        
        if let Ok(mut current_rss) = self.current_rss.lock() {
            *current_rss = 0;
        }
        
        if let Ok(mut last_update) = self.last_update.lock() {
            *last_update = None;
        }
        
        Ok(())
    }
}

/// Global memory monitor instance
pub static MEMORY_MONITOR: Lazy<Arc<Mutex<MemoryMonitor>>> = Lazy::new(|| {
    Arc::new(Mutex::new(MemoryMonitor::new()))
});

/// Initialize global memory monitor
pub fn init_memory_monitor() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = MEMORY_MONITOR.lock().map_err(|_| "Failed to lock memory monitor")?;
    *monitor = MemoryMonitor::new();
    Ok(())
}

/// Start memory monitoring
pub fn start_memory_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = MEMORY_MONITOR.lock().map_err(|_| "Failed to lock memory monitor")?;
    monitor.start()
}

/// Stop memory monitoring
pub fn stop_memory_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = MEMORY_MONITOR.lock().map_err(|_| "Failed to lock memory monitor")?;
    monitor.stop()
}

/// Update memory usage
pub fn update_memory_usage() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = MEMORY_MONITOR.lock().map_err(|_| "Failed to lock memory monitor")?;
    monitor.update()
}

/// Get current memory usage statistics
pub fn get_memory_usage() -> Option<MemoryUsage> {
    let monitor = MEMORY_MONITOR.lock().ok()?;
    monitor.get_memory_usage().ok()
}

/// Get memory statistics
pub fn get_memory_stats() -> Result<MemoryUsage, Box<dyn std::error::Error>> {
    let monitor = MEMORY_MONITOR.lock()
        .map_err(|_| "Failed to lock memory monitor")?;
    
    Ok(monitor.get_stats())
}

/// Reset memory monitor
pub fn reset_memory_monitor() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = MEMORY_MONITOR.lock().map_err(|_| "Failed to lock memory monitor")?;
    monitor.reset()
}

/// Check if memory monitor is running
pub fn is_memory_monitor_running() -> bool {
    MEMORY_MONITOR.lock().map(|monitor| monitor.is_running()).unwrap_or(false)
}

/// Get memory usage for a specific time period
pub fn get_memory_usage_for_period(_period: TimePeriod) -> Option<MemoryUsage> {
    let monitor = MEMORY_MONITOR.lock().ok()?;
    monitor.get_memory_usage().ok()
}

/// Get heap statistics
pub fn get_heap_stats() -> Option<HeapStats> {
    let monitor = MEMORY_MONITOR.lock().ok()?;
    Some(monitor.get_heap_statistics())
}

/// Format memory usage for display
pub fn format_memory_usage() -> String {
    match get_memory_stats() {
        Ok(stats) => {
            format!(
                "Memory Usage:\n  RSS: {} MB (15s: {:.2} MB, 30s: {:.2} MB, 1m: {:.2} MB, 3m: {:.2} MB, 5m: {:.2} MB)\n  VMS: {} MB\n  Heap Used: {} MB\n  Heap Total: {} MB\n  External: {} MB\n  Array Buffers: {} MB\n  Timestamp: {}",
                stats.rss / 1024 / 1024,
                stats.avg_rss_15s / 1024.0 / 1024.0,
                stats.avg_rss_30s / 1024.0 / 1024.0,
                stats.avg_rss_1m / 1024.0 / 1024.0,
                stats.avg_rss_3m / 1024.0 / 1024.0,
                stats.avg_rss_5m / 1024.0 / 1024.0,
                stats.vms / 1024 / 1024,
                stats.heap_used / 1024 / 1024,
                stats.heap_total / 1024 / 1024,
                stats.external / 1024 / 1024,
                stats.array_buffers / 1024 / 1024,
                stats.timestamp
            )
        }
        Err(_) => "Memory monitoring not available".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_memory_monitor_creation() {
        let monitor = MemoryMonitor::new();
        assert!(!monitor.is_monitoring);
    }

    #[test]
    fn test_memory_monitor_start_stop() {
        let mut monitor = MemoryMonitor::new();
        assert!(!monitor.is_running());
        
        monitor.start().unwrap();
        assert!(monitor.is_running());
        
        monitor.stop().unwrap();
        assert!(!monitor.is_running());
    }

    #[test]
    fn test_memory_usage() {
        let monitor = MemoryMonitor::new();
        let usage = monitor.get_memory_usage();
        assert!(usage.is_ok());
        
        let usage = usage.unwrap();
        assert!(usage.rss > 0);
        assert_eq!(usage.timestamp, 0); // Initial timestamp should be 0
    }

    #[test]
    fn test_memory_update() {
        let mut monitor = MemoryMonitor::new();
        monitor.update().unwrap();
        
        let stats = monitor.get_stats();
        assert!(stats.rss > 0);
        assert!(stats.timestamp > 0);
    }

    #[test]
    fn test_memory_averages() {
        let mut monitor = MemoryMonitor::new();
        
        // Update multiple times to build history
        for _ in 0..5 {
            monitor.update().unwrap();
            thread::sleep(Duration::from_millis(10));
        }
        
        let stats = monitor.get_stats();
        // Averages should be calculated based on history
        assert!(stats.avg_rss_15s >= 0.0);
        assert!(stats.avg_rss_30s >= 0.0);
    }

    #[test]
    fn test_memory_reset() {
        let mut monitor = MemoryMonitor::new();
        
        // Add some data
        monitor.update().unwrap();
        monitor.reset();
        
        // Check that history is cleared
        assert_eq!(monitor.history_rss_15s.len(), 0);
        assert_eq!(monitor.history_rss_30s.len(), 0);
    }

    #[test]
    fn test_global_functions() {
        init_memory_monitor().unwrap();
        start_memory_monitoring().unwrap();
        assert!(is_memory_monitor_running());
        
        update_memory_usage().unwrap();
        let stats = get_memory_stats();
        assert!(stats.is_ok());
        
        stop_memory_monitoring().unwrap();
        assert!(!is_memory_monitor_running());
        
        reset_memory_monitor().unwrap();
    }

    #[test]
    fn test_format_memory_usage() {
        init_memory_monitor().unwrap();
        update_memory_usage().unwrap();
        
        let formatted = format_memory_usage();
        assert!(formatted.contains("Memory Usage"));
        assert!(formatted.contains("RSS:"));
        assert!(formatted.contains("MB"));
    }

    #[test]
    fn test_calculate_average() {
        let monitor = MemoryMonitor::new();
        
        // Test with empty history
        let avg = monitor.calculate_average(&monitor.history_rss_15s);
        assert_eq!(avg, 0.0);
        
        // Test with some data
        let mut test_queue = VecDeque::new();
        test_queue.push_back(100);
        test_queue.push_back(200);
        test_queue.push_back(300);
        
        let avg = monitor.calculate_average(&test_queue);
        assert_eq!(avg, 200.0); // (100 + 200 + 300) / 3
    }
    
    #[test]
    fn test_memory_usage_structure() {
        let usage = MemoryUsage {
            rss: 1024 * 1024,
            vms: 2048 * 1024,
            heap_used: 512 * 1024,
            heap_total: 1024 * 1024,
            external: 256 * 1024,
            array_buffers: 128 * 1024,
            avg_rss_15s: 0.0,
            avg_rss_30s: 0.0,
            avg_rss_1m: 0.0,
            avg_rss_3m: 0.0,
            avg_rss_5m: 0.0,
            timestamp: 0,
        };
        
        assert_eq!(usage.rss, 1024 * 1024);
        assert_eq!(usage.vms, 2048 * 1024);
    }
}