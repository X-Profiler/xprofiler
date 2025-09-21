//! GC Profiler implementation
//!
//! This module provides garbage collection profiling capabilities,
//! including GC event monitoring, performance analysis, and memory reclamation statistics.

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::thread;
use serde::{Deserialize, Serialize};
use crate::error::{XProfilerError, XProfilerResult, MonitorType};
use crate::profiler::{Profiler, ProfilerConfig};

/// Types of garbage collection events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GcEventType {
    /// Minor GC (young generation)
    MinorGc,
    /// Major GC (old generation)
    MajorGc,
    /// Full GC (entire heap)
    FullGc,
    /// Concurrent GC
    ConcurrentGc,
    /// Incremental GC
    IncrementalGc,
    /// Other GC types
    Other(String),
}

/// GC event phases
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GcPhase {
    /// GC started
    Start,
    /// Mark phase
    Mark,
    /// Sweep phase
    Sweep,
    /// Compact phase
    Compact,
    /// GC completed
    End,
}

/// Represents a garbage collection event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcEvent {
    /// Unique event ID
    pub id: u64,
    /// Type of GC event
    pub event_type: GcEventType,
    /// GC phase
    pub phase: GcPhase,
    /// Timestamp when event occurred
    pub timestamp: u64,
    /// Duration of the GC event in microseconds
    pub duration_us: u64,
    /// Memory before GC in bytes
    pub memory_before: usize,
    /// Memory after GC in bytes
    pub memory_after: usize,
    /// Memory reclaimed in bytes
    pub memory_reclaimed: usize,
    /// Heap size at the time of GC
    pub heap_size: usize,
    /// Thread ID where GC occurred
    pub thread_id: String,
    /// GC reason/trigger
    pub reason: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// GC generation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcGenerationStats {
    /// Generation name (young, old, etc.)
    pub name: String,
    /// Number of collections
    pub collection_count: usize,
    /// Total time spent in GC for this generation
    pub total_time_us: u64,
    /// Average GC time
    pub average_time_us: u64,
    /// Maximum GC time
    pub max_time_us: u64,
    /// Minimum GC time
    pub min_time_us: u64,
    /// Total memory reclaimed
    pub total_memory_reclaimed: usize,
    /// Average memory reclaimed per collection
    pub average_memory_reclaimed: usize,
}

/// GC profiling statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcProfileStats {
    /// Total number of GC events
    pub total_gc_events: usize,
    /// Total time spent in GC
    pub total_gc_time_us: u64,
    /// Average GC time
    pub average_gc_time_us: u64,
    /// Maximum GC time
    pub max_gc_time_us: u64,
    /// Minimum GC time
    pub min_gc_time_us: u64,
    /// GC frequency (events per second)
    pub gc_frequency: f64,
    /// Total memory reclaimed
    pub total_memory_reclaimed: usize,
    /// GC overhead percentage
    pub gc_overhead_percent: f64,
    /// Statistics by GC type
    pub gc_type_stats: HashMap<String, GcGenerationStats>,
    /// Recent GC events (last N events)
    pub recent_events: Vec<GcEvent>,
    /// GC time distribution
    pub time_distribution: HashMap<String, usize>, // time range -> count
    /// Memory reclamation efficiency
    pub reclamation_efficiency: f64, // percentage
    /// Duration of profiling session
    pub duration_ms: u64,
    /// Longest pause time
    pub longest_pause_us: u64,
    /// GC throughput (MB/s)
    pub throughput_mb_per_sec: f64,
}

/// GC profiler implementation
#[derive(Debug)]
pub struct GcProfiler {
    /// Profiler configuration
    config: ProfilerConfig,
    /// Whether profiler is currently running
    is_running: bool,
    /// Start time of profiling session
    start_time: Option<Instant>,
    /// All GC events
    gc_events: Arc<Mutex<Vec<GcEvent>>>,
    /// Recent GC events (circular buffer)
    recent_events: Arc<Mutex<VecDeque<GcEvent>>>,
    /// Event ID counter
    event_id_counter: Arc<Mutex<u64>>,
    /// Monitoring thread handle
    monitoring_thread: Option<thread::JoinHandle<()>>,
    /// Stop signal for monitoring thread
    stop_signal: Arc<Mutex<bool>>,
    /// Maximum number of recent events to keep
    max_recent_events: usize,
}

impl GcProfiler {
    /// Create a new GC profiler
    pub fn new(config: ProfilerConfig) -> XProfilerResult<Self> {
        Ok(Self {
            config,
            is_running: false,
            start_time: None,
            gc_events: Arc::new(Mutex::new(Vec::new())),
            recent_events: Arc::new(Mutex::new(VecDeque::new())),
            event_id_counter: Arc::new(Mutex::new(0)),
            monitoring_thread: None,
            stop_signal: Arc::new(Mutex::new(false)),
            max_recent_events: 100,
        })
    }

    /// Record a GC event
    pub fn record_gc_event(
        &self,
        event_type: GcEventType,
        phase: GcPhase,
        duration_us: u64,
        memory_before: usize,
        memory_after: usize,
        heap_size: usize,
        reason: String,
        metadata: HashMap<String, String>,
    ) -> XProfilerResult<()> {
        if !self.is_running {
            return Ok(());
        }

        let event_id = {
            let mut counter = self.event_id_counter
                .lock()
                .map_err(|_| XProfilerError::Monitoring {
                    message: "Failed to lock event ID counter".to_string(),
                    monitor_type: MonitorType::GcProfiler,
                })?;
            *counter += 1;
            *counter
        };

        let memory_reclaimed = memory_before.saturating_sub(memory_after);
        
        let event = GcEvent {
            id: event_id,
            event_type,
            phase,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            duration_us,
            memory_before,
            memory_after,
            memory_reclaimed,
            heap_size,
            thread_id: format!("{:?}", thread::current().id()),
            reason,
            metadata,
        };

        // Record event
        self.gc_events
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock GC events".to_string(),
                monitor_type: MonitorType::GcProfiler,
            })?
            .push(event.clone());

        // Add to recent events (circular buffer)
        {
            let mut recent = self.recent_events
                .lock()
                .map_err(|_| XProfilerError::Monitoring {
                    message: "Failed to lock recent events".to_string(),
                    monitor_type: MonitorType::GcProfiler,
                })?;
            
            recent.push_back(event);
            if recent.len() > self.max_recent_events {
                recent.pop_front();
            }
        }

        Ok(())
    }

    /// Start monitoring GC events (simplified implementation)
    fn start_monitoring(&mut self) -> XProfilerResult<()> {
        let gc_events = Arc::clone(&self.gc_events);
        let stop_signal = Arc::clone(&self.stop_signal);
        let sample_interval = self.config.sampling_interval;

        let handle = thread::spawn(move || {
            let mut last_gc_time = Instant::now();
            
            loop {
                // Check stop signal
                if let Ok(stop) = stop_signal.lock() {
                    if *stop {
                        break;
                    }
                }

                // Simulate GC event detection
                // In a real implementation, this would hook into the actual GC system
                if last_gc_time.elapsed() > Duration::from_secs(5) {
                    // Simulate a minor GC event
                    if let Ok(mut events) = gc_events.lock() {
                        let event = GcEvent {
                            id: events.len() as u64 + 1,
                            event_type: GcEventType::MinorGc,
                            phase: GcPhase::End,
                            timestamp: SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_millis() as u64,
                            duration_us: 1000, // 1ms
                            memory_before: 10 * 1024 * 1024, // 10MB
                            memory_after: 8 * 1024 * 1024,   // 8MB
                            memory_reclaimed: 2 * 1024 * 1024, // 2MB
                            heap_size: 50 * 1024 * 1024,     // 50MB
                            thread_id: format!("{:?}", thread::current().id()),
                            reason: "Allocation threshold reached".to_string(),
                            metadata: HashMap::new(),
                        };
                        events.push(event);
                    }
                    last_gc_time = Instant::now();
                }

                thread::sleep(sample_interval);
            }
        });

        self.monitoring_thread = Some(handle);
        Ok(())
    }

    /// Calculate GC statistics by type
    fn calculate_gc_type_stats(&self, events: &[GcEvent]) -> HashMap<String, GcGenerationStats> {
        let mut stats_map = HashMap::new();

        for event in events {
            let type_name = match &event.event_type {
                GcEventType::MinorGc => "Minor GC".to_string(),
                GcEventType::MajorGc => "Major GC".to_string(),
                GcEventType::FullGc => "Full GC".to_string(),
                GcEventType::ConcurrentGc => "Concurrent GC".to_string(),
                GcEventType::IncrementalGc => "Incremental GC".to_string(),
                GcEventType::Other(name) => name.clone(),
            };

            let stats = stats_map.entry(type_name.clone()).or_insert(GcGenerationStats {
                name: type_name,
                collection_count: 0,
                total_time_us: 0,
                average_time_us: 0,
                max_time_us: 0,
                min_time_us: u64::MAX,
                total_memory_reclaimed: 0,
                average_memory_reclaimed: 0,
            });

            stats.collection_count += 1;
            stats.total_time_us += event.duration_us;
            stats.max_time_us = stats.max_time_us.max(event.duration_us);
            stats.min_time_us = stats.min_time_us.min(event.duration_us);
            stats.total_memory_reclaimed += event.memory_reclaimed;
        }

        // Calculate averages
        for stats in stats_map.values_mut() {
            if stats.collection_count > 0 {
                stats.average_time_us = stats.total_time_us / stats.collection_count as u64;
                stats.average_memory_reclaimed = stats.total_memory_reclaimed / stats.collection_count;
                if stats.min_time_us == u64::MAX {
                    stats.min_time_us = 0;
                }
            }
        }

        stats_map
    }

    /// Get profiling statistics
    pub fn get_stats(&self) -> XProfilerResult<GcProfileStats> {
        let events = self.gc_events
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock GC events".to_string(),
                monitor_type: MonitorType::GcProfiler,
            })?;

        let recent_events = self.recent_events
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock recent events".to_string(),
                monitor_type: MonitorType::GcProfiler,
            })?
            .iter()
            .cloned()
            .collect();

        if events.is_empty() {
            return Ok(GcProfileStats {
                total_gc_events: 0,
                total_gc_time_us: 0,
                average_gc_time_us: 0,
                max_gc_time_us: 0,
                min_gc_time_us: 0,
                gc_frequency: 0.0,
                total_memory_reclaimed: 0,
                gc_overhead_percent: 0.0,
                gc_type_stats: HashMap::new(),
                recent_events,
                time_distribution: HashMap::new(),
                reclamation_efficiency: 0.0,
                duration_ms: 0,
                longest_pause_us: 0,
                throughput_mb_per_sec: 0.0,
            });
        }

        // Calculate basic statistics
        let total_gc_events = events.len();
        let total_gc_time_us: u64 = events.iter().map(|e| e.duration_us).sum();
        let average_gc_time_us = total_gc_time_us / total_gc_events as u64;
        let max_gc_time_us = events.iter().map(|e| e.duration_us).max().unwrap_or(0);
        let min_gc_time_us = events.iter().map(|e| e.duration_us).min().unwrap_or(0);
        let total_memory_reclaimed: usize = events.iter().map(|e| e.memory_reclaimed).sum();
        let longest_pause_us = max_gc_time_us;

        // Calculate duration and frequency
        let duration_ms = self.start_time
            .map(|start| start.elapsed().as_millis() as u64)
            .unwrap_or(0);
        
        let gc_frequency = if duration_ms > 0 {
            (total_gc_events as f64) / (duration_ms as f64 / 1000.0)
        } else {
            0.0
        };

        // Calculate GC overhead
        let total_time_ms = duration_ms;
        let gc_time_ms = total_gc_time_us / 1000;
        let gc_overhead_percent = if total_time_ms > 0 {
            (gc_time_ms as f64 / total_time_ms as f64) * 100.0
        } else {
            0.0
        };

        // Calculate throughput
        let throughput_mb_per_sec = if duration_ms > 0 {
            let total_mb = total_memory_reclaimed as f64 / (1024.0 * 1024.0);
            let duration_sec = duration_ms as f64 / 1000.0;
            total_mb / duration_sec
        } else {
            0.0
        };

        // Calculate reclamation efficiency
        let total_memory_before: usize = events.iter().map(|e| e.memory_before).sum();
        let reclamation_efficiency = if total_memory_before > 0 {
            (total_memory_reclaimed as f64 / total_memory_before as f64) * 100.0
        } else {
            0.0
        };

        // Time distribution
        let mut time_distribution = HashMap::new();
        for event in events.iter() {
            let range = match event.duration_us {
                0..=1000 => "0-1ms",
                1001..=10000 => "1-10ms",
                10001..=100000 => "10-100ms",
                100001..=1000000 => "100ms-1s",
                _ => ">1s",
            };
            *time_distribution.entry(range.to_string()).or_insert(0) += 1;
        }

        let gc_type_stats = self.calculate_gc_type_stats(&events);

        Ok(GcProfileStats {
            total_gc_events,
            total_gc_time_us,
            average_gc_time_us,
            max_gc_time_us,
            min_gc_time_us,
            gc_frequency,
            total_memory_reclaimed,
            gc_overhead_percent,
            gc_type_stats,
            recent_events,
            time_distribution,
            reclamation_efficiency,
            duration_ms,
            longest_pause_us,
            throughput_mb_per_sec,
        })
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
pub use tests::*;

impl Profiler for GcProfiler {
    fn start(&mut self) -> XProfilerResult<()> {
        if self.is_running {
            return Err(XProfilerError::Monitoring {
                message: "GC profiler is already running".to_string(),
                monitor_type: MonitorType::GcProfiler,
            });
        }

        // Reset state
        self.gc_events
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock GC events".to_string(),
                monitor_type: MonitorType::GcProfiler,
            })?
            .clear();

        self.recent_events
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock recent events".to_string(),
                monitor_type: MonitorType::GcProfiler,
            })?
            .clear();

        *self.event_id_counter
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock event ID counter".to_string(),
                monitor_type: MonitorType::GcProfiler,
            })? = 0;

        *self.stop_signal
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock stop signal".to_string(),
                monitor_type: MonitorType::GcProfiler,
            })? = false;

        self.start_time = Some(Instant::now());
        self.is_running = true;

        // Start monitoring thread
        self.start_monitoring()?;

        Ok(())
    }

    fn stop(&mut self) -> XProfilerResult<()> {
        if !self.is_running {
            return Ok(());
        }

        // Signal monitoring thread to stop
        *self.stop_signal
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock stop signal".to_string(),
                monitor_type: MonitorType::GcProfiler,
            })? = true;

        // Wait for monitoring thread to finish
        if let Some(handle) = self.monitoring_thread.take() {
            handle
                .join()
                .map_err(|_| XProfilerError::Monitoring {
                    message: "Failed to join monitoring thread".to_string(),
                    monitor_type: MonitorType::GcProfiler,
                })?;
        }

        self.is_running = false;
        Ok(())
    }

    fn get_results(&self) -> XProfilerResult<String> {
        let stats = self.get_stats()?;
        serde_json::to_string_pretty(&stats).map_err(|e| XProfilerError::Monitoring {
            message: format!("Failed to serialize GC profile results: {}", e),
            monitor_type: MonitorType::GcProfiler,
        })
    }

    fn reset(&mut self) -> XProfilerResult<()> {
        if self.is_running {
            self.stop()?;
        }

        self.gc_events
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock GC events".to_string(),
                monitor_type: MonitorType::GcProfiler,
            })?
            .clear();

        self.recent_events
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock recent events".to_string(),
                monitor_type: MonitorType::GcProfiler,
            })?
            .clear();

        *self.event_id_counter
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock event ID counter".to_string(),
                monitor_type: MonitorType::GcProfiler,
            })? = 0;

        self.start_time = None;
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.is_running
    }
}