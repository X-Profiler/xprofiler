//! GC monitoring NAPI bindings

use napi::bindgen_prelude::*;
use napi_derive::napi;
use crate::monitoring::gc::{self, GcType, GcEvent, GcStats};
use std::collections::HashMap;
use std::time::Duration;

/// GC type for JavaScript
#[napi]
pub enum JsGcType {
    Scavenge,
    MarkSweepCompact,
    IncrementalMarking,
    ProcessWeakCallbacks,
    All,
}

impl From<GcType> for JsGcType {
    fn from(gc_type: GcType) -> Self {
        match gc_type {
            GcType::Scavenge => JsGcType::Scavenge,
            GcType::MarkSweepCompact => JsGcType::MarkSweepCompact,
            GcType::IncrementalMarking => JsGcType::IncrementalMarking,
            GcType::ProcessWeakCallbacks => JsGcType::ProcessWeakCallbacks,
            GcType::All => JsGcType::All,
        }
    }
}

impl From<JsGcType> for GcType {
    fn from(js_gc_type: JsGcType) -> Self {
        match js_gc_type {
            JsGcType::Scavenge => GcType::Scavenge,
            JsGcType::MarkSweepCompact => GcType::MarkSweepCompact,
            JsGcType::IncrementalMarking => GcType::IncrementalMarking,
            JsGcType::ProcessWeakCallbacks => GcType::ProcessWeakCallbacks,
            JsGcType::All => GcType::All,
        }
    }
}

/// GC event for JavaScript
#[napi(object)]
pub struct JsGcEvent {
    /// Type of GC
    pub gc_type: String,
    /// Duration in milliseconds
    pub duration: f64,
    /// Timestamp in milliseconds since epoch
    pub timestamp: f64,
    /// Heap size before GC in bytes
    pub heap_size_before: f64,
    /// Heap size after GC in bytes
    pub heap_size_after: f64,
}

impl From<GcEvent> for JsGcEvent {
    fn from(event: GcEvent) -> Self {
        Self {
            gc_type: event.gc_type.as_str().to_string(),
            duration: event.duration.as_secs_f64() * 1000.0,
            timestamp: event.timestamp.elapsed().as_secs_f64() * 1000.0,
            heap_size_before: event.heap_size_before as f64,
            heap_size_after: event.heap_size_after as f64,
        }
    }
}

/// GC statistics for JavaScript
#[napi(object)]
pub struct JsGcStats {
    /// Total GC count
    pub total_gc_count: u32,
    /// Total GC time in milliseconds
    pub total_gc_time: f64,
    /// GC counts by type
    pub gc_counts: HashMap<String, u32>,
    /// GC durations by type in milliseconds
    pub gc_durations: HashMap<String, f64>,
    /// Average GC durations by type in milliseconds
    pub gc_avg_durations: HashMap<String, f64>,
    /// Recent GC events (last 100)
    pub recent_events: Vec<JsGcEvent>,
}

impl From<GcStats> for JsGcStats {
    fn from(stats: GcStats) -> Self {
        let mut gc_counts = HashMap::new();
        let mut gc_durations = HashMap::new();
        let mut gc_avg_durations = HashMap::new();
        
        for (gc_type, count) in stats.gc_counts {
            gc_counts.insert(gc_type.as_str().to_string(), count);
        }
        
        for (gc_type, duration) in stats.gc_durations {
            gc_durations.insert(gc_type.as_str().to_string(), duration.as_secs_f64() * 1000.0);
        }
        
        for (gc_type, duration) in stats.gc_avg_durations {
            gc_avg_durations.insert(gc_type.as_str().to_string(), duration.as_secs_f64() * 1000.0);
        }
        
        let recent_events = stats.recent_events.into_iter().map(|event| event.into()).collect();
        
        Self {
            total_gc_count: stats.total_gc_count,
            total_gc_time: stats.total_gc_time.as_secs_f64() * 1000.0,
            gc_counts,
            gc_durations,
            gc_avg_durations,
            recent_events,
        }
    }
}

/// Initialize GC monitoring
#[napi]
pub fn init_gc_monitor() -> Result<()> {
    gc::init_gc_monitor()
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to initialize GC monitor: {}", e)))
}

/// Start GC monitoring
#[napi]
pub fn start_gc_monitoring() -> Result<()> {
    gc::start_gc_monitoring()
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to start GC monitoring: {}", e)))
}

/// Stop GC monitoring
#[napi]
pub fn stop_gc_monitoring() -> Result<()> {
    gc::stop_gc_monitoring()
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to stop GC monitoring: {}", e)))
}

/// Record a GC event
#[napi]
pub fn record_gc_event(
    gc_type: u32,
    duration_ms: f64,
    heap_before: f64,
    heap_after: f64,
) {
    let gc_type = GcType::from_v8_type(gc_type);
    let duration = Duration::from_secs_f64(duration_ms / 1000.0);
    gc::record_gc_event(gc_type, duration, heap_before as u64, heap_after as u64);
}

/// Get total GC count
#[napi]
pub fn get_total_gc_count() -> u32 {
    gc::get_total_gc_count()
}

/// Get GC statistics
#[napi]
pub fn get_gc_stats() -> Option<JsGcStats> {
    gc::get_gc_stats().map(|stats| stats.into())
}

/// Format GC statistics for logging
#[napi]
pub fn format_gc_stats() -> String {
    gc::format_gc_stats()
}

/// Get GC count by type
#[napi]
pub fn get_gc_count_by_type(gc_type_str: String) -> u32 {
    let gc_type = match gc_type_str.as_str() {
        "scavenge" => GcType::Scavenge,
        "mark_sweep_compact" => GcType::MarkSweepCompact,
        "incremental_marking" => GcType::IncrementalMarking,
        "process_weak_callbacks" => GcType::ProcessWeakCallbacks,
        "all" => GcType::All,
        _ => return 0,
    };
    
    // This would need to be implemented in the gc module
    // For now, return 0
    0
}

/// Get GC statistics summary for easy consumption
#[napi]
pub fn get_gc_summary() -> HashMap<String, f64> {
    let mut summary = HashMap::new();
    
    if let Some(stats) = gc::get_gc_stats() {
        summary.insert("total_count".to_string(), stats.total_gc_count as f64);
        summary.insert("total_time_ms".to_string(), stats.total_gc_time.as_secs_f64() * 1000.0);
        
        // Add individual GC type counts
        for (gc_type, count) in stats.gc_counts {
            summary.insert(format!("{}_count", gc_type.as_str()), count as f64);
        }
        
        // Add individual GC type durations
        for (gc_type, duration) in stats.gc_durations {
            summary.insert(format!("{}_time_ms", gc_type.as_str()), duration.as_secs_f64() * 1000.0);
        }
        
        // Add average durations
        for (gc_type, duration) in stats.gc_avg_durations {
            summary.insert(format!("{}_avg_ms", gc_type.as_str()), duration.as_secs_f64() * 1000.0);
        }
    }
    
    summary
}

/// Get recent GC events (last N events)
#[napi]
pub fn get_recent_gc_events(count: Option<u32>) -> Vec<JsGcEvent> {
    let count = count.unwrap_or(10) as usize;
    
    if let Some(stats) = gc::get_gc_stats() {
        let events = if stats.recent_events.len() > count {
            &stats.recent_events[stats.recent_events.len() - count..]
        } else {
            &stats.recent_events
        };
        
        events.iter().map(|event| event.clone().into()).collect()
    } else {
        Vec::new()
    }
}

/// Clear all GC monitoring data
#[napi]
pub fn clear_gc_data() -> Result<()> {
    // This would need to be implemented in the gc module
    Ok(())
}

/// Check if GC monitoring is active
#[napi]
pub fn is_gc_monitoring_active() -> bool {
    // This would need to be implemented in the gc module
    // For now, return false
    false
}