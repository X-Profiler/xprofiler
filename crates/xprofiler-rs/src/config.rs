//! Configuration system for xprofiler-rs
//!
//! This module provides thread-safe configuration management with defaults
//! matching the original C++ implementation.

use napi::bindgen_prelude::*;
use napi_derive::napi;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use crate::constants;

/// Configuration for xprofiler
#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object)]
pub struct XprofilerConfig {
    /// Directory for log files
    #[napi(js_name = "log_dir")]
    pub log_dir: String,

    /// Log interval in seconds (default: 60)
    #[napi(js_name = "log_interval")]
    pub log_interval: u32,

    /// Log level: 0=info, 1=error, 2=debug (default: 1)
    #[napi(js_name = "log_level")]
    pub log_level: u32,

    /// Log type: 0=file, 1=console (default: 0)
    #[napi(js_name = "log_type")]
    pub log_type: u32,

    /// Use Alinode log format (default: false)
    #[napi(js_name = "log_format_alinode")]
    pub log_format_alinode: bool,

    /// Enable HTTP module patching (default: true)
    #[napi(js_name = "patch_http")]
    pub patch_http: bool,

    /// HTTP request timeout in seconds (default: 30)
    #[napi(js_name = "patch_http_timeout")]
    pub patch_http_timeout: u32,

    /// Throw error on check failure (default: true)
    #[napi(js_name = "check_throw")]
    pub check_throw: bool,

    /// Enable fatal error hook (default: true)
    #[napi(js_name = "enable_fatal_error_hook")]
    pub enable_fatal_error_hook: bool,

    /// Enable diagnostic report on fatal error (default: true)
    #[napi(js_name = "enable_fatal_error_report")]
    pub enable_fatal_error_report: bool,

    /// Enable coredump on fatal error (default: false)
    #[napi(js_name = "enable_fatal_error_coredump")]
    pub enable_fatal_error_coredump: bool,

    /// Enable HTTP profiling during CPU sampling (default: false)
    #[napi(js_name = "enable_http_profiling")]
    pub enable_http_profiling: bool,

    /// Enable auto heap limit increase (default: false)
    #[napi(js_name = "enable_auto_incr_heap_limit")]
    pub enable_auto_incr_heap_limit: bool,

    /// Heap limit increase size in MB (default: 256)
    #[napi(js_name = "auto_incr_heap_limit_size")]
    pub auto_incr_heap_limit_size: u32,

    /// Enable RSS leak avoidance via mallopt (default: false)
    #[napi(js_name = "enable_avoid_rss_leak")]
    pub enable_avoid_rss_leak: bool,

    /// Enable libuv handle logging (default: true)
    #[napi(js_name = "enable_log_uv_handles")]
    pub enable_log_uv_handles: bool,

    /// mmap threshold in KB for mallopt (default: 128)
    #[napi(js_name = "m_mmap_threshold")]
    pub m_mmap_threshold: i32,
}

impl Default for XprofilerConfig {
    fn default() -> Self {
        Self {
            log_dir: std::env::temp_dir().to_string_lossy().to_string(),
            log_interval: constants::DEFAULT_LOG_INTERVAL,
            log_level: constants::DEFAULT_LOG_LEVEL,
            log_type: constants::DEFAULT_LOG_TYPE,
            log_format_alinode: false,
            patch_http: true,
            patch_http_timeout: constants::DEFAULT_PATCH_HTTP_TIMEOUT,
            check_throw: true,
            enable_fatal_error_hook: true,
            enable_fatal_error_report: true,
            enable_fatal_error_coredump: false,
            enable_http_profiling: false,
            enable_auto_incr_heap_limit: false,
            auto_incr_heap_limit_size: constants::DEFAULT_AUTO_INCR_HEAP_LIMIT_SIZE,
            enable_avoid_rss_leak: false,
            enable_log_uv_handles: true,
            m_mmap_threshold: constants::DEFAULT_MMAP_THRESHOLD,
        }
    }
}

/// Global configuration store
static CONFIG: Lazy<RwLock<XprofilerConfig>> = Lazy::new(|| RwLock::new(XprofilerConfig::default()));

/// Set the configuration
pub fn set_config(config: XprofilerConfig) -> Result<()> {
    let mut guard = CONFIG.write();
    *guard = config;
    Ok(())
}

/// Configure from array of config items (matching C++ API)
pub fn configure_from_array(items: Vec<crate::ConfigItem>) -> Result<bool> {
    let mut guard = CONFIG.write();

    for item in items {
        match item.name.as_str() {
            "log_dir" => {
                if let Some(s) = item.value.as_str() {
                    guard.log_dir = s.to_string();
                }
            }
            "log_interval" => {
                if let Some(n) = item.value.as_u64() {
                    guard.log_interval = n as u32;
                }
            }
            "log_level" => {
                if let Some(n) = item.value.as_u64() {
                    guard.log_level = n as u32;
                }
            }
            "log_type" => {
                if let Some(n) = item.value.as_u64() {
                    guard.log_type = n as u32;
                }
            }
            "log_format_alinode" => {
                if let Some(b) = item.value.as_bool() {
                    guard.log_format_alinode = b;
                }
            }
            "patch_http" => {
                if let Some(b) = item.value.as_bool() {
                    guard.patch_http = b;
                }
            }
            "patch_http_timeout" => {
                if let Some(n) = item.value.as_u64() {
                    guard.patch_http_timeout = n as u32;
                }
            }
            "check_throw" => {
                if let Some(b) = item.value.as_bool() {
                    guard.check_throw = b;
                }
            }
            "enable_fatal_error_hook" => {
                if let Some(b) = item.value.as_bool() {
                    guard.enable_fatal_error_hook = b;
                }
            }
            "enable_fatal_error_report" => {
                if let Some(b) = item.value.as_bool() {
                    guard.enable_fatal_error_report = b;
                }
            }
            "enable_fatal_error_coredump" => {
                if let Some(b) = item.value.as_bool() {
                    guard.enable_fatal_error_coredump = b;
                }
            }
            "enable_http_profiling" => {
                if let Some(b) = item.value.as_bool() {
                    guard.enable_http_profiling = b;
                }
            }
            "enable_auto_incr_heap_limit" => {
                if let Some(b) = item.value.as_bool() {
                    guard.enable_auto_incr_heap_limit = b;
                }
            }
            "auto_incr_heap_limit_size" => {
                if let Some(n) = item.value.as_u64() {
                    guard.auto_incr_heap_limit_size = n as u32;
                }
            }
            "enable_avoid_rss_leak" => {
                if let Some(b) = item.value.as_bool() {
                    guard.enable_avoid_rss_leak = b;
                }
            }
            "enable_log_uv_handles" => {
                if let Some(b) = item.value.as_bool() {
                    guard.enable_log_uv_handles = b;
                }
            }
            "m_mmap_threshold" => {
                if let Some(n) = item.value.as_i64() {
                    guard.m_mmap_threshold = n as i32;
                }
            }
            _ => {
                // Unknown config key, ignore
            }
        }
    }

    Ok(true)
}

/// Get the current configuration (clone)
pub fn get_config() -> XprofilerConfig {
    CONFIG.read().clone()
}

/// Get a read lock on the configuration
pub fn with_config<F, R>(f: F) -> R
where
    F: FnOnce(&XprofilerConfig) -> R,
{
    let guard = CONFIG.read();
    f(&guard)
}

/// Update configuration with a closure
pub fn update_config<F>(f: F) -> Result<()>
where
    F: FnOnce(&mut XprofilerConfig),
{
    let mut guard = CONFIG.write();
    f(&mut guard);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = XprofilerConfig::default();
        assert_eq!(config.log_interval, 60);
        assert_eq!(config.log_level, 1);
        assert_eq!(config.log_type, 0);
        assert!(config.patch_http);
        assert!(config.enable_fatal_error_hook);
    }

    #[test]
    fn test_set_and_get_config() {
        let mut config = XprofilerConfig::default();
        config.log_interval = 120;
        set_config(config).unwrap();

        let retrieved = get_config();
        assert_eq!(retrieved.log_interval, 120);
    }

    #[test]
    fn test_update_config() {
        update_config(|c| {
            c.log_level = 2;
        })
        .unwrap();

        let config = get_config();
        assert_eq!(config.log_level, 2);
    }
}
