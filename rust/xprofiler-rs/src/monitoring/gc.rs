//! Garbage Collection monitoring module
//!
//! This module provides GC monitoring capabilities including
//! GC statistics, timing, frequency analysis, and historical tracking
//! with time-based averages for different periods (15s, 30s, 1m, 3m, 5m).

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use once_cell::sync::Lazy;
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
    /// Average GC frequency (events per second) over different time periods
    pub avg_gc_frequency_15s: f64,
    pub avg_gc_frequency_30s: f64,
    pub avg_gc_frequency_1m: f64,
    pub avg_gc_frequency_3m: f64,
    pub avg_gc_frequency_5m: f64,
    /// Average GC pause time over different time periods
    pub avg_gc_pause_15s: f64,
    pub avg_gc_pause_30s: f64,
    pub avg_gc_pause_1m: f64,
    pub avg_gc_pause_3m: f64,
    pub avg_gc_pause_5m: f64,
    /// Timestamp when statistics were collected
    pub timestamp: u64,
}

/// GC monitor implementation
#[derive(Debug)]
pub struct GcMonitor {
    /// GC event history
    events: Vec<GcEvent>,
    /// Maximum number of events to keep
    max_events: usize,
    /// Whether monitoring is active
    is_monitoring: bool,
    /// Start time for monitoring
    start_time: Option<Instant>,
    /// Historical GC frequency data for different time periods (timestamp, event_count)
    history_frequency_15s: VecDeque<(u64, u32)>,
    history_frequency_30s: VecDeque<(u64, u32)>,
    history_frequency_1m: VecDeque<(u64, u32)>,
    history_frequency_3m: VecDeque<(u64, u32)>,
    history_frequency_5m: VecDeque<(u64, u32)>,
    /// Historical GC pause time data for different time periods (timestamp, total_pause_ms)
    history_pause_15s: VecDeque<(u64, f64)>,
    history_pause_30s: VecDeque<(u64, f64)>,
    history_pause_1m: VecDeque<(u64, f64)>,
    history_pause_3m: VecDeque<(u64, f64)>,
    history_pause_5m: VecDeque<(u64, f64)>,
    /// Current GC count for frequency calculation
    current_gc_count: Arc<Mutex<u32>>,
    /// Current total pause time for average calculation
    current_pause_time: Arc<Mutex<f64>>,
    /// Last update timestamp
    last_update: Arc<Mutex<Option<Instant>>>,
}

impl GcMonitor {
    /// Create a new GC monitor
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            max_events: 1000, // Keep last 1000 GC events
            is_monitoring: false,
            start_time: None,
            // Initialize history queues with appropriate capacities
            history_frequency_15s: VecDeque::with_capacity(15),
            history_frequency_30s: VecDeque::with_capacity(30),
            history_frequency_1m: VecDeque::with_capacity(60),
            history_frequency_3m: VecDeque::with_capacity(180),
            history_frequency_5m: VecDeque::with_capacity(300),
            history_pause_15s: VecDeque::with_capacity(15),
            history_pause_30s: VecDeque::with_capacity(30),
            history_pause_1m: VecDeque::with_capacity(60),
            history_pause_3m: VecDeque::with_capacity(180),
            history_pause_5m: VecDeque::with_capacity(300),
            current_gc_count: Arc::new(Mutex::new(0)),
            current_pause_time: Arc::new(Mutex::new(0.0)),
            last_update: Arc::new(Mutex::new(None)),
        }
    }
    
    /// Record a GC event
    pub fn record_gc_event(&mut self, event: GcEvent) {
        if !self.is_monitoring {
            return;
        }
        
        self.events.push(event.clone());
        
        // Keep only the most recent events
        if self.events.len() > self.max_events {
            self.events.remove(0);
        }
        
        // Update current counters
        if let Ok(mut count) = self.current_gc_count.lock() {
            *count += 1;
        }
        
        if let Ok(mut pause_time) = self.current_pause_time.lock() {
            *pause_time += event.duration.as_millis() as f64;
        }
        
        // Update historical data
        self.update_history();
    }
    
    /// Update historical data for time-based averages
    fn update_history(&mut self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let should_update = {
            if let Ok(mut last_update) = self.last_update.lock() {
                if let Some(last) = *last_update {
                    if last.elapsed().as_secs() >= 1 {
                        *last_update = Some(Instant::now());
                        true
                    } else {
                        false
                    }
                } else {
                    *last_update = Some(Instant::now());
                    true
                }
            } else {
                false
            }
        };
        
        if should_update {
            self.add_to_frequency_history(now);
            self.add_to_pause_history(now);
        }
    }
    
    /// Add current GC frequency to history queues
    fn add_to_frequency_history(&mut self, timestamp: u64) {
        let current_count = if let Ok(count) = self.current_gc_count.lock() {
            *count
        } else {
            0
        };
        
        // Update each history queue separately to avoid borrow checker issues
        Self::add_to_history_static(&mut self.history_frequency_15s, timestamp, current_count, 15);
        Self::add_to_history_static(&mut self.history_frequency_30s, timestamp, current_count, 30);
        Self::add_to_history_static(&mut self.history_frequency_1m, timestamp, current_count, 60);
        Self::add_to_history_static(&mut self.history_frequency_3m, timestamp, current_count, 180);
        Self::add_to_history_static(&mut self.history_frequency_5m, timestamp, current_count, 300);
    }
    
    /// Add current GC pause time to history queues
    fn add_to_pause_history(&mut self, timestamp: u64) {
        let current_pause = if let Ok(pause) = self.current_pause_time.lock() {
            *pause
        } else {
            0.0
        };
        
        // Update each history queue separately to avoid borrow checker issues
        Self::add_to_history_static(&mut self.history_pause_15s, timestamp, current_pause, 15);
        Self::add_to_history_static(&mut self.history_pause_30s, timestamp, current_pause, 30);
        Self::add_to_history_static(&mut self.history_pause_1m, timestamp, current_pause, 60);
        Self::add_to_history_static(&mut self.history_pause_3m, timestamp, current_pause, 180);
        Self::add_to_history_static(&mut self.history_pause_5m, timestamp, current_pause, 300);
    }
    
    /// Generic method to add data to history queue (static version)
    fn add_to_history_static<T: Copy>(queue: &mut VecDeque<(u64, T)>, timestamp: u64, value: T, max_age: u64) {
        queue.push_back((timestamp, value));
        
        // Remove old entries
        while let Some(&(ts, _)) = queue.front() {
            if timestamp - ts > max_age {
                queue.pop_front();
            } else {
                break;
            }
        }
    }
    
    /// Calculate average frequency from history
    fn calculate_frequency_average(&self, queue: &VecDeque<(u64, u32)>) -> f64 {
        if queue.len() < 2 {
            return 0.0;
        }
        
        let total_events: u32 = queue.iter().map(|(_, count)| *count).sum();
        let time_span = queue.back().unwrap().0 - queue.front().unwrap().0;
        
        if time_span > 0 {
            total_events as f64 / time_span as f64
        } else {
            0.0
        }
    }
    
    /// Calculate average pause time from history
    fn calculate_pause_average(&self, queue: &VecDeque<(u64, f64)>) -> f64 {
        if queue.is_empty() {
            return 0.0;
        }
        
        let total_pause: f64 = queue.iter().map(|(_, pause)| *pause).sum();
        total_pause / queue.len() as f64
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
        
        // Calculate time-based averages
        let avg_gc_frequency_15s = self.calculate_frequency_average(&self.history_frequency_15s);
        let avg_gc_frequency_30s = self.calculate_frequency_average(&self.history_frequency_30s);
        let avg_gc_frequency_1m = self.calculate_frequency_average(&self.history_frequency_1m);
        let avg_gc_frequency_3m = self.calculate_frequency_average(&self.history_frequency_3m);
        let avg_gc_frequency_5m = self.calculate_frequency_average(&self.history_frequency_5m);
        
        let avg_gc_pause_15s = self.calculate_pause_average(&self.history_pause_15s);
        let avg_gc_pause_30s = self.calculate_pause_average(&self.history_pause_30s);
        let avg_gc_pause_1m = self.calculate_pause_average(&self.history_pause_1m);
        let avg_gc_pause_3m = self.calculate_pause_average(&self.history_pause_3m);
        let avg_gc_pause_5m = self.calculate_pause_average(&self.history_pause_5m);
        
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        GcStats {
            gc_counts,
            gc_durations,
            gc_avg_durations,
            recent_events,
            total_gc_time,
            total_gc_count,
            avg_gc_frequency_15s,
            avg_gc_frequency_30s,
            avg_gc_frequency_1m,
            avg_gc_frequency_3m,
            avg_gc_frequency_5m,
            avg_gc_pause_15s,
            avg_gc_pause_30s,
            avg_gc_pause_1m,
            avg_gc_pause_3m,
            avg_gc_pause_5m,
            timestamp,
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
    type Stats = GcStats;
    
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.is_monitoring = true;
        self.start_time = Some(Instant::now());
        if let Ok(mut last_update) = self.last_update.lock() {
            *last_update = Some(Instant::now());
        }
        Ok(())
    }
    
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.is_monitoring = false;
        self.start_time = None;
        Ok(())
    }
    
    fn get_stats(&self) -> Self::Stats {
        self.get_gc_stats()
    }
    
    fn is_running(&self) -> bool {
        self.is_monitoring
    }
    
    fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.events.clear();
        self.is_monitoring = false;
        self.start_time = None;
        
        // Clear all history queues
        self.history_frequency_15s.clear();
        self.history_frequency_30s.clear();
        self.history_frequency_1m.clear();
        self.history_frequency_3m.clear();
        self.history_frequency_5m.clear();
        self.history_pause_15s.clear();
        self.history_pause_30s.clear();
        self.history_pause_1m.clear();
        self.history_pause_3m.clear();
        self.history_pause_5m.clear();
        
        // Reset current counters
        if let Ok(mut count) = self.current_gc_count.lock() {
            *count = 0;
        }
        if let Ok(mut pause_time) = self.current_pause_time.lock() {
            *pause_time = 0.0;
        }
        if let Ok(mut last_update) = self.last_update.lock() {
            *last_update = None;
        }
        
        Ok(())
    }
}

/// Global GC monitor instance
pub static GC_MONITOR: Lazy<Arc<Mutex<GcMonitor>>> = Lazy::new(|| {
    Arc::new(Mutex::new(GcMonitor::new()))
});

/// Initialize GC monitor
pub fn init_gc_monitor() {
    let _monitor = &*GC_MONITOR;
}

/// Start GC monitoring
pub fn start_gc_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = GC_MONITOR.lock().map_err(|_| "Failed to lock GC monitor")?;
    monitor.start()
}

/// Stop GC monitoring
pub fn stop_gc_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = GC_MONITOR.lock().map_err(|_| "Failed to lock GC monitor")?;
    monitor.stop()
}

/// Record a GC event globally
pub fn record_gc_event(event: GcEvent) {
    if let Ok(mut monitor) = GC_MONITOR.lock() {
        monitor.record_gc_event(event);
    }
}

/// Get GC statistics
pub fn get_gc_stats() -> Option<GcStats> {
    if let Ok(monitor) = GC_MONITOR.lock() {
        Some(monitor.get_gc_stats())
    } else {
        None
    }
}

/// Get total GC count
pub fn get_total_gc_count() -> u32 {
    if let Ok(monitor) = GC_MONITOR.lock() {
        monitor.get_gc_stats().total_gc_count
    } else {
        0
    }
}

/// Reset GC monitor
pub fn reset_gc_monitor() -> Result<(), Box<dyn std::error::Error>> {
    let mut monitor = GC_MONITOR.lock().map_err(|_| "Failed to lock GC monitor")?;
    monitor.reset()
}

/// Check if GC monitoring is running
pub fn is_gc_monitor_running() -> bool {
    if let Ok(monitor) = GC_MONITOR.lock() {
        monitor.is_running()
    } else {
        false
    }
}

/// Format GC statistics as string
pub fn format_gc_stats() -> String {
    match get_gc_stats() {
        Some(stats) => {
            format!(
                "GC Statistics:\n\
                 Total GC Count: {}\n\
                 Total GC Time: {:.2}ms\n\
                 Avg GC Frequency (15s): {:.2} events/sec\n\
                 Avg GC Frequency (30s): {:.2} events/sec\n\
                 Avg GC Frequency (1m): {:.2} events/sec\n\
                 Avg GC Frequency (3m): {:.2} events/sec\n\
                 Avg GC Frequency (5m): {:.2} events/sec\n\
                 Avg GC Pause (15s): {:.2}ms\n\
                 Avg GC Pause (30s): {:.2}ms\n\
                 Avg GC Pause (1m): {:.2}ms\n\
                 Avg GC Pause (3m): {:.2}ms\n\
                 Avg GC Pause (5m): {:.2}ms\n\
                 Timestamp: {}",
                stats.total_gc_count,
                stats.total_gc_time.as_millis() as f64,
                stats.avg_gc_frequency_15s,
                stats.avg_gc_frequency_30s,
                stats.avg_gc_frequency_1m,
                stats.avg_gc_frequency_3m,
                stats.avg_gc_frequency_5m,
                stats.avg_gc_pause_15s,
                stats.avg_gc_pause_30s,
                stats.avg_gc_pause_1m,
                stats.avg_gc_pause_3m,
                stats.avg_gc_pause_5m,
                stats.timestamp
            )
        }
        None => "GC Monitor: Unable to access monitor".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    
    
    #[test]
    fn test_gc_monitor_creation() {
        let monitor = GcMonitor::new();
        assert_eq!(monitor.events.len(), 0);
        assert!(!monitor.is_monitoring);
    }
    
    #[test]
    fn test_record_gc_event() {
        let mut monitor = GcMonitor::new();
        monitor.start();
        
        let event = GcEvent {
            gc_type: GcType::Scavenge,
            duration: Duration::from_millis(10),
            timestamp: Instant::now(),
            heap_size_before: 1000,
            heap_size_after: 800,
        };
        
        monitor.record_gc_event(event);
        assert_eq!(monitor.events.len(), 1);
        
        // Check that current counters are updated
        if let Ok(count) = monitor.current_gc_count.lock() {
            assert_eq!(*count, 1);
        }
        
        if let Ok(pause_time) = monitor.current_pause_time.lock() {
            assert_eq!(*pause_time, 10.0);
        };
    }
    
    #[test]
    fn test_gc_statistics() {
        let mut monitor = GcMonitor::new();
        monitor.start();
        
        // Add some test events
        let event1 = GcEvent {
            gc_type: GcType::Scavenge,
            duration: Duration::from_millis(10),
            timestamp: Instant::now(),
            heap_size_before: 1000,
            heap_size_after: 800,
        };
        
        let event2 = GcEvent {
            gc_type: GcType::MarkSweepCompact,
            duration: Duration::from_millis(20),
            timestamp: Instant::now(),
            heap_size_before: 800,
            heap_size_after: 600,
        };
        
        monitor.record_gc_event(event1);
        monitor.record_gc_event(event2);
        
        let stats = monitor.get_gc_stats();
        assert_eq!(stats.total_gc_count, 2);
        assert_eq!(stats.total_gc_time, Duration::from_millis(30));
        assert!(stats.timestamp > 0);
    }
    
    #[test]
    fn test_monitor_trait() {
        let mut monitor = GcMonitor::new();
        
        assert!(!monitor.is_running());
        monitor.start();
        assert!(monitor.is_running());
        
        monitor.stop();
        assert!(!monitor.is_running());
        
        // Add some data before reset
        let event = GcEvent {
            gc_type: GcType::Scavenge,
            duration: Duration::from_millis(5),
            timestamp: Instant::now(),
            heap_size_before: 500,
            heap_size_after: 400,
        };
        monitor.record_gc_event(event);
        
        monitor.reset();
        assert_eq!(monitor.events.len(), 0);
        assert!(!monitor.is_running());
        
        // Check that history queues are cleared
        assert_eq!(monitor.history_frequency_15s.len(), 0);
        assert_eq!(monitor.history_pause_15s.len(), 0);
    }
    
    #[test]
    fn test_history_management() {
        let mut monitor = GcMonitor::new();
        let timestamp = 1000;
        
        // Test pause history
        monitor.add_to_pause_history(timestamp);
        assert_eq!(monitor.history_pause_15s.len(), 1);
        
        // Test frequency history
        monitor.add_to_frequency_history(timestamp);
        assert_eq!(monitor.history_frequency_15s.len(), 1);
    }
    
    #[test]
    fn test_average_calculations() {
        let mut monitor = GcMonitor::new();
        
        // Add some history data
        monitor.add_to_frequency_history(1000);
        monitor.add_to_frequency_history(1010);
        monitor.add_to_pause_history(1000);
        monitor.add_to_pause_history(1001);
        
        // Test that history queues have data
        assert!(monitor.history_frequency_15s.len() > 0);
        assert!(monitor.history_pause_15s.len() > 0);
    }
    
    #[test]
    fn test_gc_type_conversion() {
        assert_eq!(GcType::from_v8_type(1), GcType::Scavenge);
        assert_eq!(GcType::from_v8_type(2), GcType::MarkSweepCompact);
        assert_eq!(GcType::Scavenge.as_str(), "scavenge");
    }
    
    #[test]
    fn test_global_functions() {
        init_gc_monitor();
        
        // Test start/stop
        start_gc_monitoring();
        assert!(is_gc_monitor_running());
        
        let _ = stop_gc_monitoring();
        assert!(!is_gc_monitor_running());
        
        // Reset before testing
        let _ = reset_gc_monitor();
        
        // Start monitoring again to record events
        start_gc_monitoring();
        
        let event = GcEvent {
            gc_type: GcType::Scavenge,
            duration: Duration::from_millis(5),
            timestamp: Instant::now(),
            heap_size_before: 500,
            heap_size_after: 400,
        };
        
        record_gc_event(event);
        
        let count = get_total_gc_count();
        assert!(count > 0);
        
        let stats = get_gc_stats();
        assert!(stats.is_some());
        
        let formatted = format_gc_stats();
        assert!(!formatted.is_empty());
        assert!(formatted.contains("GC Statistics"));
    }
    
    #[test]
    fn test_format_gc_stats() {
        let _ = reset_gc_monitor();
        
        // Start monitoring to record events
        start_gc_monitoring();
        
        let event = GcEvent {
            gc_type: GcType::Scavenge,
            duration: Duration::from_millis(15),
            timestamp: Instant::now(),
            heap_size_before: 1000,
            heap_size_after: 800,
        };
        
        record_gc_event(event);
        
        let formatted = format_gc_stats();
        assert!(formatted.contains("Total GC Count: 1"));
        assert!(formatted.contains("Total GC Time: 15.00ms"));
        assert!(formatted.contains("Timestamp:"));
    }
}