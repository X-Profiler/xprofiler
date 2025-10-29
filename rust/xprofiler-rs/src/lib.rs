//! XProfiler Rust Implementation
//!
//! A high-performance Node.js profiler written in Rust.

pub mod config;
pub mod environment;
pub mod error;
pub mod monitoring;
pub mod platform;
pub mod profiler;
pub mod utils;

#[cfg(feature = "napi")]
pub mod bindings;

// Re-export commonly used types
pub use error::{XProfilerError, XProfilerResult};

/// Initialize the profiler
#[cfg(feature = "napi")]
pub fn init() -> napi::Result<()> {
  env_logger::init();
  log::info!("XProfiler initialized");
  Ok(())
}
