//! Log formatters for different output formats

use super::{LogEntry, LogLevel};
use serde_json;
// use std::fmt; // Commented out unused import

/// Trait for log formatters
pub trait LogFormatter {
    /// Format a log entry into a string
    fn format(&self, entry: &LogEntry) -> Result<String, Box<dyn std::error::Error>>;
}

/// JSON formatter
pub struct JsonFormatter {
    pretty: bool,
    include_metadata: bool,
}

impl JsonFormatter {
    /// Create a new JSON formatter
    pub fn new() -> Self {
        Self {
            pretty: false,
            include_metadata: true,
        }
    }
    
    /// Create a pretty-printed JSON formatter
    pub fn pretty() -> Self {
        Self {
            pretty: true,
            include_metadata: true,
        }
    }
    
    /// Set whether to include metadata
    pub fn with_metadata(mut self, include: bool) -> Self {
        self.include_metadata = include;
        self
    }
}

impl LogFormatter for JsonFormatter {
    fn format(&self, entry: &LogEntry) -> Result<String, Box<dyn std::error::Error>> {
        let mut json_entry = serde_json::json!({
            "timestamp": entry.timestamp,
            "level": entry.level.as_str(),
            "message": entry.message,
            "thread_id": entry.thread_id,
        });
        
        // Add optional fields
        if let Some(ref module) = entry.module {
            json_entry["module"] = serde_json::Value::String(module.clone());
        }
        
        if let Some(ref file) = entry.file {
            json_entry["file"] = serde_json::Value::String(file.clone());
        }
        
        if let Some(line) = entry.line {
            json_entry["line"] = serde_json::Value::Number(line.into());
        }
        
        // Add metadata if enabled and present
        if self.include_metadata && !entry.metadata.is_empty() {
            json_entry["metadata"] = serde_json::Value::Object(
                entry.metadata.iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect()
            );
        }
        
        let formatted = if self.pretty {
            serde_json::to_string_pretty(&json_entry)?
        } else {
            serde_json::to_string(&json_entry)?
        };
        
        Ok(formatted + "\n")
    }
}

/// Text formatter
pub struct TextFormatter {
    include_timestamp: bool,
    include_thread_id: bool,
    include_location: bool,
    include_metadata: bool,
    timestamp_format: TimestampFormat,
}

/// Timestamp format options
#[derive(Debug, Clone, Copy)]
pub enum TimestampFormat {
    /// Unix timestamp in milliseconds
    Unix,
    /// ISO 8601 format (YYYY-MM-DDTHH:MM:SS.sssZ)
    Iso8601,
    /// Human readable format (YYYY-MM-DD HH:MM:SS.sss)
    Human,
}

impl TextFormatter {
    /// Create a new text formatter with default settings
    pub fn new() -> Self {
        Self {
            include_timestamp: true,
            include_thread_id: true,
            include_location: false,
            include_metadata: true,
            timestamp_format: TimestampFormat::Human,
        }
    }
    
    /// Set whether to include timestamp
    pub fn with_timestamp(mut self, include: bool) -> Self {
        self.include_timestamp = include;
        self
    }
    
    /// Set whether to include thread ID
    pub fn with_thread_id(mut self, include: bool) -> Self {
        self.include_thread_id = include;
        self
    }
    
    /// Set whether to include location information
    pub fn with_location(mut self, include: bool) -> Self {
        self.include_location = include;
        self
    }
    
    /// Set whether to include metadata
    pub fn with_metadata(mut self, include: bool) -> Self {
        self.include_metadata = include;
        self
    }
    
    /// Set timestamp format
    pub fn with_timestamp_format(mut self, format: TimestampFormat) -> Self {
        self.timestamp_format = format;
        self
    }
    
    /// Format timestamp according to the configured format
    fn format_timestamp(&self, timestamp: u64) -> String {
        match self.timestamp_format {
            TimestampFormat::Unix => timestamp.to_string(),
            TimestampFormat::Iso8601 => {
                let dt = std::time::UNIX_EPOCH + std::time::Duration::from_millis(timestamp);
                // Simple ISO 8601 approximation
                format!("{:?}", dt)
            },
            TimestampFormat::Human => {
                let dt = std::time::UNIX_EPOCH + std::time::Duration::from_millis(timestamp);
                // Simple human readable approximation
                format!("{:?}", dt)
            },
        }
    }
    
    /// Get level color for terminal output
    fn get_level_color(&self, level: LogLevel) -> &'static str {
        match level {
            LogLevel::Trace => "\x1b[37m", // White
            LogLevel::Debug => "\x1b[36m", // Cyan
            LogLevel::Info => "\x1b[32m",  // Green
            LogLevel::Warn => "\x1b[33m",  // Yellow
            LogLevel::Error => "\x1b[31m", // Red
        }
    }
    
    /// Reset color
    fn reset_color(&self) -> &'static str {
        "\x1b[0m"
    }
}

impl LogFormatter for TextFormatter {
    fn format(&self, entry: &LogEntry) -> Result<String, Box<dyn std::error::Error>> {
        let mut parts = Vec::new();
        
        // Timestamp
        if self.include_timestamp {
            parts.push(format!("[{}]", self.format_timestamp(entry.timestamp)));
        }
        
        // Level with color
        let level_color = self.get_level_color(entry.level);
        let reset_color = self.reset_color();
        parts.push(format!(
            "{}{:5}{}",
            level_color,
            entry.level.as_str(),
            reset_color
        ));
        
        // Thread ID
        if self.include_thread_id {
            parts.push(format!("[{}]", entry.thread_id));
        }
        
        // Location (module, file, line)
        if self.include_location {
            let mut location_parts = Vec::new();
            
            if let Some(ref module) = entry.module {
                location_parts.push(module.clone());
            }
            
            if let Some(ref file) = entry.file {
                if let Some(line) = entry.line {
                    location_parts.push(format!("{}:{}", file, line));
                } else {
                    location_parts.push(file.clone());
                }
            }
            
            if !location_parts.is_empty() {
                parts.push(format!("({})", location_parts.join(":")));
            }
        }
        
        // Message
        parts.push(entry.message.clone());
        
        // Metadata
        if self.include_metadata && !entry.metadata.is_empty() {
            let metadata_str = entry.metadata
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join(" ");
            parts.push(format!("[{}]", metadata_str));
        }
        
        Ok(parts.join(" ") + "\n")
    }
}

/// Compact formatter for minimal output
pub struct CompactFormatter;

impl CompactFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl LogFormatter for CompactFormatter {
    fn format(&self, entry: &LogEntry) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!(
            "{} [{}] {}\n",
            entry.level.as_str(),
            entry.thread_id,
            entry.message
        ))
    }
}

/// Custom formatter that allows user-defined format strings
pub struct CustomFormatter {
    format_string: String,
}

impl CustomFormatter {
    /// Create a new custom formatter with a format string
    /// 
    /// Format placeholders:
    /// - {timestamp} - Unix timestamp
    /// - {level} - Log level
    /// - {message} - Log message
    /// - {thread_id} - Thread ID
    /// - {module} - Module name (if available)
    /// - {file} - File name (if available)
    /// - {line} - Line number (if available)
    pub fn new(format_string: String) -> Self {
        Self { format_string }
    }
}

impl LogFormatter for CustomFormatter {
    fn format(&self, entry: &LogEntry) -> Result<String, Box<dyn std::error::Error>> {
        let mut result = self.format_string.clone();
        
        // Replace placeholders
        result = result.replace("{timestamp}", &entry.timestamp.to_string());
        result = result.replace("{level}", entry.level.as_str());
        result = result.replace("{message}", &entry.message);
        result = result.replace("{thread_id}", &entry.thread_id.to_string());
        
        if let Some(ref module) = entry.module {
            result = result.replace("{module}", module);
        } else {
            result = result.replace("{module}", "unknown");
        }
        
        if let Some(ref file) = entry.file {
            result = result.replace("{file}", file);
        } else {
            result = result.replace("{file}", "unknown");
        }
        
        if let Some(line) = entry.line {
            result = result.replace("{line}", &line.to_string());
        } else {
            result = result.replace("{line}", "0");
        }
        
        Ok(result + "\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logger::LogLevel;
    use std::collections::HashMap;

    fn create_test_entry() -> LogEntry {
        let mut metadata = HashMap::new();
        metadata.insert("key1".to_string(), serde_json::Value::String("value1".to_string()));
        metadata.insert("key2".to_string(), serde_json::Value::Number(42.into()));
        
        LogEntry {
            timestamp: 1640995200000, // 2022-01-01 00:00:00 UTC
            level: LogLevel::Info,
            message: "Test message".to_string(),
            module: Some("test_module".to_string()),
            file: Some("test.rs".to_string()),
            line: Some(42),
            thread_id: 12345,
            metadata,
        }
    }

    #[test]
    fn test_json_formatter() {
        let formatter = JsonFormatter::new();
        let entry = create_test_entry();
        let result = formatter.format(&entry).unwrap();
        
        assert!(result.contains("\"level\":\"INFO\""));
        assert!(result.contains("\"message\":\"Test message\""));
        assert!(result.contains("\"thread_id\":12345"));
        assert!(result.contains("\"metadata\""));
        assert!(result.ends_with("\n"));
    }

    #[test]
    fn test_json_formatter_pretty() {
        let formatter = JsonFormatter::pretty();
        let entry = create_test_entry();
        let result = formatter.format(&entry).unwrap();
        
        // Pretty format should contain newlines and indentation
        assert!(result.contains("\n"));
        assert!(result.contains("  "));
    }

    #[test]
    fn test_text_formatter() {
        let formatter = TextFormatter::new();
        let entry = create_test_entry();
        let result = formatter.format(&entry).unwrap();
        
        assert!(result.contains("INFO"));
        assert!(result.contains("Test message"));
        assert!(result.contains("12345"));
        assert!(result.ends_with("\n"));
    }

    #[test]
    fn test_text_formatter_without_timestamp() {
        let formatter = TextFormatter::new().with_timestamp(false);
        let entry = create_test_entry();
        let result = formatter.format(&entry).unwrap();
        
        assert!(!result.contains("1640995200000"));
        assert!(result.contains("INFO"));
        assert!(result.contains("Test message"));
    }

    #[test]
    fn test_text_formatter_with_location() {
        let formatter = TextFormatter::new().with_location(true);
        let entry = create_test_entry();
        let result = formatter.format(&entry).unwrap();
        
        assert!(result.contains("test_module"));
        assert!(result.contains("test.rs:42"));
    }

    #[test]
    fn test_compact_formatter() {
        let formatter = CompactFormatter::new();
        let entry = create_test_entry();
        let result = formatter.format(&entry).unwrap();
        
        assert_eq!(result, "INFO [12345] Test message\n");
    }

    #[test]
    fn test_custom_formatter() {
        let formatter = CustomFormatter::new(
            "{level} - {message} (thread: {thread_id})".to_string()
        );
        let entry = create_test_entry();
        let result = formatter.format(&entry).unwrap();
        
        assert_eq!(result, "INFO - Test message (thread: 12345)\n");
    }

    #[test]
    fn test_custom_formatter_with_location() {
        let formatter = CustomFormatter::new(
            "{file}:{line} [{level}] {message}".to_string()
        );
        let entry = create_test_entry();
        let result = formatter.format(&entry).unwrap();
        
        assert_eq!(result, "test.rs:42 [INFO] Test message\n");
    }

    #[test]
    fn test_formatter_with_missing_fields() {
        let formatter = TextFormatter::new().with_location(true);
        let mut entry = create_test_entry();
        entry.module = None;
        entry.file = None;
        entry.line = None;
        
        let result = formatter.format(&entry).unwrap();
        assert!(result.contains("INFO"));
        assert!(result.contains("Test message"));
        // Should not contain location info
        assert!(!result.contains("("));
    }
}