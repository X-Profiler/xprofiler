//! Unix-specific platform implementations

use napi::bindgen_prelude::*;
use std::mem::MaybeUninit;

use super::CpuUsage;

/// Get CPU usage on Unix using getrusage
pub fn get_cpu_usage_impl() -> CpuUsage {
    unsafe {
        let mut usage = MaybeUninit::<libc::rusage>::uninit();
        if libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) == 0 {
            let usage = usage.assume_init();
            CpuUsage {
                user_time: usage.ru_utime.tv_sec as f64
                    + usage.ru_utime.tv_usec as f64 / 1_000_000.0,
                system_time: usage.ru_stime.tv_sec as f64
                    + usage.ru_stime.tv_usec as f64 / 1_000_000.0,
                ..Default::default()
            }
        } else {
            CpuUsage::default()
        }
    }
}

/// Initialize mallopt settings (Linux only)
#[cfg(target_os = "linux")]
pub fn init_mallopt_impl() -> Result<()> {
    let cfg = crate::config::get_config();

    if cfg.enable_avoid_rss_leak {
        // M_MMAP_THRESHOLD = -3
        const M_MMAP_THRESHOLD: i32 = -3;
        let threshold = cfg.m_mmap_threshold * 1024; // Convert KB to bytes

        unsafe {
            libc::mallopt(M_MMAP_THRESHOLD, threshold);
        }
    }

    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn init_mallopt_impl() -> Result<()> {
    Ok(())
}

/// Get the IPC socket path for this process
pub fn get_ipc_socket_path(log_dir: &str) -> String {
    format!(
        "{}/xprofiler-uds-path-{}.sock",
        log_dir,
        std::process::id()
    )
}

/// Get the control IPC socket path
pub fn get_ctl_socket_path(log_dir: &str) -> String {
    format!("{}/xprofiler-ctl-uds-path.sock", log_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cpu_usage() {
        let usage = get_cpu_usage_impl();
        // CPU times should be non-negative
        assert!(usage.user_time >= 0.0);
        assert!(usage.system_time >= 0.0);
    }

    #[test]
    fn test_get_ipc_socket_path() {
        let path = get_ipc_socket_path("/tmp");
        assert!(path.starts_with("/tmp/xprofiler-uds-path-"));
        assert!(path.ends_with(".sock"));
    }
}
