//! Logging system for XProfiler
//!
//! This module provides a comprehensive logging system that supports:
//! - Multiple log levels (trace, debug, info, warn, error)
//! - Structured logging with JSON format
//! - Thread-safe logging
//! - Log rotation and file management
//! - Integration with Node.js console logging

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;

pub mod formatter;
pub mod writer;
pub mod config;

use formatter::LogFormatter;
use writer::LogWriter;
use config::LogConfig;

/// Log levels supported by the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl LogLevel {
    /// Convert log level to string
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }

    /// Parse log level from string
    pub fn from_str(s: &str) -> Result<LogLevel, Box<dyn std::error::Error>> {
        match s.to_uppercase().as_str() {
            "TRACE" => Ok(LogLevel::Trace),
            "DEBUG" => Ok(LogLevel::Debug),
            "INFO" => Ok(LogLevel::Info),
            "WARN" => Ok(LogLevel::Warn),
            "ERROR" => Ok(LogLevel::Error),
            _ => Err(format!("Invalid log level: {}", s).into()),
        }
    }
}

/// Log entry structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub message: String,
    pub module: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub thread_id: u32,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl LogEntry {
    /// Create a new log entry
    pub fn new(
        level: LogLevel,
        message: String,
        module: Option<String>,
        file: Option<String>,
        line: Option<u32>,
    ) -> Self {
        Self {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            level,
            message,
            module,
            file,
            line,
            thread_id: get_thread_id(),
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to the log entry
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Add multiple metadata entries
    pub fn with_metadata_map(mut self, metadata: HashMap<String, serde_json::Value>) -> Self {
        self.metadata.extend(metadata);
        self
    }
}

/// Logger instance
pub struct Logger {
    config: LogConfig,
    formatter: Box<dyn LogFormatter + Send + Sync>,
    writer: Box<dyn LogWriter + Send + Sync>,
}

impl Logger {
    /// Create a new logger with configuration
    pub fn new(
        config: LogConfig,
        formatter: Box<dyn LogFormatter + Send + Sync>,
        writer: Box<dyn LogWriter + Send + Sync>,
    ) -> Self {
        Self {
            config,
            formatter,
            writer,
        }
    }

    /// Log a message at the specified level
    pub fn log(&self, entry: LogEntry) -> Result<(), Box<dyn std::error::Error>> {
        // Check if the log level is enabled
        if entry.level < self.config.min_level {
            return Ok(());
        }

        // Format the log entry
        let formatted = self.formatter.format(&entry)?;

        // Write the formatted log
        self.writer.write(&formatted)?;

        Ok(())
    }

    /// Log a trace message
    pub fn trace(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let entry = LogEntry::new(
            LogLevel::Trace,
            message.to_string(),
            None,
            None,
            None,
        );
        self.log(entry)
    }

    /// Log a debug message
    pub fn debug(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let entry = LogEntry::new(
            LogLevel::Debug,
            message.to_string(),
            None,
            None,
            None,
        );
        self.log(entry)
    }

    /// Log an info message
    pub fn info(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let entry = LogEntry::new(
            LogLevel::Info,
            message.to_string(),
            None,
            None,
            None,
        );
        self.log(entry)
    }

    /// Log a warning message
    pub fn warn(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let entry = LogEntry::new(
            LogLevel::Warn,
            message.to_string(),
            None,
            None,
            None,
        );
        self.log(entry)
    }

    /// Log an error message
    pub fn error(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let entry = LogEntry::new(
            LogLevel::Error,
            message.to_string(),
            None,
            None,
            None,
        );
        self.log(entry)
    }

    /// Update logger configuration
    pub fn update_config(&mut self, config: LogConfig) {
        self.config = config;
    }

    /// Get current configuration
    pub fn get_config(&self) -> &LogConfig {
        &self.config
    }

    /// Flush any buffered logs
    pub fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.writer.flush()
    }
}

/// Global logger instance
static GLOBAL_LOGGER: Lazy<Arc<Mutex<Option<Logger>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))
});

/// Initialize the global logger
pub fn init_logger(logger: Logger) -> Result<(), Box<dyn std::error::Error>> {
    let mut global_logger = GLOBAL_LOGGER.lock().map_err(|e| {
        format!("Failed to acquire logger lock: {}", e)
    })?;
    
    *global_logger = Some(logger);
    Ok(())
}

/// Get a reference to the global logger
pub fn with_logger<F, R>(f: F) -> Result<R, Box<dyn std::error::Error>>
where
    F: FnOnce(&Logger) -> Result<R, Box<dyn std::error::Error>>,
{
    let global_logger = GLOBAL_LOGGER.lock().map_err(|e| {
        format!("Failed to acquire logger lock: {}", e)
    })?;
    
    match global_logger.as_ref() {
        Some(logger) => f(logger),
        None => Err("Logger not initialized".into()),
    }
}

/// Log a trace message using the global logger
pub fn trace(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    with_logger(|logger| logger.trace(message))
}

/// Log a debug message using the global logger
pub fn debug(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    with_logger(|logger| logger.debug(message))
}

/// Log an info message using the global logger
pub fn info(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    with_logger(|logger| logger.info(message))
}

/// Log a warning message using the global logger
pub fn warn(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    with_logger(|logger| logger.warn(message))
}

/// Log an error message using the global logger
pub fn error(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    with_logger(|logger| logger.error(message))
}

/// Flush the global logger
pub fn flush() -> Result<(), Box<dyn std::error::Error>> {
    with_logger(|logger| logger.flush())
}

/// Convenience macros for logging
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        $crate::logger::trace(&format!($($arg)*)).unwrap_or_else(|e| {
            eprintln!("Failed to log trace: {}", e);
        });
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::logger::debug(&format!($($arg)*)).unwrap_or_else(|e| {
            eprintln!("Failed to log debug: {}", e);
        });
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::logger::info(&format!($($arg)*)).unwrap_or_else(|e| {
            eprintln!("Failed to log info: {}", e);
        });
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::logger::warn(&format!($($arg)*)).unwrap_or_else(|e| {
            eprintln!("Failed to log warn: {}", e);
        });
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::logger::error(&format!($($arg)*)).unwrap_or_else(|e| {
            eprintln!("Failed to log error: {}", e);
        });
    };
}

/// Get current thread ID
fn get_thread_id() -> u32 {
    #[cfg(unix)]
    {
        unsafe { libc::pthread_self() as u32 }
    }
    #[cfg(windows)]
    {
        unsafe { winapi::um::processthreadsapi::GetCurrentThreadId() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logger::formatter::JsonFormatter;
    use crate::logger::writer::ConsoleWriter;
    use crate::logger::config::LogConfig;

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Trace < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Error);
    }

    #[test]
    fn test_log_level_string_conversion() {
        assert_eq!(LogLevel::Info.as_str(), "INFO");
        assert_eq!(LogLevel::from_str("INFO").unwrap(), LogLevel::Info);
        assert_eq!(LogLevel::from_str("info").unwrap(), LogLevel::Info);
        assert!(LogLevel::from_str("INVALID").is_err());
    }

    #[test]
    fn test_log_entry_creation() {
        let entry = LogEntry::new(
            LogLevel::Info,
            "test message".to_string(),
            Some("test_module".to_string()),
            Some("test.rs".to_string()),
            Some(42),
        );
        
        assert_eq!(entry.level, LogLevel::Info);
        assert_eq!(entry.message, "test message");
        assert_eq!(entry.module, Some("test_module".to_string()));
        assert_eq!(entry.file, Some("test.rs".to_string()));
        assert_eq!(entry.line, Some(42));
        assert!(entry.timestamp > 0);
    }

    #[test]
    fn test_logger_creation() {
        let config = LogConfig::default();
        let formatter = Box::new(JsonFormatter::new());
        let writer = Box::new(ConsoleWriter::new());
        
        let logger = Logger::new(config, formatter, writer);
        assert_eq!(logger.get_config().min_level, LogLevel::Info);
    }
}