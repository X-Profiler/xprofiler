//! xprofiler-rs - Rust implementation of xprofiler Node.js addon
//!
//! This module provides runtime profiling and performance monitoring
//! capabilities for Node.js applications.

#![allow(dead_code)]

mod config;
mod constants;
mod env;
mod error;
mod logger;
mod platform;
mod utils;

use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Setup options passed from JavaScript
#[napi(object)]
pub struct SetupOptions {
    pub thread_id: i64,
    pub is_main_thread: bool,
    pub node_version: String,
}

/// Setup environment data for the current thread/isolate
#[napi]
pub fn setup(options: SetupOptions) -> Result<()> {
    env::setup_environment(options.thread_id, options.is_main_thread, &options.node_version)
}

/// Configure xprofiler with the given configuration object
#[napi]
pub fn configure(config_obj: config::XprofilerConfig) -> Result<()> {
    config::set_config(config_obj)
}

/// Get the current configuration
#[napi]
pub fn get_config() -> config::XprofilerConfig {
    config::get_config()
}

/// Check if the socket path is valid (not too long for Unix sockets)
#[napi]
pub fn check_socket_path(log_dir: String) -> Result<bool> {
    platform::check_socket_path(&log_dir)
}

/// Log an info message
#[napi]
pub fn info(content: String) -> Result<()> {
    logger::info(&content)
}

/// Log an error message
#[napi]
pub fn error(content: String) -> Result<()> {
    logger::error(&content)
}

/// Log a debug message (only if log_level >= 2)
#[napi]
pub fn debug(content: String) -> Result<()> {
    logger::debug(&content)
}

// HTTP tracking APIs

/// Add a live HTTP request
#[napi]
pub fn add_live_request(request_id: String) -> Result<()> {
    env::with_current_env(|env_data| {
        env_data.http_stats.add_live_request(&request_id);
        Ok(())
    })
}

/// Add a closed HTTP request with response time
#[napi]
pub fn add_close_request(request_id: String, rt: f64) -> Result<()> {
    env::with_current_env(|env_data| {
        env_data.http_stats.add_close_request(&request_id, rt);
        Ok(())
    })
}

/// Add a sent HTTP request with response time
#[napi]
pub fn add_sent_request(request_id: String, rt: f64) -> Result<()> {
    env::with_current_env(|env_data| {
        env_data.http_stats.add_sent_request(&request_id, rt);
        Ok(())
    })
}

/// Add a timed out HTTP request
#[napi]
pub fn add_request_timeout(request_id: String) -> Result<()> {
    env::with_current_env(|env_data| {
        env_data.http_stats.add_request_timeout(&request_id);
        Ok(())
    })
}

/// Add an HTTP status code
#[napi]
pub fn add_http_status_code(status_code: u32) -> Result<()> {
    env::with_current_env(|env_data| {
        env_data.http_stats.add_status_code(status_code as u16);
        Ok(())
    })
}

/// Add HTTP profiling detail
#[napi]
pub fn add_http_profiling_detail(detail: String) -> Result<()> {
    env::with_current_env(|env_data| {
        env_data.http_stats.add_profiling_detail(&detail);
        Ok(())
    })
}

/// Initialize mallopt settings (Linux only)
#[napi]
pub fn init_mallopt() -> Result<()> {
    platform::init_mallopt()
}

/// Run the log bypass thread for statistics collection
#[napi]
pub fn run_log_bypass() -> Result<()> {
    // TODO: Implement in Phase 4
    Ok(())
}

/// Run the commands listener thread for IPC
#[napi]
pub fn run_commands_listener() -> Result<()> {
    // TODO: Implement in Phase 3
    Ok(())
}

/// Set V8 hooks (fatal error handler, heap limit)
#[napi]
pub fn set_hooks() -> Result<()> {
    // TODO: Implement in Phase 6
    Ok(())
}

/// Get xprofiler version
#[napi]
pub fn get_xprofiler_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
