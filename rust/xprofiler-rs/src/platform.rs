//! Platform-specific optimizations and utilities
//!
//! This module provides platform-specific optimizations, CPU architecture detection,
//! and performance enhancements for different operating systems and hardware.

use crate::error::{XProfilerError, XProfilerResult};
use std::sync::OnceLock;

/// CPU architecture information
#[derive(Debug, Clone, PartialEq)]
pub enum CpuArch {
    X86,
    X86_64,
    Arm,
    Arm64,
    Mips,
    Mips64,
    PowerPC,
    PowerPC64,
    Riscv64,
    S390x,
    Unknown(String),
}

/// Operating system information
#[derive(Debug, Clone, PartialEq)]
pub enum OperatingSystem {
    Linux,
    MacOS,
    Windows,
    FreeBSD,
    OpenBSD,
    NetBSD,
    Unknown(String),
}

/// Platform capabilities and features
#[derive(Debug, Clone)]
pub struct PlatformCapabilities {
    pub cpu_arch: CpuArch,
    pub os: OperatingSystem,
    pub page_size: usize,
    pub cpu_count: usize,
    pub has_rdtsc: bool,
    pub has_perf_counters: bool,
    pub has_high_res_timer: bool,
    pub supports_memory_mapping: bool,
    pub supports_process_monitoring: bool,
}

/// Global platform capabilities (initialized once)
static PLATFORM_CAPS: OnceLock<PlatformCapabilities> = OnceLock::new();

/// Get platform capabilities (cached)
pub fn get_platform_capabilities() -> &'static PlatformCapabilities {
    PLATFORM_CAPS.get_or_init(|| {
        detect_platform_capabilities().unwrap_or_else(|_| PlatformCapabilities {
            cpu_arch: CpuArch::Unknown("detection_failed".to_string()),
            os: OperatingSystem::Unknown("detection_failed".to_string()),
            page_size: 4096,
            cpu_count: 1,
            has_rdtsc: false,
            has_perf_counters: false,
            has_high_res_timer: false,
            supports_memory_mapping: false,
            supports_process_monitoring: false,
        })
    })
}

/// Detect platform capabilities
fn detect_platform_capabilities() -> XProfilerResult<PlatformCapabilities> {
    let cpu_arch = detect_cpu_arch();
    let os = detect_operating_system();
    let page_size = get_page_size()?;
    let cpu_count = num_cpus::get();
    
    let has_rdtsc = detect_rdtsc_support();
    let has_perf_counters = detect_perf_counter_support();
    let has_high_res_timer = detect_high_res_timer_support();
    let supports_memory_mapping = detect_memory_mapping_support();
    let supports_process_monitoring = detect_process_monitoring_support();

    Ok(PlatformCapabilities {
        cpu_arch,
        os,
        page_size,
        cpu_count,
        has_rdtsc,
        has_perf_counters,
        has_high_res_timer,
        supports_memory_mapping,
        supports_process_monitoring,
    })
}

/// Detect CPU architecture
fn detect_cpu_arch() -> CpuArch {
    match std::env::consts::ARCH {
        "x86" => CpuArch::X86,
        "x86_64" => CpuArch::X86_64,
        "arm" => CpuArch::Arm,
        "aarch64" => CpuArch::Arm64,
        "mips" => CpuArch::Mips,
        "mips64" => CpuArch::Mips64,
        "powerpc" => CpuArch::PowerPC,
        "powerpc64" => CpuArch::PowerPC64,
        "riscv64" => CpuArch::Riscv64,
        "s390x" => CpuArch::S390x,
        other => CpuArch::Unknown(other.to_string()),
    }
}

/// Detect operating system
fn detect_operating_system() -> OperatingSystem {
    match std::env::consts::OS {
        "linux" => OperatingSystem::Linux,
        "macos" => OperatingSystem::MacOS,
        "windows" => OperatingSystem::Windows,
        "freebsd" => OperatingSystem::FreeBSD,
        "openbsd" => OperatingSystem::OpenBSD,
        "netbsd" => OperatingSystem::NetBSD,
        other => OperatingSystem::Unknown(other.to_string()),
    }
}

/// Get system page size
fn get_page_size() -> XProfilerResult<usize> {
    #[cfg(unix)]
    {
        let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
        if page_size > 0 {
            Ok(page_size as usize)
        } else {
            Ok(4096) // Default fallback
        }
    }
    
    #[cfg(windows)]
    {
        use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};
        let mut sys_info: SYSTEM_INFO = unsafe { std::mem::zeroed() };
        unsafe { GetSystemInfo(&mut sys_info) };
        Ok(sys_info.dwPageSize as usize)
    }
    
    #[cfg(not(any(unix, windows)))]
    {
        Ok(4096) // Default fallback
    }
}

/// Detect RDTSC (Read Time-Stamp Counter) support
fn detect_rdtsc_support() -> bool {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        // Check CPUID for RDTSC support
        #[cfg(target_feature = "rdtsc")]
        {
            true
        }
        #[cfg(not(target_feature = "rdtsc"))]
        {
            // Try to detect at runtime
            is_x86_feature_detected!("rdtsc")
        }
    }
    
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        false
    }
}

/// Detect performance counter support
fn detect_perf_counter_support() -> bool {
    #[cfg(target_os = "linux")]
    {
        // Check if perf_event_open is available
        std::path::Path::new("/proc/sys/kernel/perf_event_paranoid").exists()
    }
    
    #[cfg(target_os = "macos")]
    {
        // macOS has limited perf counter support
        false
    }
    
    #[cfg(target_os = "windows")]
    {
        // Windows has Performance Data Helper (PDH)
        true
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        false
    }
}

/// Detect high-resolution timer support
fn detect_high_res_timer_support() -> bool {
    // Most modern platforms support high-resolution timers
    true
}

/// Detect memory mapping support
fn detect_memory_mapping_support() -> bool {
    #[cfg(unix)]
    {
        true // Unix systems generally support mmap
    }
    
    #[cfg(windows)]
    {
        true // Windows supports memory mapping
    }
    
    #[cfg(not(any(unix, windows)))]
    {
        false
    }
}

/// Detect process monitoring support
fn detect_process_monitoring_support() -> bool {
    #[cfg(target_os = "linux")]
    {
        std::path::Path::new("/proc").exists()
    }
    
    #[cfg(target_os = "macos")]
    {
        true // macOS supports process monitoring via sysctl
    }
    
    #[cfg(target_os = "windows")]
    {
        true // Windows supports process monitoring via WinAPI
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        false
    }
}

/// Platform-specific optimizations
pub mod optimizations {
    use super::*;

    /// Get optimized timestamp function based on platform capabilities
    pub fn get_timestamp_fn() -> fn() -> u64 {
        let _caps = get_platform_capabilities();
        
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if _caps.has_rdtsc {
                return rdtsc_timestamp;
            }
        }
        
        // Fallback to standard timestamp
        standard_timestamp
    }

    /// RDTSC-based timestamp (x86/x86_64 only)
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn rdtsc_timestamp() -> u64 {
        #[cfg(target_feature = "rdtsc")]
        {
            unsafe { std::arch::x86_64::_rdtsc() }
        }
        #[cfg(not(target_feature = "rdtsc"))]
        {
            // Fallback if RDTSC not available at compile time
            standard_timestamp()
        }
    }

    /// Standard timestamp using system clock
    fn standard_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64
    }

    /// Get optimized memory allocation alignment
    pub fn get_memory_alignment() -> usize {
        let caps = get_platform_capabilities();
        
        match caps.cpu_arch {
            CpuArch::X86_64 | CpuArch::Arm64 => 64, // Cache line size
            CpuArch::X86 | CpuArch::Arm => 32,
            _ => 16, // Conservative default
        }
    }

    /// Get optimized buffer size for I/O operations
    pub fn get_io_buffer_size() -> usize {
        let caps = get_platform_capabilities();
        
        // Use page size as base, but ensure minimum size
        std::cmp::max(caps.page_size, 8192)
    }

    /// Check if we should use memory mapping for large files
    pub fn should_use_memory_mapping(file_size: usize) -> bool {
        let caps = get_platform_capabilities();
        
        caps.supports_memory_mapping && file_size > caps.page_size * 4
    }
}

/// Platform-specific system information
pub mod sysinfo {
    use super::*;
    use crate::error::platform;

    /// Get CPU frequency in Hz
    pub fn get_cpu_frequency() -> XProfilerResult<u64> {
        #[cfg(target_os = "linux")]
        {
            get_cpu_frequency_linux()
        }
        
        #[cfg(target_os = "macos")]
        {
            get_cpu_frequency_macos()
        }
        
        #[cfg(target_os = "windows")]
        {
            get_cpu_frequency_windows()
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            Err(XProfilerError::Platform {
                message: "CPU frequency detection not supported on this platform".to_string(),
                platform: std::env::consts::OS.to_string(),
                details: None,
            })
        }
    }

    #[cfg(target_os = "linux")]
    fn get_cpu_frequency_linux() -> XProfilerResult<u64> {
        use std::fs;
        
        // Try to read from /proc/cpuinfo
        let cpuinfo = fs::read_to_string("/proc/cpuinfo")
            .map_err(|e| XProfilerError::Io {
                message: format!("Failed to read /proc/cpuinfo: {}", e),
                path: Some("/proc/cpuinfo".to_string()),
                kind: crate::error::IoErrorKind::Other,
            })?;
        
        for line in cpuinfo.lines() {
            if line.starts_with("cpu MHz") {
                if let Some(freq_str) = line.split(':').nth(1) {
                    if let Ok(freq_mhz) = freq_str.trim().parse::<f64>() {
                        return Ok((freq_mhz * 1_000_000.0) as u64);
                    }
                }
            }
        }
        
        Err(XProfilerError::Platform {
            message: "Could not parse CPU frequency from /proc/cpuinfo".to_string(),
            platform: "Linux".to_string(),
            details: None,
        })
    }

    #[cfg(target_os = "macos")]
    fn get_cpu_frequency_macos() -> XProfilerResult<u64> {
        // Use sysctl to get CPU frequency
        use std::ffi::CString;
        use std::mem;
        
        let name = CString::new("hw.cpufrequency_max").unwrap();
        let mut freq: u64 = 0;
        let mut size = mem::size_of::<u64>();
        
        let result = unsafe {
            libc::sysctlbyname(
                name.as_ptr(),
                &mut freq as *mut _ as *mut libc::c_void,
                &mut size,
                std::ptr::null_mut(),
                0,
            )
        };
        
        if result == 0 {
            Ok(freq)
        } else {
            Err(platform::last_error(Some("sysctlbyname")))
        }
    }

    #[cfg(target_os = "windows")]
    fn get_cpu_frequency_windows() -> XProfilerResult<u64> {
        // Use Windows registry or WMI to get CPU frequency
        // For now, return a placeholder
        Err(XProfilerError::Platform {
            message: "CPU frequency detection not yet implemented for Windows".to_string(),
            platform: "Windows".to_string(),
            details: Some("Use WMI or registry".to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detection() {
        let caps = get_platform_capabilities();
        
        // Basic sanity checks
        assert!(caps.page_size > 0);
        assert!(caps.cpu_count > 0);
        
        // Architecture should be detected
        assert!(!matches!(caps.cpu_arch, CpuArch::Unknown(_)));
        assert!(!matches!(caps.os, OperatingSystem::Unknown(_)));
    }

    #[test]
    fn test_optimizations() {
        let timestamp_fn = optimizations::get_timestamp_fn();
        let ts1 = timestamp_fn();
        let ts2 = timestamp_fn();
        
        // Timestamps should be increasing
        assert!(ts2 >= ts1);
        
        // Memory alignment should be reasonable
        let alignment = optimizations::get_memory_alignment();
        assert!(alignment >= 8 && alignment <= 128);
        
        // I/O buffer size should be reasonable
        let buffer_size = optimizations::get_io_buffer_size();
        assert!(buffer_size >= 4096);
    }
}