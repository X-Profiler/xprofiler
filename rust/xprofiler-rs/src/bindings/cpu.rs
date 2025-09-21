//! CPU monitoring NAPI bindings

use napi::bindgen_prelude::*;
use napi_derive::napi;
use crate::monitoring::cpu::{self, CpuUsage};
use crate::monitoring::Monitor;
use std::collections::HashMap;

/// CPU usage information for JavaScript
#[napi(object)]
pub struct JsCpuUsage {
    /// Current CPU usage percentage
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
    /// Timestamp when the measurement was taken
    pub timestamp: f64,
}

impl From<CpuUsage> for JsCpuUsage {
    fn from(usage: CpuUsage) -> Self {
        Self {
            current: usage.current,
            avg_15s: usage.avg_15s,
            avg_30s: usage.avg_30s,
            avg_1m: usage.avg_1m,
            avg_3m: usage.avg_3m,
            avg_5m: usage.avg_5m,
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
    cpu::start_cpu_monitor()
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to start CPU monitoring: {}", e)))
}

/// Stop CPU monitoring
#[napi]
pub fn stop_cpu_monitoring() -> Result<()> {
    cpu::stop_cpu_monitor()
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to stop CPU monitoring: {}", e)))
}

/// Get current CPU usage
#[napi]
pub fn get_cpu_usage() -> JsCpuUsage {
    let usage = cpu::get_cpu_usage();
    usage.into()
}

/// Get current CPU usage
#[napi]
pub fn get_current_cpu_usage() -> Result<JsCpuUsage> {
    let monitor = cpu::CPU_MONITOR.lock()
        .map_err(|_| Error::new(Status::GenericFailure, "Failed to lock CPU monitor".to_string()))?;
    let usage = (*monitor).get_stats();
    Ok(JsCpuUsage::from(usage))
}

/// Get CPU usage for a specific time period
#[napi]
pub fn get_cpu_usage_for_period(_period_seconds: u32) -> Result<JsCpuUsage> {
    let monitor = cpu::CPU_MONITOR.lock()
        .map_err(|_| Error::new(Status::GenericFailure, "Failed to lock CPU monitor".to_string()))?;
    let usage = (*monitor).get_stats();
    Ok(JsCpuUsage::from(usage))
}

/// Get formatted CPU usage string
#[napi]
pub fn format_cpu_usage() -> Result<String> {
    let monitor = cpu::CPU_MONITOR.lock()
        .map_err(|_| Error::new(Status::GenericFailure, "Failed to lock CPU monitor".to_string()))?;
    Ok(monitor.format_cpu_usage(false))
}

/// Update CPU usage (called periodically)
#[napi]
pub fn update_cpu_usage() -> Result<()> {
    let mut monitor = cpu::CPU_MONITOR.lock()
        .map_err(|_| Error::new(Status::GenericFailure, "Failed to lock CPU monitor".to_string()))?;
    monitor.update()
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to update CPU usage: {}", e)))?;
    Ok(())
}

/// Reset CPU monitoring data
#[napi]
pub fn reset_cpu_monitor() -> Result<()> {
    cpu::reset_cpu_monitor()
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to reset CPU monitor: {}", e)))
}

#[napi]
pub fn get_cpu_usage_history() -> HashMap<String, f64> {
    let mut result = HashMap::new();
    
    if let Ok(monitor) = cpu::CPU_MONITOR.lock() {
        let usage = (*monitor).get_stats();
        result.insert("15s".to_string(), usage.avg_15s);
        result.insert("30s".to_string(), usage.avg_30s);
        result.insert("1m".to_string(), usage.avg_1m);
        result.insert("3m".to_string(), usage.avg_3m);
        result.insert("5m".to_string(), usage.avg_5m);
        result.insert("10m".to_string(), usage.avg_5m);
    }
    
    result
}