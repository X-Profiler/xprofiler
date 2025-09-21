//! libuv monitoring module
//!
//! This module provides libuv event loop monitoring capabilities
//! including handle counts, active handles, and event loop metrics.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use super::Monitor;
use super::error::{MonitoringResult, MonitoringError};

/// libuv handle types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HandleType {
    Timer,
    Tcp,
    Udp,
    Pipe,
    Tty,
    Poll,
    Prepare,
    Check,
    Idle,
    Async,
    FsEvent,
    FsPoll,
    Signal,
    Process,
    Unknown,
}

impl HandleType {
    pub fn from_uv_type(handle_type: u32) -> Self {
        match handle_type {
            1 => HandleType::Timer,
            2 => HandleType::Tcp,
            3 => HandleType::Udp,
            4 => HandleType::Pipe,
            5 => HandleType::Tty,
            6 => HandleType::Poll,
            7 => HandleType::Prepare,
            8 => HandleType::Check,
            9 => HandleType::Idle,
            10 => HandleType::Async,
            11 => HandleType::FsEvent,
            12 => HandleType::FsPoll,
            13 => HandleType::Signal,
            14 => HandleType::Process,
            _ => HandleType::Unknown,
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            HandleType::Timer => "timer",
            HandleType::Tcp => "tcp",
            HandleType::Udp => "udp",
            HandleType::Pipe => "pipe",
            HandleType::Tty => "tty",
            HandleType::Poll => "poll",
            HandleType::Prepare => "prepare",
            HandleType::Check => "check",
            HandleType::Idle => "idle",
            HandleType::Async => "async",
            HandleType::FsEvent => "fs_event",
            HandleType::FsPoll => "fs_poll",
            HandleType::Signal => "signal",
            HandleType::Process => "process",
            HandleType::Unknown => "unknown",
        }
    }
}

/// libuv handle information
#[derive(Debug, Clone)]
pub struct HandleInfo {
    /// Handle type
    pub handle_type: HandleType,
    /// Whether the handle is active
    pub is_active: bool,
    /// Whether the handle is referenced
    pub is_referenced: bool,
    /// Handle creation timestamp
    pub created_at: Instant,
}

/// libuv event loop metrics
#[derive(Debug, Clone)]
pub struct LoopMetrics {
    /// Number of active handles
    pub active_handles: u32,
    /// Number of active requests
    pub active_requests: u32,
    /// Event loop iteration count
    pub loop_count: u64,
    /// Total time spent in event loop
    pub loop_time: Duration,
    /// Average loop iteration time
    pub avg_loop_time: Duration,
    /// Maximum loop iteration time
    pub max_loop_time: Duration,
    /// Minimum loop iteration time
    pub min_loop_time: Duration,
    /// Event loop idle time
    pub idle_time: Duration,
    /// Event loop prepare time
    pub prepare_time: Duration,
    /// Event loop check time
    pub check_time: Duration,
    /// Event loop poll time
    pub poll_time: Duration,
}

/// libuv monitoring statistics
#[derive(Debug, Clone)]
pub struct LibuvStats {
    /// Handle counts by type
    pub handle_counts: HashMap<HandleType, u32>,
    /// Active handle counts by type
    pub active_handle_counts: HashMap<HandleType, u32>,
    /// Total number of handles
    pub total_handles: u32,
    /// Total number of active handles
    pub total_active_handles: u32,
    /// Event loop metrics
    pub loop_metrics: LoopMetrics,
    /// Recent handle activities
    pub recent_activities: Vec<String>,
}

/// libuv monitor implementation
pub struct LibuvMonitor {
    /// Handle information by ID
    handles: HashMap<u64, HandleInfo>,
    /// Handle ID counter
    next_handle_id: u64,
    /// Event loop metrics
    loop_metrics: LoopMetrics,
    /// Loop iteration times
    loop_times: Vec<Duration>,
    /// Maximum number of loop times to keep
    max_loop_times: usize,
    /// Whether monitoring is active
    is_monitoring: bool,
    /// Start time for monitoring
    start_time: Option<Instant>,
    /// Recent activities log
    recent_activities: Vec<String>,
    /// Maximum number of activities to keep
    max_activities: usize,
}

impl LibuvMonitor {
    /// Create a new libuv monitor
    pub fn new() -> Self {
        Self {
            handles: HashMap::new(),
            next_handle_id: 1,
            loop_metrics: LoopMetrics {
                active_handles: 0,
                active_requests: 0,
                loop_count: 0,
                loop_time: Duration::ZERO,
                avg_loop_time: Duration::ZERO,
                max_loop_time: Duration::ZERO,
                min_loop_time: Duration::MAX,
                idle_time: Duration::ZERO,
                prepare_time: Duration::ZERO,
                check_time: Duration::ZERO,
                poll_time: Duration::ZERO,
            },
            loop_times: Vec::new(),
            max_loop_times: 1000, // Keep last 1000 loop times
            is_monitoring: false,
            start_time: None,
            recent_activities: Vec::new(),
            max_activities: 100, // Keep last 100 activities
        }
    }
    
    /// Register a new handle
    pub fn register_handle(&mut self, handle_type: HandleType, is_active: bool, is_referenced: bool) -> u64 {
        if !self.is_monitoring {
            return 0;
        }
        
        let handle_id = self.next_handle_id;
        self.next_handle_id += 1;
        
        let handle_info = HandleInfo {
            handle_type,
            is_active,
            is_referenced,
            created_at: Instant::now(),
        };
        
        self.handles.insert(handle_id, handle_info);
        
        let activity = format!("Handle {} ({}) registered", handle_id, handle_type.as_str());
        self.add_activity(activity);
        
        handle_id
    }
    
    /// Unregister a handle
    pub fn unregister_handle(&mut self, handle_id: u64) {
        if !self.is_monitoring {
            return;
        }
        
        if let Some(handle_info) = self.handles.remove(&handle_id) {
            let activity = format!("Handle {} ({}) unregistered", handle_id, handle_info.handle_type.as_str());
            self.add_activity(activity);
        }
    }
    
    /// Update handle status
    pub fn update_handle_status(&mut self, handle_id: u64, is_active: bool, is_referenced: bool) {
        if !self.is_monitoring {
            return;
        }
        
        if let Some(handle_info) = self.handles.get_mut(&handle_id) {
            let old_active = handle_info.is_active;
            handle_info.is_active = is_active;
            handle_info.is_referenced = is_referenced;
            
            if old_active != is_active {
                let activity = format!(
                    "Handle {} ({}) status changed: active={}",
                    handle_id,
                    handle_info.handle_type.as_str(),
                    is_active
                );
                self.add_activity(activity);
            }
        }
    }
    
    /// Record event loop iteration
    pub fn record_loop_iteration(&mut self, iteration_time: Duration, idle_time: Duration, prepare_time: Duration, check_time: Duration, poll_time: Duration) {
        if !self.is_monitoring {
            return;
        }
        
        self.loop_metrics.loop_count += 1;
        self.loop_metrics.loop_time += iteration_time;
        self.loop_metrics.idle_time += idle_time;
        self.loop_metrics.prepare_time += prepare_time;
        self.loop_metrics.check_time += check_time;
        self.loop_metrics.poll_time += poll_time;
        
        // Update min/max times
        if iteration_time > self.loop_metrics.max_loop_time {
            self.loop_metrics.max_loop_time = iteration_time;
        }
        if iteration_time < self.loop_metrics.min_loop_time {
            self.loop_metrics.min_loop_time = iteration_time;
        }
        
        // Calculate average
        self.loop_metrics.avg_loop_time = self.loop_metrics.loop_time / self.loop_metrics.loop_count as u32;
        
        // Store iteration time
        self.loop_times.push(iteration_time);
        if self.loop_times.len() > self.max_loop_times {
            self.loop_times.remove(0);
        }
    }
    
    /// Update active handles and requests count
    pub fn update_active_counts(&mut self, active_handles: u32, active_requests: u32) {
        if !self.is_monitoring {
            return;
        }
        
        self.loop_metrics.active_handles = active_handles;
        self.loop_metrics.active_requests = active_requests;
    }
    
    /// Add activity to recent activities log
    fn add_activity(&mut self, activity: String) {
        self.recent_activities.push(activity);
        if self.recent_activities.len() > self.max_activities {
            self.recent_activities.remove(0);
        }
    }
    
    /// Get libuv statistics
    pub fn get_libuv_stats(&self) -> LibuvStats {
        let mut handle_counts = HashMap::new();
        let mut active_handle_counts = HashMap::new();
        
        for handle_info in self.handles.values() {
            *handle_counts.entry(handle_info.handle_type).or_insert(0) += 1;
            
            if handle_info.is_active {
                *active_handle_counts.entry(handle_info.handle_type).or_insert(0) += 1;
            }
        }
        
        let total_handles = self.handles.len() as u32;
        let total_active_handles = self.handles.values()
            .filter(|h| h.is_active)
            .count() as u32;
        
        LibuvStats {
            handle_counts,
            active_handle_counts,
            total_handles,
            total_active_handles,
            loop_metrics: self.loop_metrics.clone(),
            recent_activities: self.recent_activities.clone(),
        }
    }
    
    /// Format libuv statistics for logging
    pub fn format_libuv_stats(&self) -> String {
        let stats = self.get_libuv_stats();
        
        let timer_count = stats.handle_counts.get(&HandleType::Timer).unwrap_or(&0);
        let tcp_count = stats.handle_counts.get(&HandleType::Tcp).unwrap_or(&0);
        let udp_count = stats.handle_counts.get(&HandleType::Udp).unwrap_or(&0);
        let pipe_count = stats.handle_counts.get(&HandleType::Pipe).unwrap_or(&0);
        let fs_event_count = stats.handle_counts.get(&HandleType::FsEvent).unwrap_or(&0);
        
        format!(
            "libuv total_handles: {}, active_handles: {}, timer: {}, tcp: {}, udp: {}, \
             pipe: {}, fs_event: {}, loop_count: {}, avg_loop_time: {}μs, max_loop_time: {}μs",
            stats.total_handles,
            stats.total_active_handles,
            timer_count,
            tcp_count,
            udp_count,
            pipe_count,
            fs_event_count,
            stats.loop_metrics.loop_count,
            stats.loop_metrics.avg_loop_time.as_micros(),
            stats.loop_metrics.max_loop_time.as_micros()
        )
    }
    
    /// Clear all monitoring data
    pub fn clear_data(&mut self) {
        self.handles.clear();
        self.loop_times.clear();
        self.recent_activities.clear();
        self.loop_metrics = LoopMetrics {
            active_handles: 0,
            active_requests: 0,
            loop_count: 0,
            loop_time: Duration::ZERO,
            avg_loop_time: Duration::ZERO,
            max_loop_time: Duration::ZERO,
            min_loop_time: Duration::MAX,
            idle_time: Duration::ZERO,
            prepare_time: Duration::ZERO,
            check_time: Duration::ZERO,
            poll_time: Duration::ZERO,
        };
    }
}

impl Default for LibuvMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Monitor for LibuvMonitor {
    type Stats = LibuvStats;
    
    fn start(&mut self) -> MonitoringResult<()> {
        self.is_monitoring = true;
        self.start_time = Some(Instant::now());
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
        Ok(self.get_libuv_stats())
    }
    
    fn reset(&mut self) -> MonitoringResult<()> {
        self.clear_data();
        self.start_time = None;
        Ok(())
    }
    
    fn update(&mut self) -> MonitoringResult<()> {
        // libuv monitoring doesn't need periodic updates
        Ok(())
    }
    
    fn module_name(&self) -> &'static str {
        "libuv"
    }
}

/// Global libuv monitor instance
static LIBUV_MONITOR: Mutex<Option<LibuvMonitor>> = Mutex::new(None);

/// Initialize global libuv monitor
pub fn init_libuv_monitor() -> MonitoringResult<()> {
    let mut monitor = LIBUV_MONITOR.lock()
        .map_err(|_| MonitoringError::LockFailed {
            resource: "libuv monitor".to_string(),
            details: "Failed to acquire lock".to_string(),
        })?;
    *monitor = Some(LibuvMonitor::new());
    Ok(())
}

/// Register a new handle
pub fn register_handle(handle_type: HandleType, is_active: bool, is_referenced: bool) -> MonitoringResult<u64> {
    let mut monitor = LIBUV_MONITOR.lock()
        .map_err(|_| MonitoringError::LockFailed {
            resource: "libuv monitor".to_string(),
            details: "Failed to acquire lock".to_string(),
        })?;
    if let Some(ref mut libuv_monitor) = monitor.as_mut() {
        Ok(libuv_monitor.register_handle(handle_type, is_active, is_referenced))
    } else {
        Err(MonitoringError::NotInitialized {
            module: "libuv monitor".to_string(),
        })
    }
}

/// Unregister a handle
pub fn unregister_handle(handle_id: u64) -> MonitoringResult<()> {
    let mut monitor = LIBUV_MONITOR.lock()
        .map_err(|_| MonitoringError::LockFailed {
            resource: "libuv monitor".to_string(),
            details: "Failed to acquire lock".to_string(),
        })?;
    if let Some(ref mut libuv_monitor) = monitor.as_mut() {
        libuv_monitor.unregister_handle(handle_id);
        Ok(())
    } else {
        Err(MonitoringError::NotInitialized {
            module: "libuv monitor".to_string(),
        })
    }
}

/// Update handle status
pub fn update_handle_status(handle_id: u64, is_active: bool, is_referenced: bool) -> MonitoringResult<()> {
    let mut monitor = LIBUV_MONITOR.lock()
        .map_err(|_| MonitoringError::LockFailed {
            resource: "libuv monitor".to_string(),
            details: "Failed to acquire lock".to_string(),
        })?;
    if let Some(ref mut libuv_monitor) = monitor.as_mut() {
        libuv_monitor.update_handle_status(handle_id, is_active, is_referenced);
        Ok(())
    } else {
        Err(MonitoringError::NotInitialized {
            module: "libuv monitor".to_string(),
        })
    }
}

/// Record event loop iteration
pub fn record_loop_iteration(iteration_time: Duration, idle_time: Duration, prepare_time: Duration, check_time: Duration, poll_time: Duration) -> MonitoringResult<()> {
    let mut monitor = LIBUV_MONITOR.lock()
        .map_err(|_| MonitoringError::LockFailed {
            resource: "libuv monitor".to_string(),
            details: "Failed to acquire lock".to_string(),
        })?;
    if let Some(ref mut libuv_monitor) = monitor.as_mut() {
        libuv_monitor.record_loop_iteration(iteration_time, idle_time, prepare_time, check_time, poll_time);
        Ok(())
    } else {
        Err(MonitoringError::NotInitialized {
            module: "libuv monitor".to_string(),
        })
    }
}

/// Update active handles and requests count
pub fn update_active_counts(active_handles: u32, active_requests: u32) -> MonitoringResult<()> {
    let mut monitor = LIBUV_MONITOR.lock()
        .map_err(|_| MonitoringError::LockFailed {
            resource: "libuv monitor".to_string(),
            details: "Failed to acquire lock".to_string(),
        })?;
    if let Some(ref mut libuv_monitor) = monitor.as_mut() {
        libuv_monitor.update_active_counts(active_handles, active_requests);
        Ok(())
    } else {
        Err(MonitoringError::NotInitialized {
            module: "libuv monitor".to_string(),
        })
    }
}

/// Get libuv statistics
pub fn get_libuv_stats() -> MonitoringResult<LibuvStats> {
    let monitor = LIBUV_MONITOR.lock()
        .map_err(|_| MonitoringError::LockFailed {
            resource: "libuv monitor".to_string(),
            details: "Failed to acquire lock".to_string(),
        })?;
    monitor.as_ref()
        .map(|m| m.get_libuv_stats())
        .ok_or_else(|| MonitoringError::NotInitialized {
            module: "libuv monitor".to_string(),
        })
}

/// Format libuv statistics for logging
pub fn format_libuv_stats() -> String {
    let monitor = LIBUV_MONITOR.lock().unwrap();
    monitor
        .as_ref()
        .map(|m| m.format_libuv_stats())
        .unwrap_or_else(|| "libuv monitor not initialized".to_string())
}

/// Start libuv monitoring
pub fn start_libuv_monitoring() -> MonitoringResult<()> {
    let mut monitor = LIBUV_MONITOR.lock()
        .map_err(|_| MonitoringError::LockFailed {
            resource: "libuv monitor".to_string(),
            details: "Failed to acquire lock".to_string(),
        })?;
    if let Some(ref mut libuv_monitor) = monitor.as_mut() {
        libuv_monitor.start()?;
    }
    Ok(())
}

/// Stop libuv monitoring
pub fn stop_libuv_monitoring() -> MonitoringResult<()> {
    let mut monitor = LIBUV_MONITOR.lock()
        .map_err(|_| MonitoringError::LockFailed {
            resource: "libuv monitor".to_string(),
            details: "Failed to acquire lock".to_string(),
        })?;
    if let Some(ref mut libuv_monitor) = monitor.as_mut() {
        libuv_monitor.stop()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_libuv_monitor_creation() {
        let monitor = LibuvMonitor::new();
        assert!(!monitor.is_monitoring);
        assert_eq!(monitor.handles.len(), 0);
    }
    
    #[test]
    fn test_handle_registration() {
        let mut monitor = LibuvMonitor::new();
        monitor.start().unwrap();
        
        let handle_id = monitor.register_handle(HandleType::Timer, true, true);
        assert!(handle_id > 0);
        assert_eq!(monitor.handles.len(), 1);
        
        let stats = monitor.get_libuv_stats();
        assert_eq!(stats.total_handles, 1);
        assert_eq!(stats.total_active_handles, 1);
        assert_eq!(*stats.handle_counts.get(&HandleType::Timer).unwrap(), 1);
    }
    
    #[test]
    fn test_handle_type_conversion() {
        assert_eq!(HandleType::from_uv_type(1), HandleType::Timer);
        assert_eq!(HandleType::from_uv_type(2), HandleType::Tcp);
        assert_eq!(HandleType::Timer.as_str(), "timer");
    }
}