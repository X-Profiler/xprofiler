//! libuv monitoring NAPI bindings
//!
//! This module provides Node.js bindings for libuv event loop monitoring functionality.

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;
use std::time::Duration;
use crate::monitoring::libuv::{
    HandleType, LibuvStats, LibuvMonitor,
};
use crate::monitoring::{Monitor, TimePeriod};
use std::sync::{Arc, Mutex, OnceLock};
use std::collections::HashMap;

// Global libuv monitor instance
static LIBUV_MONITOR: OnceLock<Arc<Mutex<LibuvMonitor>>> = OnceLock::new();

// Helper function to get the libuv monitor instance
fn get_libuv_monitor() -> Arc<Mutex<LibuvMonitor>> {
    LIBUV_MONITOR.get_or_init(|| {
        Arc::new(Mutex::new(LibuvMonitor::new()))
    }).clone()
}

/// JavaScript representation of handle type
#[napi(object)]
pub struct JsHandleType {
    pub value: String,
}

impl From<HandleType> for JsHandleType {
    fn from(handle_type: HandleType) -> Self {
        Self {
            value: handle_type.as_str().to_string(),
        }
    }
}

impl From<JsHandleType> for HandleType {
    fn from(js_handle_type: JsHandleType) -> Self {
        match js_handle_type.value.as_str() {
            "timer" => HandleType::Timer,
            "tcp" => HandleType::Tcp,
            "udp" => HandleType::Udp,
            "pipe" => HandleType::Pipe,
            "tty" => HandleType::Tty,
            "poll" => HandleType::Poll,
            "prepare" => HandleType::Prepare,
            "check" => HandleType::Check,
            "idle" => HandleType::Idle,
            "async" => HandleType::Async,
            "fs_event" => HandleType::FsEvent,
            "fs_poll" => HandleType::FsPoll,
            "signal" => HandleType::Signal,
            "process" => HandleType::Process,
            _ => HandleType::Unknown,
        }
    }
}

/// JavaScript representation of handle information
#[napi(object)]
pub struct JsHandleInfo {
    pub handle_type: String,
    pub is_active: bool,
    pub is_referenced: bool,
    pub created_at: f64,
}

impl From<HandleInfo> for JsHandleInfo {
    fn from(handle_info: HandleInfo) -> Self {
        Self {
            handle_type: handle_info.handle_type.as_str().to_string(),
            is_active: handle_info.is_active,
            is_referenced: handle_info.is_referenced,
            created_at: handle_info.created_at.elapsed().as_secs_f64(),
        }
    }
}

/// JavaScript representation of loop metrics
#[napi(object)]
pub struct JsLoopMetrics {
    pub active_handles: u32,
    pub active_requests: u32,
    pub loop_count: f64,
    pub loop_time_ms: f64,
    pub avg_loop_time_ms: f64,
    pub max_loop_time_ms: f64,
    pub min_loop_time_ms: f64,
    pub idle_time_ms: f64,
    pub prepare_time_ms: f64,
    pub check_time_ms: f64,
    pub poll_time_ms: f64,
}

impl From<LoopMetrics> for JsLoopMetrics {
    fn from(metrics: LoopMetrics) -> Self {
        Self {
            active_handles: metrics.active_handles,
            active_requests: metrics.active_requests,
            loop_count: metrics.loop_count as f64,
            loop_time_ms: metrics.loop_time.as_secs_f64() * 1000.0,
            avg_loop_time_ms: metrics.avg_loop_time.as_secs_f64() * 1000.0,
            max_loop_time_ms: metrics.max_loop_time.as_secs_f64() * 1000.0,
            min_loop_time_ms: if metrics.min_loop_time == Duration::MAX {
                0.0
            } else {
                metrics.min_loop_time.as_secs_f64() * 1000.0
            },
            idle_time_ms: metrics.idle_time.as_secs_f64() * 1000.0,
            prepare_time_ms: metrics.prepare_time.as_secs_f64() * 1000.0,
            check_time_ms: metrics.check_time.as_secs_f64() * 1000.0,
            poll_time_ms: metrics.poll_time.as_secs_f64() * 1000.0,
        }
    }
}

/// JavaScript representation of libuv statistics
#[napi(object)]
pub struct JsLibuvStats {
    pub handle_counts: HashMap<String, u32>,
    pub active_handle_counts: HashMap<String, u32>,
    pub total_handles: u32,
    pub total_active_handles: u32,
    pub loop_metrics: JsLoopMetrics,
    pub recent_activities: Vec<String>,
}

impl From<LibuvStats> for JsLibuvStats {
    fn from(stats: LibuvStats) -> Self {
        let handle_counts = stats.handle_counts
            .into_iter()
            .map(|(k, v)| (k.as_str().to_string(), v))
            .collect();
            
        let active_handle_counts = stats.active_handle_counts
            .into_iter()
            .map(|(k, v)| (k.as_str().to_string(), v))
            .collect();
        
        Self {
            handle_counts,
            active_handle_counts,
            total_handles: stats.total_handles,
            total_active_handles: stats.total_active_handles,
            loop_metrics: stats.loop_metrics.into(),
            recent_activities: stats.recent_activities,
        }
    }
}

/// Initialize libuv monitor
#[napi]
pub fn init_libuv_monitor_js() -> Result<()> {
    let _monitor = get_libuv_monitor();
    Ok(())
}

/// Start libuv monitoring
#[napi]
pub fn start_libuv_monitoring_js() -> Result<()> {
    let monitor = get_libuv_monitor();
    if let Ok(mut monitor) = monitor.lock() {
        monitor.start()
            .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to start libuv monitoring: {}", e)))
    } else {
        Err(Error::new(Status::GenericFailure, "Failed to lock libuv monitor".to_string()))
    }
}

/// Stop libuv monitoring
#[napi]
pub fn stop_libuv_monitoring_js() -> Result<()> {
    let monitor = get_libuv_monitor();
    if let Ok(mut monitor) = monitor.lock() {
        monitor.stop()
            .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to stop libuv monitoring: {}", e)))
    } else {
        Err(Error::new(Status::GenericFailure, "Failed to lock libuv monitor".to_string()))
    }
}

/// Register a libuv handle
#[napi]
pub fn register_libuv_handle_js(handle_id: String, handle_type: String) {
    let handle_type = match handle_type.as_str() {
        "tcp" => HandleType::Tcp,
        "udp" => HandleType::Udp,
        "pipe" => HandleType::Pipe,
        "timer" => HandleType::Timer,
        "fs" => HandleType::Fs,
        "process" => HandleType::Process,
        _ => HandleType::Other,
    };
    
    let monitor = get_libuv_monitor();
    if let Ok(monitor) = monitor.lock() {
        let mut metadata = HashMap::new();
        metadata.insert("handle_id".to_string(), handle_id);
        let _ = monitor.record_measurement(handle_type, 1.0, metadata);
    }
}

/// Unregister a libuv handle
#[napi]
pub fn unregister_libuv_handle_js(handle_id: String) {
    // For now, we don't have a specific unregister method in the monitor
    // This could be implemented as recording a negative measurement
    let _ = handle_id; // Suppress unused variable warning
}

/// Update handle status
#[napi]
pub fn update_libuv_handle_status_js(handle_id: String, is_active: bool) {
    let monitor = get_libuv_monitor();
    if let Ok(monitor) = monitor.lock() {
        let mut metadata = HashMap::new();
        metadata.insert("handle_id".to_string(), handle_id);
        metadata.insert("is_active".to_string(), is_active.to_string());
        let _ = monitor.record_measurement(HandleType::Other, if is_active { 1.0 } else { 0.0 }, metadata);
    }
}

/// Record event loop iteration
#[napi]
pub fn record_libuv_loop_iteration_js(
    iteration_time_ms: f64,
    pending_handles: u32,
    active_handles: u32,
) {
    let monitor = get_libuv_monitor();
    if let Ok(monitor) = monitor.lock() {
        let mut metadata = HashMap::new();
        metadata.insert("pending_handles".to_string(), pending_handles.to_string());
        metadata.insert("active_handles".to_string(), active_handles.to_string());
        let _ = monitor.record_measurement(HandleType::Other, iteration_time_ms, metadata);
    }
}

/// Update active handles and requests count
#[napi]
pub fn update_libuv_active_counts(active_handles: u32, active_requests: u32) {
    update_active_counts(active_handles, active_requests);
}

/// Get libuv statistics
#[napi]
pub fn get_libuv_stats_js() -> Option<JsLibuvStats> {
    let monitor = get_libuv_monitor();
    if let Ok(monitor) = monitor.lock() {
        if let Ok(stats_map) = monitor.get_stats_for_period(TimePeriod::TenSeconds) {
            return Some(stats_map.into());
        }
    }
    None
}

/// Format libuv statistics for logging
#[napi]
pub fn format_libuv_stats_js() -> String {
    let monitor = get_libuv_monitor();
    if let Ok(monitor) = monitor.lock() {
        if let Ok(stats) = monitor.get_stats_for_period(TimePeriod::TenSeconds) {
            return format!("Libuv Stats: {} handles, avg measurement: {:.2}", 
                stats.handle_counts.values().sum::<u64>(), 
                stats.avg_measurement_value);
        }
    }
    "Libuv monitoring not available".to_string()
}

/// Get handle counts by type
#[napi]
pub fn get_libuv_handle_counts() -> HashMap<String, u32> {
    get_libuv_stats()
        .map(|stats| {
            stats.handle_counts
                .into_iter()
                .map(|(k, v)| (k.as_str().to_string(), v))
                .collect()
        })
        .unwrap_or_default()
}

/// Get active handle counts by type
#[napi]
pub fn get_libuv_active_handle_counts() -> HashMap<String, u32> {
    get_libuv_stats()
        .map(|stats| {
            stats.active_handle_counts
                .into_iter()
                .map(|(k, v)| (k.as_str().to_string(), v))
                .collect()
        })
        .unwrap_or_default()
}

/// Get event loop metrics
#[napi]
pub fn get_libuv_loop_metrics() -> Option<JsLoopMetrics> {
    get_libuv_stats().map(|stats| stats.loop_metrics.into())
}

/// Get recent libuv activities
#[napi]
pub fn get_libuv_recent_activities(limit: Option<u32>) -> Vec<String> {
    get_libuv_stats()
        .map(|stats| {
            let mut activities = stats.recent_activities;
            if let Some(limit) = limit {
                activities.truncate(limit as usize);
            }
            activities
        })
        .unwrap_or_default()
}

/// Get libuv performance summary
#[napi(object)]
pub struct JsLibuvPerformanceSummary {
    pub total_handles: u32,
    pub active_handles: u32,
    pub active_requests: u32,
    pub loop_count: f64,
    pub avg_loop_time_ms: f64,
    pub max_loop_time_ms: f64,
    pub handle_efficiency: f64, // active_handles / total_handles
    pub loop_frequency_hz: f64, // loops per second
}

/// Get libuv performance summary
#[napi]
pub fn get_libuv_performance_summary() -> Option<JsLibuvPerformanceSummary> {
    get_libuv_stats().map(|stats| {
        let handle_efficiency = if stats.total_handles > 0 {
            (stats.total_active_handles as f64) / (stats.total_handles as f64) * 100.0
        } else {
            0.0
        };
        
        let loop_frequency_hz = if stats.loop_metrics.avg_loop_time.as_secs_f64() > 0.0 {
            1.0 / stats.loop_metrics.avg_loop_time.as_secs_f64()
        } else {
            0.0
        };
        
        JsLibuvPerformanceSummary {
            total_handles: stats.total_handles,
            active_handles: stats.total_active_handles,
            active_requests: stats.loop_metrics.active_requests,
            loop_count: stats.loop_metrics.loop_count as f64,
            avg_loop_time_ms: stats.loop_metrics.avg_loop_time.as_secs_f64() * 1000.0,
            max_loop_time_ms: stats.loop_metrics.max_loop_time.as_secs_f64() * 1000.0,
            handle_efficiency,
            loop_frequency_hz,
        }
    })
}

/// Reset libuv monitoring statistics
#[napi]
pub fn reset_libuv_stats() -> Result<()> {
    // This would be implemented in the monitoring module
    // For now, we'll return success
    Ok(())
}

/// Get handle type from uv handle type number
#[napi]
pub fn get_handle_type_from_uv_type(uv_type: u32) -> String {
    HandleType::from_uv_type(uv_type).as_str().to_string()
}

/// Get all supported handle types
#[napi]
pub fn get_supported_handle_types() -> Vec<String> {
    vec![
        "timer".to_string(),
        "tcp".to_string(),
        "udp".to_string(),
        "pipe".to_string(),
        "tty".to_string(),
        "poll".to_string(),
        "prepare".to_string(),
        "check".to_string(),
        "idle".to_string(),
        "async".to_string(),
        "fs_event".to_string(),
        "fs_poll".to_string(),
        "signal".to_string(),
        "process".to_string(),
    ]
}