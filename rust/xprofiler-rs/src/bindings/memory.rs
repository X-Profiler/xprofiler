//! Memory monitoring NAPI bindings

use napi::bindgen_prelude::*;
use napi_derive::napi;
use crate::monitoring::memory::{self, MemoryUsage, HeapStats};
use std::collections::HashMap;

/// Memory usage information for JavaScript
#[napi(object)]
pub struct JsMemoryUsage {
    /// Resident Set Size in bytes
    pub rss: f64,
    /// Virtual Memory Size in bytes
    pub vms: f64,
    /// Heap used in bytes
    pub heap_used: f64,
    /// Heap total in bytes
    pub heap_total: f64,
    /// External memory in bytes
    pub external: f64,
    /// Array buffers in bytes
    pub array_buffers: f64,
    /// Timestamp when the measurement was taken
    pub timestamp: f64,
}

impl From<MemoryUsage> for JsMemoryUsage {
    fn from(usage: MemoryUsage) -> Self {
        Self {
            rss: usage.rss as f64,
            vms: usage.vms as f64,
            heap_used: usage.heap_used as f64,
            heap_total: usage.heap_total as f64,
            external: usage.external as f64,
            array_buffers: usage.array_buffers as f64,
            timestamp: usage.timestamp as f64 * 1000.0,
        }
    }
}

/// Heap statistics for JavaScript
#[napi(object)]
pub struct JsHeapStats {
    /// Total heap size in bytes
    pub total_heap_size: f64,
    /// Total heap size executable in bytes
    pub total_heap_size_executable: f64,
    /// Total physical size in bytes
    pub total_physical_size: f64,
    /// Total available size in bytes
    pub total_available_size: f64,
    /// Used heap size in bytes
    pub used_heap_size: f64,
    /// Heap size limit in bytes
    pub heap_size_limit: f64,
    /// Malloced memory in bytes
    pub malloced_memory: f64,
    /// Peak malloced memory in bytes
    pub peak_malloced_memory: f64,
    /// External memory in bytes
    pub external_memory: f64,
}

impl From<HeapStats> for JsHeapStats {
    fn from(stats: HeapStats) -> Self {
        Self {
            total_heap_size: stats.total_heap_size as f64,
            total_heap_size_executable: stats.total_heap_size_executable as f64,
            total_physical_size: stats.total_physical_size as f64,
            total_available_size: stats.total_available_size as f64,
            used_heap_size: stats.used_heap_size as f64,
            heap_size_limit: stats.heap_size_limit as f64,
            malloced_memory: stats.malloced_memory as f64,
            peak_malloced_memory: stats.peak_malloced_memory as f64,
            external_memory: stats.external_memory as f64,
        }
    }
}

/// Initialize memory monitoring
#[napi]
pub fn init_memory_monitor() -> Result<()> {
    memory::init_memory_monitor()
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to initialize memory monitor: {}", e)))
}

/// Start memory monitoring
#[napi]
pub fn start_memory_monitoring() -> Result<()> {
    memory::start_memory_monitoring()
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to start memory monitoring: {}", e)))
}

/// Stop memory monitoring
#[napi]
pub fn stop_memory_monitoring() -> Result<()> {
    memory::stop_memory_monitoring()
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to stop memory monitoring: {}", e)))
}

/// Get current memory usage
#[napi]
pub fn get_memory_usage() -> Option<JsMemoryUsage> {
    memory::get_memory_usage().map(|usage| usage.into())
}

/// Get memory usage for a specific time period
#[napi]
pub fn get_memory_usage_for_period(period_seconds: u32) -> Option<JsMemoryUsage> {
    use crate::monitoring::TimePeriod;
    
    let period = match period_seconds {
        10 => TimePeriod::TenSeconds,
        30 => TimePeriod::ThirtySeconds,
        60 => TimePeriod::OneMinute,
        120 => TimePeriod::TwoMinutes,
        300 => TimePeriod::FiveMinutes,
        _ => return None,
    };
    
    memory::get_memory_usage_for_period(period).map(|usage| usage.into())
}

/// Get current heap statistics
#[napi]
pub fn get_heap_stats() -> Option<JsHeapStats> {
    memory::get_heap_stats().map(|stats| stats.into())
}

/// Get formatted memory usage string
#[napi]
pub fn format_memory_usage() -> String {
    memory::format_memory_usage()
}

/// Update memory usage (called periodically)
#[napi]
pub fn update_memory_usage() -> Result<()> {
    let mut monitor = memory::MEMORY_MONITOR.lock()
        .map_err(|_| Error::new(Status::GenericFailure, "Failed to lock memory monitor".to_string()))?;
    monitor.update()
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to update memory usage: {}", e)))?;
    Ok(())
}

/// Reset memory monitoring data
#[napi]
pub fn reset_memory_monitor() -> Result<()> {
    memory::reset_memory_monitor()
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to reset memory monitor: {}", e)))
}

/// Get memory usage history for all time periods
#[napi]
pub fn get_memory_usage_history() -> HashMap<String, Option<JsMemoryUsage>> {
    let mut history = HashMap::new();
    
    if let Some(usage) = memory::get_memory_usage_for_period(crate::monitoring::TimePeriod::OneMinute) {
        history.insert("1m".to_string(), Some(usage.into()));
    } else {
        history.insert("1m".to_string(), None);
    }
    
    if let Some(usage) = memory::get_memory_usage_for_period(crate::monitoring::TimePeriod::FiveMinutes) {
        history.insert("5m".to_string(), Some(usage.into()));
    } else {
        history.insert("5m".to_string(), None);
    }
    
    if let Some(usage) = memory::get_memory_usage_for_period(crate::monitoring::TimePeriod::TenSeconds) {
        history.insert("10s".to_string(), Some(usage.into()));
    } else {
        history.insert("10s".to_string(), None);
    }
    
    if let Some(usage) = memory::get_memory_usage_for_period(crate::monitoring::TimePeriod::ThirtySeconds) {
        history.insert("30s".to_string(), Some(usage.into()));
    } else {
        history.insert("30s".to_string(), None);
    }
    
    if let Some(usage) = memory::get_memory_usage_for_period(crate::monitoring::TimePeriod::TwoMinutes) {
        history.insert("2m".to_string(), Some(usage.into()));
    } else {
        history.insert("2m".to_string(), None);
    }
    
    history
}

/// Get memory usage in MB for easier reading
#[napi]
pub fn get_memory_usage_mb() -> Option<HashMap<String, f64>> {
    memory::get_memory_usage().map(|usage| {
        let mut result = HashMap::new();
        result.insert("rss".to_string(), usage.rss as f64 / 1024.0 / 1024.0);
        result.insert("vms".to_string(), usage.vms as f64 / 1024.0 / 1024.0);
        result.insert("heap_used".to_string(), usage.heap_used as f64 / 1024.0 / 1024.0);
        result.insert("heap_total".to_string(), usage.heap_total as f64 / 1024.0 / 1024.0);
        result.insert("external".to_string(), usage.external as f64 / 1024.0 / 1024.0);
        result
    })
}

/// Get heap statistics in MB for easier reading
#[napi]
pub fn get_heap_stats_mb() -> Option<HashMap<String, f64>> {
    memory::get_heap_stats().map(|stats| {
        let mut result = HashMap::new();
        result.insert("total_heap_size".to_string(), stats.total_heap_size as f64 / 1024.0 / 1024.0);
        result.insert("used_heap_size".to_string(), stats.used_heap_size as f64 / 1024.0 / 1024.0);
        result.insert("heap_size_limit".to_string(), stats.heap_size_limit as f64 / 1024.0 / 1024.0);
        result.insert("external_memory".to_string(), stats.external_memory as f64 / 1024.0 / 1024.0);
        result
    })
}