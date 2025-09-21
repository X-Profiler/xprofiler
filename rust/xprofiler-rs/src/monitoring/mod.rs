//! Monitoring modules for various system metrics
//!
//! This module provides monitoring capabilities for:
//! - CPU usage and performance metrics
//! - Memory usage and heap statistics
//! - Garbage collection events and statistics
//! - HTTP request/response monitoring
//! - libuv event loop monitoring

use std::time::Duration;

/// Common trait for all monitoring modules
pub trait Monitor {
    type Output;
    
    /// Start monitoring
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Stop monitoring
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Get current metrics
    fn get_metrics(&self) -> Self::Output;
    
    /// Reset monitoring data
    fn reset(&mut self);
}

/// Time period for averaging metrics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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