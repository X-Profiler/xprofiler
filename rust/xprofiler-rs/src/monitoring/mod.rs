//! Monitoring modules for various system metrics
//!
//! This module provides monitoring capabilities for:
//! - CPU usage and performance metrics
//! - Memory usage and heap statistics
//! - Garbage collection events and statistics
//! - HTTP request/response monitoring
//! - libuv event loop monitoring

use std::time::Duration;

// Export error handling
pub mod error;
pub use error::{MonitoringError, MonitoringResult, IntoMonitoringError};

/// Common trait for all monitoring modules
pub trait Monitor {
    type Stats;
    
    /// Start monitoring
    fn start(&mut self) -> MonitoringResult<()>;
    
    /// Stop monitoring
    fn stop(&mut self) -> MonitoringResult<()>;
    
    /// Check if monitoring is running
    fn is_running(&self) -> bool;
    
    /// Get current statistics
    fn get_stats(&self) -> MonitoringResult<Self::Stats>;
    
    /// Reset monitoring data
    fn reset(&mut self) -> MonitoringResult<()>;
    
    /// Update monitoring data (optional, for modules that need periodic updates)
    fn update(&mut self) -> MonitoringResult<()> {
        Ok(())
    }
    
    /// Get module name for error reporting
    fn module_name(&self) -> &'static str;
}

/// Time period for averaging metrics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimePeriod {
    /// 10 seconds
    TenSeconds,
    /// 30 seconds
    ThirtySeconds,
    /// 60 seconds
    OneMinute,
    /// 120 seconds
    TwoMinutes,
    /// 300 seconds (5 minutes)
    FiveMinutes,
}

impl TimePeriod {
    /// Get duration for the time period
    pub fn duration(&self) -> Duration {
        match self {
            TimePeriod::TenSeconds => Duration::from_secs(10),
            TimePeriod::ThirtySeconds => Duration::from_secs(30),
            TimePeriod::OneMinute => Duration::from_secs(60),
            TimePeriod::TwoMinutes => Duration::from_secs(120),
            TimePeriod::FiveMinutes => Duration::from_secs(300),
        }
    }
    
    /// Get duration in seconds
    pub fn as_seconds(&self) -> u64 {
        match self {
            TimePeriod::TenSeconds => 10,
            TimePeriod::ThirtySeconds => 30,
            TimePeriod::OneMinute => 60,
            TimePeriod::TwoMinutes => 120,
            TimePeriod::FiveMinutes => 300,
        }
    }
    
    /// Get all available time periods
    pub fn all() -> Vec<TimePeriod> {
        vec![
            TimePeriod::TenSeconds,
            TimePeriod::ThirtySeconds,
            TimePeriod::OneMinute,
            TimePeriod::TwoMinutes,
            TimePeriod::FiveMinutes,
        ]
    }
}

// Export monitoring modules
pub mod cpu;
pub mod memory;
pub mod gc;
pub mod http;
pub mod libuv;