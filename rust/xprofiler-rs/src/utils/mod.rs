//! Utility functions and helpers for xprofiler

pub mod time;
pub mod string;
pub mod fs;
pub mod process;
pub mod memory;
pub mod thread;

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Global counter for generating unique IDs
static GLOBAL_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Generate a unique ID
pub fn generate_id() -> u64 {
    GLOBAL_COUNTER.fetch_add(1, Ordering::SeqCst)
}

/// Generate a unique ID with timestamp prefix
pub fn generate_timestamped_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let counter = generate_id();
    format!("{}-{}", timestamp, counter)
}

/// Convert bytes to human readable format
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    const THRESHOLD: f64 = 1024.0;
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= THRESHOLD && unit_index < UNITS.len() - 1 {
        size /= THRESHOLD;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

/// Convert duration to human readable format
pub fn format_duration(duration_ms: u64) -> String {
    if duration_ms < 1000 {
        format!("{}ms", duration_ms)
    } else if duration_ms < 60_000 {
        format!("{:.2}s", duration_ms as f64 / 1000.0)
    } else if duration_ms < 3_600_000 {
        let minutes = duration_ms / 60_000;
        let seconds = (duration_ms % 60_000) as f64 / 1000.0;
        format!("{}m {:.1}s", minutes, seconds)
    } else {
        let hours = duration_ms / 3_600_000;
        let minutes = (duration_ms % 3_600_000) / 60_000;
        format!("{}h {}m", hours, minutes)
    }
}

/// Calculate percentage with precision
pub fn calculate_percentage(value: f64, total: f64, precision: usize) -> f64 {
    if total == 0.0 {
        return 0.0;
    }
    
    let percentage = (value / total) * 100.0;
    let multiplier = 10_f64.powi(precision as i32);
    (percentage * multiplier).round() / multiplier
}

/// Safe division that returns 0 if divisor is 0
pub fn safe_divide(dividend: f64, divisor: f64) -> f64 {
    if divisor == 0.0 {
        0.0
    } else {
        dividend / divisor
    }
}

/// Clamp a value between min and max
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Calculate moving average
pub struct MovingAverage {
    values: Vec<f64>,
    capacity: usize,
    sum: f64,
}

impl MovingAverage {
    /// Create a new moving average calculator
    pub fn new(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            capacity,
            sum: 0.0,
        }
    }
    
    /// Add a new value and return the current average
    pub fn add(&mut self, value: f64) -> f64 {
        if self.values.len() >= self.capacity {
            let old_value = self.values.remove(0);
            self.sum -= old_value;
        }
        
        self.values.push(value);
        self.sum += value;
        
        self.sum / self.values.len() as f64
    }
    
    /// Get the current average
    pub fn average(&self) -> f64 {
        if self.values.is_empty() {
            0.0
        } else {
            self.sum / self.values.len() as f64
        }
    }
    
    /// Get the number of values
    pub fn count(&self) -> usize {
        self.values.len()
    }
    
    /// Clear all values
    pub fn clear(&mut self) {
        self.values.clear();
        self.sum = 0.0;
    }
}

/// Rate limiter for controlling operation frequency
pub struct RateLimiter {
    last_execution: Arc<std::sync::Mutex<SystemTime>>,
    interval_ms: u64,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(interval_ms: u64) -> Self {
        Self {
            last_execution: Arc::new(std::sync::Mutex::new(UNIX_EPOCH)),
            interval_ms,
        }
    }
    
    /// Check if operation is allowed
    pub fn is_allowed(&self) -> bool {
        let now = SystemTime::now();
        let mut last = self.last_execution.lock().unwrap();
        
        if let Ok(elapsed) = now.duration_since(*last) {
            if elapsed.as_millis() >= self.interval_ms as u128 {
                *last = now;
                return true;
            }
        }
        
        false
    }
    
    /// Execute a function if rate limit allows
    pub fn execute<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce() -> R,
    {
        if self.is_allowed() {
            Some(f())
        } else {
            None
        }
    }
}

/// Simple cache with TTL support
pub struct Cache<K, V> {
    data: Arc<std::sync::Mutex<HashMap<K, (V, SystemTime)>>>,
    ttl_ms: u64,
}

impl<K, V> Cache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    /// Create a new cache with TTL
    pub fn new(ttl_ms: u64) -> Self {
        Self {
            data: Arc::new(std::sync::Mutex::new(HashMap::new())),
            ttl_ms,
        }
    }
    
    /// Get a value from cache
    pub fn get(&self, key: &K) -> Option<V> {
        let mut data = self.data.lock().unwrap();
        
        if let Some((value, timestamp)) = data.get(key) {
            let now = SystemTime::now();
            if let Ok(elapsed) = now.duration_since(*timestamp) {
                if elapsed.as_millis() < self.ttl_ms as u128 {
                    return Some(value.clone());
                }
            }
            // Entry expired, remove it
            data.remove(key);
        }
        
        None
    }
    
    /// Set a value in cache
    pub fn set(&self, key: K, value: V) {
        let mut data = self.data.lock().unwrap();
        data.insert(key, (value, SystemTime::now()));
    }
    
    /// Remove a value from cache
    pub fn remove(&self, key: &K) -> Option<V> {
        let mut data = self.data.lock().unwrap();
        data.remove(key).map(|(value, _)| value)
    }
    
    /// Clear all entries
    pub fn clear(&self) {
        let mut data = self.data.lock().unwrap();
        data.clear();
    }
    
    /// Get cache size
    pub fn size(&self) -> usize {
        let data = self.data.lock().unwrap();
        data.len()
    }
    
    /// Clean expired entries
    pub fn cleanup(&self) {
        let mut data = self.data.lock().unwrap();
        let now = SystemTime::now();
        
        data.retain(|_, (_, timestamp)| {
            if let Ok(elapsed) = now.duration_since(*timestamp) {
                elapsed.as_millis() < self.ttl_ms as u128
            } else {
                false
            }
        });
    }
}

/// Error handling utilities
pub mod error {
    use std::fmt;
    
    /// Create a boxed error from a string
    pub fn boxed_error(msg: &str) -> Box<dyn std::error::Error> {
        msg.to_string().into()
    }
    
    /// Create a boxed error with context
    pub fn boxed_error_with_context(msg: &str, context: &str) -> Box<dyn std::error::Error> {
        format!("{}: {}", context, msg).into()
    }
    
    /// Convert any error to a string
    pub fn error_to_string(error: &dyn std::error::Error) -> String {
        let mut result = error.to_string();
        let mut source = error.source();
        
        while let Some(err) = source {
            result.push_str(&format!(": {}", err));
            source = err.source();
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_generate_id() {
        let id1 = generate_id();
        let id2 = generate_id();
        assert!(id2 > id1);
    }

    #[test]
    fn test_generate_timestamped_id() {
        let id1 = generate_timestamped_id();
        thread::sleep(Duration::from_millis(1));
        let id2 = generate_timestamped_id();
        assert_ne!(id1, id2);
        assert!(id1.contains('-'));
        assert!(id2.contains('-'));
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1536), "1.50 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(500), "500ms");
        assert_eq!(format_duration(1500), "1.50s");
        assert_eq!(format_duration(65000), "1m 5.0s");
        assert_eq!(format_duration(3665000), "1h 1m");
    }

    #[test]
    fn test_calculate_percentage() {
        assert_eq!(calculate_percentage(25.0, 100.0, 2), 25.0);
        assert_eq!(calculate_percentage(1.0, 3.0, 2), 33.33);
        assert_eq!(calculate_percentage(10.0, 0.0, 2), 0.0);
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10.0, 2.0), 5.0);
        assert_eq!(safe_divide(10.0, 0.0), 0.0);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-1, 0, 10), 0);
        assert_eq!(clamp(15, 0, 10), 10);
    }

    #[test]
    fn test_moving_average() {
        let mut avg = MovingAverage::new(3);
        
        assert_eq!(avg.add(10.0), 10.0);
        assert_eq!(avg.add(20.0), 15.0);
        assert_eq!(avg.add(30.0), 20.0);
        assert_eq!(avg.add(40.0), 30.0); // Should drop 10.0
        
        assert_eq!(avg.count(), 3);
        avg.clear();
        assert_eq!(avg.count(), 0);
    }

    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new(100); // 100ms interval
        
        assert!(limiter.is_allowed()); // First call should be allowed
        assert!(!limiter.is_allowed()); // Second call should be blocked
        
        thread::sleep(Duration::from_millis(101));
        assert!(limiter.is_allowed()); // Should be allowed after interval
    }

    #[test]
    fn test_cache() {
        let cache = Cache::new(100); // 100ms TTL
        
        cache.set("key1", "value1");
        assert_eq!(cache.get(&"key1"), Some("value1"));
        assert_eq!(cache.size(), 1);
        
        thread::sleep(Duration::from_millis(101));
        assert_eq!(cache.get(&"key1"), None); // Should be expired
        
        cache.set("key2", "value2");
        cache.set("key3", "value3");
        assert_eq!(cache.size(), 2);
        
        cache.clear();
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn test_error_utilities() {
        let err = error::boxed_error("test error");
        assert_eq!(err.to_string(), "test error");
        
        let err_with_context = error::boxed_error_with_context("test error", "context");
        assert_eq!(err_with_context.to_string(), "context: test error");
    }
}