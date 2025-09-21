//! NAPI bindings for profiler functionality
//!
//! This module provides JavaScript bindings for CPU, Heap, and GC profilers.

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use once_cell::sync::Lazy;

use crate::profiler::{
    Profiler, ProfilerConfig, CpuProfiler, HeapProfiler, GcProfiler
};
use crate::profiler::heap_profiler::AllocationType;
use crate::profiler::gc_profiler::{GcEventType, GcPhase};

/// Global profiler instances
static CPU_PROFILER: Lazy<Arc<Mutex<Option<CpuProfiler>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(None)));
static HEAP_PROFILER: Lazy<Arc<Mutex<Option<HeapProfiler>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(None)));
static GC_PROFILER: Lazy<Arc<Mutex<Option<GcProfiler>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(None)));

/// Profiler configuration options for JavaScript
#[napi(object)]
pub struct ProfilerOptions {
    /// Sample interval in milliseconds
    pub sample_interval_ms: Option<u32>,
    /// Maximum number of samples to collect
    pub max_samples: Option<u32>,
    /// Whether to enable call stack collection
    pub enable_call_stack: Option<bool>,
}

impl From<ProfilerOptions> for ProfilerConfig {
    fn from(options: ProfilerOptions) -> Self {
        Self {
            sample_interval: Duration::from_millis(
                options.sample_interval_ms.unwrap_or(10) as u64
            ),
            max_samples: options.max_samples.unwrap_or(1000) as usize,
            enable_call_stack: options.enable_call_stack.unwrap_or(true),
        }
    }
}

/// CPU Profiler JavaScript bindings

/// Create and start CPU profiler
#[napi]
pub fn start_cpu_profiler(options: Option<ProfilerOptions>) -> napi::Result<()> {
    let config = options.map(ProfilerConfig::from).unwrap_or_default();
    
    let mut profiler = CpuProfiler::new(config)
        .map_err(|e| napi::Error::from_reason(format!("Failed to create CPU profiler: {}", e)))?;
    
    profiler.start()
        .map_err(|e| napi::Error::from_reason(format!("Failed to start CPU profiler: {}", e)))?;
    
    let mut cpu_profiler_guard = CPU_PROFILER.lock()
        .map_err(|_| napi::Error::from_reason("Failed to lock CPU profiler"))?;
    *cpu_profiler_guard = Some(profiler);
    
    Ok(())
}

/// Stop CPU profiler
#[napi]
pub fn stop_cpu_profiler() -> napi::Result<()> {
    let mut cpu_profiler_guard = CPU_PROFILER.lock()
        .map_err(|_| napi::Error::from_reason("Failed to lock CPU profiler"))?;
    
    if let Some(ref mut profiler) = cpu_profiler_guard.as_mut() {
        profiler.stop()
            .map_err(|e| napi::Error::from_reason(format!("Failed to stop CPU profiler: {}", e)))?;
    }
    
    Ok(())
}

/// Get CPU profiler results
#[napi]
pub fn get_cpu_profile_results() -> napi::Result<String> {
    let cpu_profiler_guard = CPU_PROFILER.lock()
        .map_err(|_| napi::Error::from_reason("Failed to lock CPU profiler"))?;
    
    if let Some(ref profiler) = cpu_profiler_guard.as_ref() {
        profiler.get_results()
            .map_err(|e| napi::Error::from_reason(format!("Failed to get CPU profile results: {}", e)))
    } else {
        Err(napi::Error::from_reason("CPU profiler not initialized"))
    }
}

/// Reset CPU profiler
#[napi]
pub fn reset_cpu_profiler() -> napi::Result<()> {
    let mut cpu_profiler_guard = CPU_PROFILER.lock()
        .map_err(|_| napi::Error::from_reason("Failed to lock CPU profiler"))?;
    
    if let Some(ref mut profiler) = cpu_profiler_guard.as_mut() {
        profiler.reset()
            .map_err(|e| napi::Error::from_reason(format!("Failed to reset CPU profiler: {}", e)))?;
    }
    
    Ok(())
}

/// Check if CPU profiler is running
#[napi]
pub fn is_cpu_profiler_running() -> napi::Result<bool> {
    let cpu_profiler_guard = CPU_PROFILER.lock()
        .map_err(|_| napi::Error::from_reason("Failed to lock CPU profiler"))?;
    
    Ok(cpu_profiler_guard.as_ref().map_or(false, |p| p.is_running()))
}

/// Heap Profiler JavaScript bindings

/// Create and start heap profiler
#[napi]
pub fn start_heap_profiler(options: Option<ProfilerOptions>) -> napi::Result<()> {
    let config = options.map(ProfilerConfig::from).unwrap_or_default();
    
    let mut profiler = HeapProfiler::new(config)
        .map_err(|e| napi::Error::from_reason(format!("Failed to create heap profiler: {}", e)))?;
    
    profiler.start()
        .map_err(|e| napi::Error::from_reason(format!("Failed to start heap profiler: {}", e)))?;
    
    let mut heap_profiler_guard = HEAP_PROFILER.lock()
        .map_err(|_| napi::Error::from_reason("Failed to lock heap profiler"))?;
    *heap_profiler_guard = Some(profiler);
    
    Ok(())
}

/// Stop heap profiler
#[napi]
pub fn stop_heap_profiler() -> napi::Result<()> {
    let mut heap_profiler_guard = HEAP_PROFILER.lock()
        .map_err(|_| napi::Error::from_reason("Failed to lock heap profiler"))?;
    
    if let Some(ref mut profiler) = heap_profiler_guard.as_mut() {
        profiler.stop()
            .map_err(|e| napi::Error::from_reason(format!("Failed to stop heap profiler: {}", e)))?;
    }
    
    Ok(())
}

/// Record a memory allocation
#[napi]
pub fn record_allocation(
    address: f64,
    size: u32,
    allocation_type: String,
) -> napi::Result<()> {
    let heap_profiler_guard = HEAP_PROFILER.lock()
        .map_err(|_| napi::Error::from_reason("Failed to lock heap profiler"))?;
    
    if let Some(ref profiler) = heap_profiler_guard.as_ref() {
        let alloc_type = match allocation_type.as_str() {
            "malloc" => AllocationType::Malloc,
            "calloc" => AllocationType::Calloc,
            "realloc" => AllocationType::Realloc,
            "new" => AllocationType::New,
            "new[]" => AllocationType::NewArray,
            other => AllocationType::Other(other.to_string()),
        };
        
        profiler.record_allocation(address as u64, size as usize, alloc_type)
            .map_err(|e| napi::Error::from_reason(format!("Failed to record allocation: {}", e)))?;
    }
    
    Ok(())
}

/// Record a memory deallocation
#[napi]
pub fn record_deallocation(address: f64) -> napi::Result<()> {
    let heap_profiler_guard = HEAP_PROFILER.lock()
        .map_err(|_| napi::Error::from_reason("Failed to lock heap profiler"))?;
    
    if let Some(ref profiler) = heap_profiler_guard.as_ref() {
        profiler.record_deallocation(address as u64)
            .map_err(|e| napi::Error::from_reason(format!("Failed to record deallocation: {}", e)))?;
    }
    
    Ok(())
}

/// Get heap profiler results
#[napi]
pub fn get_heap_profile_results() -> napi::Result<String> {
    let heap_profiler_guard = HEAP_PROFILER.lock()
        .map_err(|_| napi::Error::from_reason("Failed to lock heap profiler"))?;
    
    if let Some(ref profiler) = heap_profiler_guard.as_ref() {
        profiler.get_results()
            .map_err(|e| napi::Error::from_reason(format!("Failed to get heap profile results: {}", e)))
    } else {
        Err(napi::Error::from_reason("Heap profiler not initialized"))
    }
}

/// Detect memory leaks
#[napi]
pub fn detect_memory_leaks() -> napi::Result<String> {
    let heap_profiler_guard = HEAP_PROFILER.lock()
        .map_err(|_| napi::Error::from_reason("Failed to lock heap profiler"))?;
    
    if let Some(ref profiler) = heap_profiler_guard.as_ref() {
        let leaks = profiler.detect_memory_leaks()
            .map_err(|e| napi::Error::from_reason(format!("Failed to detect memory leaks: {}", e)))?;
        
        serde_json::to_string_pretty(&leaks)
            .map_err(|e| napi::Error::from_reason(format!("Failed to serialize memory leaks: {}", e)))
    } else {
        Err(napi::Error::from_reason("Heap profiler not initialized"))
    }
}

/// Reset heap profiler
#[napi]
pub fn reset_heap_profiler() -> napi::Result<()> {
    let mut heap_profiler_guard = HEAP_PROFILER.lock()
        .map_err(|_| napi::Error::from_reason("Failed to lock heap profiler"))?;
    
    if let Some(ref mut profiler) = heap_profiler_guard.as_mut() {
        profiler.reset()
            .map_err(|e| napi::Error::from_reason(format!("Failed to reset heap profiler: {}", e)))?;
    }
    
    Ok(())
}

/// Check if heap profiler is running
#[napi]
pub fn is_heap_profiler_running() -> napi::Result<bool> {
    let heap_profiler_guard = HEAP_PROFILER.lock()
        .map_err(|_| napi::Error::from_reason("Failed to lock heap profiler"))?;
    
    Ok(heap_profiler_guard.as_ref().map_or(false, |p| p.is_running()))
}

/// GC Profiler JavaScript bindings

/// Create and start GC profiler
#[napi]
pub fn start_gc_profiler(options: Option<ProfilerOptions>) -> napi::Result<()> {
    let config = options.map(ProfilerConfig::from).unwrap_or_default();
    
    let mut profiler = GcProfiler::new(config)
        .map_err(|e| napi::Error::from_reason(format!("Failed to create GC profiler: {}", e)))?;
    
    profiler.start()
        .map_err(|e| napi::Error::from_reason(format!("Failed to start GC profiler: {}", e)))?;
    
    let mut gc_profiler_guard = GC_PROFILER.lock()
        .map_err(|_| napi::Error::from_reason("Failed to lock GC profiler"))?;
    *gc_profiler_guard = Some(profiler);
    
    Ok(())
}

/// Stop GC profiler
#[napi]
pub fn stop_gc_profiler() -> napi::Result<()> {
    let mut gc_profiler_guard = GC_PROFILER.lock()
        .map_err(|_| napi::Error::from_reason("Failed to lock GC profiler"))?;
    
    if let Some(ref mut profiler) = gc_profiler_guard.as_mut() {
        profiler.stop()
            .map_err(|e| napi::Error::from_reason(format!("Failed to stop GC profiler: {}", e)))?;
    }
    
    Ok(())
}

/// Record a GC event
#[napi]
pub fn record_gc_event(
    event_type: String,
    phase: String,
    duration_us: f64,
    memory_before: f64,
    memory_after: f64,
    heap_size: f64,
    reason: String,
    metadata: Option<HashMap<String, String>>,
) -> napi::Result<()> {
    let gc_profiler_guard = GC_PROFILER.lock()
        .map_err(|_| napi::Error::from_reason("Failed to lock GC profiler"))?;
    
    if let Some(ref profiler) = gc_profiler_guard.as_ref() {
        let gc_event_type = match event_type.as_str() {
            "minor" => GcEventType::MinorGc,
            "major" => GcEventType::MajorGc,
            "full" => GcEventType::FullGc,
            "concurrent" => GcEventType::ConcurrentGc,
            "incremental" => GcEventType::IncrementalGc,
            other => GcEventType::Other(other.to_string()),
        };
        
        let gc_phase = match phase.as_str() {
            "start" => GcPhase::Start,
            "mark" => GcPhase::Mark,
            "sweep" => GcPhase::Sweep,
            "compact" => GcPhase::Compact,
            "end" => GcPhase::End,
            _ => GcPhase::End, // Default to end
        };
        
        profiler.record_gc_event(
            gc_event_type,
            gc_phase,
            duration_us as u64,
            memory_before as usize,
            memory_after as usize,
            heap_size as usize,
            reason,
            metadata.unwrap_or_default(),
        ).map_err(|e| napi::Error::from_reason(format!("Failed to record GC event: {}", e)))?;
    }
    
    Ok(())
}

/// Get GC profiler results
#[napi]
pub fn get_gc_profile_results() -> napi::Result<String> {
    let gc_profiler_guard = GC_PROFILER.lock()
        .map_err(|_| napi::Error::from_reason("Failed to lock GC profiler"))?;
    
    if let Some(ref profiler) = gc_profiler_guard.as_ref() {
        profiler.get_results()
            .map_err(|e| napi::Error::from_reason(format!("Failed to get GC profile results: {}", e)))
    } else {
        Err(napi::Error::from_reason("GC profiler not initialized"))
    }
}

/// Reset GC profiler
#[napi]
pub fn reset_gc_profiler() -> napi::Result<()> {
    let mut gc_profiler_guard = GC_PROFILER.lock()
        .map_err(|_| napi::Error::from_reason("Failed to lock GC profiler"))?;
    
    if let Some(ref mut profiler) = gc_profiler_guard.as_mut() {
        profiler.reset()
            .map_err(|e| napi::Error::from_reason(format!("Failed to reset GC profiler: {}", e)))?;
    }
    
    Ok(())
}

/// Check if GC profiler is running
#[napi]
pub fn is_gc_profiler_running() -> napi::Result<bool> {
    let gc_profiler_guard = GC_PROFILER.lock()
        .map_err(|_| napi::Error::from_reason("Failed to lock GC profiler"))?;
    
    Ok(gc_profiler_guard.as_ref().map_or(false, |p| p.is_running()))
}

/// Utility functions

/// Start all profilers with the same configuration
#[napi]
pub fn start_all_profilers(options: Option<ProfilerOptions>) -> napi::Result<()> {
    start_cpu_profiler(options.clone())?;
    start_heap_profiler(options.clone())?;
    start_gc_profiler(options)?;
    Ok(())
}

/// Stop all profilers
#[napi]
pub fn stop_all_profilers() -> napi::Result<()> {
    stop_cpu_profiler()?;
    stop_heap_profiler()?;
    stop_gc_profiler()?;
    Ok(())
}

/// Reset all profilers
#[napi]
pub fn reset_all_profilers() -> napi::Result<()> {
    reset_cpu_profiler()?;
    reset_heap_profiler()?;
    reset_gc_profiler()?;
    Ok(())
}

/// Get status of all profilers
#[napi(object)]
pub struct ProfilerStatus {
    pub cpu_profiler_running: bool,
    pub heap_profiler_running: bool,
    pub gc_profiler_running: bool,
}

/// Get the status of all profilers
#[napi]
pub fn get_profiler_status() -> napi::Result<ProfilerStatus> {
    Ok(ProfilerStatus {
        cpu_profiler_running: is_cpu_profiler_running()?,
        heap_profiler_running: is_heap_profiler_running()?,
        gc_profiler_running: is_gc_profiler_running()?,
    })
}

/// Get comprehensive profiling results from all profilers
#[napi(object)]
pub struct ComprehensiveProfileResults {
    pub cpu_profile: Option<String>,
    pub heap_profile: Option<String>,
    pub gc_profile: Option<String>,
    pub memory_leaks: Option<String>,
}

/// Get results from all active profilers
#[napi]
pub fn get_all_profile_results() -> napi::Result<ComprehensiveProfileResults> {
    let cpu_profile = if is_cpu_profiler_running()? {
        Some(get_cpu_profile_results()?)
    } else {
        None
    };
    
    let heap_profile = if is_heap_profiler_running()? {
        Some(get_heap_profile_results()?)
    } else {
        None
    };
    
    let gc_profile = if is_gc_profiler_running()? {
        Some(get_gc_profile_results()?)
    } else {
        None
    };
    
    let memory_leaks = if is_heap_profiler_running()? {
        Some(detect_memory_leaks()?)
    } else {
        None
    };
    
    Ok(ComprehensiveProfileResults {
        cpu_profile,
        heap_profile,
        gc_profile,
        memory_leaks,
    })
}