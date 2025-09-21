//! Garbage Collection monitoring module
//!
//! This module provides GC monitoring capabilities including
//! GC statistics, timing, and frequency analysis.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use super::Monitor;

/// GC event types (matching V8 GC types)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GcType {
    Scavenge,
    MarkSweepCompact,
    IncrementalMarking,
    ProcessWeakCallbacks,
    All,
}

impl GcType {
    pub fn from_v8_type(gc_type: u32) -> Self {
        match gc_type {
            1 => GcType::Scavenge,
            2 => GcType::MarkSweepCompact,
            4 => GcType::IncrementalMarking,
            8 => GcType::ProcessWeakCallbacks,
            15 => GcType::All,
            _ => GcType::All,
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            GcType::Scavenge => "scavenge",
            GcType::MarkSweepCompact => "mark_sweep_compact",
            GcType::IncrementalMarking => "incremental_marking",
            GcType::ProcessWeakCallbacks => "process_weak_callbacks",
            GcType::All => "all",
        }
    }
}

/// GC event information
#[derive(Debug, Clone)]
pub struct GcEvent {
    /// Type of GC
    pub gc_type: GcType,
    /// Duration of the GC event
    pub duration: Duration,
    /// Timestamp when the event occurred
    pub timestamp: Instant,
    /// Heap size before GC
    pub heap_size_before: u64,
    /// Heap size after GC
    pub heap_size_after: u64,
}

/// GC statistics
#[derive(Debug, Clone)]
pub struct GcStats {
    /// Total number of GC events by type
    pub gc_counts: HashMap<GcType, u32>,
    /// Total time spent in GC by type
    pub gc_durations: HashMap<GcType, Duration>,
    /// Average GC duration by type
    pub gc_avg_durations: HashMap<GcType, Duration>,
    /// Recent GC events (last 100)
    pub recent_events: Vec<GcEvent>,
    /// Total GC time
    pub total_gc_time: Duration,
    /// Total GC count
    pub total_gc_count: u32,
}

/// GC monitor implementation
pub struct GcMonitor {
    /// GC event history
    events: Vec<GcEvent>,
    /// Maximum number of events to keep
    max_events: usize,
    /// Whether monitoring is active
    is_monitoring: bool,
    /// Start time for monitoring
    start_time: Option<Instant>,
}

impl GcMonitor {
    /// Create a new GC monitor
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            max_events: 1000, // Keep last 1000 GC events
            is_monitoring: false,
            start_time: None,
        }
    }
    
    /// Record a GC event
    pub fn record_gc_event(&mut self, event: GcEvent) {
        if !self.is_monitoring {
            return;
        }
        
        self.events.push(event);
        
        // Keep only the most recent events
        if self.events.len() > self.max_events {
            self.events.remove(0);
        }
    }
    
    /// Get GC statistics
    pub fn get_gc_stats(&self) -> GcStats {
        let mut gc_counts = HashMap::new();
        let mut gc_durations = HashMap::new();
        
        for event in &self.events {
            *gc_counts.entry(event.gc_type).or_insert(0) += 1;
            *gc_durations.entry(event.gc_type).or_insert(Duration::ZERO) += event.duration;
        }
        
        // Calculate average durations
        let mut gc_avg_durations = HashMap::new();
        for (gc_type, total_duration) in &gc_durations {
            let count = gc_counts.get(gc_type).unwrap_or(&0);
            if *count > 0 {
                gc_avg_durations.insert(*gc_type, *total_duration / *count);
            }
        }
        
        let total_gc_time = gc_durations.values().sum();
        let total_gc_count = gc_counts.values().sum();
        
        // Get recent events (last 100)
        let recent_events = if self.events.len() > 100 {
            self.events[self.events.len() - 100..].to_vec()
        } else {
            self.events.clone()
        };
        
        GcStats {
            gc_counts,
            gc_durations,
            gc_avg_durations,
            recent_events,
            total_gc_time,
            total_gc_count,
        }
    }
    
    /// Get total GC count
    pub fn get_total_gc_count(&self) -> u32 {
        self.events.len() as u32
    }
    
    /// Get GC count by type
    pub fn get_gc_count_by_type(&self, gc_type: GcType) -> u32 {
        self.events.iter()
            .filter(|event| event.gc_type == gc_type)
            .count() as u32
    }
    
    /// Get total GC time
    pub fn get_total_gc_time(&self) -> Duration {
        self.events.iter()
            .map(|event| event.duration)
            .sum()
    }
    
    /// Format GC statistics for logging
    pub fn format_gc_stats(&self) -> String {
        let stats = self.get_gc_stats();
        
        let scavenge_count = stats.gc_counts.get(&GcType::Scavenge).unwrap_or(&0);
        let mark_sweep_count = stats.gc_counts.get(&GcType::MarkSweepCompact).unwrap_or(&0);
        let incremental_count = stats.gc_counts.get(&GcType::IncrementalMarking).unwrap_or(&0);
        
        let scavenge_time = stats.gc_durations.get(&GcType::Scavenge)
            .unwrap_or(&Duration::ZERO).as_millis();
        let mark_sweep_time = stats.gc_durations.get(&GcType::MarkSweepCompact)
            .unwrap_or(&Duration::ZERO).as_millis();
        let incremental_time = stats.gc_durations.get(&GcType::IncrementalMarking)
            .unwrap_or(&Duration::ZERO).as_millis();
        
        format!(
            "gc total_gc_times: {}, scavenge_count: {}, scavenge_duration: {}ms, \
             marksweep_count: {}, marksweep_duration: {}ms, incremental_count: {}, incremental_duration: {}ms",
            stats.total_gc_count,
            scavenge_count, scavenge_time,
            mark_sweep_count, mark_sweep_time,
            incremental_count, incremental_time
        )
    }
    
    /// Clear all recorded events
    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}

impl Default for GcMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Monitor for GcMonitor {
    type Output = GcStats;
    
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.is_monitoring = true;
        self.start_time = Some(Instant::now());
        Ok(())
    }
    
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.is_monitoring = false;
        Ok(())
    }
    
    fn get_metrics(&self) -> Self::Output {
        self.get_gc_stats()
    }
    
    fn reset(&mut self) {
        self.events.clear();
        self.start_time = None;
    }
}

/// Global GC monitor instance
static GC_MONITOR: Mutex<Option<GcMonitor>> = Mutex::new(None);

/// Initialize global GC monitor
pub fn init_gc_monitor() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = GC_MONITOR.lock().unwrap();
    *monitor = Some(GcMonitor::new());
    Ok(())
}

/// Record a GC event
pub fn record_gc_event(gc_type: GcType, duration: Duration, heap_before: u64, heap_after: u64) {
    let mut monitor = GC_MONITOR.lock().unwrap();
    if let Some(ref mut gc_monitor) = monitor.as_mut() {
        let event = GcEvent {
            gc_type,
            duration,
            timestamp: Instant::now(),
            heap_size_before: heap_before,
            heap_size_after: heap_after,
        };
        gc_monitor.record_gc_event(event);
    }
}

/// Get total GC count
pub fn get_total_gc_count() -> u32 {
    let monitor = GC_MONITOR.lock().unwrap();
    monitor.as_ref().map(|m| m.get_total_gc_count()).unwrap_or(0)
}

/// Get GC statistics
pub fn get_gc_stats() -> Option<GcStats> {
    let monitor = GC_MONITOR.lock().unwrap();
    monitor.as_ref().map(|m| m.get_gc_stats())
}

/// Format GC statistics for logging
pub fn format_gc_stats() -> String {
    let monitor = GC_MONITOR.lock().unwrap();
    monitor
        .as_ref()
        .map(|m| m.format_gc_stats())
        .unwrap_or_else(|| "GC monitor not initialized".to_string())
}

/// Start GC monitoring
pub fn start_gc_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = GC_MONITOR.lock().unwrap();
    if let Some(ref mut gc_monitor) = monitor.as_mut() {
        gc_monitor.start()?;
    }
    Ok(())
}

/// Stop GC monitoring
pub fn stop_gc_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = GC_MONITOR.lock().unwrap();
    if let Some(ref mut gc_monitor) = monitor.as_mut() {
        gc_monitor.stop()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gc_monitor_creation() {
        let monitor = GcMonitor::new();
        assert!(!monitor.is_monitoring);
        assert_eq!(monitor.events.len(), 0);
    }
    
    #[test]
    fn test_gc_event_recording() {
        let mut monitor = GcMonitor::new();
        monitor.start().unwrap();
        
        let event = GcEvent {
            gc_type: GcType::Scavenge,
            duration: Duration::from_millis(10),
            timestamp: Instant::now(),
            heap_size_before: 1024 * 1024,
            heap_size_after: 512 * 1024,
        };
        
        monitor.record_gc_event(event);
        assert_eq!(monitor.events.len(), 1);
        assert_eq!(monitor.get_total_gc_count(), 1);
    }
    
    #[test]
    fn test_gc_type_conversion() {
        assert_eq!(GcType::from_v8_type(1), GcType::Scavenge);
        assert_eq!(GcType::from_v8_type(2), GcType::MarkSweepCompact);
        assert_eq!(GcType::Scavenge.as_str(), "scavenge");
    }
    
    #[test]
    fn test_gc_stats() {
        let mut monitor = GcMonitor::new();
        monitor.start().unwrap();
        
        // Add some test events
        for i in 0..5 {
            let event = GcEvent {
                gc_type: if i % 2 == 0 { GcType::Scavenge } else { GcType::MarkSweepCompact },
                duration: Duration::from_millis(10 + i),
                timestamp: Instant::now(),
                heap_size_before: 1024 * 1024,
                heap_size_after: 512 * 1024,
            };
            monitor.record_gc_event(event);
        }
        
        let stats = monitor.get_gc_stats();
        assert_eq!(stats.total_gc_count, 5);
        assert_eq!(*stats.gc_counts.get(&GcType::Scavenge).unwrap(), 3);
        assert_eq!(*stats.gc_counts.get(&GcType::MarkSweepCompact).unwrap(), 2);
    }
}