//! CPU monitoring NAPI bindings

use napi::bindgen_prelude::*;
use napi_derive::napi;
use crate::monitoring::cpu::{self, CpuUsage};
use std::collections::HashMap;

/// CPU usage information for JavaScript
#[napi(object)]
pub struct JsCpuUsage {
    /// Process CPU usage percentage
    pub process_cpu: f64,
    /// System CPU usage percentage
    pub system_cpu: f64,
    /// User CPU time in milliseconds
    pub user_time: f64,
    /// System CPU time in milliseconds
    pub sys_time: f64,
    /// Total CPU time in milliseconds
    pub total_time: f64,
    /// Timestamp when the measurement was taken
    pub timestamp: f64,
}

impl From<CpuUsage> for JsCpuUsage {
    fn from(usage: CpuUsage) -> Self {
        Self {
            process_cpu: usage.process_cpu,
            system_cpu: usage.system_cpu,
            user_time: usage.user_time.as_secs_f64() * 1000.0,
            sys_time: usage.sys_time.as_secs_f64() * 1000.0,
            total_time: usage.total_time.as_secs_f64() * 1000.0,
            timestamp: usage.timestamp.elapsed().as_secs_f64() * 1000.0,
        }
    }
}

/// Initialize CPU monitoring
#[napi]
pub fn init_cpu_monitor() -> Result<()> {
    cpu::init_cpu_monitor()
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to initialize CPU monitor: {}", e)))
}

/// Start CPU monitoring
#[napi]
pub fn start_cpu_monitoring() -> Result<()> {
    cpu::start_cpu_monitoring()
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to start CPU monitoring: {}", e)))
}

/// Stop CPU monitoring
#[napi]
pub fn stop_cpu_monitoring() -> Result<()> {
    cpu::stop_cpu_monitoring()
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to stop CPU monitoring: {}", e)))
}

/// Get current CPU usage
#[napi]
pub fn get_cpu_usage() -> Option<JsCpuUsage> {
    cpu::get_cpu_usage().map(|usage| usage.into())
}

/// Get CPU usage for a specific time period
#[napi]
pub fn get_cpu_usage_for_period(period_seconds: u32) -> Option<JsCpuUsage> {
    use crate::monitoring::TimePeriod;
    
    let period = match period_seconds {
        60 => TimePeriod::OneMinute,
        300 => TimePeriod::FiveMinutes,
        900 => TimePeriod::FifteenMinutes,
        1800 => TimePeriod::ThirtyMinutes,
        3600 => TimePeriod::OneHour,
        _ => return None,
    };
    
    cpu::get_cpu_usage_for_period(period).map(|usage| usage.into())
}

/// Get formatted CPU usage string
#[napi]
pub fn format_cpu_usage() -> String {
    cpu::format_cpu_usage()
}

/// Update CPU usage (called periodically)
#[napi]
pub fn update_cpu_usage() -> Result<()> {
    cpu::update_cpu_usage()
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to update CPU usage: {}", e)))
}

/// Reset CPU monitoring data
#[napi]
pub fn reset_cpu_monitor() -> Result<()> {
    cpu::reset_cpu_monitor()
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to reset CPU monitor: {}", e)))
}

/// Get CPU usage history for all time periods
#[napi]
pub fn get_cpu_usage_history() -> HashMap<String, Option<JsCpuUsage>> {
    let mut history = HashMap::new();
    
    if let Some(usage) = cpu::get_cpu_usage_for_period(crate::monitoring::TimePeriod::OneMinute) {
        history.insert("1m".to_string(), Some(usage.into()));
    } else {
        history.insert("1m".to_string(), None);
    }
    
    if let Some(usage) = cpu::get_cpu_usage_for_period(crate::monitoring::TimePeriod::FiveMinutes) {
        history.insert("5m".to_string(), Some(usage.into()));
    } else {
        history.insert("5m".to_string(), None);
    }
    
    if let Some(usage) = cpu::get_cpu_usage_for_period(crate::monitoring::TimePeriod::FifteenMinutes) {
        history.insert("15m".to_string(), Some(usage.into()));
    } else {
        history.insert("15m".to_string(), None);
    }
    
    if let Some(usage) = cpu::get_cpu_usage_for_period(crate::monitoring::TimePeriod::ThirtyMinutes) {
        history.insert("30m".to_string(), Some(usage.into()));
    } else {
        history.insert("30m".to_string(), None);
    }
    
    if let Some(usage) = cpu::get_cpu_usage_for_period(crate::monitoring::TimePeriod::OneHour) {
        history.insert("1h".to_string(), Some(usage.into()));
    } else {
        history.insert("1h".to_string(), None);
    }
    
    history
}