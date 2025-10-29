//! NAPI bindings for exposing Rust monitoring functionality to Node.js
//!
//! This module provides JavaScript bindings for all monitoring capabilities.

// Export CPU monitoring bindings
pub mod cpu;
pub use cpu::*;

// Export memory monitoring bindings
pub mod memory;
pub use memory::{
  format_memory_usage as format_memory_usage_stats, get_heap_stats, get_memory_usage,
  get_memory_usage_mb, reset_memory_monitor, start_memory_monitoring, stop_memory_monitoring,
};

// Export GC monitoring bindings
pub mod gc;
pub use gc::*;

// Export HTTP monitoring bindings
pub mod http;
pub use http::*;

// Export libuv monitoring bindings
pub mod libuv;
pub use libuv::*;

// Export utility bindings
pub mod utils;
pub use utils::{
  format_bytes_js, format_duration_js, format_memory_usage as format_memory_bytes, get_hrtime_ns,
  get_node_version_js, get_timestamp_ms, get_timestamp_seconds, get_v8_version_js, ms_to_ns,
  ns_to_ms,
};

// Export profiler bindings
pub mod profiler;
pub use profiler::*;
