//! Utility functions and helpers
//!
//! This module provides various utility functions for system information,
//! formatting, and common operations.

use std::time::{Duration, Instant};
// use std::collections::HashMap; // Commented out unused import

/// Process information
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub command_line: Vec<String>,
    pub working_directory: String,
    pub start_time: Instant,
    pub user_time: Duration,
    pub system_time: Duration,
    pub memory_usage: u64,
    pub cpu_usage: f64,
}

/// System information
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub hostname: String,
    pub platform: String,
    pub arch: String,
    pub cpu_count: u32,
    pub total_memory: u64,
    pub free_memory: u64,
    pub uptime: Duration,
    pub load_average: Vec<f64>,
}

/// Platform information
#[derive(Debug, Clone)]
pub struct PlatformInfo {
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

/// Get current process information
pub fn get_process_info() -> Result<ProcessInfo, Box<dyn std::error::Error>> {
    let pid = std::process::id();
    
    #[cfg(unix)]
    {
        get_process_info_unix(pid)
    }
    
    #[cfg(windows)]
    {
        get_process_info_windows(pid)
    }
}

/// Get system information
pub fn get_system_info() -> Result<SystemInfo, Box<dyn std::error::Error>> {
    #[cfg(unix)]
    {
        get_system_info_unix()
    }
    
    #[cfg(windows)]
    {
        get_system_info_windows()
    }
}

/// Get platform information
pub fn get_platform_info() -> Result<PlatformInfo, Box<dyn std::error::Error>> {
    #[cfg(unix)]
    {
        get_platform_info_unix()
    }
    
    #[cfg(windows)]
    {
        get_platform_info_windows()
    }
}

/// Format bytes to human readable string
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    const THRESHOLD: f64 = 1024.0;
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let bytes_f = bytes as f64;
    let unit_index = (bytes_f.log10() / THRESHOLD.log10()).floor() as usize;
    let unit_index = unit_index.min(UNITS.len() - 1);
    
    let value = bytes_f / THRESHOLD.powi(unit_index as i32);
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.2} {}", value, UNITS[unit_index])
    }
}

/// Format duration to human readable string
pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    
    if total_seconds < 60 {
        format!("{:.2}s", duration.as_secs_f64())
    } else if total_seconds < 3600 {
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{}m {}s", minutes, seconds)
    } else if total_seconds < 86400 {
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        format!("{}h {}m", hours, minutes)
    } else {
        let days = total_seconds / 86400;
        let hours = (total_seconds % 86400) / 3600;
        format!("{}d {}h", days, hours)
    }
}

/// Get Node.js version
pub fn get_node_version() -> String {
    std::env::var("NODE_VERSION")
        .or_else(|_| std::env::var("npm_config_node_version"))
        .unwrap_or_else(|_| "unknown".to_string())
}

/// Get V8 version
pub fn get_v8_version() -> String {
    // This would typically be obtained from Node.js process.versions.v8
    // For now, return a placeholder
    "unknown".to_string()
}

#[cfg(unix)]
fn get_process_info_unix(pid: u32) -> Result<ProcessInfo, Box<dyn std::error::Error>> {
    use std::fs;
    
    let proc_path = format!("/proc/{}", pid);
    
    // Read process name
    let comm_path = format!("{}/comm", proc_path);
    let name = fs::read_to_string(comm_path)
        .unwrap_or_else(|_| "unknown".to_string())
        .trim()
        .to_string();
    
    // Read command line
    let cmdline_path = format!("{}/cmdline", proc_path);
    let cmdline_raw = fs::read(cmdline_path).unwrap_or_default();
    let command_line: Vec<String> = cmdline_raw
        .split(|&b| b == 0)
        .filter(|s| !s.is_empty())
        .map(|s| String::from_utf8_lossy(s).to_string())
        .collect();
    
    // Read status for PPID
    let status_path = format!("{}/status", proc_path);
    let status_content = fs::read_to_string(status_path).unwrap_or_default();
    let ppid = status_content
        .lines()
        .find(|line| line.starts_with("PPid:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    
    // Get working directory
    let cwd_path = format!("{}/cwd", proc_path);
    let working_directory = fs::read_link(cwd_path)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "/".to_string()));
    
    // Read memory usage from statm
    let statm_path = format!("{}/statm", proc_path);
    let statm_content = fs::read_to_string(statm_path).unwrap_or_default();
    let memory_pages: u64 = statm_content
        .split_whitespace()
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as u64;
    let memory_usage = memory_pages * page_size;
    
    Ok(ProcessInfo {
        pid,
        ppid,
        name,
        command_line,
        working_directory,
        start_time: Instant::now(), // Placeholder
        user_time: Duration::ZERO,  // Would need to parse /proc/pid/stat
        system_time: Duration::ZERO, // Would need to parse /proc/pid/stat
        memory_usage,
        cpu_usage: 0.0, // Would need to calculate from /proc/pid/stat
    })
}

#[cfg(unix)]
fn get_system_info_unix() -> Result<SystemInfo, Box<dyn std::error::Error>> {
    use std::fs;
    
    // Get hostname
    let hostname = fs::read_to_string("/proc/sys/kernel/hostname")
        .unwrap_or_else(|_| "unknown".to_string())
        .trim()
        .to_string();
    
    // Get platform and architecture
    let platform = std::env::consts::OS.to_string();
    let arch = std::env::consts::ARCH.to_string();
    
    // Get CPU count
    let cpu_count = num_cpus::get() as u32;
    
    // Get memory info
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let mut total_memory = 0u64;
    let mut free_memory = 0u64;
    
    for line in meminfo.lines() {
        if line.starts_with("MemTotal:") {
            if let Some(value) = line.split_whitespace().nth(1) {
                total_memory = value.parse::<u64>().unwrap_or(0) * 1024; // Convert KB to bytes
            }
        } else if line.starts_with("MemAvailable:") {
            if let Some(value) = line.split_whitespace().nth(1) {
                free_memory = value.parse::<u64>().unwrap_or(0) * 1024; // Convert KB to bytes
            }
        }
    }
    
    // Get uptime
    let uptime_str = fs::read_to_string("/proc/uptime").unwrap_or_default();
    let uptime_seconds: f64 = uptime_str
        .split_whitespace()
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0);
    let uptime = Duration::from_secs_f64(uptime_seconds);
    
    // Get load average
    let loadavg_str = fs::read_to_string("/proc/loadavg").unwrap_or_default();
    let load_average: Vec<f64> = loadavg_str
        .split_whitespace()
        .take(3)
        .filter_map(|s| s.parse().ok())
        .collect();
    
    Ok(SystemInfo {
        hostname,
        platform,
        arch,
        cpu_count,
        total_memory,
        free_memory,
        uptime,
        load_average,
    })
}

#[cfg(unix)]
fn get_platform_info_unix() -> Result<PlatformInfo, Box<dyn std::error::Error>> {
    use std::fs;
    
    // Get OS information
    let os_release = fs::read_to_string("/etc/os-release")
        .or_else(|_| fs::read_to_string("/usr/lib/os-release"))
        .unwrap_or_default();
    
    let mut os_type = "Linux".to_string();
    let mut os_version = "unknown".to_string();
    
    for line in os_release.lines() {
        if line.starts_with("NAME=") {
            os_type = line.split('=').nth(1)
                .unwrap_or("Linux")
                .trim_matches('"')
                .to_string();
        } else if line.starts_with("VERSION=") {
            os_version = line.split('=').nth(1)
                .unwrap_or("unknown")
                .trim_matches('"')
                .to_string();
        }
    }
    
    // Get kernel version
    let kernel_version = fs::read_to_string("/proc/version")
        .unwrap_or_else(|_| "unknown".to_string())
        .split_whitespace()
        .nth(2)
        .unwrap_or("unknown")
        .to_string();
    
    // Get CPU information
    let cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    let mut cpu_model = "unknown".to_string();
    let mut cpu_cores = 0u32;
    
    for line in cpuinfo.lines() {
        if line.starts_with("model name") {
            cpu_model = line.split(':').nth(1)
                .unwrap_or("unknown")
                .trim()
                .to_string();
        } else if line.starts_with("cpu cores") {
            cpu_cores = line.split(':').nth(1)
                .and_then(|s| s.trim().parse().ok())
                .unwrap_or(0);
        }
    }
    
    let cpu_threads = num_cpus::get() as u32;
    if cpu_cores == 0 {
        cpu_cores = cpu_threads;
    }
    
    // Get memory information
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let total_memory_kb: u64 = meminfo
        .lines()
        .find(|line| line.starts_with("MemTotal:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let total_memory_gb = (total_memory_kb as f64) / (1024.0 * 1024.0);
    
    // Get page size
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as u32;
    
    Ok(PlatformInfo {
        os_type,
        os_version,
        kernel_version,
        architecture: std::env::consts::ARCH.to_string(),
        cpu_model,
        cpu_cores,
        cpu_threads,
        total_memory_gb,
        page_size,
    })
}

#[cfg(windows)]
fn get_process_info_windows(pid: u32) -> Result<ProcessInfo, Box<dyn std::error::Error>> {
    // Windows implementation would use Windows API
    // For now, return a basic implementation
    Ok(ProcessInfo {
        pid,
        ppid: 0,
        name: "unknown".to_string(),
        command_line: vec![],
        working_directory: std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "C:\\".to_string()),
        start_time: Instant::now(),
        user_time: Duration::ZERO,
        system_time: Duration::ZERO,
        memory_usage: 0,
        cpu_usage: 0.0,
    })
}

#[cfg(windows)]
fn get_system_info_windows() -> Result<SystemInfo, Box<dyn std::error::Error>> {
    // Windows implementation would use Windows API
    // For now, return a basic implementation
    Ok(SystemInfo {
        hostname: std::env::var("COMPUTERNAME").unwrap_or_else(|_| "unknown".to_string()),
        platform: "windows".to_string(),
        arch: std::env::consts::ARCH.to_string(),
        cpu_count: num_cpus::get() as u32,
        total_memory: 0,
        free_memory: 0,
        uptime: Duration::ZERO,
        load_average: vec![],
    })
}

#[cfg(windows)]
fn get_platform_info_windows() -> Result<PlatformInfo, Box<dyn std::error::Error>> {
    // Windows implementation would use Windows API
    // For now, return a basic implementation
    Ok(PlatformInfo {
        os_type: "Windows".to_string(),
        os_version: "unknown".to_string(),
        kernel_version: "unknown".to_string(),
        architecture: std::env::consts::ARCH.to_string(),
        cpu_model: "unknown".to_string(),
        cpu_cores: num_cpus::get() as u32,
        cpu_threads: num_cpus::get() as u32,
        total_memory_gb: 0.0,
        page_size: 4096,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
        assert_eq!(format_bytes(1073741824), "1.00 GB");
    }
    
    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_secs(30)), "30.00s");
        assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
        assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m");
        assert_eq!(format_duration(Duration::from_secs(90061)), "1d 1h");
    }
    
    #[test]
    fn test_get_process_info() {
        let info = get_process_info();
        assert!(info.is_ok());
        let info = info.unwrap();
        assert!(info.pid > 0);
    }
    
    #[test]
    fn test_get_system_info() {
        let info = get_system_info();
        assert!(info.is_ok());
        let info = info.unwrap();
        assert!(info.cpu_count > 0);
    }
}