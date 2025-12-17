//! GC Profiler
//!
//! Collects detailed GC information and writes it to a JSON file.

use super::{clear_action, set_action_running, ProfileAction};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};

/// GC profile data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcProfile {
    #[serde(rename = "startTime")]
    pub start_time: f64,
    pub gc: Vec<GcEvent>,
    #[serde(rename = "stopTime")]
    pub stop_time: f64,
}

/// Individual GC event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcEvent {
    #[serde(rename = "totalSpentfromStart")]
    pub total_spent_from_start: f64,
    #[serde(rename = "totalTimesfromStart")]
    pub total_times_from_start: u64,
    #[serde(rename = "timeFromStart")]
    pub time_from_start: f64,
    pub start: f64,
    #[serde(rename = "type")]
    pub gc_type: String,
    pub before: Vec<HeapSpace>,
    pub end: f64,
    pub after: Vec<HeapSpace>,
}

/// Heap space statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeapSpace {
    pub name: String,
    pub space_size: u64,
    pub space_used_size: u64,
    pub space_available_size: u64,
    pub physical_space_size: u64,
}

/// GC profiler state per thread
struct GcProfilerState {
    is_profiling: AtomicBool,
    filepath: String,
    start_time: f64,
    events: Vec<GcEvent>,
    total_gc_duration: f64,
    total_gc_times: u64,
}

impl GcProfilerState {
    fn new(filepath: String) -> Self {
        Self {
            is_profiling: AtomicBool::new(true),
            filepath,
            start_time: get_timestamp_ms(),
            events: Vec::new(),
            total_gc_duration: 0.0,
            total_gc_times: 0,
        }
    }
}

/// Global GC profiler states per thread
static GC_PROFILERS: Lazy<Mutex<HashMap<i64, GcProfilerState>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Get current timestamp in milliseconds
fn get_timestamp_ms() -> f64 {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    now.as_secs_f64() * 1000.0
}

/// Start GC profiling for a thread
pub fn start_gc_profiling(thread_id: i64, filepath: String) -> Result<(), String> {
    let mut profilers = GC_PROFILERS.lock();

    if profilers.contains_key(&thread_id) {
        return Err("GC profiling already in progress".to_string());
    }

    profilers.insert(thread_id, GcProfilerState::new(filepath));
    set_action_running(thread_id, ProfileAction::StartGcProfiling);

    Ok(())
}

/// Stop GC profiling and write results to file
pub fn stop_gc_profiling(thread_id: i64) -> Result<String, String> {
    let mut profilers = GC_PROFILERS.lock();

    let state = profilers
        .remove(&thread_id)
        .ok_or_else(|| "GC profiling not in progress".to_string())?;

    state.is_profiling.store(false, Ordering::SeqCst);

    let profile = GcProfile {
        start_time: state.start_time,
        gc: state.events,
        stop_time: get_timestamp_ms(),
    };

    // Write to file
    let filepath = state.filepath.clone();
    let json = serde_json::to_string_pretty(&profile)
        .map_err(|e| format!("Failed to serialize GC profile: {}", e))?;

    let mut file =
        File::create(&filepath).map_err(|e| format!("Failed to create file {}: {}", filepath, e))?;

    file.write_all(json.as_bytes())
        .map_err(|e| format!("Failed to write GC profile: {}", e))?;

    clear_action(thread_id, ProfileAction::StartGcProfiling);
    clear_action(thread_id, ProfileAction::StopGcProfiling);

    Ok(filepath)
}

/// Check if GC profiling is active for a thread
pub fn is_gc_profiling_active(thread_id: i64) -> bool {
    let profilers = GC_PROFILERS.lock();
    profilers
        .get(&thread_id)
        .map(|s| s.is_profiling.load(Ordering::SeqCst))
        .unwrap_or(false)
}

/// Record a GC event (called from GC hooks)
pub fn record_gc_event(
    thread_id: i64,
    gc_type: &str,
    duration_ms: f64,
    before_spaces: Vec<HeapSpace>,
    after_spaces: Vec<HeapSpace>,
) {
    let mut profilers = GC_PROFILERS.lock();

    if let Some(state) = profilers.get_mut(&thread_id) {
        if !state.is_profiling.load(Ordering::SeqCst) {
            return;
        }

        let now = get_timestamp_ms();
        state.total_gc_duration += duration_ms;
        state.total_gc_times += 1;

        let event = GcEvent {
            total_spent_from_start: state.total_gc_duration,
            total_times_from_start: state.total_gc_times,
            time_from_start: now - state.start_time,
            start: now - duration_ms,
            gc_type: gc_type.to_string(),
            before: before_spaces,
            end: now,
            after: after_spaces,
        };

        state.events.push(event);
    }
}

/// Get the filepath for current GC profiling session
pub fn get_gc_profile_filepath(thread_id: i64) -> Option<String> {
    let profilers = GC_PROFILERS.lock();
    profilers.get(&thread_id).map(|s| s.filepath.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gc_profiling_lifecycle() {
        let thread_id = 999;
        let filepath = "/tmp/test-gc.gcprofile".to_string();

        // Start profiling
        let result = start_gc_profiling(thread_id, filepath.clone());
        assert!(result.is_ok());
        assert!(is_gc_profiling_active(thread_id));

        // Record an event
        record_gc_event(
            thread_id,
            "scavenge",
            1.5,
            vec![HeapSpace {
                name: "new_space".to_string(),
                space_size: 1024,
                space_used_size: 512,
                space_available_size: 512,
                physical_space_size: 1024,
            }],
            vec![HeapSpace {
                name: "new_space".to_string(),
                space_size: 1024,
                space_used_size: 256,
                space_available_size: 768,
                physical_space_size: 1024,
            }],
        );

        // Stop profiling
        let result = stop_gc_profiling(thread_id);
        assert!(result.is_ok());
        assert!(!is_gc_profiling_active(thread_id));

        // Cleanup
        std::fs::remove_file(&filepath).ok();
    }
}
