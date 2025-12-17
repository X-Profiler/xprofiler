//! Common utilities for xprofiler-rs

use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

/// Start time of the process
static START_TIME: Lazy<Instant> = Lazy::new(Instant::now);

/// Counter for diagnostic file IDs
static DIAG_FILE_ID: AtomicU64 = AtomicU64::new(0);

/// Get the uptime of the process in seconds
pub fn get_uptime() -> u64 {
    START_TIME.elapsed().as_secs()
}

/// Get the uptime of the process in milliseconds
pub fn get_uptime_ms() -> u64 {
    START_TIME.elapsed().as_millis() as u64
}

/// Get the next diagnostic file ID (monotonically increasing)
pub fn get_next_diag_file_id() -> u64 {
    DIAG_FILE_ID.fetch_add(1, Ordering::SeqCst)
}

/// Get the current process ID
pub fn get_pid() -> u32 {
    std::process::id()
}

/// Get the path separator for the current platform
#[cfg(unix)]
pub fn get_path_separator() -> &'static str {
    "/"
}

/// Get the path separator for the current platform
#[cfg(windows)]
pub fn get_path_separator() -> &'static str {
    "\\"
}

/// Format current timestamp as YYYYMMDD
pub fn format_date() -> String {
    chrono::Local::now().format("%Y%m%d").to_string()
}

/// Format current timestamp as YYYY-MM-DD HH:MM:SS
pub fn format_datetime() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Format current timestamp as YYYYMMDD-HHMMSS
pub fn format_datetime_compact() -> String {
    chrono::Local::now().format("%Y%m%d-%H%M%S").to_string()
}

/// Get current timestamp in milliseconds since epoch
pub fn get_timestamp_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// Sleep for the specified number of milliseconds
pub fn sleep_ms(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_uptime() {
        let uptime = get_uptime();
        assert!(uptime < 60); // Should be less than 60 seconds in tests
    }

    #[test]
    fn test_get_next_diag_file_id() {
        let id1 = get_next_diag_file_id();
        let id2 = get_next_diag_file_id();
        assert_eq!(id2, id1 + 1);
    }

    #[test]
    fn test_get_pid() {
        let pid = get_pid();
        assert!(pid > 0);
    }

    #[test]
    fn test_format_date() {
        let date = format_date();
        assert_eq!(date.len(), 8); // YYYYMMDD
    }
}
