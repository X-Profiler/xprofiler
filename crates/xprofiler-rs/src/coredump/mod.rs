//! Coredump generation module
//!
//! This module provides coredump generation functionality.
//! Full coredump support is only available on Linux and requires
//! complex kernel integration that is beyond napi-rs capabilities.
//!
//! For production use cases requiring coredump, consider:
//! - Using the C++ binding which includes the google-coredumper library
//! - Using Linux's built-in coredump mechanisms (ulimit -c unlimited)
//! - Using external tools like gcore

#[cfg(target_os = "linux")]
use crate::config;
#[cfg(target_os = "linux")]
use crate::logger;

/// Result of a coredump operation
#[derive(Debug)]
pub struct CoredumpResult {
    pub success: bool,
    pub filepath: Option<String>,
    pub message: String,
}

/// Write a coredump to the specified file
///
/// On Linux: Attempts to create a coredump file (limited functionality)
/// On other platforms: Returns an error indicating unsupported platform
pub fn write_coredump(filepath: &str) -> CoredumpResult {
    #[cfg(target_os = "linux")]
    {
        write_coredump_linux(filepath)
    }

    #[cfg(target_os = "macos")]
    {
        CoredumpResult {
            success: false,
            filepath: Some(filepath.to_string()),
            message: "Coredump generation is not supported on macOS. Use lldb to create core dumps.".to_string(),
        }
    }

    #[cfg(target_os = "windows")]
    {
        CoredumpResult {
            success: false,
            filepath: Some(filepath.to_string()),
            message: "Coredump generation is not supported on Windows. Use Windows Error Reporting or procdump.".to_string(),
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        CoredumpResult {
            success: false,
            filepath: Some(filepath.to_string()),
            message: "Coredump generation is not supported on this platform.".to_string(),
        }
    }
}

/// Linux-specific coredump implementation
#[cfg(target_os = "linux")]
fn write_coredump_linux(filepath: &str) -> CoredumpResult {
    use std::fs::File;
    use std::io::Write;
    use std::process;

    let cfg = config::get_config();
    let pid = process::id();

    // Log the attempt
    let _ = logger::info(&format!(
        "Attempting to generate coredump at: {}",
        filepath
    ));

    // Note: Full coredump generation requires the google-coredumper library
    // which performs complex operations including:
    // - Stopping all threads
    // - Reading /proc/self/maps
    // - Writing ELF core format
    //
    // For now, we create a placeholder file and suggest using external tools

    // Create a minimal info file instead of a full coredump
    let info_filepath = format!("{}.info", filepath);

    match File::create(&info_filepath) {
        Ok(mut file) => {
            let info = format!(
                "xprofiler coredump info\n\
                 ======================\n\
                 PID: {}\n\
                 Requested file: {}\n\
                 Log directory: {}\n\
                 \n\
                 Note: Full coredump generation requires the C++ binding.\n\
                 Alternative methods:\n\
                 1. Use 'gcore {}' to generate a core dump\n\
                 2. Set 'ulimit -c unlimited' and trigger SIGABRT\n\
                 3. Use the C++ xprofiler binding\n",
                pid, filepath, cfg.log_dir, pid
            );

            if let Err(e) = file.write_all(info.as_bytes()) {
                return CoredumpResult {
                    success: false,
                    filepath: Some(filepath.to_string()),
                    message: format!("Failed to write coredump info: {}", e),
                };
            }

            let _ = logger::info(&format!(
                "Coredump info written to: {} (use gcore {} for full dump)",
                info_filepath, pid
            ));

            CoredumpResult {
                success: true,
                filepath: Some(info_filepath),
                message: format!(
                    "Coredump info created. For full coredump, use: gcore {}",
                    pid
                ),
            }
        }
        Err(e) => CoredumpResult {
            success: false,
            filepath: Some(filepath.to_string()),
            message: format!("Failed to create coredump file: {}", e),
        },
    }
}

/// Check if coredump is supported on the current platform
pub fn is_coredump_supported() -> bool {
    cfg!(target_os = "linux")
}

/// Get platform-specific coredump guidance
pub fn get_coredump_guidance() -> &'static str {
    #[cfg(target_os = "linux")]
    {
        "On Linux, full coredump requires the C++ binding. \
         Alternatively, use 'gcore <pid>' or set 'ulimit -c unlimited'."
    }

    #[cfg(target_os = "macos")]
    {
        "On macOS, use 'lldb -p <pid>' then 'process save-core <file>' to create core dumps."
    }

    #[cfg(target_os = "windows")]
    {
        "On Windows, use procdump or Task Manager to create dumps."
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        "Coredump generation is not supported on this platform."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_coredump_supported() {
        let supported = is_coredump_supported();
        #[cfg(target_os = "linux")]
        assert!(supported);
        #[cfg(not(target_os = "linux"))]
        assert!(!supported);
    }

    #[test]
    fn test_get_coredump_guidance() {
        let guidance = get_coredump_guidance();
        assert!(!guidance.is_empty());
    }

    #[test]
    #[cfg(not(target_os = "linux"))]
    fn test_write_coredump_unsupported() {
        let result = write_coredump("/tmp/test.core");
        assert!(!result.success);
        assert!(result.message.contains("not supported"));
    }
}
