//! Heap Profiler implementation
//!
//! This module provides heap memory profiling capabilities,
//! including allocation tracking, memory leak detection, and object lifecycle analysis.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use std::thread;
use serde::{Deserialize, Serialize};
use crate::error::{XProfilerError, XProfilerResult, MonitorType};
use crate::profiler::{Profiler, ProfilerConfig};

/// Represents a memory allocation event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationEvent {
    /// Timestamp when allocation occurred
    pub timestamp: u64,
    /// Size of the allocation in bytes
    pub size: usize,
    /// Memory address of the allocation
    pub address: u64,
    /// Call stack at the time of allocation
    pub call_stack: Vec<String>,
    /// Thread ID where allocation occurred
    pub thread_id: String,
    /// Allocation type (malloc, new, etc.)
    pub allocation_type: AllocationType,
}

/// Represents a memory deallocation event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeallocationEvent {
    /// Timestamp when deallocation occurred
    pub timestamp: u64,
    /// Memory address that was deallocated
    pub address: u64,
    /// Thread ID where deallocation occurred
    pub thread_id: String,
}

/// Types of memory allocations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AllocationType {
    Malloc,
    Calloc,
    Realloc,
    New,
    NewArray,
    Other(String),
}

/// Represents an active memory allocation
#[derive(Debug, Clone, Serialize)]
pub struct ActiveAllocation {
    /// The original allocation event
    pub allocation: AllocationEvent,
    /// How long this allocation has been active
    pub age_ms: u64,
}

/// Memory leak information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLeak {
    /// The allocation that was never freed
    pub allocation: AllocationEvent,
    /// How long this allocation has been leaked
    pub leak_duration_ms: u64,
    /// Suspected leak confidence (0.0 - 1.0)
    pub confidence: f64,
}

/// Heap profiling statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeapProfileStats {
    /// Total number of allocations tracked
    pub total_allocations: usize,
    /// Total number of deallocations tracked
    pub total_deallocations: usize,
    /// Current number of active allocations
    pub active_allocations: usize,
    /// Total bytes allocated
    pub total_bytes_allocated: usize,
    /// Total bytes deallocated
    pub total_bytes_deallocated: usize,
    /// Current bytes in use
    pub current_bytes_in_use: usize,
    /// Peak memory usage
    pub peak_memory_usage: usize,
    /// Detected memory leaks
    pub memory_leaks: Vec<MemoryLeak>,
    /// Allocation size distribution
    pub size_distribution: HashMap<String, usize>, // size range -> count
    /// Top allocation sites by frequency
    pub top_allocation_sites: HashMap<String, usize>, // call stack -> count
    /// Duration of profiling session
    pub duration_ms: u64,
}

/// Heap profiler implementation
#[derive(Debug)]
pub struct HeapProfiler {
    /// Profiler configuration
    config: ProfilerConfig,
    /// Whether profiler is currently running
    is_running: bool,
    /// Start time of profiling session
    start_time: Option<Instant>,
    /// All allocation events
    allocations: Arc<Mutex<Vec<AllocationEvent>>>,
    /// All deallocation events
    deallocations: Arc<Mutex<Vec<DeallocationEvent>>>,
    /// Currently active allocations (address -> allocation)
    active_allocations: Arc<Mutex<HashMap<u64, AllocationEvent>>>,
    /// Peak memory usage tracker
    peak_memory: Arc<Mutex<usize>>,
    /// Current memory usage
    current_memory: Arc<Mutex<usize>>,
    /// Sampling thread handle
    sampling_thread: Option<thread::JoinHandle<()>>,
    /// Stop signal for sampling thread
    stop_signal: Arc<Mutex<bool>>,
}

impl HeapProfiler {
    /// Create a new heap profiler
    pub fn new(config: ProfilerConfig) -> XProfilerResult<Self> {
        Ok(Self {
            config,
            is_running: false,
            start_time: None,
            allocations: Arc::new(Mutex::new(Vec::new())),
            deallocations: Arc::new(Mutex::new(Vec::new())),
            active_allocations: Arc::new(Mutex::new(HashMap::new())),
            peak_memory: Arc::new(Mutex::new(0)),
            current_memory: Arc::new(Mutex::new(0)),
            sampling_thread: None,
            stop_signal: Arc::new(Mutex::new(false)),
        })
    }

    /// Record a memory allocation
    pub fn record_allocation(
        &self,
        address: u64,
        size: usize,
        allocation_type: AllocationType,
    ) -> XProfilerResult<()> {
        if !self.is_running {
            return Ok(());
        }

        let call_stack = self.get_current_call_stack()?;
        let allocation = AllocationEvent {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            size,
            address,
            call_stack,
            thread_id: format!("{:?}", thread::current().id()),
            allocation_type,
        };

        // Record allocation
        self.allocations
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock allocations".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })?
            .push(allocation.clone());

        // Add to active allocations
        self.active_allocations
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock active allocations".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })?
            .insert(address, allocation);

        // Update current memory usage
        let mut current = self.current_memory
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock current memory".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })?;
        *current += size;

        // Update peak memory if necessary
        let mut peak = self.peak_memory
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock peak memory".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })?;
        if *current > *peak {
            *peak = *current;
        }

        Ok(())
    }

    /// Record a memory deallocation
    pub fn record_deallocation(&self, address: u64) -> XProfilerResult<()> {
        if !self.is_running {
            return Ok(());
        }

        let deallocation = DeallocationEvent {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            address,
            thread_id: format!("{:?}", thread::current().id()),
        };

        // Record deallocation
        self.deallocations
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock deallocations".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })?
            .push(deallocation);

        // Remove from active allocations and update current memory
        if let Some(allocation) = self
            .active_allocations
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock active allocations".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })?
            .remove(&address)
        {
            let mut current = self.current_memory
                .lock()
                .map_err(|_| XProfilerError::Monitoring {
                    message: "Failed to lock current memory".to_string(),
                    monitor_type: MonitorType::HeapProfiler,
                })?;
            *current = current.saturating_sub(allocation.size);
        }

        Ok(())
    }

    /// Get current call stack (simplified implementation)
    fn get_current_call_stack(&self) -> XProfilerResult<Vec<String>> {
        // This is a simplified implementation
        // In practice, you would use platform-specific APIs or backtrace
        Ok(vec![
            "heap_profiler::record_allocation".to_string(),
            "<caller_function>".to_string(),
        ])
    }

    /// Detect potential memory leaks
    pub fn detect_memory_leaks(&self) -> XProfilerResult<Vec<MemoryLeak>> {
        let active_allocations = self.active_allocations
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock active allocations".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })?;

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let mut leaks = Vec::new();
        let leak_threshold_ms = 60000; // 1 minute

        for allocation in active_allocations.values() {
            let age = current_time.saturating_sub(allocation.timestamp);
            if age > leak_threshold_ms {
                let confidence = (age as f64 / (leak_threshold_ms as f64 * 10.0)).min(1.0);
                leaks.push(MemoryLeak {
                    allocation: allocation.clone(),
                    leak_duration_ms: age,
                    confidence,
                });
            }
        }

        // Sort by leak duration (longest first)
        leaks.sort_by(|a, b| b.leak_duration_ms.cmp(&a.leak_duration_ms));

        Ok(leaks)
    }

    /// Get profiling statistics
    pub fn get_stats(&self) -> XProfilerResult<HeapProfileStats> {
        let allocations = self.allocations
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock allocations".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })?;

        let deallocations = self.deallocations
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock deallocations".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })?;

        let active_allocations = self.active_allocations
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock active allocations".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })?;

        let current_memory = *self.current_memory
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock current memory".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })?;

        let peak_memory = *self.peak_memory
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock peak memory".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })?;

        // Calculate statistics
        let total_bytes_allocated: usize = allocations.iter().map(|a| a.size).sum();
        let total_bytes_deallocated: usize = deallocations.len() * 1024; // Simplified

        // Size distribution
        let mut size_distribution = HashMap::new();
        for allocation in allocations.iter() {
            let range = match allocation.size {
                0..=1024 => "0-1KB",
                1025..=10240 => "1-10KB",
                10241..=102400 => "10-100KB",
                102401..=1048576 => "100KB-1MB",
                _ => ">1MB",
            };
            *size_distribution.entry(range.to_string()).or_insert(0) += 1;
        }

        // Top allocation sites
        let mut top_allocation_sites = HashMap::new();
        for allocation in allocations.iter() {
            let site = allocation.call_stack.join(" -> ");
            *top_allocation_sites.entry(site).or_insert(0) += 1;
        }

        let memory_leaks = self.detect_memory_leaks()?;

        let duration_ms = self.start_time
            .map(|start| start.elapsed().as_millis() as u64)
            .unwrap_or(0);

        Ok(HeapProfileStats {
            total_allocations: allocations.len(),
            total_deallocations: deallocations.len(),
            active_allocations: active_allocations.len(),
            total_bytes_allocated,
            total_bytes_deallocated,
            current_bytes_in_use: current_memory,
            peak_memory_usage: peak_memory,
            memory_leaks,
            size_distribution,
            top_allocation_sites,
            duration_ms,
        })
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
pub use tests::*;

impl Profiler for HeapProfiler {
    fn start(&mut self) -> XProfilerResult<()> {
        if self.is_running {
            return Err(XProfilerError::Monitoring {
                message: "Heap profiler is already running".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            });
        }

        // Reset state
        self.allocations
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock allocations".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })?
            .clear();

        self.deallocations
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock deallocations".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })?
            .clear();

        self.active_allocations
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock active allocations".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })?
            .clear();

        *self.current_memory
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock current memory".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })? = 0;

        *self.peak_memory
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock peak memory".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })? = 0;

        *self.stop_signal
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock stop signal".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })? = false;

        self.start_time = Some(Instant::now());
        self.is_running = true;

        Ok(())
    }

    fn stop(&mut self) -> XProfilerResult<()> {
        if !self.is_running {
            return Ok(());
        }

        // Signal sampling thread to stop
        *self.stop_signal
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock stop signal".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })? = true;

        // Wait for sampling thread to finish
        if let Some(handle) = self.sampling_thread.take() {
            handle
                .join()
                .map_err(|_| XProfilerError::Monitoring {
                    message: "Failed to join sampling thread".to_string(),
                    monitor_type: MonitorType::HeapProfiler,
                })?;
        }

        self.is_running = false;
        Ok(())
    }

    fn get_results(&self) -> XProfilerResult<String> {
        let stats = self.get_stats()?;
        serde_json::to_string_pretty(&stats).map_err(|e| XProfilerError::Monitoring {
            message: format!("Failed to serialize heap profile results: {}", e),
            monitor_type: MonitorType::HeapProfiler,
        })
    }

    fn reset(&mut self) -> XProfilerResult<()> {
        if self.is_running {
            self.stop()?;
        }

        self.allocations
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock allocations".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })?
            .clear();

        self.deallocations
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock deallocations".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })?
            .clear();

        self.active_allocations
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock active allocations".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })?
            .clear();

        *self.current_memory
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock current memory".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })? = 0;

        *self.peak_memory
            .lock()
            .map_err(|_| XProfilerError::Monitoring {
                message: "Failed to lock peak memory".to_string(),
                monitor_type: MonitorType::HeapProfiler,
            })? = 0;

        self.start_time = None;
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.is_running
    }
}