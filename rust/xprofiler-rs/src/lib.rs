//! XProfiler Rust Implementation
//!
//! A high-performance Node.js profiler written in Rust.

pub mod config;
pub mod error;
pub mod environment;
pub mod utils;
pub mod platform;
pub mod monitoring;

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
