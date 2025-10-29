//! Monitoring bindings for JavaScript interface

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::monitoring::{GLOBAL_MONITOR, MonitoringConfig, MetricType, MetricValue};
use crate::utils::time::get_timestamp_ms;

/// Monitoring manager class for JavaScript
#[napi]
pub struct MonitoringManager {
    initialized: bool,
}

#[napi]
impl MonitoringManager {
    /// Create a new monitoring manager
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            initialized: false,
        }
    }
    
    /// Initialize the monitoring system with configuration
    #[napi]
    pub fn initialize(&mut self, config: Option<Object>) -> Result<()> {
        let monitoring_config = if let Some(config_obj) = config {
            object_to_monitoring_config(config_obj)?
        } else {
            MonitoringConfig::default()
        };
        
        GLOBAL_MONITOR.initialize(monitoring_config);
        self.initialized = true;
        Ok(())
    }
    
    /// Check if monitoring is initialized
    #[napi]
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    /// Start monitoring
    #[napi]
    pub fn start(&self) -> Result<()> {
        GLOBAL_MONITOR.start();
        Ok(())
    }
    
    /// Stop monitoring
    #[napi]
    pub fn stop(&self) -> Result<()> {
        GLOBAL_MONITOR.stop();
        Ok(())
    }
    
    /// Check if monitoring is active
    #[napi]
    pub fn is_active(&self) -> bool {
        GLOBAL_MONITOR.is_active()
    }
    
    /// Record a counter metric
    #[napi]
    pub fn record_counter(&self, name: String, value: f64, tags: Option<Object>) -> Result<()> {
        let metric_tags = if let Some(tags_obj) = tags {
            object_to_tags(tags_obj)?
        } else {
            HashMap::new()
        };
        
        GLOBAL_MONITOR.record_metric(&name, MetricValue::Counter(value), metric_tags);
        Ok(())
    }
    
    /// Record a gauge metric
    #[napi]
    pub fn record_gauge(&self, name: String, value: f64, tags: Option<Object>) -> Result<()> {
        let metric_tags = if let Some(tags_obj) = tags {
            object_to_tags(tags_obj)?
        } else {
            HashMap::new()
        };
        
        GLOBAL_MONITOR.record_metric(&name, MetricValue::Gauge(value), metric_tags);
        Ok(())
    }
    
    /// Record a histogram metric
    #[napi]
    pub fn record_histogram(&self, name: String, value: f64, tags: Option<Object>) -> Result<()> {
        let metric_tags = if let Some(tags_obj) = tags {
            object_to_tags(tags_obj)?
        } else {
            HashMap::new()
        };
        
        GLOBAL_MONITOR.record_metric(&name, MetricValue::Histogram(value), metric_tags);
        Ok(())
    }
    
    /// Record a timer metric (duration in milliseconds)
    #[napi]
    pub fn record_timer(&self, name: String, duration_ms: f64, tags: Option<Object>) -> Result<()> {
        let metric_tags = if let Some(tags_obj) = tags {
            object_to_tags(tags_obj)?
        } else {
            HashMap::new()
        };
        
        GLOBAL_MONITOR.record_metric(&name, MetricValue::Timer(Duration::from_millis(duration_ms as u64)), metric_tags);
        Ok(())
    }
    
    /// Start a timer and return a timer ID
    #[napi]
    pub fn start_timer(&self, name: String, tags: Option<Object>) -> Result<String> {
        let metric_tags = if let Some(tags_obj) = tags {
            object_to_tags(tags_obj)?
        } else {
            HashMap::new()
        };
        
        let timer_id = GLOBAL_MONITOR.start_timer(&name, metric_tags);
        Ok(timer_id)
    }
    
    /// End a timer by ID
    #[napi]
    pub fn end_timer(&self, timer_id: String) -> Result<f64> {
        match GLOBAL_MONITOR.end_timer(&timer_id) {
            Some(duration) => Ok(duration.as_millis() as f64),
            None => Err(Error::new(Status::InvalidArg, "Timer not found")),
        }
    }
    
    /// Increment a counter by 1
    #[napi]
    pub fn increment(&self, name: String, tags: Option<Object>) -> Result<()> {
        self.record_counter(name, 1.0, tags)
    }
    
    /// Increment a counter by a specific value
    #[napi]
    pub fn increment_by(&self, name: String, value: f64, tags: Option<Object>) -> Result<()> {
        self.record_counter(name, value, tags)
    }
    
    /// Decrement a counter by 1
    #[napi]
    pub fn decrement(&self, name: String, tags: Option<Object>) -> Result<()> {
        self.record_counter(name, -1.0, tags)
    }
    
    /// Decrement a counter by a specific value
    #[napi]
    pub fn decrement_by(&self, name: String, value: f64, tags: Option<Object>) -> Result<()> {
        self.record_counter(name, -value, tags)
    }
    
    /// Set a gauge value
    #[napi]
    pub fn set_gauge(&self, name: String, value: f64, tags: Option<Object>) -> Result<()> {
        self.record_gauge(name, value, tags)
    }
    
    /// Get current metrics snapshot
    #[napi]
    pub fn get_metrics(&self) -> Result<Object> {
        let metrics = GLOBAL_MONITOR.get_metrics();
        metrics_to_object(metrics)
    }
    
    /// Get metrics for a specific name
    #[napi]
    pub fn get_metric(&self, name: String) -> Result<Array> {
        let metrics = GLOBAL_MONITOR.get_metric(&name);
        let mut result = Array::new(metrics.len() as u32)?;
        
        for (index, metric) in metrics.iter().enumerate() {
            let obj = metric_to_object(metric.clone())?;
            result.set(index as u32, obj)?;
        }
        
        Ok(result)
    }
    
    /// Clear all metrics
    #[napi]
    pub fn clear_metrics(&self) -> Result<()> {
        GLOBAL_MONITOR.clear_metrics();
        Ok(())
    }
    
    /// Clear metrics for a specific name
    #[napi]
    pub fn clear_metric(&self, name: String) -> Result<()> {
        GLOBAL_MONITOR.clear_metric(&name);
        Ok(())
    }
    
    /// Get monitoring statistics
    #[napi]
    pub fn get_statistics(&self) -> Result<Object> {
        let stats = GLOBAL_MONITOR.get_statistics();
        let mut result = Object::new();
        
        result.set("total_metrics", stats.total_metrics)?;
        result.set("active_timers", stats.active_timers)?;
        result.set("start_time", stats.start_time)?;
        result.set("uptime_ms", stats.uptime_ms)?;
        result.set("last_metric_time", stats.last_metric_time)?;
        
        // Metrics by type
        let mut metrics_by_type = Object::new();
        for (metric_type, count) in stats.metrics_by_type {
            let type_name = match metric_type {
                MetricType::Counter => "counter",
                MetricType::Gauge => "gauge",
                MetricType::Histogram => "histogram",
                MetricType::Timer => "timer",
            };
            metrics_by_type.set(type_name, count)?;
        }
        result.set("metrics_by_type", metrics_by_type)?;
        
        Ok(result)
    }
    
    /// Reset monitoring statistics
    #[napi]
    pub fn reset_statistics(&self) -> Result<()> {
        GLOBAL_MONITOR.reset_statistics();
        Ok(())
    }
    
    /// Set monitoring configuration
    #[napi]
    pub fn set_config(&self, config: Object) -> Result<()> {
        let monitoring_config = object_to_monitoring_config(config)?;
        GLOBAL_MONITOR.set_config(monitoring_config);
        Ok(())
    }
    
    /// Get current monitoring configuration
    #[napi]
    pub fn get_config(&self) -> Result<Object> {
        let config = GLOBAL_MONITOR.get_config();
        monitoring_config_to_object(config)
    }
    
    /// Enable or disable monitoring
    #[napi]
    pub fn set_enabled(&self, enabled: bool) -> Result<()> {
        GLOBAL_MONITOR.set_enabled(enabled);
        Ok(())
    }
    
    /// Check if monitoring is enabled
    #[napi]
    pub fn is_enabled(&self) -> bool {
        GLOBAL_MONITOR.is_enabled()
    }
    
    /// Set sample rate (0.0 to 1.0)
    #[napi]
    pub fn set_sample_rate(&self, rate: f64) -> Result<()> {
        if rate < 0.0 || rate > 1.0 {
            return Err(Error::new(Status::InvalidArg, "Sample rate must be between 0.0 and 1.0"));
        }
        GLOBAL_MONITOR.set_sample_rate(rate);
        Ok(())
    }
    
    /// Get current sample rate
    #[napi]
    pub fn get_sample_rate(&self) -> f64 {
        GLOBAL_MONITOR.get_sample_rate()
    }
    
    /// Add a global tag to all metrics
    #[napi]
    pub fn add_global_tag(&self, key: String, value: String) -> Result<()> {
        GLOBAL_MONITOR.add_global_tag(key, value);
        Ok(())
    }
    
    /// Remove a global tag
    #[napi]
    pub fn remove_global_tag(&self, key: String) -> Result<()> {
        GLOBAL_MONITOR.remove_global_tag(&key);
        Ok(())
    }
    
    /// Clear all global tags
    #[napi]
    pub fn clear_global_tags(&self) -> Result<()> {
        GLOBAL_MONITOR.clear_global_tags();
        Ok(())
    }
    
    /// Get all global tags
    #[napi]
    pub fn get_global_tags(&self) -> Result<Object> {
        let tags = GLOBAL_MONITOR.get_global_tags();
        tags_to_object(tags)
    }
    
    /// Export metrics in a specific format
    #[napi]
    pub fn export_metrics(&self, format: String) -> Result<String> {
        match format.to_lowercase().as_str() {
            "json" => Ok(GLOBAL_MONITOR.export_json()),
            "prometheus" => Ok(GLOBAL_MONITOR.export_prometheus()),
            "csv" => Ok(GLOBAL_MONITOR.export_csv()),
            _ => Err(Error::new(Status::InvalidArg, "Unsupported export format")),
        }
    }
    
    /// Create a metric snapshot
    #[napi]
    pub fn create_snapshot(&self) -> Result<Object> {
        let snapshot = GLOBAL_MONITOR.create_snapshot();
        let mut result = Object::new();
        
        result.set("timestamp", get_timestamp_ms())?;
        result.set("metrics", metrics_to_object(snapshot.metrics)?)?;
        result.set("statistics", {
            let mut stats = Object::new();
            stats.set("total_metrics", snapshot.statistics.total_metrics)?;
            stats.set("active_timers", snapshot.statistics.active_timers)?;
            stats.set("uptime_ms", snapshot.statistics.uptime_ms)?;
            stats
        })?;
        
        Ok(result)
    }
    
    /// Restore from a metric snapshot
    #[napi]
    pub fn restore_snapshot(&self, snapshot: Object) -> Result<()> {
        // This would require implementing snapshot restoration in the monitoring module
        // For now, we'll just return an error indicating it's not implemented
        Err(Error::new(Status::GenericFailure, "Snapshot restoration not yet implemented"))
    }
}

/// Standalone monitoring functions

/// Initialize monitoring (standalone function)
#[napi]
pub fn init_monitoring(config: Option<Object>) -> Result<()> {
    let monitoring_config = if let Some(config_obj) = config {
        object_to_monitoring_config(config_obj)?
    } else {
        MonitoringConfig::default()
    };
    
    GLOBAL_MONITOR.initialize(monitoring_config);
    Ok(())
}

/// Record counter metric (standalone function)
#[napi]
pub fn record_counter(name: String, value: f64, tags: Option<Object>) -> Result<()> {
    let metric_tags = if let Some(tags_obj) = tags {
        object_to_tags(tags_obj)?
    } else {
        HashMap::new()
    };
    
    GLOBAL_MONITOR.record_metric(&name, MetricValue::Counter(value), metric_tags);
    Ok(())
}

/// Record gauge metric (standalone function)
#[napi]
pub fn record_gauge(name: String, value: f64, tags: Option<Object>) -> Result<()> {
    let metric_tags = if let Some(tags_obj) = tags {
        object_to_tags(tags_obj)?
    } else {
        HashMap::new()
    };
    
    GLOBAL_MONITOR.record_metric(&name, MetricValue::Gauge(value), metric_tags);
    Ok(())
}

/// Record histogram metric (standalone function)
#[napi]
pub fn record_histogram(name: String, value: f64, tags: Option<Object>) -> Result<()> {
    let metric_tags = if let Some(tags_obj) = tags {
        object_to_tags(tags_obj)?
    } else {
        HashMap::new()
    };
    
    GLOBAL_MONITOR.record_metric(&name, MetricValue::Histogram(value), metric_tags);
    Ok(())
}

/// Start timer (standalone function)
#[napi]
pub fn start_timer(name: String, tags: Option<Object>) -> Result<String> {
    let metric_tags = if let Some(tags_obj) = tags {
        object_to_tags(tags_obj)?
    } else {
        HashMap::new()
    };
    
    let timer_id = GLOBAL_MONITOR.start_timer(&name, metric_tags);
    Ok(timer_id)
}

/// End timer (standalone function)
#[napi]
pub fn end_timer(timer_id: String) -> Result<f64> {
    match GLOBAL_MONITOR.end_timer(&timer_id) {
        Some(duration) => Ok(duration.as_millis() as f64),
        None => Err(Error::new(Status::InvalidArg, "Timer not found")),
    }
}

/// Get metrics (standalone function)
#[napi]
pub fn get_metrics() -> Result<Object> {
    let metrics = GLOBAL_MONITOR.get_metrics();
    metrics_to_object(metrics)
}

/// Increment counter (standalone function)
#[napi]
pub fn increment(name: String, tags: Option<Object>) -> Result<()> {
    record_counter(name, 1.0, tags)
}

/// Set gauge (standalone function)
#[napi]
pub fn set_gauge(name: String, value: f64, tags: Option<Object>) -> Result<()> {
    record_gauge(name, value, tags)
}

/// Utility functions for data conversion

fn object_to_monitoring_config(obj: Object) -> Result<MonitoringConfig> {
    let mut config = MonitoringConfig::default();
    
    if let Some(enabled) = obj.get::<bool>("enabled")? {
        config.enabled = enabled;
    }
    
    if let Some(sample_rate) = obj.get::<f64>("sample_rate")? {
        config.sample_rate = sample_rate;
    }
    
    if let Some(buffer_size) = obj.get::<u32>("buffer_size")? {
        config.buffer_size = buffer_size as usize;
    }
    
    if let Some(flush_interval) = obj.get::<u32>("flush_interval_ms")? {
        config.flush_interval = Duration::from_millis(flush_interval as u64);
    }
    
    if let Some(max_metrics) = obj.get::<u32>("max_metrics")? {
        config.max_metrics = Some(max_metrics as usize);
    }
    
    Ok(config)
}

fn monitoring_config_to_object(config: MonitoringConfig) -> Result<Object> {
    let mut result = Object::new();
    
    result.set("enabled", config.enabled)?;
    result.set("sample_rate", config.sample_rate)?;
    result.set("buffer_size", config.buffer_size as u32)?;
    result.set("flush_interval_ms", config.flush_interval.as_millis() as u32)?;
    
    if let Some(max_metrics) = config.max_metrics {
        result.set("max_metrics", max_metrics as u32)?;
    }
    
    Ok(result)
}

fn object_to_tags(obj: Object) -> Result<HashMap<String, String>> {
    let mut tags = HashMap::new();
    
    let property_names = obj.get_property_names()?;
    let length = property_names.get_array_length()?;
    
    for i in 0..length {
        if let Some(key) = property_names.get::<String>(i)? {
            if let Some(value) = obj.get::<String>(&key)? {
                tags.insert(key, value);
            } else if let Some(value) = obj.get::<f64>(&key)? {
                tags.insert(key, value.to_string());
            } else if let Some(value) = obj.get::<bool>(&key)? {
                tags.insert(key, value.to_string());
            }
        }
    }
    
    Ok(tags)
}

fn tags_to_object(tags: HashMap<String, String>) -> Result<Object> {
    let mut result = Object::new();
    
    for (key, value) in tags {
        result.set(key, value)?;
    }
    
    Ok(result)
}

fn metrics_to_object(metrics: HashMap<String, Vec<crate::monitoring::Metric>>) -> Result<Object> {
    let mut result = Object::new();
    
    for (name, metric_list) in metrics {
        let mut metric_array = Array::new(metric_list.len() as u32)?;
        
        for (index, metric) in metric_list.iter().enumerate() {
            let metric_obj = metric_to_object(metric.clone())?;
            metric_array.set(index as u32, metric_obj)?;
        }
        
        result.set(name, metric_array)?;
    }
    
    Ok(result)
}

fn metric_to_object(metric: crate::monitoring::Metric) -> Result<Object> {
    let mut result = Object::new();
    
    result.set("name", metric.name)?;
    result.set("timestamp", metric.timestamp)?;
    
    // Convert metric value based on type
    match metric.value {
        MetricValue::Counter(value) => {
            result.set("type", "counter")?;
            result.set("value", value)?;
        }
        MetricValue::Gauge(value) => {
            result.set("type", "gauge")?;
            result.set("value", value)?;
        }
        MetricValue::Histogram(value) => {
            result.set("type", "histogram")?;
            result.set("value", value)?;
        }
        MetricValue::Timer(duration) => {
            result.set("type", "timer")?;
            result.set("value", duration.as_millis() as f64)?;
        }
    }
    
    // Convert tags
    let tags_obj = tags_to_object(metric.tags)?;
    result.set("tags", tags_obj)?;
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_monitoring_manager_creation() {
        let manager = MonitoringManager::new();
        assert!(!manager.is_initialized());
    }
    
    #[test]
    fn test_tags_conversion() {
        let mut tags = HashMap::new();
        tags.insert("key1".to_string(), "value1".to_string());
        tags.insert("key2".to_string(), "value2".to_string());
        
        let obj = tags_to_object(tags).unwrap();
        // In a real test, we would verify the object contents
        // This is just a compilation test
    }
    
    #[test]
    fn test_monitoring_config_conversion() {
        let config = MonitoringConfig {
            enabled: true,
            sample_rate: 0.5,
            buffer_size: 1000,
            flush_interval: Duration::from_millis(5000),
            max_metrics: Some(10000),
        };
        
        let obj = monitoring_config_to_object(config).unwrap();
        // In a real test, we would verify the object contents
        // This is just a compilation test
    }
}