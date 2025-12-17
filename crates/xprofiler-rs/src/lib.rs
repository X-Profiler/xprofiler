//! xprofiler-rs - Rust implementation of xprofiler Node.js addon
//!
//! This module provides runtime profiling and performance monitoring
//! capabilities for Node.js applications.

#![allow(dead_code)]

mod commands;
mod config;
mod constants;
mod coredump;
mod env;
mod error;
mod hooks;
mod ipc;
mod logbypass;
mod logger;
mod platform;
mod profilers;
mod utils;

use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Setup options passed from JavaScript
#[napi(object)]
pub struct SetupOptions {
    #[napi(js_name = "threadId")]
    pub thread_id: i64,
    #[napi(js_name = "isMainThread")]
    pub is_main_thread: bool,
    #[napi(js_name = "nodeVersion")]
    pub node_version: String,
}

/// Setup environment data for the current thread/isolate
#[napi]
pub fn setup(options: SetupOptions) -> Result<()> {
    env::setup_environment(options.thread_id, options.is_main_thread, &options.node_version)
}

/// Configuration item from JavaScript (array element)
#[napi(object)]
pub struct ConfigItem {
    pub name: String,
    pub format: String,
    pub value: serde_json::Value,
    #[napi(js_name = "configurable")]
    pub configurable: bool,
}

/// Configure xprofiler with the given configuration array
#[napi]
pub fn configure(config_array: Vec<ConfigItem>) -> Result<bool> {
    config::configure_from_array(config_array)
}

/// Get the current configuration
#[napi]
pub fn get_config() -> config::XprofilerConfig {
    config::get_config()
}

/// Check if the socket path is valid (not too long for Unix sockets)
/// log_error: whether to log an error if the path is invalid
#[napi]
pub fn check_socket_path(log_error: bool) -> Result<bool> {
    let cfg = config::get_config();
    let result = platform::check_socket_path(&cfg.log_dir)?;

    if !result && log_error {
        let _ = logger::error(&format!(
            "Socket path is too long. log_dir: {}",
            cfg.log_dir
        ));
    }

    Ok(result)
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

/// HTTP config object from JavaScript
#[napi(object)]
pub struct HttpConfig {
    #[napi(js_name = "http_detail_profiling")]
    pub http_detail_profiling: bool,
    #[napi(js_name = "start_time")]
    pub start_time: f64,
}

/// Set HTTP config (called from patch/http.js)
#[napi]
pub fn set_http_config(_config: HttpConfig) -> Result<()> {
    // Store the HTTP config for profiling
    // For now, we just acknowledge it - the actual profiling happens in env stats
    // TODO: Store http_detail_profiling flag if needed
    Ok(())
}

/// Add a live HTTP request
#[napi]
pub fn add_live_request() -> Result<()> {
    env::with_current_env(|env_data| {
        env_data.http_stats.add_live_request("");
        Ok(())
    })
}

/// Add a closed HTTP request
#[napi]
pub fn add_close_request() -> Result<()> {
    env::with_current_env(|env_data| {
        env_data.http_stats.add_close_request("", 0.0);
        Ok(())
    })
}

/// Add a sent HTTP request with response time
#[napi]
pub fn add_sent_request(rt: f64) -> Result<()> {
    env::with_current_env(|env_data| {
        env_data.http_stats.add_sent_request("", rt);
        Ok(())
    })
}

/// Add a timed out HTTP request
#[napi]
pub fn add_request_timeout() -> Result<()> {
    env::with_current_env(|env_data| {
        env_data.http_stats.add_request_timeout("");
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
    logbypass::start_log_bypass()
        .map_err(|e| Error::from_reason(e))
}

/// Run the commands listener thread for IPC
#[napi]
pub fn run_commands_listener() -> Result<()> {
    commands::listener::start_commands_listener()
        .map_err(|e| Error::from_reason(e))
}

/// Set V8 hooks (fatal error handler, heap limit)
///
/// Note: Due to napi-rs limitations, actual V8 hook registration is not possible.
/// This function logs the configuration state for compatibility.
/// For heap management, use --max-old-space-size or NODE_OPTIONS.
#[napi]
pub fn set_hooks() -> Result<()> {
    hooks::set_hooks_impl()
}

/// Get xprofiler version
#[napi]
pub fn get_xprofiler_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
