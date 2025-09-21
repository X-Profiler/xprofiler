//! Logger bindings for JavaScript interface

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;

use crate::logger::{GLOBAL_LOGGER, LogLevel, LogConfig, LogFormat};

/// Logger manager class for JavaScript
#[napi]
pub struct LoggerManager {
    initialized: bool,
}

#[napi]
impl LoggerManager {
    /// Create a new logger manager
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            initialized: false,
        }
    }
    
    /// Initialize the logger with configuration
    #[napi]
    pub fn initialize(&mut self, config: Option<Object>) -> Result<()> {
        let log_config = if let Some(config_obj) = config {
            object_to_log_config(config_obj)?
        } else {
            LogConfig::default()
        };
        
        GLOBAL_LOGGER.initialize(log_config);
        self.initialized = true;
        Ok(())
    }
    
    /// Check if logger is initialized
    #[napi]
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    /// Set log level
    #[napi]
    pub fn set_level(&self, level: String) -> Result<()> {
        let log_level = string_to_log_level(&level)?;
        GLOBAL_LOGGER.set_level(log_level);
        Ok(())
    }
    
    /// Get current log level
    #[napi]
    pub fn get_level(&self) -> String {
        log_level_to_string(GLOBAL_LOGGER.get_level())
    }
    
    /// Log a trace message
    #[napi]
    pub fn trace(&self, message: String, metadata: Option<Object>) -> Result<()> {
        let meta = if let Some(obj) = metadata {
            object_to_metadata(obj)?
        } else {
            HashMap::new()
        };
        
        GLOBAL_LOGGER.trace(&message, meta);
        Ok(())
    }
    
    /// Log a debug message
    #[napi]
    pub fn debug(&self, message: String, metadata: Option<Object>) -> Result<()> {
        let meta = if let Some(obj) = metadata {
            object_to_metadata(obj)?
        } else {
            HashMap::new()
        };
        
        GLOBAL_LOGGER.debug(&message, meta);
        Ok(())
    }
    
    /// Log an info message
    #[napi]
    pub fn info(&self, message: String, metadata: Option<Object>) -> Result<()> {
        let meta = if let Some(obj) = metadata {
            object_to_metadata(obj)?
        } else {
            HashMap::new()
        };
        
        GLOBAL_LOGGER.info(&message, meta);
        Ok(())
    }
    
    /// Log a warning message
    #[napi]
    pub fn warn(&self, message: String, metadata: Option<Object>) -> Result<()> {
        let meta = if let Some(obj) = metadata {
            object_to_metadata(obj)?
        } else {
            HashMap::new()
        };
        
        GLOBAL_LOGGER.warn(&message, meta);
        Ok(())
    }
    
    /// Log an error message
    #[napi]
    pub fn error(&self, message: String, metadata: Option<Object>) -> Result<()> {
        let meta = if let Some(obj) = metadata {
            object_to_metadata(obj)?
        } else {
            HashMap::new()
        };
        
        GLOBAL_LOGGER.error(&message, meta);
        Ok(())
    }
    
    /// Log a fatal message
    #[napi]
    pub fn fatal(&self, message: String, metadata: Option<Object>) -> Result<()> {
        let meta = if let Some(obj) = metadata {
            object_to_metadata(obj)?
        } else {
            HashMap::new()
        };
        
        GLOBAL_LOGGER.fatal(&message, meta);
        Ok(())
    }
    
    /// Log with specific level
    #[napi]
    pub fn log(&self, level: String, message: String, metadata: Option<Object>) -> Result<()> {
        let log_level = string_to_log_level(&level)?;
        let meta = if let Some(obj) = metadata {
            object_to_metadata(obj)?
        } else {
            HashMap::new()
        };
        
        GLOBAL_LOGGER.log(log_level, &message, meta);
        Ok(())
    }
    
    /// Check if level is enabled
    #[napi]
    pub fn is_level_enabled(&self, level: String) -> Result<bool> {
        let log_level = string_to_log_level(&level)?;
        Ok(GLOBAL_LOGGER.is_level_enabled(log_level))
    }
    
    /// Flush all pending log messages
    #[napi]
    pub fn flush(&self) -> Result<()> {
        GLOBAL_LOGGER.flush();
        Ok(())
    }
    
    /// Get logger configuration
    #[napi]
    pub fn get_config(&self) -> Result<Object> {
        let config = GLOBAL_LOGGER.get_config();
        log_config_to_object(config)
    }
    
    /// Update logger configuration
    #[napi]
    pub fn update_config(&self, config: Object) -> Result<()> {
        let log_config = object_to_log_config(config)?;
        GLOBAL_LOGGER.update_config(log_config);
        Ok(())
    }
    
    /// Enable or disable the logger
    #[napi]
    pub fn set_enabled(&self, enabled: bool) -> Result<()> {
        GLOBAL_LOGGER.set_enabled(enabled);
        Ok(())
    }
    
    /// Check if logger is enabled
    #[napi]
    pub fn is_enabled(&self) -> bool {
        GLOBAL_LOGGER.is_enabled()
    }
    
    /// Add a custom field to all log messages
    #[napi]
    pub fn add_global_field(&self, key: String, value: String) -> Result<()> {
        GLOBAL_LOGGER.add_global_field(key, value);
        Ok(())
    }
    
    /// Remove a global field
    #[napi]
    pub fn remove_global_field(&self, key: String) -> Result<()> {
        GLOBAL_LOGGER.remove_global_field(&key);
        Ok(())
    }
    
    /// Clear all global fields
    #[napi]
    pub fn clear_global_fields(&self) -> Result<()> {
        GLOBAL_LOGGER.clear_global_fields();
        Ok(())
    }
    
    /// Get all global fields
    #[napi]
    pub fn get_global_fields(&self) -> Result<Object> {
        let fields = GLOBAL_LOGGER.get_global_fields();
        let mut result = Object::new();
        
        for (key, value) in fields {
            result.set(key, value)?;
        }
        
        Ok(result)
    }
    
    /// Create a child logger with additional context
    #[napi]
    pub fn create_child(&self, context: Object) -> Result<LoggerManager> {
        let ctx = object_to_metadata(context)?;
        GLOBAL_LOGGER.create_child(ctx);
        
        Ok(LoggerManager {
            initialized: true,
        })
    }
    
    /// Set log format
    #[napi]
    pub fn set_format(&self, format: String) -> Result<()> {
        let log_format = string_to_log_format(&format)?;
        GLOBAL_LOGGER.set_format(log_format);
        Ok(())
    }
    
    /// Get current log format
    #[napi]
    pub fn get_format(&self) -> String {
        log_format_to_string(GLOBAL_LOGGER.get_format())
    }
    
    /// Set output file path
    #[napi]
    pub fn set_output_file(&self, file_path: Option<String>) -> Result<()> {
        GLOBAL_LOGGER.set_output_file(file_path);
        Ok(())
    }
    
    /// Get current output file path
    #[napi]
    pub fn get_output_file(&self) -> Option<String> {
        GLOBAL_LOGGER.get_output_file()
    }
    
    /// Rotate log file (if file output is enabled)
    #[napi]
    pub fn rotate_log_file(&self) -> Result<()> {
        GLOBAL_LOGGER.rotate_log_file();
        Ok(())
    }
    
    /// Get logger statistics
    #[napi]
    pub fn get_statistics(&self) -> Result<Object> {
        let stats = GLOBAL_LOGGER.get_statistics();
        let mut result = Object::new();
        
        result.set("total_messages", stats.total_messages)?;
        result.set("messages_by_level", {
            let mut level_stats = Object::new();
            for (level, count) in stats.messages_by_level {
                level_stats.set(log_level_to_string(level), count)?;
            }
            level_stats
        })?;
        result.set("start_time", stats.start_time)?;
        result.set("last_message_time", stats.last_message_time)?;
        
        Ok(result)
    }
    
    /// Reset logger statistics
    #[napi]
    pub fn reset_statistics(&self) -> Result<()> {
        GLOBAL_LOGGER.reset_statistics();
        Ok(())
    }
}

/// Standalone logger functions

/// Initialize logger with configuration (standalone function)
#[napi]
pub fn init_logger(config: Option<Object>) -> Result<()> {
    let log_config = if let Some(config_obj) = config {
        object_to_log_config(config_obj)?
    } else {
        LogConfig::default()
    };
    
    GLOBAL_LOGGER.initialize(log_config);
    Ok(())
}

/// Log trace message (standalone function)
#[napi]
pub fn log_trace(message: String, metadata: Option<Object>) -> Result<()> {
    let meta = if let Some(obj) = metadata {
        object_to_metadata(obj)?
    } else {
        HashMap::new()
    };
    
    GLOBAL_LOGGER.trace(&message, meta);
    Ok(())
}

/// Log debug message (standalone function)
#[napi]
pub fn log_debug(message: String, metadata: Option<Object>) -> Result<()> {
    let meta = if let Some(obj) = metadata {
        object_to_metadata(obj)?
    } else {
        HashMap::new()
    };
    
    GLOBAL_LOGGER.debug(&message, meta);
    Ok(())
}

/// Log info message (standalone function)
#[napi]
pub fn log_info(message: String, metadata: Option<Object>) -> Result<()> {
    let meta = if let Some(obj) = metadata {
        object_to_metadata(obj)?
    } else {
        HashMap::new()
    };
    
    GLOBAL_LOGGER.info(&message, meta);
    Ok(())
}

/// Log warning message (standalone function)
#[napi]
pub fn log_warn(message: String, metadata: Option<Object>) -> Result<()> {
    let meta = if let Some(obj) = metadata {
        object_to_metadata(obj)?
    } else {
        HashMap::new()
    };
    
    GLOBAL_LOGGER.warn(&message, meta);
    Ok(())
}

/// Log error message (standalone function)
#[napi]
pub fn log_error(message: String, metadata: Option<Object>) -> Result<()> {
    let meta = if let Some(obj) = metadata {
        object_to_metadata(obj)?
    } else {
        HashMap::new()
    };
    
    GLOBAL_LOGGER.error(&message, meta);
    Ok(())
}

/// Log fatal message (standalone function)
#[napi]
pub fn log_fatal(message: String, metadata: Option<Object>) -> Result<()> {
    let meta = if let Some(obj) = metadata {
        object_to_metadata(obj)?
    } else {
        HashMap::new()
    };
    
    GLOBAL_LOGGER.fatal(&message, meta);
    Ok(())
}

/// Set log level (standalone function)
#[napi]
pub fn set_log_level(level: String) -> Result<()> {
    let log_level = string_to_log_level(&level)?;
    GLOBAL_LOGGER.set_level(log_level);
    Ok(())
}

/// Get log level (standalone function)
#[napi]
pub fn get_log_level() -> String {
    log_level_to_string(GLOBAL_LOGGER.get_level())
}

/// Check if log level is enabled (standalone function)
#[napi]
pub fn is_log_level_enabled(level: String) -> Result<bool> {
    let log_level = string_to_log_level(&level)?;
    Ok(GLOBAL_LOGGER.is_level_enabled(log_level))
}

/// Flush logger (standalone function)
#[napi]
pub fn flush_logger() -> Result<()> {
    GLOBAL_LOGGER.flush();
    Ok(())
}

/// Utility functions for data conversion

fn string_to_log_level(level: &str) -> Result<LogLevel> {
    match level.to_lowercase().as_str() {
        "trace" => Ok(LogLevel::Trace),
        "debug" => Ok(LogLevel::Debug),
        "info" => Ok(LogLevel::Info),
        "warn" | "warning" => Ok(LogLevel::Warn),
        "error" => Ok(LogLevel::Error),
        "fatal" => Ok(LogLevel::Fatal),
        _ => Err(Error::new(Status::InvalidArg, format!("Invalid log level: {}", level))),
    }
}

fn log_level_to_string(level: LogLevel) -> String {
    match level {
        LogLevel::Trace => "trace".to_string(),
        LogLevel::Debug => "debug".to_string(),
        LogLevel::Info => "info".to_string(),
        LogLevel::Warn => "warn".to_string(),
        LogLevel::Error => "error".to_string(),
        LogLevel::Fatal => "fatal".to_string(),
    }
}

fn string_to_log_format(format: &str) -> Result<LogFormat> {
    match format.to_lowercase().as_str() {
        "json" => Ok(LogFormat::Json),
        "text" | "plain" => Ok(LogFormat::Text),
        "compact" => Ok(LogFormat::Compact),
        "custom" => Ok(LogFormat::Custom),
        _ => Err(Error::new(Status::InvalidArg, format!("Invalid log format: {}", format))),
    }
}

fn log_format_to_string(format: LogFormat) -> String {
    match format {
        LogFormat::Json => "json".to_string(),
        LogFormat::Text => "text".to_string(),
        LogFormat::Compact => "compact".to_string(),
        LogFormat::Custom => "custom".to_string(),
    }
}

fn object_to_log_config(obj: Object) -> Result<LogConfig> {
    let mut config = LogConfig::default();
    
    if let Some(level_str) = obj.get::<String>("level")? {
        config.level = string_to_log_level(&level_str)?;
    }
    
    if let Some(format_str) = obj.get::<String>("format")? {
        config.format = string_to_log_format(&format_str)?;
    }
    
    if let Some(enabled) = obj.get::<bool>("enabled")? {
        config.enabled = enabled;
    }
    
    if let Some(file_path) = obj.get::<String>("file_path")? {
        config.file_path = Some(file_path);
    }
    
    if let Some(console_output) = obj.get::<bool>("console_output")? {
        config.console_output = console_output;
    }
    
    if let Some(buffer_size) = obj.get::<u32>("buffer_size")? {
        config.buffer_size = buffer_size as usize;
    }
    
    if let Some(max_file_size) = obj.get::<u64>("max_file_size")? {
        config.max_file_size = Some(max_file_size);
    }
    
    if let Some(max_files) = obj.get::<u32>("max_files")? {
        config.max_files = Some(max_files);
    }
    
    Ok(config)
}

fn log_config_to_object(config: LogConfig) -> Result<Object> {
    let mut result = Object::new();
    
    result.set("level", log_level_to_string(config.level))?;
    result.set("format", log_format_to_string(config.format))?;
    result.set("enabled", config.enabled)?;
    result.set("console_output", config.console_output)?;
    result.set("buffer_size", config.buffer_size as u32)?;
    
    if let Some(file_path) = config.file_path {
        result.set("file_path", file_path)?;
    }
    
    if let Some(max_file_size) = config.max_file_size {
        result.set("max_file_size", max_file_size)?;
    }
    
    if let Some(max_files) = config.max_files {
        result.set("max_files", max_files)?;
    }
    
    Ok(result)
}

fn object_to_metadata(obj: Object) -> Result<HashMap<String, String>> {
    let mut metadata = HashMap::new();
    
    // Get all property names
    let property_names = obj.get_property_names()?;
    let length = property_names.get_array_length()?;
    
    for i in 0..length {
        if let Some(key) = property_names.get::<String>(i)? {
            if let Some(value) = obj.get::<String>(&key)? {
                metadata.insert(key, value);
            } else if let Some(value) = obj.get::<f64>(&key)? {
                metadata.insert(key, value.to_string());
            } else if let Some(value) = obj.get::<bool>(&key)? {
                metadata.insert(key, value.to_string());
            }
        }
    }
    
    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_logger_manager_creation() {
        let manager = LoggerManager::new();
        assert!(!manager.is_initialized());
    }
    
    #[test]
    fn test_log_level_conversion() {
        assert_eq!(string_to_log_level("trace").unwrap(), LogLevel::Trace);
        assert_eq!(string_to_log_level("debug").unwrap(), LogLevel::Debug);
        assert_eq!(string_to_log_level("info").unwrap(), LogLevel::Info);
        assert_eq!(string_to_log_level("warn").unwrap(), LogLevel::Warn);
        assert_eq!(string_to_log_level("error").unwrap(), LogLevel::Error);
        assert_eq!(string_to_log_level("fatal").unwrap(), LogLevel::Fatal);
        
        assert_eq!(log_level_to_string(LogLevel::Trace), "trace");
        assert_eq!(log_level_to_string(LogLevel::Debug), "debug");
        assert_eq!(log_level_to_string(LogLevel::Info), "info");
        assert_eq!(log_level_to_string(LogLevel::Warn), "warn");
        assert_eq!(log_level_to_string(LogLevel::Error), "error");
        assert_eq!(log_level_to_string(LogLevel::Fatal), "fatal");
    }
    
    #[test]
    fn test_log_format_conversion() {
        assert_eq!(string_to_log_format("json").unwrap(), LogFormat::Json);
        assert_eq!(string_to_log_format("text").unwrap(), LogFormat::Text);
        assert_eq!(string_to_log_format("compact").unwrap(), LogFormat::Compact);
        assert_eq!(string_to_log_format("custom").unwrap(), LogFormat::Custom);
        
        assert_eq!(log_format_to_string(LogFormat::Json), "json");
        assert_eq!(log_format_to_string(LogFormat::Text), "text");
        assert_eq!(log_format_to_string(LogFormat::Compact), "compact");
        assert_eq!(log_format_to_string(LogFormat::Custom), "custom");
    }
    
    #[test]
    fn test_invalid_log_level() {
        assert!(string_to_log_level("invalid").is_err());
    }
    
    #[test]
    fn test_invalid_log_format() {
        assert!(string_to_log_format("invalid").is_err());
    }
}