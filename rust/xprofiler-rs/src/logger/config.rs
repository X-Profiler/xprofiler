//! Logger configuration

use super::LogLevel;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Logger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    /// Minimum log level to output
    pub min_level: LogLevel,
    
    /// Whether to enable console output
    pub console_enabled: bool,
    
    /// Whether to enable file output
    pub file_enabled: bool,
    
    /// Log file path (if file output is enabled)
    pub file_path: Option<PathBuf>,
    
    /// Maximum log file size in bytes before rotation
    pub max_file_size: u64,
    
    /// Maximum number of rotated log files to keep
    pub max_files: u32,
    
    /// Log format ("json" or "text")
    pub format: LogFormat,
    
    /// Whether to include timestamps
    pub include_timestamp: bool,
    
    /// Whether to include thread ID
    pub include_thread_id: bool,
    
    /// Whether to include file and line information
    pub include_location: bool,
    
    /// Buffer size for async logging
    pub buffer_size: usize,
    
    /// Flush interval in milliseconds
    pub flush_interval_ms: u64,
    
    /// Whether to enable async logging
    pub async_logging: bool,
}

/// Log format options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogFormat {
    Json,
    Text,
}

impl LogFormat {
    /// Convert to string
    pub fn as_str(&self) -> &'static str {
        match self {
            LogFormat::Json => "json",
            LogFormat::Text => "text",
        }
    }
    
    /// Parse from string
    pub fn from_str(s: &str) -> Result<LogFormat, Box<dyn std::error::Error>> {
        match s.to_lowercase().as_str() {
            "json" => Ok(LogFormat::Json),
            "text" => Ok(LogFormat::Text),
            _ => Err(format!("Invalid log format: {}", s).into()),
        }
    }
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            min_level: LogLevel::Info,
            console_enabled: true,
            file_enabled: false,
            file_path: None,
            max_file_size: 10 * 1024 * 1024, // 10MB
            max_files: 5,
            format: LogFormat::Json,
            include_timestamp: true,
            include_thread_id: true,
            include_location: false,
            buffer_size: 1024,
            flush_interval_ms: 1000,
            async_logging: false,
        }
    }
}

impl LogConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set minimum log level
    pub fn with_min_level(mut self, level: LogLevel) -> Self {
        self.min_level = level;
        self
    }
    
    /// Enable console output
    pub fn with_console(mut self, enabled: bool) -> Self {
        self.console_enabled = enabled;
        self
    }
    
    /// Enable file output with path
    pub fn with_file<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.file_enabled = true;
        self.file_path = Some(path.into());
        self
    }
    
    /// Set log format
    pub fn with_format(mut self, format: LogFormat) -> Self {
        self.format = format;
        self
    }
    
    /// Set file rotation settings
    pub fn with_rotation(mut self, max_size: u64, max_files: u32) -> Self {
        self.max_file_size = max_size;
        self.max_files = max_files;
        self
    }
    
    /// Enable/disable timestamp inclusion
    pub fn with_timestamp(mut self, enabled: bool) -> Self {
        self.include_timestamp = enabled;
        self
    }
    
    /// Enable/disable thread ID inclusion
    pub fn with_thread_id(mut self, enabled: bool) -> Self {
        self.include_thread_id = enabled;
        self
    }
    
    /// Enable/disable location information
    pub fn with_location(mut self, enabled: bool) -> Self {
        self.include_location = enabled;
        self
    }
    
    /// Enable async logging with buffer settings
    pub fn with_async(mut self, buffer_size: usize, flush_interval_ms: u64) -> Self {
        self.async_logging = true;
        self.buffer_size = buffer_size;
        self.flush_interval_ms = flush_interval_ms;
        self
    }
    
    /// Validate the configuration
    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.console_enabled && !self.file_enabled {
            return Err("At least one output method (console or file) must be enabled".into());
        }
        
        if self.file_enabled && self.file_path.is_none() {
            return Err("File path must be specified when file output is enabled".into());
        }
        
        if self.max_file_size == 0 {
            return Err("Maximum file size must be greater than 0".into());
        }
        
        if self.max_files == 0 {
            return Err("Maximum number of files must be greater than 0".into());
        }
        
        if self.buffer_size == 0 {
            return Err("Buffer size must be greater than 0".into());
        }
        
        if self.flush_interval_ms == 0 {
            return Err("Flush interval must be greater than 0".into());
        }
        
        Ok(())
    }
    
    /// Create a development configuration (debug level, console only)
    pub fn development() -> Self {
        Self::default()
            .with_min_level(LogLevel::Debug)
            .with_console(true)
            .with_format(LogFormat::Text)
            .with_location(true)
    }
    
    /// Create a production configuration (info level, file output)
    pub fn production<P: Into<PathBuf>>(log_file: P) -> Self {
        Self::default()
            .with_min_level(LogLevel::Info)
            .with_console(false)
            .with_file(log_file)
            .with_format(LogFormat::Json)
            .with_async(2048, 5000)
    }
    
    /// Create a configuration for testing (warn level, console only)
    pub fn testing() -> Self {
        Self::default()
            .with_min_level(LogLevel::Warn)
            .with_console(true)
            .with_format(LogFormat::Text)
    }
    
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = Self::default();
        
        // Log level
        if let Ok(level_str) = std::env::var("XPROFILER_LOG_LEVEL") {
            config.min_level = LogLevel::from_str(&level_str)?;
        }
        
        // Console output
        if let Ok(console_str) = std::env::var("XPROFILER_LOG_CONSOLE") {
            config.console_enabled = console_str.parse::<bool>()
                .unwrap_or(config.console_enabled);
        }
        
        // File output
        if let Ok(file_path) = std::env::var("XPROFILER_LOG_FILE") {
            config = config.with_file(file_path);
        }
        
        // Log format
        if let Ok(format_str) = std::env::var("XPROFILER_LOG_FORMAT") {
            config.format = LogFormat::from_str(&format_str)?;
        }
        
        // Max file size
        if let Ok(size_str) = std::env::var("XPROFILER_LOG_MAX_SIZE") {
            config.max_file_size = size_str.parse::<u64>()
                .unwrap_or(config.max_file_size);
        }
        
        // Max files
        if let Ok(files_str) = std::env::var("XPROFILER_LOG_MAX_FILES") {
            config.max_files = files_str.parse::<u32>()
                .unwrap_or(config.max_files);
        }
        
        // Async logging
        if let Ok(async_str) = std::env::var("XPROFILER_LOG_ASYNC") {
            config.async_logging = async_str.parse::<bool>()
                .unwrap_or(config.async_logging);
        }
        
        config.validate()?;
        Ok(config)
    }
    
    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(self)?)
    }
    
    /// Load from JSON string
    pub fn from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config: Self = serde_json::from_str(json)?;
        config.validate()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_config() {
        let config = LogConfig::default();
        assert_eq!(config.min_level, LogLevel::Info);
        assert!(config.console_enabled);
        assert!(!config.file_enabled);
        assert_eq!(config.format, LogFormat::Json);
    }

    #[test]
    fn test_config_builder() {
        let config = LogConfig::new()
            .with_min_level(LogLevel::Debug)
            .with_console(false)
            .with_file("/tmp/test.log")
            .with_format(LogFormat::Text);
        
        assert_eq!(config.min_level, LogLevel::Debug);
        assert!(!config.console_enabled);
        assert!(config.file_enabled);
        assert_eq!(config.file_path, Some(PathBuf::from("/tmp/test.log")));
        assert_eq!(config.format, LogFormat::Text);
    }

    #[test]
    fn test_config_validation() {
        // Valid config
        let valid_config = LogConfig::default();
        assert!(valid_config.validate().is_ok());
        
        // Invalid config - no output methods
        let invalid_config = LogConfig::default()
            .with_console(false);
        assert!(invalid_config.validate().is_err());
        
        // Invalid config - file enabled but no path
        let mut invalid_config = LogConfig::default();
        invalid_config.file_enabled = true;
        invalid_config.file_path = None;
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_preset_configs() {
        let dev_config = LogConfig::development();
        assert_eq!(dev_config.min_level, LogLevel::Debug);
        assert!(dev_config.console_enabled);
        assert_eq!(dev_config.format, LogFormat::Text);
        assert!(dev_config.include_location);
        
        let prod_config = LogConfig::production("/var/log/app.log");
        assert_eq!(prod_config.min_level, LogLevel::Info);
        assert!(!prod_config.console_enabled);
        assert!(prod_config.file_enabled);
        assert_eq!(prod_config.format, LogFormat::Json);
        assert!(prod_config.async_logging);
        
        let test_config = LogConfig::testing();
        assert_eq!(test_config.min_level, LogLevel::Warn);
        assert!(test_config.console_enabled);
        assert_eq!(test_config.format, LogFormat::Text);
    }

    #[test]
    fn test_log_format_conversion() {
        assert_eq!(LogFormat::Json.as_str(), "json");
        assert_eq!(LogFormat::Text.as_str(), "text");
        
        assert_eq!(LogFormat::from_str("json").unwrap(), LogFormat::Json);
        assert_eq!(LogFormat::from_str("JSON").unwrap(), LogFormat::Json);
        assert_eq!(LogFormat::from_str("text").unwrap(), LogFormat::Text);
        assert!(LogFormat::from_str("invalid").is_err());
    }

    #[test]
    fn test_json_serialization() {
        let config = LogConfig::development();
        let json = config.to_json().unwrap();
        let deserialized = LogConfig::from_json(&json).unwrap();
        
        assert_eq!(config.min_level, deserialized.min_level);
        assert_eq!(config.console_enabled, deserialized.console_enabled);
        assert_eq!(config.format, deserialized.format);
    }

    #[test]
    fn test_env_config() {
        // Set environment variables
        env::set_var("XPROFILER_LOG_LEVEL", "DEBUG");
        env::set_var("XPROFILER_LOG_CONSOLE", "false");
        env::set_var("XPROFILER_LOG_FILE", "/tmp/test.log");
        env::set_var("XPROFILER_LOG_FORMAT", "text");
        
        let config = LogConfig::from_env().unwrap();
        
        assert_eq!(config.min_level, LogLevel::Debug);
        assert!(!config.console_enabled);
        assert!(config.file_enabled);
        assert_eq!(config.file_path, Some(PathBuf::from("/tmp/test.log")));
        assert_eq!(config.format, LogFormat::Text);
        
        // Clean up
        env::remove_var("XPROFILER_LOG_LEVEL");
        env::remove_var("XPROFILER_LOG_CONSOLE");
        env::remove_var("XPROFILER_LOG_FILE");
        env::remove_var("XPROFILER_LOG_FORMAT");
    }
}