//! Utility NAPI bindings
//!
//! This module provides Node.js bindings for utility functions.

use crate::utils::{
  format_bytes, format_duration, get_node_version, get_platform_info, get_process_info,
  get_system_info, get_v8_version,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;

/// JavaScript representation of process information
#[napi(object)]
pub struct JsProcessInfo {
  pub pid: u32,
  pub ppid: u32,
  pub name: String,
  pub command_line: Vec<String>,
  pub working_directory: String,
  pub start_time: f64,
  pub user_time: f64,
  pub system_time: f64,
  pub memory_usage: f64,
  pub cpu_usage: f64,
}

/// JavaScript representation of system information
#[napi(object)]
pub struct JsSystemInfo {
  pub hostname: String,
  pub platform: String,
  pub arch: String,
  pub cpu_count: u32,
  pub total_memory: f64,
  pub free_memory: f64,
  pub uptime: f64,
  pub load_average: Vec<f64>,
}

/// JavaScript representation of platform information
#[napi(object)]
pub struct JsPlatformInfo {
  pub os_type: String,
  pub os_version: String,
  pub kernel_version: String,
  pub architecture: String,
  pub cpu_model: String,
  pub cpu_cores: u32,
  pub cpu_threads: u32,
  pub total_memory_gb: f64,
  pub page_size: u32,
}

/// Get process information
#[napi]
pub fn get_process_info_js() -> Result<JsProcessInfo> {
  let info = get_process_info().map_err(|e| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to get process info: {}", e),
    )
  })?;

  Ok(JsProcessInfo {
    pid: info.pid,
    ppid: info.ppid,
    name: info.name,
    command_line: info.command_line,
    working_directory: info.working_directory,
    start_time: info.start_time.elapsed().as_secs_f64(),
    user_time: info.user_time.as_secs_f64(),
    system_time: info.system_time.as_secs_f64(),
    memory_usage: info.memory_usage as f64,
    cpu_usage: info.cpu_usage,
  })
}

/// Get system information
#[napi]
pub fn get_system_info_js() -> Result<JsSystemInfo> {
  let info = get_system_info().map_err(|e| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to get system info: {}", e),
    )
  })?;

  Ok(JsSystemInfo {
    hostname: info.hostname,
    platform: info.platform,
    arch: info.arch,
    cpu_count: info.cpu_count,
    total_memory: info.total_memory as f64,
    free_memory: info.free_memory as f64,
    uptime: info.uptime.as_secs_f64(),
    load_average: info.load_average,
  })
}

/// Get platform information
#[napi]
pub fn get_platform_info_js() -> Result<JsPlatformInfo> {
  let info = get_platform_info().map_err(|e| {
    Error::new(
      Status::GenericFailure,
      format!("Failed to get platform info: {}", e),
    )
  })?;

  Ok(JsPlatformInfo {
    os_type: info.os_type,
    os_version: info.os_version,
    kernel_version: info.kernel_version,
    architecture: info.architecture,
    cpu_model: info.cpu_model,
    cpu_cores: info.cpu_cores,
    cpu_threads: info.cpu_threads,
    total_memory_gb: info.total_memory_gb,
    page_size: info.page_size,
  })
}

/// Format bytes to human readable string
#[napi]
pub fn format_bytes_js(bytes: f64) -> String {
  format_bytes(bytes as u64)
}

/// Format duration to human readable string
#[napi]
pub fn format_duration_js(seconds: f64) -> String {
  format_duration(std::time::Duration::from_secs_f64(seconds))
}

/// Get Node.js version
#[napi]
pub fn get_node_version_js() -> String {
  get_node_version()
}

/// Get V8 version
#[napi]
pub fn get_v8_version_js() -> String {
  get_v8_version()
}

/// Get current timestamp in milliseconds
#[napi]
pub fn get_timestamp_ms() -> f64 {
  std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap_or_default()
    .as_millis() as f64
}

/// Get current timestamp in seconds
#[napi]
pub fn get_timestamp_seconds() -> f64 {
  std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap_or_default()
    .as_secs_f64()
}

/// Get high resolution timestamp in nanoseconds
#[napi]
pub fn get_hrtime_ns() -> f64 {
  std::time::Instant::now().elapsed().as_nanos() as f64
}

/// Convert nanoseconds to milliseconds
#[napi]
pub fn ns_to_ms(nanoseconds: f64) -> f64 {
  nanoseconds / 1_000_000.0
}

/// Convert milliseconds to nanoseconds
#[napi]
pub fn ms_to_ns(milliseconds: f64) -> f64 {
  milliseconds * 1_000_000.0
}

/// Get memory usage in different units
#[napi(object)]
pub struct JsMemoryUsageFormatted {
  pub bytes: f64,
  pub kb: f64,
  pub mb: f64,
  pub gb: f64,
  pub formatted: String,
}

/// Format memory usage
#[napi]
pub fn format_memory_usage(bytes: f64) -> JsMemoryUsageFormatted {
  JsMemoryUsageFormatted {
    bytes,
    kb: bytes / 1024.0,
    mb: bytes / (1024.0 * 1024.0),
    gb: bytes / (1024.0 * 1024.0 * 1024.0),
    formatted: format_bytes(bytes as u64),
  }
}

/// Get environment variables
#[napi]
pub fn get_env_vars() -> HashMap<String, String> {
  std::env::vars().collect()
}

/// Get specific environment variable
#[napi]
pub fn get_env_var(key: String) -> Option<String> {
  std::env::var(key).ok()
}

/// Check if running in development mode
#[napi]
pub fn is_development() -> bool {
  std::env::var("NODE_ENV").unwrap_or_default().to_lowercase() == "development"
}

/// Check if running in production mode
#[napi]
pub fn is_production() -> bool {
  std::env::var("NODE_ENV").unwrap_or_default().to_lowercase() == "production"
}

/// Get current working directory
#[napi]
pub fn get_cwd() -> Result<String> {
  std::env::current_dir()
    .map(|path| path.to_string_lossy().to_string())
    .map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to get current directory: {}", e),
      )
    })
}

/// Get executable path
#[napi]
pub fn get_executable_path() -> Result<String> {
  std::env::current_exe()
    .map(|path| path.to_string_lossy().to_string())
    .map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to get executable path: {}", e),
      )
    })
}

/// Get command line arguments
#[napi]
pub fn get_args() -> Vec<String> {
  std::env::args().collect()
}

/// Calculate percentage
#[napi]
pub fn calculate_percentage(value: f64, total: f64) -> f64 {
  if total == 0.0 {
    0.0
  } else {
    (value / total) * 100.0
  }
}

/// Calculate average
#[napi]
pub fn calculate_average(values: Vec<f64>) -> f64 {
  if values.is_empty() {
    0.0
  } else {
    values.iter().sum::<f64>() / values.len() as f64
  }
}

/// Calculate median
#[napi]
pub fn calculate_median(mut values: Vec<f64>) -> f64 {
  if values.is_empty() {
    return 0.0;
  }

  values.sort_by(|a, b| a.partial_cmp(b).unwrap());
  let len = values.len();

  if len % 2 == 0 {
    (values[len / 2 - 1] + values[len / 2]) / 2.0
  } else {
    values[len / 2]
  }
}

/// Calculate standard deviation
#[napi]
pub fn calculate_std_deviation(values: Vec<f64>) -> f64 {
  if values.len() < 2 {
    return 0.0;
  }

  let mean = calculate_average(values.clone());
  let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (values.len() - 1) as f64;

  variance.sqrt()
}

/// Get min and max from values
#[napi(object)]
pub struct JsMinMax {
  pub min: f64,
  pub max: f64,
}

/// Calculate min and max
#[napi]
pub fn calculate_min_max(values: Vec<f64>) -> Option<JsMinMax> {
  if values.is_empty() {
    return None;
  }

  let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
  let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

  Some(JsMinMax { min, max })
}

/// Sleep for specified milliseconds (blocking)
#[napi]
pub fn sleep_ms(milliseconds: u32) {
  std::thread::sleep(std::time::Duration::from_millis(milliseconds as u64));
}

/// Generate UUID v4
#[napi]
pub fn generate_uuid() -> String {
  use std::fmt::Write;
  use std::time::{SystemTime, UNIX_EPOCH};

  let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_nanos();

  let mut uuid = String::with_capacity(36);
  let _ = write!(
    uuid,
    "{:08x}-{:04x}-4{:03x}-{:04x}-{:012x}",
    (timestamp >> 96) as u32,
    ((timestamp >> 80) & 0xFFFF) as u16,
    ((timestamp >> 64) & 0x0FFF) as u16,
    ((timestamp >> 48) & 0x3FFF | 0x8000) as u16,
    (timestamp & 0xFFFFFFFFFFFF) as u64
  );

  uuid
}
