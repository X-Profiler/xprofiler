//! Windows-specific platform implementations

use super::CpuUsage;

/// Get CPU usage on Windows
pub fn get_cpu_usage_impl() -> CpuUsage {
    // TODO: Implement using Windows Performance Counters
    // For now, return default values
    CpuUsage::default()
}

/// Get the IPC named pipe path for this process
pub fn get_ipc_pipe_path() -> String {
    format!(r"\\.\pipe\xprofiler-named-pipe-{}", std::process::id())
}

/// Get the control IPC named pipe path
pub fn get_ctl_pipe_path() -> String {
    r"\\.\pipe\xprofiler-ctl".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ipc_pipe_path() {
        let path = get_ipc_pipe_path();
        assert!(path.starts_with(r"\\.\pipe\xprofiler-named-pipe-"));
    }

    #[test]
    fn test_get_ctl_pipe_path() {
        let path = get_ctl_pipe_path();
        assert_eq!(path, r"\\.\pipe\xprofiler-ctl");
    }
}
