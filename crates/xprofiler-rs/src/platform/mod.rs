//! Platform abstraction layer for xprofiler-rs
//!
//! This module provides cross-platform implementations for:
//! - CPU usage collection
//! - IPC (Unix sockets / Windows named pipes)
//! - Socket path validation
//! - mallopt configuration

use napi::bindgen_prelude::*;

#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub use unix::{get_ctl_socket_path, get_ipc_socket_path};

#[cfg(windows)]
pub use windows::{get_ctl_pipe_path, get_ipc_pipe_path};

use crate::constants;
use crate::utils;

/// Check if the socket path is valid (not too long for Unix sockets)
pub fn check_socket_path(log_dir: &str) -> Result<bool> {
    #[cfg(unix)]
    {
        let path = format!(
            "{}/xprofiler-uds-path-{}.sock",
            log_dir,
            utils::get_pid()
        );
        Ok(path.len() < constants::MAX_UNIX_SOCKET_PATH_LEN)
    }

    #[cfg(windows)]
    {
        // Windows named pipes don't have the same path length constraints
        let _ = log_dir;
        Ok(true)
    }
}

/// Initialize mallopt settings (Linux only, no-op on other platforms)
pub fn init_mallopt() -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        unix::init_mallopt_impl()
    }

    #[cfg(not(target_os = "linux"))]
    {
        Ok(())
    }
}

/// CPU usage statistics
#[derive(Debug, Clone, Default)]
pub struct CpuUsage {
    pub user_time: f64,
    pub system_time: f64,
    pub cpu_now: f64,
    pub cpu_15: f64,
    pub cpu_30: f64,
    pub cpu_60: f64,
}

/// Get current CPU usage
pub fn get_cpu_usage() -> CpuUsage {
    #[cfg(unix)]
    {
        unix::get_cpu_usage_impl()
    }

    #[cfg(windows)]
    {
        windows::get_cpu_usage_impl()
    }
}
