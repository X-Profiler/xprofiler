//! Logger system for xprofiler-rs
//!
//! Provides file-based logging with automatic date-based rotation.

use napi::bindgen_prelude::*;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use crate::config;
use crate::constants;
use crate::utils;

/// Log file handle with metadata
struct LogFile {
    file: File,
    date: u32, // YYYYMMDD format
    path: PathBuf,
}

/// Global log file handle
static LOG_FILE: Lazy<Mutex<Option<LogFile>>> = Lazy::new(|| Mutex::new(None));

/// Log level enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info = 0,
    Error = 1,
    Debug = 2,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Info => "info",
            LogLevel::Error => "error",
            LogLevel::Debug => "debug",
        }
    }
}

/// Log an info message
pub fn info(content: &str) -> Result<()> {
    write_log(LogLevel::Info, content)
}

/// Log an error message
pub fn error(content: &str) -> Result<()> {
    write_log(LogLevel::Error, content)
}

/// Log a debug message (only if log_level >= 2)
pub fn debug(content: &str) -> Result<()> {
    let cfg = config::get_config();
    if cfg.log_level >= constants::LOG_LEVEL_DEBUG {
        write_log(LogLevel::Debug, content)?;
    }
    Ok(())
}

/// Write a log message
fn write_log(level: LogLevel, content: &str) -> Result<()> {
    let cfg = config::get_config();

    // If log_type is console, print to stderr
    if cfg.log_type == constants::LOG_TYPE_CONSOLE {
        eprintln!(
            "[{}] [{}] [{}] {}",
            utils::format_datetime(),
            utils::get_pid(),
            level.as_str(),
            content
        );
        return Ok(());
    }

    // File logging
    let now = chrono::Local::now();
    let today = (now.year() as u32) * 10000 + (now.month() as u32) * 100 + now.day() as u32;

    let mut guard = LOG_FILE.lock();

    // Rotate log file if date changed or not initialized
    if guard.as_ref().map_or(true, |f| f.date != today) {
        let log_filename = if cfg.log_format_alinode {
            format!(
                "node-{}-{}.log",
                utils::get_pid(),
                now.format("%Y%m%d")
            )
        } else {
            format!(
                "xprofiler-{}-{}.log",
                utils::get_pid(),
                now.format("%Y%m%d")
            )
        };

        let log_path = PathBuf::from(&cfg.log_dir).join(log_filename);

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .map_err(|e| Error::from_reason(format!("Failed to open log file: {}", e)))?;

        *guard = Some(LogFile {
            file,
            date: today,
            path: log_path,
        });
    }

    if let Some(log_file) = guard.as_mut() {
        let timestamp = now.format("%Y-%m-%d %H:%M:%S");
        let pid = utils::get_pid();

        let line = if cfg.log_format_alinode {
            // Alinode format: [YYYY-MM-DD HH:MM:SS] [level] [pid] content
            format!("[{}] [{}] [{}] {}\n", timestamp, level.as_str(), pid, content)
        } else {
            // xprofiler format: [YYYY-MM-DD HH:MM:SS.mmm] [pid] [level] content
            let timestamp_ms = now.format("%Y-%m-%d %H:%M:%S%.3f");
            format!(
                "[{}] [{}] [{}] {}\n",
                timestamp_ms,
                pid,
                level.as_str(),
                content
            )
        };

        log_file
            .file
            .write_all(line.as_bytes())
            .map_err(|e| Error::from_reason(format!("Failed to write log: {}", e)))?;
    }

    Ok(())
}

/// Write a log message for a specific thread
pub fn write_thread_log(level: LogLevel, thread_id: i64, content: &str) -> Result<()> {
    let cfg = config::get_config();

    // If log_type is console, print to stderr
    if cfg.log_type == constants::LOG_TYPE_CONSOLE {
        eprintln!(
            "[{}] [{}] [thread:{}] [{}] {}",
            utils::format_datetime(),
            utils::get_pid(),
            thread_id,
            level.as_str(),
            content
        );
        return Ok(());
    }

    let now = chrono::Local::now();
    let today = (now.year() as u32) * 10000 + (now.month() as u32) * 100 + now.day() as u32;

    let mut guard = LOG_FILE.lock();

    // Rotate log file if date changed or not initialized
    if guard.as_ref().map_or(true, |f| f.date != today) {
        let log_filename = format!(
            "xprofiler-{}-{}.log",
            utils::get_pid(),
            now.format("%Y%m%d")
        );

        let log_path = PathBuf::from(&cfg.log_dir).join(log_filename);

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .map_err(|e| Error::from_reason(format!("Failed to open log file: {}", e)))?;

        *guard = Some(LogFile {
            file,
            date: today,
            path: log_path,
        });
    }

    if let Some(log_file) = guard.as_mut() {
        let timestamp_ms = now.format("%Y-%m-%d %H:%M:%S%.3f");
        let pid = utils::get_pid();

        let line = format!(
            "[{}] [{}] [thread:{}] [{}] {}\n",
            timestamp_ms,
            pid,
            thread_id,
            level.as_str(),
            content
        );

        log_file
            .file
            .write_all(line.as_bytes())
            .map_err(|e| Error::from_reason(format!("Failed to write log: {}", e)))?;
    }

    Ok(())
}

/// Get the current log file path (for testing)
#[cfg(test)]
pub fn get_log_file_path() -> Option<PathBuf> {
    LOG_FILE.lock().as_ref().map(|f| f.path.clone())
}

// Re-export chrono traits for use in this module
use chrono::Datelike;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_log_level_as_str() {
        assert_eq!(LogLevel::Info.as_str(), "info");
        assert_eq!(LogLevel::Error.as_str(), "error");
        assert_eq!(LogLevel::Debug.as_str(), "debug");
    }

    #[test]
    fn test_write_log_to_file() {
        // Setup test config
        let temp_dir = std::env::temp_dir().join("xprofiler-test");
        fs::create_dir_all(&temp_dir).ok();

        config::update_config(|c| {
            c.log_dir = temp_dir.to_string_lossy().to_string();
            c.log_type = 0; // file
        })
        .unwrap();

        // Write a log
        info("test message").unwrap();

        // Verify log file exists
        let log_path = get_log_file_path();
        assert!(log_path.is_some());
        assert!(log_path.unwrap().exists());

        // Cleanup
        fs::remove_dir_all(&temp_dir).ok();
    }
}
