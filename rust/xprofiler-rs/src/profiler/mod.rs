//! Profiler module for XProfiler
//!
//! This module provides various profiling capabilities including:
//! - CPU profiling with sampling and call stack analysis
//! - Heap profiling for memory allocation tracking
//! - GC profiling for garbage collection monitoring

use crate::error::XProfilerResult;
use std::time::Duration;

pub mod cpu_profiler;
pub mod gc_profiler;
pub mod heap_profiler;

pub use cpu_profiler::CpuProfiler;
pub use gc_profiler::GcProfiler;
pub use heap_profiler::HeapProfiler;

#[cfg(test)]
mod tests;

#[cfg(test)]
pub use tests::*;

/// Common profiler trait that all profilers must implement
pub trait Profiler {
  /// Start profiling
  fn start(&mut self) -> XProfilerResult<()>;

  /// Stop profiling
  fn stop(&mut self) -> XProfilerResult<()>;

  /// Check if profiler is currently running
  fn is_running(&self) -> bool;

  /// Get profiling results as JSON string
  fn get_results(&self) -> XProfilerResult<String>;

  /// Reset profiler state
  fn reset(&mut self) -> XProfilerResult<()>;
}

/// Profiler configuration
#[derive(Debug, Clone)]
pub struct ProfilerConfig {
  /// Sampling interval for CPU profiler
  pub sampling_interval: Duration,
  /// Maximum number of samples to collect
  pub max_samples: usize,
  /// Whether to collect call stacks
  pub collect_stacks: bool,
  /// Maximum stack depth to collect
  pub max_stack_depth: usize,
  /// Output file path (optional)
  pub output_path: Option<String>,
}

impl Default for ProfilerConfig {
  fn default() -> Self {
    Self {
      sampling_interval: Duration::from_millis(10),
      max_samples: 10000,
      collect_stacks: true,
      max_stack_depth: 64,
      output_path: None,
    }
  }
}

/// Profiler manager for coordinating multiple profilers
#[derive(Debug)]
pub struct ProfilerManager {
  cpu_profiler: Option<CpuProfiler>,
  heap_profiler: Option<HeapProfiler>,
  gc_profiler: Option<GcProfiler>,
  config: ProfilerConfig,
}

impl ProfilerManager {
  /// Create a new profiler manager
  pub fn new(config: ProfilerConfig) -> Self {
    Self {
      cpu_profiler: None,
      heap_profiler: None,
      gc_profiler: None,
      config,
    }
  }

  /// Initialize CPU profiler
  pub fn init_cpu_profiler(&mut self) -> XProfilerResult<()> {
    self.cpu_profiler = Some(CpuProfiler::new(self.config.clone())?);
    Ok(())
  }

  /// Initialize heap profiler
  pub fn init_heap_profiler(&mut self) -> XProfilerResult<()> {
    self.heap_profiler = Some(HeapProfiler::new(self.config.clone())?);
    Ok(())
  }

  /// Initialize GC profiler
  pub fn init_gc_profiler(&mut self) -> XProfilerResult<()> {
    self.gc_profiler = Some(GcProfiler::new(self.config.clone())?);
    Ok(())
  }

  /// Start all initialized profilers
  pub fn start_all(&mut self) -> XProfilerResult<()> {
    if let Some(ref mut profiler) = self.cpu_profiler {
      profiler.start()?;
    }
    if let Some(ref mut profiler) = self.heap_profiler {
      profiler.start()?;
    }
    if let Some(ref mut profiler) = self.gc_profiler {
      profiler.start()?;
    }
    Ok(())
  }

  /// Stop all running profilers
  pub fn stop_all(&mut self) -> XProfilerResult<()> {
    if let Some(ref mut profiler) = self.cpu_profiler {
      profiler.stop()?;
    }
    if let Some(ref mut profiler) = self.heap_profiler {
      profiler.stop()?;
    }
    if let Some(ref mut profiler) = self.gc_profiler {
      profiler.stop()?;
    }
    Ok(())
  }

  /// Get CPU profiler reference
  pub fn cpu_profiler(&self) -> Option<&CpuProfiler> {
    self.cpu_profiler.as_ref()
  }

  /// Get mutable CPU profiler reference
  pub fn cpu_profiler_mut(&mut self) -> Option<&mut CpuProfiler> {
    self.cpu_profiler.as_mut()
  }

  /// Get heap profiler reference
  pub fn heap_profiler(&self) -> Option<&HeapProfiler> {
    self.heap_profiler.as_ref()
  }

  /// Get mutable heap profiler reference
  pub fn heap_profiler_mut(&mut self) -> Option<&mut HeapProfiler> {
    self.heap_profiler.as_mut()
  }

  /// Get GC profiler reference
  pub fn gc_profiler(&self) -> Option<&GcProfiler> {
    self.gc_profiler.as_ref()
  }

  /// Get mutable GC profiler reference
  pub fn gc_profiler_mut(&mut self) -> Option<&mut GcProfiler> {
    self.gc_profiler.as_mut()
  }
}

impl Default for ProfilerManager {
  fn default() -> Self {
    Self::new(ProfilerConfig::default())
  }
}
