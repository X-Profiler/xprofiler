//! NAPI bindings for exposing Rust monitoring functionality to Node.js
//!
//! This module provides JavaScript bindings for all monitoring capabilities.

// Export CPU monitoring bindings
pub mod cpu;
pub use cpu::*;

// Export memory monitoring bindings
pub mod memory;
pub use memory::{
    start_memory_monitoring, stop_memory_monitoring, get_memory_usage,
    get_memory_usage_mb, get_heap_stats, reset_memory_monitor,
    format_memory_usage as format_memory_usage_stats
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
    get_timestamp_ms, get_timestamp_seconds, get_hrtime_ns,
    ns_to_ms, ms_to_ns, format_memory_usage as format_memory_bytes,
    format_bytes_js, format_duration_js, get_node_version_js, get_v8_version_js
};