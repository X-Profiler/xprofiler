//! CPU Profiler implementation
//!
//! This module provides sampling-based CPU profiling capabilities,
//! including call stack collection and performance hotspot identification.

use crate::error::{MonitorType, XProfilerError, XProfilerResult};
use crate::profiler::{Profiler, ProfilerConfig};
use backtrace::Backtrace;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a single stack frame
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StackFrame {
  /// Function name
  pub function_name: String,
  /// File name
  pub file_name: Option<String>,
  /// Line number
  pub line_number: Option<u32>,
  /// Column number
  pub column_number: Option<u32>,
}

impl StackFrame {
  pub fn new(function_name: String) -> Self {
    Self {
      function_name,
      file_name: None,
      line_number: None,
      column_number: None,
    }
  }

  pub fn with_location(
    mut self,
    file_name: String,
    line_number: u32,
    column_number: Option<u32>,
  ) -> Self {
    self.file_name = Some(file_name);
    self.line_number = Some(line_number);
    self.column_number = column_number;
    self
  }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
pub use tests::*;

/// Represents a complete call stack
#[derive(Debug, Clone)]
pub struct CallStack {
  /// Stack frames from bottom to top
  pub frames: Vec<StackFrame>,
  /// Timestamp when this stack was captured
  pub timestamp: u64,
  /// Thread ID
  pub thread_id: u64,
}

impl CallStack {
  pub fn new(frames: Vec<StackFrame>, thread_id: u64) -> Self {
    let timestamp = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap_or_default()
      .as_nanos() as u64;

    Self {
      frames,
      timestamp,
      thread_id,
    }
  }

  /// Get the top frame (currently executing function)
  pub fn top_frame(&self) -> Option<&StackFrame> {
    self.frames.last()
  }

  /// Get stack depth
  pub fn depth(&self) -> usize {
    self.frames.len()
  }
}

/// CPU profiling sample
#[derive(Debug, Clone)]
pub struct CpuSample {
  /// Call stack at the time of sampling
  pub call_stack: CallStack,
  /// CPU usage percentage at sampling time
  pub cpu_usage: f64,
  /// Memory usage in bytes
  pub memory_usage: u64,
}

/// CPU profiling statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuProfileStats {
  /// Total number of samples collected
  pub total_samples: usize,
  /// Profiling duration in milliseconds
  pub duration_ms: u64,
  /// Average CPU usage
  pub avg_cpu_usage: f64,
  /// Peak CPU usage
  pub peak_cpu_usage: f64,
  /// Function call frequency map
  pub function_frequency: HashMap<String, usize>,
  /// Hot functions (top 10 by frequency)
  pub hot_functions: Vec<(String, usize)>,
}

/// CPU Profiler implementation
#[derive(Debug)]
pub struct CpuProfiler {
  config: ProfilerConfig,
  is_running: bool,
  samples: Arc<Mutex<Vec<CpuSample>>>,
  start_time: Option<Instant>,
  sampling_thread: Option<thread::JoinHandle<()>>,
  stop_signal: Arc<Mutex<bool>>,
}

impl CpuProfiler {
  /// Create a new CPU profiler
  pub fn new(config: ProfilerConfig) -> XProfilerResult<Self> {
    Ok(Self {
      config,
      is_running: false,
      samples: Arc::new(Mutex::new(Vec::new())),
      start_time: None,
      sampling_thread: None,
      stop_signal: Arc::new(Mutex::new(false)),
    })
  }

  /// Get current call stack (platform-specific implementation)
  fn get_current_call_stack(&self) -> XProfilerResult<CallStack> {
    // This is a simplified implementation
    // In a real implementation, you would use platform-specific APIs
    // like backtrace, libunwind, or Windows StackWalk

    let thread_id = self.get_current_thread_id();
    let frames = self.collect_stack_frames()?;

    Ok(CallStack::new(frames, thread_id))
  }

  /// Get current thread ID
  fn get_current_thread_id(&self) -> u64 {
    // Platform-specific thread ID retrieval
    #[cfg(unix)]
    {
      unsafe { libc::pthread_self() as u64 }
    }

    #[cfg(windows)]
    {
      unsafe { winapi::um::processthreadsapi::GetCurrentThreadId() as u64 }
    }

    #[cfg(not(any(unix, windows)))]
    {
      // Fallback for other platforms
      std::thread::current().id().as_u64().get()
    }
  }

  /// Collect stack frames using backtrace
  fn collect_stack_frames(&self) -> XProfilerResult<Vec<StackFrame>> {
    let mut frames = Vec::new();

    // Use backtrace crate for cross-platform stack walking
    backtrace::trace(|frame| {
      if frames.len() >= self.config.max_stack_depth {
        return false; // Stop collecting
      }

      backtrace::resolve_frame(frame, |symbol| {
        let function_name = symbol
          .name()
          .map(|name| format!("{}", name))
          .unwrap_or_else(|| "<unknown>".to_string());

        let mut stack_frame = StackFrame::new(function_name);

        if let Some(filename) = symbol.filename() {
          if let Some(lineno) = symbol.lineno() {
            stack_frame = stack_frame.with_location(
              filename.to_string_lossy().to_string(),
              lineno,
              symbol.colno(),
            );
          }
        }

        frames.push(stack_frame);
      });

      true // Continue collecting
    });

    Ok(frames)
  }

  /// Convert backtrace to call stack
  fn backtrace_to_call_stack(backtrace: &Backtrace) -> CallStack {
    let mut frames = Vec::new();

    // Parse backtrace frames
    let backtrace_str = format!("{:?}", backtrace);
    let lines: Vec<&str> = backtrace_str.lines().collect();

    for (_i, line) in lines.iter().enumerate().take(32) {
      // Simple parsing of backtrace output
      // Format is typically: "   0: function_name at file:line"
      if let Some(parts) = line.split_once(":") {
        let content = parts.1.trim();

        let (function_name, location) = if let Some(at_pos) = content.find(" at ") {
          let func = content[..at_pos].trim();
          let loc = content[at_pos + 4..].trim();
          (func.to_string(), loc.to_string())
        } else {
          (content.to_string(), "<unknown>".to_string())
        };

        let (file_name, line_number) = if let Some(colon_pos) = location.rfind(':') {
          let file = location[..colon_pos].to_string();
          let line = location[colon_pos + 1..].parse::<u32>().unwrap_or(0);
          (Some(file), Some(line))
        } else {
          (Some(location), Some(0))
        };

        let mut stack_frame = StackFrame::new(function_name);
        if let (Some(file), Some(line)) = (file_name, line_number) {
          stack_frame = stack_frame.with_location(file, line, None);
        }

        frames.push(stack_frame);
      }
    }

    CallStack::new(frames, 0)
  }

  /// Get current CPU usage
  fn get_current_cpu_usage(&self) -> XProfilerResult<f64> {
    // This would typically use platform-specific APIs
    // For now, return a placeholder value
    Ok(0.0)
  }

  /// Get current memory usage
  fn get_current_memory_usage(&self) -> XProfilerResult<u64> {
    // This would typically use platform-specific APIs
    // For now, return a placeholder value
    Ok(0)
  }

  /// Sampling loop that runs in a separate thread
  fn sampling_loop(
    samples: Arc<Mutex<Vec<CpuSample>>>,
    config: ProfilerConfig,
    stop_signal: Arc<Mutex<bool>>,
  ) {
    let profiler = match Self::new(config.clone()) {
      Ok(p) => p,
      Err(_) => return,
    };

    loop {
      // Check stop signal
      if let Ok(should_stop) = stop_signal.lock() {
        if *should_stop {
          break;
        }
      }

      // Collect sample
      if let Ok(call_stack) = profiler.get_current_call_stack() {
        if let (Ok(cpu_usage), Ok(memory_usage)) = (
          profiler.get_current_cpu_usage(),
          profiler.get_current_memory_usage(),
        ) {
          let sample = CpuSample {
            call_stack,
            cpu_usage,
            memory_usage,
          };

          if let Ok(mut samples_guard) = samples.lock() {
            if samples_guard.len() < config.max_samples {
              samples_guard.push(sample);
            }
          }
        }
      }

      // Sleep for sampling interval
      thread::sleep(config.sampling_interval);
    }
  }

  /// Generate profiling statistics
  pub fn get_stats(&self) -> XProfilerResult<CpuProfileStats> {
    let samples = self
      .samples
      .lock()
      .map_err(|_| XProfilerError::Monitoring {
        message: "Failed to lock samples".to_string(),
        monitor_type: MonitorType::CpuProfiler,
      })?;

    let total_samples = samples.len();
    if total_samples == 0 {
      return Ok(CpuProfileStats {
        total_samples: 0,
        duration_ms: 0,
        avg_cpu_usage: 0.0,
        peak_cpu_usage: 0.0,
        function_frequency: HashMap::new(),
        hot_functions: Vec::new(),
      });
    }

    let duration_ms = self
      .start_time
      .map(|start| start.elapsed().as_millis() as u64)
      .unwrap_or(0);

    // Calculate CPU usage statistics
    let cpu_usages: Vec<f64> = samples.iter().map(|s| s.cpu_usage).collect();
    let avg_cpu_usage = cpu_usages.iter().sum::<f64>() / cpu_usages.len() as f64;
    let peak_cpu_usage = cpu_usages.iter().fold(0.0f64, |max, &val| max.max(val));

    // Calculate function frequency
    let mut function_frequency = HashMap::new();
    for sample in samples.iter() {
      for frame in &sample.call_stack.frames {
        *function_frequency
          .entry(frame.function_name.clone())
          .or_insert(0) += 1;
      }
    }

    // Get hot functions (top 10)
    let mut hot_functions: Vec<(String, usize)> = function_frequency
      .iter()
      .map(|(name, count)| (name.clone(), *count))
      .collect();
    hot_functions.sort_by(|a, b| b.1.cmp(&a.1));
    hot_functions.truncate(10);

    Ok(CpuProfileStats {
      total_samples,
      duration_ms,
      avg_cpu_usage,
      peak_cpu_usage,
      function_frequency,
      hot_functions,
    })
  }

  /// Get all collected samples
  pub fn get_samples(&self) -> XProfilerResult<Vec<CpuSample>> {
    let samples = self
      .samples
      .lock()
      .map_err(|_| XProfilerError::Monitoring {
        message: "Failed to lock samples".to_string(),
        monitor_type: MonitorType::CpuProfiler,
      })?;
    Ok(samples.clone())
  }
}

impl Profiler for CpuProfiler {
  fn start(&mut self) -> XProfilerResult<()> {
    if self.is_running {
      return Err(XProfilerError::Monitoring {
        message: "CPU profiler is already running".to_string(),
        monitor_type: MonitorType::CpuProfiler,
      });
    }

    // Reset state
    self
      .samples
      .lock()
      .map_err(|_| XProfilerError::Monitoring {
        message: "Failed to lock samples".to_string(),
        monitor_type: MonitorType::CpuProfiler,
      })?
      .clear();

    *self
      .stop_signal
      .lock()
      .map_err(|_| XProfilerError::Monitoring {
        message: "Failed to lock stop signal".to_string(),
        monitor_type: MonitorType::CpuProfiler,
      })? = false;

    self.start_time = Some(Instant::now());

    // Start sampling thread
    let samples = Arc::clone(&self.samples);
    let config = self.config.clone();
    let stop_signal = Arc::clone(&self.stop_signal);

    let handle = thread::spawn(move || {
      Self::sampling_loop(samples, config, stop_signal);
    });

    self.sampling_thread = Some(handle);
    self.is_running = true;

    Ok(())
  }

  fn stop(&mut self) -> XProfilerResult<()> {
    if !self.is_running {
      return Err(XProfilerError::Monitoring {
        message: "CPU profiler is not running".to_string(),
        monitor_type: MonitorType::CpuProfiler,
      });
    }

    // Signal sampling thread to stop
    *self
      .stop_signal
      .lock()
      .map_err(|_| XProfilerError::Monitoring {
        message: "Failed to lock stop signal".to_string(),
        monitor_type: MonitorType::CpuProfiler,
      })? = true;

    // Wait for sampling thread to finish
    if let Some(handle) = self.sampling_thread.take() {
      handle.join().map_err(|_| XProfilerError::Monitoring {
        message: "Failed to join sampling thread".to_string(),
        monitor_type: MonitorType::CpuProfiler,
      })?;
    }

    self.is_running = false;
    Ok(())
  }

  fn is_running(&self) -> bool {
    self.is_running
  }

  fn get_results(&self) -> XProfilerResult<String> {
    let stats = self.get_stats()?;
    serde_json::to_string_pretty(&stats).map_err(|e| XProfilerError::Monitoring {
      message: format!("Failed to serialize CPU profile results: {}", e),
      monitor_type: MonitorType::CpuProfiler,
    })
  }

  fn reset(&mut self) -> XProfilerResult<()> {
    if self.is_running {
      self.stop()?;
    }

    self
      .samples
      .lock()
      .map_err(|_| XProfilerError::Monitoring {
        message: "Failed to lock samples".to_string(),
        monitor_type: MonitorType::CpuProfiler,
      })?
      .clear();

    self.start_time = None;

    Ok(())
  }
}
