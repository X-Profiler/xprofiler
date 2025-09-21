//! Time utilities for xprofiler

use std::time::{SystemTime, UNIX_EPOCH, Instant, Duration};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
// use std::path::Path; // Commented out unused import

/// Get current timestamp in milliseconds
pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// Get current timestamp in microseconds
pub fn now_us() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros() as u64
}

/// Get current timestamp in nanoseconds
pub fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64
}

/// Get current timestamp in seconds
pub fn now_sec() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Convert milliseconds to human readable format
pub fn format_timestamp(timestamp_ms: u64) -> String {
    let datetime = UNIX_EPOCH + Duration::from_millis(timestamp_ms);
    
    // Simple formatting - in a real implementation you might want to use chrono
    format!("timestamp:{}", timestamp_ms)
}

/// Format duration in a human readable way
pub fn format_duration_detailed(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let nanos = duration.subsec_nanos();
    
    if total_secs >= 3600 {
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if total_secs >= 60 {
        let minutes = total_secs / 60;
        let seconds = total_secs % 60;
        let millis = nanos / 1_000_000;
        format!("{}m {}s {}ms", minutes, seconds, millis)
    } else if total_secs > 0 {
        let millis = nanos / 1_000_000;
        format!("{}s {}ms", total_secs, millis)
    } else if nanos >= 1_000_000 {
        let millis = nanos / 1_000_000;
        let micros = (nanos % 1_000_000) / 1_000;
        format!("{}ms {}μs", millis, micros)
    } else if nanos >= 1_000 {
        let micros = nanos / 1_000;
        let remaining_nanos = nanos % 1_000;
        format!("{}μs {}ns", micros, remaining_nanos)
    } else {
        format!("{}ns", nanos)
    }
}

/// High precision timer for performance measurements
#[derive(Debug, Clone)]
pub struct Timer {
    start: Instant,
    name: String,
}

impl Timer {
    /// Create a new timer with a name
    pub fn new(name: &str) -> Self {
        Self {
            start: Instant::now(),
            name: name.to_string(),
        }
    }
    
    /// Create and start a timer
    pub fn start(name: &str) -> Self {
        Self::new(name)
    }
    
    /// Get elapsed time since timer creation
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
    
    /// Get elapsed time in milliseconds
    pub fn elapsed_ms(&self) -> u64 {
        self.elapsed().as_millis() as u64
    }
    
    /// Get elapsed time in microseconds
    pub fn elapsed_us(&self) -> u64 {
        self.elapsed().as_micros() as u64
    }
    
    /// Get elapsed time in nanoseconds
    pub fn elapsed_ns(&self) -> u64 {
        self.elapsed().as_nanos() as u64
    }
    
    /// Stop the timer and return elapsed duration
    pub fn stop(self) -> Duration {
        self.elapsed()
    }
    
    /// Stop the timer and return elapsed time in milliseconds
    pub fn stop_ms(self) -> u64 {
        self.elapsed_ms()
    }
    
    /// Get timer name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Reset the timer
    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
}

/// Timer registry for managing multiple named timers
#[derive(Debug)]
pub struct TimerRegistry {
    timers: Arc<Mutex<HashMap<String, Timer>>>,
}

impl TimerRegistry {
    /// Create a new timer registry
    pub fn new() -> Self {
        Self {
            timers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Start a named timer
    pub fn start_timer(&self, name: &str) -> Result<(), String> {
        let mut timers = self.timers.lock().map_err(|e| {
            format!("Failed to acquire timers lock: {}", e)
        })?;
        
        if timers.contains_key(name) {
            return Err(format!("Timer '{}' already exists", name));
        }
        
        timers.insert(name.to_string(), Timer::new(name));
        Ok(())
    }
    
    /// Stop a named timer and return elapsed duration
    pub fn stop_timer(&self, name: &str) -> Result<Duration, String> {
        let mut timers = self.timers.lock().map_err(|e| {
            format!("Failed to acquire timers lock: {}", e)
        })?;
        
        if let Some(timer) = timers.remove(name) {
            Ok(timer.stop())
        } else {
            Err(format!("Timer '{}' not found", name))
        }
    }
    
    /// Get elapsed time for a named timer without stopping it
    pub fn get_elapsed(&self, name: &str) -> Result<Duration, String> {
        let timers = self.timers.lock().map_err(|e| {
            format!("Failed to acquire timers lock: {}", e)
        })?;
        
        if let Some(timer) = timers.get(name) {
            Ok(timer.elapsed())
        } else {
            Err(format!("Timer '{}' not found", name))
        }
    }
    
    /// Reset a named timer
    pub fn reset_timer(&self, name: &str) -> Result<(), String> {
        let mut timers = self.timers.lock().map_err(|e| {
            format!("Failed to acquire timers lock: {}", e)
        })?;
        
        if let Some(timer) = timers.get_mut(name) {
            timer.reset();
            Ok(())
        } else {
            Err(format!("Timer '{}' not found", name))
        }
    }
    
    /// Get all active timer names
    pub fn get_timer_names(&self) -> Result<Vec<String>, String> {
        let timers = self.timers.lock().map_err(|e| {
            format!("Failed to acquire timers lock: {}", e)
        })?;
        
        Ok(timers.keys().cloned().collect())
    }
    
    /// Clear all timers
    pub fn clear(&self) -> Result<(), String> {
        let mut timers = self.timers.lock().map_err(|e| {
            format!("Failed to acquire timers lock: {}", e)
        })?;
        
        timers.clear();
        Ok(())
    }
    
    /// Get number of active timers
    pub fn timer_count(&self) -> Result<usize, String> {
        let timers = self.timers.lock().map_err(|e| {
            format!("Failed to acquire timers lock: {}", e)
        })?;
        
        Ok(timers.len())
    }
}

impl Default for TimerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Stopwatch for measuring intervals
#[derive(Debug)]
pub struct Stopwatch {
    start_time: Option<Instant>,
    elapsed_time: Duration,
    is_running: bool,
}

impl Stopwatch {
    /// Create a new stopwatch
    pub fn new() -> Self {
        Self {
            start_time: None,
            elapsed_time: Duration::new(0, 0),
            is_running: false,
        }
    }
    
    /// Start the stopwatch
    pub fn start(&mut self) {
        if !self.is_running {
            self.start_time = Some(Instant::now());
            self.is_running = true;
        }
    }
    
    /// Stop the stopwatch
    pub fn stop(&mut self) {
        if self.is_running {
            if let Some(start) = self.start_time {
                self.elapsed_time += start.elapsed();
            }
            self.is_running = false;
            self.start_time = None;
        }
    }
    
    /// Reset the stopwatch
    pub fn reset(&mut self) {
        self.start_time = None;
        self.elapsed_time = Duration::new(0, 0);
        self.is_running = false;
    }
    
    /// Restart the stopwatch (reset and start)
    pub fn restart(&mut self) {
        self.reset();
        self.start();
    }
    
    /// Get total elapsed time
    pub fn elapsed(&self) -> Duration {
        let mut total = self.elapsed_time;
        
        if self.is_running {
            if let Some(start) = self.start_time {
                total += start.elapsed();
            }
        }
        
        total
    }
    
    /// Get elapsed time in milliseconds
    pub fn elapsed_ms(&self) -> u64 {
        self.elapsed().as_millis() as u64
    }
    
    /// Check if stopwatch is running
    pub fn is_running(&self) -> bool {
        self.is_running
    }
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::new()
    }
}

/// Time-based rate calculator
#[derive(Debug)]
pub struct RateCalculator {
    window_duration: Duration,
    events: Vec<Instant>,
}

impl RateCalculator {
    /// Create a new rate calculator with a time window
    pub fn new(window_duration: Duration) -> Self {
        Self {
            window_duration,
            events: Vec::new(),
        }
    }
    
    /// Record an event
    pub fn record_event(&mut self) {
        let now = Instant::now();
        self.events.push(now);
        self.cleanup_old_events(now);
    }
    
    /// Get current rate (events per second)
    pub fn get_rate(&mut self) -> f64 {
        let now = Instant::now();
        self.cleanup_old_events(now);
        
        if self.events.is_empty() {
            return 0.0;
        }
        
        let count = self.events.len() as f64;
        let window_secs = self.window_duration.as_secs_f64();
        
        count / window_secs
    }
    
    /// Get event count in current window
    pub fn get_count(&mut self) -> usize {
        let now = Instant::now();
        self.cleanup_old_events(now);
        self.events.len()
    }
    
    /// Clear all events
    pub fn clear(&mut self) {
        self.events.clear();
    }
    
    /// Remove events outside the time window
    fn cleanup_old_events(&mut self, now: Instant) {
        let cutoff = now - self.window_duration;
        self.events.retain(|&event_time| event_time > cutoff);
    }
}

/// Sleep utilities
pub mod sleep {
    use std::time::Duration;
    use std::thread;
    
    /// Sleep for specified milliseconds
    pub fn sleep_ms(ms: u64) {
        thread::sleep(Duration::from_millis(ms));
    }
    
    /// Sleep for specified microseconds
    pub fn sleep_us(us: u64) {
        thread::sleep(Duration::from_micros(us));
    }
    
    /// Sleep for specified nanoseconds
    pub fn sleep_ns(ns: u64) {
        thread::sleep(Duration::from_nanos(ns));
    }
    
    /// Yield current thread
    pub fn yield_now() {
        thread::yield_now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_timestamp_functions() {
        let ms1 = now_ms();
        let us1 = now_us();
        let ns1 = now_ns();
        let sec1 = now_sec();
        
        thread::sleep(Duration::from_millis(1));
        
        let ms2 = now_ms();
        let us2 = now_us();
        let ns2 = now_ns();
        let sec2 = now_sec();
        
        assert!(ms2 >= ms1);
        assert!(us2 > us1);
        assert!(ns2 > ns1);
        assert!(sec2 >= sec1);
    }

    #[test]
    fn test_timer() {
        let timer = Timer::new("test");
        assert_eq!(timer.name(), "test");
        
        thread::sleep(Duration::from_millis(10));
        
        let elapsed = timer.elapsed();
        assert!(elapsed.as_millis() >= 10);
        assert!(timer.elapsed_ms() >= 10);
        
        let duration = timer.stop();
        assert!(duration.as_millis() >= 10);
    }

    #[test]
    fn test_timer_registry() {
        let registry = TimerRegistry::new();
        
        // Start a timer
        assert!(registry.start_timer("test1").is_ok());
        assert_eq!(registry.timer_count().unwrap(), 1);
        
        // Try to start the same timer again (should fail)
        assert!(registry.start_timer("test1").is_err());
        
        thread::sleep(Duration::from_millis(10));
        
        // Check elapsed time
        let elapsed = registry.get_elapsed("test1").unwrap();
        assert!(elapsed.as_millis() >= 10);
        
        // Stop the timer
        let duration = registry.stop_timer("test1").unwrap();
        assert!(duration.as_millis() >= 10);
        assert_eq!(registry.timer_count().unwrap(), 0);
        
        // Try to stop non-existent timer
        assert!(registry.stop_timer("nonexistent").is_err());
    }

    #[test]
    fn test_stopwatch() {
        let mut stopwatch = Stopwatch::new();
        assert!(!stopwatch.is_running());
        assert_eq!(stopwatch.elapsed_ms(), 0);
        
        stopwatch.start();
        assert!(stopwatch.is_running());
        
        thread::sleep(Duration::from_millis(10));
        
        stopwatch.stop();
        assert!(!stopwatch.is_running());
        let elapsed1 = stopwatch.elapsed_ms();
        assert!(elapsed1 >= 10);
        
        // Start again
        stopwatch.start();
        thread::sleep(Duration::from_millis(10));
        stopwatch.stop();
        
        let elapsed2 = stopwatch.elapsed_ms();
        assert!(elapsed2 >= elapsed1 + 10);
        
        stopwatch.reset();
        assert_eq!(stopwatch.elapsed_ms(), 0);
    }

    #[test]
    fn test_rate_calculator() {
        let mut calc = RateCalculator::new(Duration::from_secs(1));
        
        // Record some events
        calc.record_event();
        calc.record_event();
        calc.record_event();
        
        assert_eq!(calc.get_count(), 3);
        let rate = calc.get_rate();
        assert!(rate > 0.0);
        
        calc.clear();
        assert_eq!(calc.get_count(), 0);
        assert_eq!(calc.get_rate(), 0.0);
    }

    #[test]
    fn test_format_duration_detailed() {
        let duration1 = Duration::from_millis(500);
        let formatted1 = format_duration_detailed(duration1);
        assert!(formatted1.contains("ms"));
        
        let duration2 = Duration::from_secs(65);
        let formatted2 = format_duration_detailed(duration2);
        assert!(formatted2.contains("m") && formatted2.contains("s"));
        
        let duration3 = Duration::from_secs(3665);
        let formatted3 = format_duration_detailed(duration3);
        assert!(formatted3.contains("h"));
    }

    #[test]
    fn test_sleep_functions() {
        let start = Instant::now();
        sleep::sleep_ms(10);
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() >= 10);
    }
}