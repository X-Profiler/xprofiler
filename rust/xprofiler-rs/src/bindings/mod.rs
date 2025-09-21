//! NAPI bindings for JavaScript interface

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;

use crate::config::{CONFIG_STORE, ConfigValue};
use crate::environment::{ENVIRONMENT_REGISTRY, EnvironmentData};
use crate::logger::{GLOBAL_LOGGER, LogLevel};
use crate::utils::time::get_timestamp_ms;

// Re-export sub-modules
pub mod config;
pub mod environment;
pub mod logger;
pub mod monitoring;

// Re-export main classes for convenience
pub use config::ConfigManager;
pub use environment::EnvironmentManager;
pub use logger::LoggerManager;
pub use monitoring::MonitoringManager;

/// Main XProfiler class for JavaScript
#[napi]
pub struct XProfiler {
    initialized: bool,
    config_manager: Option<ConfigManager>,
    environment_manager: Option<EnvironmentManager>,
    logger_manager: Option<LoggerManager>,
    monitoring_manager: Option<MonitoringManager>,
}

#[napi]
impl XProfiler {
    /// Create a new XProfiler instance
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            initialized: false,
            config_manager: None,
            environment_manager: None,
            logger_manager: None,
            monitoring_manager: None,
        }
    }
    
    /// Initialize XProfiler with configuration
    #[napi]
    pub fn initialize(&mut self, config: Option<Object>) -> Result<()> {
        // Initialize configuration
        CONFIG_STORE.initialize();
        
        // Apply configuration if provided
        if let Some(config_obj) = config {
            self.apply_config(config_obj)?;
        }
        
        // Initialize sub-managers
        let mut config_manager = ConfigManager::new();
        config_manager.initialize()?;
        self.config_manager = Some(config_manager);
        
        let mut environment_manager = EnvironmentManager::new();
        environment_manager.initialize()?;
        self.environment_manager = Some(environment_manager);
        
        let mut logger_manager = LoggerManager::new();
        logger_manager.initialize(None)?;
        self.logger_manager = Some(logger_manager);
        
        let mut monitoring_manager = MonitoringManager::new();
        monitoring_manager.initialize(None)?;
        self.monitoring_manager = Some(monitoring_manager);
        
        self.initialized = true;
        Ok(())
    }
    
    /// Check if XProfiler is initialized
    #[napi]
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    /// Get configuration manager
    #[napi]
    pub fn get_config_manager(&self) -> Result<ConfigManager> {
        match &self.config_manager {
            Some(manager) => Ok(manager.clone()),
            None => Err(Error::new(Status::InvalidArg, "XProfiler not initialized")),
        }
    }
    
    /// Get environment manager
    #[napi]
    pub fn get_environment_manager(&self) -> Result<EnvironmentManager> {
        match &self.environment_manager {
            Some(manager) => Ok(manager.clone()),
            None => Err(Error::new(Status::InvalidArg, "XProfiler not initialized")),
        }
    }
    
    /// Get logger manager
    #[napi]
    pub fn get_logger_manager(&self) -> Result<LoggerManager> {
        match &self.logger_manager {
            Some(manager) => Ok(manager.clone()),
            None => Err(Error::new(Status::InvalidArg, "XProfiler not initialized")),
        }
    }
    
    /// Get monitoring manager
    #[napi]
    pub fn get_monitoring_manager(&self) -> Result<MonitoringManager> {
        match &self.monitoring_manager {
            Some(manager) => Ok(manager.clone()),
            None => Err(Error::new(Status::InvalidArg, "XProfiler not initialized")),
        }
    }
    
    /// Apply configuration from JavaScript object
    #[napi]
    pub fn apply_config(&mut self, config: Object) -> Result<()> {
        let property_names = config.get_property_names()?;
        let length = property_names.get_array_length()?;
        
        for i in 0..length {
            if let Some(key) = property_names.get::<String>(i)? {
                if let Some(value) = config.get::<String>(&key)? {
                    CONFIG_STORE.set(&key, ConfigValue::String(value));
                } else if let Some(value) = config.get::<f64>(&key)? {
                    CONFIG_STORE.set(&key, ConfigValue::Number(value));
                } else if let Some(value) = config.get::<bool>(&key)? {
                    CONFIG_STORE.set(&key, ConfigValue::Boolean(value));
                }
            }
        }
        
        Ok(())
    }
    
    /// Get current configuration as JavaScript object
    #[napi]
    pub fn get_config(&self) -> Result<Object> {
        let config_map = CONFIG_STORE.get_all();
        let mut result = Object::new();
        
        for (key, value) in config_map {
            match value {
                ConfigValue::String(s) => result.set(key, s)?,
                ConfigValue::Number(n) => result.set(key, n)?,
                ConfigValue::Boolean(b) => result.set(key, b)?,
                ConfigValue::Array(arr) => {
                    let mut js_array = Array::new(arr.len() as u32)?;
                    for (index, item) in arr.iter().enumerate() {
                        match item {
                            ConfigValue::String(s) => js_array.set(index as u32, s.clone())?,
                            ConfigValue::Number(n) => js_array.set(index as u32, *n)?,
                            ConfigValue::Boolean(b) => js_array.set(index as u32, *b)?,
                            _ => {} // Skip nested arrays/objects for now
                        }
                    }
                    result.set(key, js_array)?;
                }
                ConfigValue::Object(_) => {
                    // Skip objects for now - would need recursive conversion
                }
            }
        }
        
        Ok(result)
    }
    
    /// Set a configuration value
    #[napi]
    pub fn set_config(&self, key: String, value: JsUnknown) -> Result<()> {
        let config_value = js_value_to_config_value(value)?;
        CONFIG_STORE.set(&key, config_value);
        Ok(())
    }
    
    /// Get a configuration value
    #[napi]
    pub fn get_config_value(&self, key: String) -> Option<String> {
        CONFIG_STORE.get(&key).map(|value| match value {
            ConfigValue::String(s) => s,
            ConfigValue::Number(n) => n.to_string(),
            ConfigValue::Boolean(b) => b.to_string(),
            _ => "[complex value]".to_string(),
        })
    }
    
    /// Start profiling
    #[napi]
    pub fn start_profiling(&self) -> Result<()> {
        if !self.initialized {
            return Err(Error::new(Status::InvalidArg, "XProfiler not initialized"));
        }
        
        // Register current thread for monitoring
        let env_data = EnvironmentData::new();
        ENVIRONMENT_REGISTRY.register_thread(env_data);
        
        // Start monitoring if available
        if let Some(monitoring_manager) = &self.monitoring_manager {
            monitoring_manager.start()?;
        }
        
        GLOBAL_LOGGER.info("XProfiler started", HashMap::new());
        Ok(())
    }
    
    /// Stop profiling
    #[napi]
    pub fn stop_profiling(&self) -> Result<()> {
        ENVIRONMENT_REGISTRY.unregister_current_thread();
        
        // Stop monitoring if available
        if let Some(monitoring_manager) = &self.monitoring_manager {
            monitoring_manager.stop()?;
        }
        
        GLOBAL_LOGGER.info("XProfiler stopped", HashMap::new());
        Ok(())
    }
    
    /// Get current environment data
    #[napi]
    pub fn get_environment_data(&self) -> Result<Object> {
        match ENVIRONMENT_REGISTRY.get_current_thread_data() {
            Some(env_data) => {
                let mut result = Object::new();
                result.set("thread_id", env_data.thread_id)?;
                result.set("is_main_thread", env_data.is_main_thread)?;
                result.set("start_time", env_data.start_time)?;
                result.set("last_update", env_data.last_update)?;
                result.set("sample_count", env_data.sample_count)?;
                Ok(result)
            }
            None => Err(Error::new(Status::InvalidArg, "No environment data available")),
        }
    }
    
    /// Log a message with specified level
    #[napi]
    pub fn log(&self, level: String, message: String, metadata: Option<Object>) -> Result<()> {
        let log_level = match level.to_lowercase().as_str() {
            "trace" => LogLevel::Trace,
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" | "warning" => LogLevel::Warn,
            "error" => LogLevel::Error,
            "fatal" => LogLevel::Fatal,
            _ => return Err(Error::new(Status::InvalidArg, "Invalid log level")),
        };
        
        let meta = if let Some(meta_obj) = metadata {
            object_to_metadata(meta_obj)?
        } else {
            HashMap::new()
        };
        
        GLOBAL_LOGGER.log(log_level, &message, meta);
        Ok(())
    }
    
    /// Get profiling statistics
    #[napi]
    pub fn get_statistics(&self) -> Result<Object> {
        let env_stats = ENVIRONMENT_REGISTRY.get_statistics();
        let log_stats = GLOBAL_LOGGER.get_statistics();
        
        let mut result = Object::new();
        
        // Environment statistics
        let mut env_stats_obj = Object::new();
        env_stats_obj.set("total_threads", env_stats.total_threads)?;
        env_stats_obj.set("active_threads", env_stats.active_threads)?;
        env_stats_obj.set("total_samples", env_stats.total_samples)?;
        env_stats_obj.set("uptime_ms", env_stats.uptime_ms)?;
        result.set("environment", env_stats_obj)?;
        
        // Logger statistics
        let mut log_stats_obj = Object::new();
        log_stats_obj.set("total_messages", log_stats.total_messages)?;
        log_stats_obj.set("start_time", log_stats.start_time)?;
        log_stats_obj.set("last_message_time", log_stats.last_message_time)?;
        result.set("logger", log_stats_obj)?;
        
        // Monitoring statistics if available
        if let Some(monitoring_manager) = &self.monitoring_manager {
            if let Ok(monitoring_stats) = monitoring_manager.get_statistics() {
                result.set("monitoring", monitoring_stats)?;
            }
        }
        
        // General statistics
        result.set("timestamp", get_timestamp_ms())?;
        result.set("initialized", self.initialized)?;
        
        Ok(result)
    }
    
    /// Reset all statistics
    #[napi]
    pub fn reset_statistics(&self) -> Result<()> {
        GLOBAL_LOGGER.reset_statistics();
        
        if let Some(monitoring_manager) = &self.monitoring_manager {
            monitoring_manager.reset_statistics()?;
        }
        
        Ok(())
    }
    
    /// Flush all pending operations
    #[napi]
    pub fn flush(&self) -> Result<()> {
        GLOBAL_LOGGER.flush();
        Ok(())
    }
    
    /// Shutdown XProfiler
    #[napi]
    pub fn shutdown(&mut self) -> Result<()> {
        if self.initialized {
            self.stop_profiling()?;
            self.flush()?;
            self.initialized = false;
            self.config_manager = None;
            self.environment_manager = None;
            self.logger_manager = None;
            self.monitoring_manager = None;
        }
        Ok(())
    }
}

/// Standalone utility functions

/// Initialize XProfiler with default settings (standalone function)
#[napi]
pub fn init_xprofiler(config: Option<Object>) -> Result<()> {
    CONFIG_STORE.initialize();
    
    if let Some(config_obj) = config {
        apply_global_config(config_obj)?;
    }
    
    ENVIRONMENT_REGISTRY.initialize();
    GLOBAL_LOGGER.initialize(crate::logger::LogConfig::default());
    
    Ok(())
}

/// Apply global configuration (standalone function)
#[napi]
pub fn apply_global_config(config: Object) -> Result<()> {
    let property_names = config.get_property_names()?;
    let length = property_names.get_array_length()?;
    
    for i in 0..length {
        if let Some(key) = property_names.get::<String>(i)? {
            if let Some(value) = config.get::<String>(&key)? {
                CONFIG_STORE.set(&key, ConfigValue::String(value));
            } else if let Some(value) = config.get::<f64>(&key)? {
                CONFIG_STORE.set(&key, ConfigValue::Number(value));
            } else if let Some(value) = config.get::<bool>(&key)? {
                CONFIG_STORE.set(&key, ConfigValue::Boolean(value));
            }
        }
    }
    
    Ok(())
}

/// Get global configuration (standalone function)
#[napi]
pub fn get_global_config() -> Result<Object> {
    let config_map = CONFIG_STORE.get_all();
    let mut result = Object::new();
    
    for (key, value) in config_map {
        match value {
            ConfigValue::String(s) => result.set(key, s)?,
            ConfigValue::Number(n) => result.set(key, n)?,
            ConfigValue::Boolean(b) => result.set(key, b)?,
            _ => {} // Skip complex types for now
        }
    }
    
    Ok(result)
}

/// Get XProfiler version (standalone function)
#[napi]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Get build information (standalone function)
#[napi]
pub fn get_build_info() -> Result<Object> {
    let mut result = Object::new();
    
    result.set("version", env!("CARGO_PKG_VERSION"))?;
    result.set("target", env!("TARGET"))?;
    result.set("profile", if cfg!(debug_assertions) { "debug" } else { "release" })?;
    result.set("timestamp", get_timestamp_ms())?;
    
    Ok(result)
}

/// Utility functions for data conversion

fn js_value_to_config_value(value: JsUnknown) -> Result<ConfigValue> {
    if let Ok(s) = value.coerce_to_string() {
        Ok(ConfigValue::String(s.into_utf8()?.into_owned()?))
    } else if let Ok(n) = value.coerce_to_number() {
        Ok(ConfigValue::Number(n.get_double()?))
    } else if let Ok(b) = value.coerce_to_bool() {
        Ok(ConfigValue::Boolean(b.get_value()?))
    } else {
        Err(Error::new(Status::InvalidArg, "Unsupported value type"))
    }
}

fn object_to_metadata(obj: Object) -> Result<HashMap<String, String>> {
    let mut metadata = HashMap::new();
    
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
    fn test_xprofiler_creation() {
        let xprofiler = XProfiler::new();
        assert!(!xprofiler.is_initialized());
    }
    
    #[test]
    fn test_version() {
        let version = get_version();
        assert!(!version.is_empty());
    }
}

/// Initialize xprofiler with configuration
#[napi]
pub fn init_xprofiler(config: Option<Object>) -> Result<()> {
    // Initialize logger first
    LOGGER.init();
    
    // Process configuration if provided
    if let Some(config_obj) = config {
        let config_map = object_to_hashmap(config_obj)?;
        
        for (key, value) in config_map {
            let config_value = js_value_to_config_value(value)?;
            CONFIG_STORE.set(&key, config_value)
                .map_err(|e| Error::new(Status::InvalidArg, format!("Config error: {}", e)))?;
        }
    }
    
    // Initialize environment registry
    ENVIRONMENT_REGISTRY.initialize();
    
    log::info!("XProfiler initialized successfully");
    Ok(())
}

/// Get current configuration
#[napi]
pub fn get_config() -> Result<Object> {
    let config = CONFIG_STORE.get_all();
    let mut result = Object::new();
    
    for (key, value) in config {
        let js_value = config_value_to_js_value(value)?;
        result.set(&key, js_value)?;
    }
    
    Ok(result)
}

/// Set configuration value
#[napi]
pub fn set_config(key: String, value: Unknown) -> Result<()> {
    let config_value = js_value_to_config_value(value)?;
    CONFIG_STORE.set(&key, config_value)
        .map_err(|e| Error::new(Status::InvalidArg, format!("Config error: {}", e)))?;
    
    log::debug!("Configuration updated: {} = {:?}", key, config_value);
    Ok(())
}

/// Get configuration value
#[napi]
pub fn get_config_value(key: String) -> Result<Unknown> {
    match CONFIG_STORE.get(&key) {
        Some(value) => config_value_to_js_value(value),
        None => Ok(Unknown::from_undefined()),
    }
}

/// Start profiling
#[napi]
pub fn start_profiling() -> Result<()> {
    log::info!("Starting profiling");
    
    // Register current thread environment
    let env_data = EnvironmentData::new();
    ENVIRONMENT_REGISTRY.register_thread(env_data);
    
    Ok(())
}

/// Stop profiling
#[napi]
pub fn stop_profiling() -> Result<()> {
    log::info!("Stopping profiling");
    
    // Unregister current thread
    ENVIRONMENT_REGISTRY.unregister_current_thread();
    
    Ok(())
}

/// Get current environment data
#[napi]
pub fn get_environment_data() -> Result<Object> {
    let env_data = ENVIRONMENT_REGISTRY.get_current_thread_data()
        .ok_or_else(|| Error::new(Status::InvalidArg, "No environment data available"))?;
    
    environment_data_to_object(env_data)
}

/// Get all threads environment data
#[napi]
pub fn get_all_environment_data() -> Result<Array> {
    let all_data = ENVIRONMENT_REGISTRY.get_all_threads_data();
    let mut result = Array::new(all_data.len() as u32)?;
    
    for (index, env_data) in all_data.iter().enumerate() {
        let obj = environment_data_to_object(env_data.clone())?;
        result.set(index as u32, obj)?;
    }
    
    Ok(result)
}

/// Get profiling statistics
#[napi]
pub fn get_profiling_stats() -> Result<Object> {
    let stats = ENVIRONMENT_REGISTRY.get_statistics();
    let mut result = Object::new();
    
    result.set("total_threads", stats.total_threads)?;
    result.set("active_threads", stats.active_threads)?;
    result.set("total_samples", stats.total_samples)?;
    result.set("uptime_ms", stats.uptime_ms)?;
    
    Ok(result)
}

/// Clear all profiling data
#[napi]
pub fn clear_profiling_data() -> Result<()> {
    ENVIRONMENT_REGISTRY.clear_all();
    log::info!("All profiling data cleared");
    Ok(())
}

/// Set log level
#[napi]
pub fn set_log_level(level: String) -> Result<()> {
    let log_level = match level.to_lowercase().as_str() {
        "trace" => log::Level::Trace,
        "debug" => log::Level::Debug,
        "info" => log::Level::Info,
        "warn" => log::Level::Warn,
        "error" => log::Level::Error,
        _ => return Err(Error::new(Status::InvalidArg, "Invalid log level")),
    };
    
    LOGGER.set_level(log_level);
    log::info!("Log level set to: {}", level);
    Ok(())
}

/// Get current log level
#[napi]
pub fn get_log_level() -> String {
    LOGGER.get_level().to_string().to_lowercase()
}

/// Log a message
#[napi]
pub fn log_message(level: String, message: String, metadata: Option<Object>) -> Result<()> {
    let log_level = match level.to_lowercase().as_str() {
        "trace" => log::Level::Trace,
        "debug" => log::Level::Debug,
        "info" => log::Level::Info,
        "warn" => log::Level::Warn,
        "error" => log::Level::Error,
        _ => return Err(Error::new(Status::InvalidArg, "Invalid log level")),
    };
    
    if let Some(meta) = metadata {
        let meta_map = object_to_hashmap(meta)?;
        LOGGER.log_with_metadata(log_level, &message, meta_map);
    } else {
        LOGGER.log(log_level, &message);
    }
    
    Ok(())
}

/// Get version information
#[napi]
pub fn get_version_info() -> Result<Object> {
    let mut result = Object::new();
    
    result.set("version", env!("CARGO_PKG_VERSION"))?;
    result.set("name", env!("CARGO_PKG_NAME"))?;
    result.set("description", env!("CARGO_PKG_DESCRIPTION"))?;
    result.set("rust_version", env!("CARGO_PKG_RUST_VERSION").unwrap_or("unknown"))?;
    result.set("build_time", chrono::Utc::now().timestamp())?;
    
    Ok(result)
}

/// Check if profiling is active
#[napi]
pub fn is_profiling_active() -> bool {
    ENVIRONMENT_REGISTRY.is_active()
}

/// Get memory usage information
#[napi]
pub fn get_memory_usage() -> Result<Object> {
    let mut result = Object::new();
    
    // Get current process memory usage
    #[cfg(unix)]
    {
        use std::fs;
        if let Ok(status) = fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    if let Some(rss) = line.split_whitespace().nth(1) {
                        if let Ok(rss_kb) = rss.parse::<u64>() {
                            result.set("rss", rss_kb * 1024)?; // Convert to bytes
                        }
                    }
                }
                if line.starts_with("VmSize:") {
                    if let Some(vsize) = line.split_whitespace().nth(1) {
                        if let Ok(vsize_kb) = vsize.parse::<u64>() {
                            result.set("vsize", vsize_kb * 1024)?; // Convert to bytes
                        }
                    }
                }
            }
        }
    }
    
    #[cfg(windows)]
    {
        // Windows memory usage implementation would go here
        result.set("rss", 0)?;
        result.set("vsize", 0)?;
    }
    
    Ok(result)
}

/// Utility function to convert JavaScript Object to HashMap
fn object_to_hashmap(obj: Object) -> Result<HashMap<String, Unknown>> {
    let mut map = HashMap::new();
    let keys = obj.get_property_names()?;
    
    for i in 0..keys.len() {
        if let Some(key) = keys.get::<String>(i)? {
            if let Some(value) = obj.get::<Unknown>(&key)? {
                map.insert(key, value);
            }
        }
    }
    
    Ok(map)
}

/// Convert JavaScript value to ConfigValue
fn js_value_to_config_value(value: Unknown) -> Result<ConfigValue> {
    if value.is_string() {
        Ok(ConfigValue::String(value.coerce_to_string()?.into_utf8()?.as_str()?.to_string()))
    } else if value.is_number() {
        if let Ok(int_val) = value.coerce_to_number()?.get_int64() {
            Ok(ConfigValue::Integer(int_val))
        } else {
            let float_val = value.coerce_to_number()?.get_double()?;
            Ok(ConfigValue::Float(float_val))
        }
    } else if value.is_boolean() {
        Ok(ConfigValue::Boolean(value.coerce_to_bool()?.get_value()?))
    } else {
        Err(Error::new(Status::InvalidArg, "Unsupported config value type"))
    }
}

/// Convert ConfigValue to JavaScript value
fn config_value_to_js_value(value: ConfigValue) -> Result<Unknown> {
    match value {
        ConfigValue::String(s) => Ok(Unknown::from_string(s)?),
        ConfigValue::Integer(i) => Ok(Unknown::from_i64(i)?),
        ConfigValue::Float(f) => Ok(Unknown::from_f64(f)?),
        ConfigValue::Boolean(b) => Ok(Unknown::from_bool(b)?),
    }
}

/// Convert EnvironmentData to JavaScript Object
fn environment_data_to_object(env_data: EnvironmentData) -> Result<Object> {
    let mut result = Object::new();
    
    result.set("thread_id", env_data.thread_id)?;
    result.set("is_main_thread", env_data.is_main_thread)?;
    result.set("start_time", env_data.start_time)?;
    result.set("last_update", env_data.last_update)?;
    result.set("sample_count", env_data.sample_count)?;
    
    // Heap statistics
    let mut heap_stats = Object::new();
    heap_stats.set("total_heap_size", env_data.heap_stats.total_heap_size)?;
    heap_stats.set("total_heap_size_executable", env_data.heap_stats.total_heap_size_executable)?;
    heap_stats.set("total_physical_size", env_data.heap_stats.total_physical_size)?;
    heap_stats.set("total_available_size", env_data.heap_stats.total_available_size)?;
    heap_stats.set("used_heap_size", env_data.heap_stats.used_heap_size)?;
    heap_stats.set("heap_size_limit", env_data.heap_stats.heap_size_limit)?;
    heap_stats.set("malloced_memory", env_data.heap_stats.malloced_memory)?;
    heap_stats.set("peak_malloced_memory", env_data.heap_stats.peak_malloced_memory)?;
    heap_stats.set("does_zap_garbage", env_data.heap_stats.does_zap_garbage)?;
    heap_stats.set("number_of_native_contexts", env_data.heap_stats.number_of_native_contexts)?;
    heap_stats.set("number_of_detached_contexts", env_data.heap_stats.number_of_detached_contexts)?;
    result.set("heap_statistics", heap_stats)?;
    
    // GC statistics
    let mut gc_stats = Object::new();
    gc_stats.set("gc_count", env_data.gc_stats.gc_count)?;
    gc_stats.set("gc_time_ms", env_data.gc_stats.gc_time_ms)?;
    gc_stats.set("scavenge_count", env_data.gc_stats.scavenge_count)?;
    gc_stats.set("scavenge_time_ms", env_data.gc_stats.scavenge_time_ms)?;
    gc_stats.set("mark_sweep_count", env_data.gc_stats.mark_sweep_count)?;
    gc_stats.set("mark_sweep_time_ms", env_data.gc_stats.mark_sweep_time_ms)?;
    result.set("gc_statistics", gc_stats)?;
    
    // UV statistics
    let mut uv_stats = Object::new();
    uv_stats.set("handle_count", env_data.uv_stats.handle_count)?;
    uv_stats.set("request_count", env_data.uv_stats.request_count)?;
    uv_stats.set("active_handles", env_data.uv_stats.active_handles)?;
    uv_stats.set("active_requests", env_data.uv_stats.active_requests)?;
    result.set("uv_statistics", uv_stats)?;
    
    Ok(result)
}

/// Error handling utilities
#[napi]
pub fn get_last_error() -> Option<String> {
    // This would integrate with a global error tracking system
    None
}

/// Performance monitoring utilities
#[napi]
pub fn start_performance_timer(name: String) -> Result<()> {
    crate::utils::start_timer(&name);
    Ok(())
}

#[napi]
pub fn end_performance_timer(name: String) -> Result<f64> {
    match crate::utils::end_timer(&name) {
        Some(duration) => Ok(duration),
        None => Err(Error::new(Status::InvalidArg, "Timer not found")),
    }
}

/// Utility functions for debugging
#[napi]
pub fn dump_config() -> Result<String> {
    let config = CONFIG_STORE.get_all();
    serde_json::to_string_pretty(&config)
        .map_err(|e| Error::new(Status::GenericFailure, format!("Serialization error: {}", e)))
}

#[napi]
pub fn dump_environment_data() -> Result<String> {
    let all_data = ENVIRONMENT_REGISTRY.get_all_threads_data();
    serde_json::to_string_pretty(&all_data)
        .map_err(|e| Error::new(Status::GenericFailure, format!("Serialization error: {}", e)))
}

/// Health check function
#[napi]
pub fn health_check() -> Result<Object> {
    let mut result = Object::new();
    
    result.set("status", "healthy")?;
    result.set("timestamp", chrono::Utc::now().timestamp())?;
    result.set("profiling_active", is_profiling_active())?;
    result.set("config_loaded", !CONFIG_STORE.get_all().is_empty())?;
    result.set("logger_initialized", LOGGER.is_initialized())?;
    
    Ok(result)
}

/// Cleanup function
#[napi]
pub fn cleanup() -> Result<()> {
    log::info!("Cleaning up xprofiler");
    
    // Stop profiling if active
    if is_profiling_active() {
        stop_profiling()?;
    }
    
    // Clear all data
    clear_profiling_data()?;
    
    // Flush logs
    LOGGER.flush();
    
    log::info!("XProfiler cleanup completed");
    Ok(())
}