//! HTTP monitoring module
//!
//! This module provides HTTP request/response monitoring functionality,
//! including request counts, response times, status codes, and error rates.
//! 
//! The implementation tracks HTTP traffic patterns and provides statistics
//! for performance analysis and debugging.

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
// Removed unused serde imports
use regex::Regex;

use super::{Monitor, MonitoringResult, MonitoringError, TimePeriod};
// Removed unused IntoMonitoringError import

/// HTTP request information
#[derive(Debug, Clone)]
pub struct HttpRequest {
    /// HTTP method (GET, POST, etc.)
    pub method: String,
    /// Request URL or path
    pub url: String,
    /// Request headers size in bytes
    pub headers_size: u64,
    /// Request body size in bytes
    pub body_size: u64,
    /// Request timestamp
    pub timestamp: Instant,
    /// User agent string
    pub user_agent: Option<String>,
    /// Remote IP address
    pub remote_ip: Option<String>,
}

/// HTTP response information
#[derive(Debug, Clone)]
pub struct HttpResponse {
    /// HTTP status code
    pub status_code: u16,
    /// Response headers size in bytes
    pub headers_size: u64,
    /// Response body size in bytes
    pub body_size: u64,
    /// Response timestamp
    pub timestamp: Instant,
    /// Response time in milliseconds
    pub response_time: Duration,
}

/// HTTP statistics for a specific time period
#[derive(Debug, Clone)]
pub struct HttpStats {
    /// Total number of requests
    pub total_requests: u64,
    /// Total number of responses
    pub total_responses: u64,
    /// Average response time in milliseconds
    pub avg_response_time: f64,
    /// Minimum response time in milliseconds
    pub min_response_time: f64,
    /// Maximum response time in milliseconds
    pub max_response_time: f64,
    /// 95th percentile response time in milliseconds
    pub p95_response_time: f64,
    /// 99th percentile response time in milliseconds
    pub p99_response_time: f64,
    /// Requests per second
    pub requests_per_second: f64,
    /// Error rate (percentage)
    pub error_rate: f64,
    /// Status code distribution
    pub status_codes: HashMap<u16, u64>,
    /// HTTP method distribution
    pub methods: HashMap<String, u64>,
    /// Total bytes sent (request bodies)
    pub bytes_sent: u64,
    /// Total bytes received (response bodies)
    pub bytes_received: u64,
    /// URL pattern distribution
    pub url_patterns: HashMap<String, u64>,
    /// Slow requests count (> 1 second)
    pub slow_requests: u64,
    /// Very slow requests count (> 5 seconds)
    pub very_slow_requests: u64,
    /// Top slow requests
    pub top_slow_requests: Vec<SlowRequest>,
    /// Time period for these statistics
    pub period: TimePeriod,
    /// Timestamp when stats were calculated
    pub timestamp: Instant,
}

/// Slow request information
#[derive(Debug, Clone)]
pub struct SlowRequest {
    /// Request method
    pub method: String,
    /// Request URL
    pub url: String,
    /// Response time in milliseconds
    pub response_time: f64,
    /// Status code
    pub status_code: u16,
    /// Timestamp
    pub timestamp: Instant,
}

/// URL pattern matcher for grouping similar URLs
#[derive(Debug)]
pub struct UrlPatternMatcher {
    patterns: Vec<(Regex, String)>,
}

impl UrlPatternMatcher {
    pub fn new() -> Self {
        let mut patterns = Vec::new();
        
        // Common patterns for REST APIs
        patterns.push((Regex::new(r"/api/v\d+/users/\d+").unwrap(), "/api/v*/users/*".to_string()));
        patterns.push((Regex::new(r"/api/v\d+/posts/\d+").unwrap(), "/api/v*/posts/*".to_string()));
        patterns.push((Regex::new(r"/api/v\d+/orders/\d+").unwrap(), "/api/v*/orders/*".to_string()));
        patterns.push((Regex::new(r"/users/\d+").unwrap(), "/users/*".to_string()));
        patterns.push((Regex::new(r"/posts/\d+").unwrap(), "/posts/*".to_string()));
        patterns.push((Regex::new(r"/orders/\d+").unwrap(), "/orders/*".to_string()));
        patterns.push((Regex::new(r"/files/[a-f0-9-]+").unwrap(), "/files/*".to_string()));
        
        Self { patterns }
    }
    
    pub fn match_pattern(&self, url: &str) -> String {
        for (regex, pattern) in &self.patterns {
            if regex.is_match(url) {
                return pattern.clone();
            }
        }
        url.to_string()
    }
}

/// HTTP transaction (request + response pair)
#[derive(Debug, Clone)]
struct HttpTransaction {
    pub request: HttpRequest,
    pub response: Option<HttpResponse>,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
}

/// HTTP monitor implementation
#[derive(Debug)]
pub struct HttpMonitor {
    /// Active HTTP transactions (pending responses)
    active_transactions: Arc<Mutex<HashMap<String, HttpTransaction>>>,
    /// Completed transactions for different time periods
    completed_transactions: Arc<Mutex<VecDeque<HttpTransaction>>>,
    /// Statistics cache for different periods
    stats_cache: Arc<Mutex<HashMap<TimePeriod, HttpStats>>>,
    /// URL pattern matcher for grouping similar URLs
    url_matcher: UrlPatternMatcher,
    /// Slow requests tracker (top 100 slowest)
    slow_requests: Arc<Mutex<VecDeque<SlowRequest>>>,
    /// Whether monitoring is active
    is_monitoring: bool,
    /// Maximum number of completed transactions to keep
    max_completed_transactions: usize,
    /// Maximum number of slow requests to track
    max_slow_requests: usize,
    /// Slow request threshold in milliseconds
    slow_request_threshold: u64,
    /// Very slow request threshold in milliseconds
    very_slow_request_threshold: u64,
    /// Last cleanup time
    last_cleanup: Instant,
}

impl HttpMonitor {
    /// Create a new HTTP monitor
    pub fn new() -> Self {
        Self {
            active_transactions: Arc::new(Mutex::new(HashMap::new())),
            completed_transactions: Arc::new(Mutex::new(VecDeque::new())),
            stats_cache: Arc::new(Mutex::new(HashMap::new())),
            url_matcher: UrlPatternMatcher::new(),
            slow_requests: Arc::new(Mutex::new(VecDeque::new())),
            is_monitoring: false,
            max_completed_transactions: 10000, // Keep last 10k transactions
            max_slow_requests: 100, // Keep top 100 slow requests
            slow_request_threshold: 1000, // 1 second
            very_slow_request_threshold: 5000, // 5 seconds
            last_cleanup: Instant::now(),
        }
    }
    
    /// Record an HTTP request
    pub fn record_request(&self, request_id: String, request: HttpRequest) -> MonitoringResult<()> {
        if !self.is_monitoring {
            return Ok(());
        }
        
        let transaction = HttpTransaction {
            start_time: request.timestamp,
            request: request.clone(),
            response: None,
            end_time: None,
        };
        
        if let Ok(mut active) = self.active_transactions.lock() {
            active.insert(request_id, transaction);
        }
        
        Ok(())
    }
    
    /// Record an HTTP response
    pub fn record_response(&self, request_id: String, response: HttpResponse) -> MonitoringResult<()> {
        if !self.is_monitoring {
            return Ok(());
        }
        
        let mut transaction_opt = None;
        
        // Remove from active transactions
        if let Ok(mut active) = self.active_transactions.lock() {
            if let Some(mut transaction) = active.remove(&request_id) {
                transaction.response = Some(response.clone());
                transaction.end_time = Some(response.timestamp);
                transaction_opt = Some(transaction);
            }
        }
        
        // Add to completed transactions and track slow requests
        if let Some(transaction) = transaction_opt {
            // Track slow requests
            let response_time_ms = response.response_time.as_millis() as u64;
            if response_time_ms >= self.slow_request_threshold {
                let slow_request = SlowRequest {
                    method: transaction.request.method.clone(),
                    url: transaction.request.url.clone(),
                    response_time: response_time_ms as f64,
                    status_code: response.status_code,
                    timestamp: response.timestamp,
                };
                
                if let Ok(mut slow_reqs) = self.slow_requests.lock() {
                    slow_reqs.push_back(slow_request);
                    
                    // Keep only the slowest requests, sorted by response time
                    if slow_reqs.len() > self.max_slow_requests {
                        // Convert to vector, sort, and keep top N
                        let mut sorted_reqs: Vec<_> = slow_reqs.drain(..).collect();
                        sorted_reqs.sort_by(|a, b| b.response_time.partial_cmp(&a.response_time).unwrap_or(std::cmp::Ordering::Equal));
                        sorted_reqs.truncate(self.max_slow_requests);
                        slow_reqs.extend(sorted_reqs);
                    }
                }
            }
            
            if let Ok(mut completed) = self.completed_transactions.lock() {
                completed.push_back(transaction);
                
                // Cleanup old transactions if needed
                while completed.len() > self.max_completed_transactions {
                    completed.pop_front();
                }
            }
            
            // Invalidate stats cache
            if let Ok(mut cache) = self.stats_cache.lock() {
                cache.clear();
            }
        }
        
        Ok(())
    }
    
    /// Get HTTP statistics for a specific time period
    pub fn get_stats_for_period(&self, period: TimePeriod) -> MonitoringResult<HttpStats> {
        // Check cache first
        if let Ok(cache) = self.stats_cache.lock() {
            if let Some(stats) = cache.get(&period) {
                // Return cached stats if they're recent (within 1 second)
                if stats.timestamp.elapsed() < Duration::from_secs(1) {
                    return Ok(stats.clone());
                }
            }
        }
        
        // Calculate new stats
        let stats = self.calculate_stats_for_period(period)?;
        
        // Cache the stats
        if let Ok(mut cache) = self.stats_cache.lock() {
            cache.insert(period, stats.clone());
        }
        
        Ok(stats)
    }
    
    /// Calculate statistics for a specific time period
    fn calculate_stats_for_period(&self, period: TimePeriod) -> MonitoringResult<HttpStats> {
        let now = Instant::now();
        let period_duration = period.duration();
        let cutoff_time = now - period_duration;
        
        let completed = self.completed_transactions.lock()
            .map_err(|_| MonitoringError::LockFailed {
                resource: "completed_transactions".to_string(),
                details: "Failed to lock completed transactions".to_string(),
            })?;
        
        // Filter transactions within the time period
        let relevant_transactions: Vec<_> = completed
            .iter()
            .filter(|t| t.start_time >= cutoff_time)
            .collect();
        
        let total_requests = relevant_transactions.len() as u64;
        let mut total_responses = 0u64;
        let mut response_times = Vec::new();
        let mut status_codes = HashMap::new();
        let mut methods = HashMap::new();
        let mut bytes_sent = 0u64;
        let mut bytes_received = 0u64;
        let mut error_count = 0u64;
        let mut url_patterns = HashMap::new();
        let mut slow_requests = 0u64;
        let mut very_slow_requests = 0u64;
        
        for transaction in &relevant_transactions {
            // Count request method
            *methods.entry(transaction.request.method.clone()).or_insert(0) += 1;
            bytes_sent += transaction.request.body_size;
            
            // Count URL patterns
            let pattern = self.url_matcher.match_pattern(&transaction.request.url);
            *url_patterns.entry(pattern).or_insert(0) += 1;
            
            if let Some(ref response) = transaction.response {
                total_responses += 1;
                
                // Count status code
                *status_codes.entry(response.status_code).or_insert(0) += 1;
                bytes_received += response.body_size;
                
                // Count errors (4xx and 5xx status codes)
                if response.status_code >= 400 {
                    error_count += 1;
                }
                
                // Record response time
                let response_time_ms = response.response_time.as_millis() as u64;
                response_times.push(response_time_ms as f64);
                
                // Count slow requests
                if response_time_ms >= self.slow_request_threshold {
                    slow_requests += 1;
                }
                if response_time_ms >= self.very_slow_request_threshold {
                    very_slow_requests += 1;
                }
            }
        }
        
        // Calculate response time statistics
        response_times.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        let avg_response_time = if response_times.is_empty() {
            0.0
        } else {
            response_times.iter().sum::<f64>() / response_times.len() as f64
        };
        
        let min_response_time = response_times.first().copied().unwrap_or(0.0);
        let max_response_time = response_times.last().copied().unwrap_or(0.0);
        
        let p95_response_time = if response_times.is_empty() {
            0.0
        } else {
            let index = ((response_times.len() as f64 * 0.95) as usize).min(response_times.len() - 1);
            response_times[index]
        };
        
        let p99_response_time = if response_times.is_empty() {
            0.0
        } else {
            let index = ((response_times.len() as f64 * 0.99) as usize).min(response_times.len() - 1);
            response_times[index]
        };
        
        let requests_per_second = total_requests as f64 / period.as_seconds() as f64;
        let error_rate = if total_responses > 0 {
            (error_count as f64 / total_responses as f64) * 100.0
        } else {
            0.0
        };
        
        // Get top slow requests within the time period
        let top_slow_requests = if let Ok(slow_reqs) = self.slow_requests.lock() {
            slow_reqs
                .iter()
                .filter(|req| req.timestamp >= cutoff_time)
                .cloned()
                .collect()
        } else {
            Vec::new()
        };
        
        Ok(HttpStats {
            total_requests,
            total_responses,
            avg_response_time,
            min_response_time,
            max_response_time,
            p95_response_time,
            p99_response_time,
            requests_per_second,
            error_rate,
            status_codes,
            methods,
            bytes_sent,
            bytes_received,
            url_patterns,
            slow_requests,
            very_slow_requests,
            top_slow_requests,
            period,
            timestamp: now,
        })
    }
    
    /// Cleanup old transactions and active requests that have timed out
    fn cleanup(&mut self) -> MonitoringResult<()> {
        let now = Instant::now();
        
        // Only cleanup every 60 seconds
        if now.duration_since(self.last_cleanup) < Duration::from_secs(60) {
            return Ok(());
        }
        
        // Cleanup timed out active transactions (older than 5 minutes)
        let timeout = Duration::from_secs(300);
        if let Ok(mut active) = self.active_transactions.lock() {
            active.retain(|_, transaction| {
                now.duration_since(transaction.start_time) < timeout
            });
        }
        
        // Cleanup old completed transactions (older than 1 hour)
        let old_cutoff = now - Duration::from_secs(3600);
        if let Ok(mut completed) = self.completed_transactions.lock() {
            while let Some(front) = completed.front() {
                if front.start_time < old_cutoff {
                    completed.pop_front();
                } else {
                    break;
                }
            }
        }
        
        // Cleanup old slow requests
        if let Ok(mut slow_reqs) = self.slow_requests.lock() {
            slow_reqs.retain(|req| req.timestamp >= old_cutoff);
        }
        
        self.last_cleanup = now;
        Ok(())
    }
}

impl Monitor for HttpMonitor {
    type Stats = HashMap<TimePeriod, HttpStats>;
    
    fn start(&mut self) -> MonitoringResult<()> {
        self.is_monitoring = true;
        Ok(())
    }
    
    fn stop(&mut self) -> MonitoringResult<()> {
        self.is_monitoring = false;
        Ok(())
    }
    
    fn is_running(&self) -> bool {
        self.is_monitoring
    }
    
    fn get_stats(&self) -> MonitoringResult<Self::Stats> {
        let mut all_stats = HashMap::new();
        
        for period in TimePeriod::all() {
            let stats = self.get_stats_for_period(period)?;
            all_stats.insert(period, stats);
        }
        
        Ok(all_stats)
    }
    
    fn reset(&mut self) -> MonitoringResult<()> {
        if let Ok(mut active) = self.active_transactions.lock() {
            active.clear();
        }
        
        if let Ok(mut completed) = self.completed_transactions.lock() {
            completed.clear();
        }
        
        if let Ok(mut cache) = self.stats_cache.lock() {
            cache.clear();
        }
        
        Ok(())
    }
    
    fn update(&mut self) -> MonitoringResult<()> {
        self.cleanup()
    }
    
    fn module_name(&self) -> &'static str {
        "http"
    }
}

impl Default for HttpMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_http_monitor_creation() {
        let monitor = HttpMonitor::new();
        assert!(!monitor.is_running());
    }
    
    #[test]
    fn test_http_monitor_start_stop() {
        let mut monitor = HttpMonitor::new();
        
        assert!(monitor.start().is_ok());
        assert!(monitor.is_running());
        
        assert!(monitor.stop().is_ok());
        assert!(!monitor.is_running());
    }
    
    #[test]
    fn test_record_request_response() {
        let mut monitor = HttpMonitor::new();
        monitor.start().unwrap();
        
        let request = HttpRequest {
            method: "GET".to_string(),
            url: "/api/test".to_string(),
            headers_size: 1024,
            body_size: 0,
            timestamp: Instant::now(),
            user_agent: Some("test-agent".to_string()),
            remote_ip: Some("127.0.0.1".to_string()),
        };
        
        assert!(monitor.record_request("test-123".to_string(), request).is_ok());
        
        let response = HttpResponse {
            status_code: 200,
            headers_size: 1024,
            body_size: 1024,
            timestamp: Instant::now(),
            response_time: Duration::from_millis(50),
        };
        
        assert!(monitor.record_response("test-123".to_string(), response).is_ok());
    }
    
    #[test]
    fn test_get_stats() {
        let mut monitor = HttpMonitor::new();
        monitor.start().unwrap();
        
        // Record some test data
        for i in 0..5 {
            let request = HttpRequest {
                method: "GET".to_string(),
                url: format!("/api/test/{}", i),
                headers_size: 1024,
                body_size: 100,
                timestamp: Instant::now(),
                user_agent: Some("test-agent".to_string()),
                remote_ip: Some("127.0.0.1".to_string()),
            };
            
            monitor.record_request(format!("test-{}", i), request).unwrap();
            
            let response = HttpResponse {
                status_code: if i % 2 == 0 { 200 } else { 404 },
                headers_size: 500,
                body_size: 500,
                timestamp: Instant::now(),
                response_time: Duration::from_millis(50 + i * 10),
            };
            
            monitor.record_response(format!("test-{}", i), response).unwrap();
        }
        
        let stats = monitor.get_stats().unwrap();
        assert!(!stats.is_empty());
        
        if let Some(ten_sec_stats) = stats.get(&TimePeriod::TenSeconds) {
            assert_eq!(ten_sec_stats.total_requests, 5);
            assert_eq!(ten_sec_stats.total_responses, 5);
            assert!(ten_sec_stats.avg_response_time > 0.0);
        }
    }
    
    #[test]
    fn test_reset() {
        let mut monitor = HttpMonitor::new();
        monitor.start().unwrap();
        
        // Add some data
        let request = HttpRequest {
            method: "GET".to_string(),
            url: "/test".to_string(),
            headers_size: 1024,
            body_size: 0,
            timestamp: Instant::now(),
            user_agent: Some("test-agent".to_string()),
            remote_ip: Some("127.0.0.1".to_string()),
        };
        
        monitor.record_request("test".to_string(), request).unwrap();
        
        // Reset should clear all data
        assert!(monitor.reset().is_ok());
        
        let stats = monitor.get_stats().unwrap();
        for (_, period_stats) in stats {
            assert_eq!(period_stats.total_requests, 0);
        }
    }
    
    #[test]
    fn test_slow_request_tracking() {
        let mut monitor = HttpMonitor::new();
        monitor.start().unwrap();
        
        // Record a slow request
        let request = HttpRequest {
            method: "GET".to_string(),
            url: "/api/slow".to_string(),
            headers_size: 1024,
            body_size: 0,
            timestamp: Instant::now(),
            user_agent: Some("test-agent".to_string()),
            remote_ip: Some("127.0.0.1".to_string()),
        };
        
        monitor.record_request("slow-1".to_string(), request).unwrap();
        
        // Simulate slow response (2 seconds)
        let response = HttpResponse {
            status_code: 200,
            headers_size: 1024,
            body_size: 1024,
            timestamp: Instant::now(),
            response_time: Duration::from_millis(2000),
        };
        
        monitor.record_response("slow-1".to_string(), response).unwrap();
        
        let stats = monitor.get_stats().unwrap();
        if let Some(one_min_stats) = stats.get(&TimePeriod::OneMinute) {
            assert_eq!(one_min_stats.total_requests, 1);
            assert_eq!(one_min_stats.slow_requests, 1);
        }
    }
    
    #[test]
    fn test_url_pattern_matching() {
        let matcher = UrlPatternMatcher::new();
        
        // Test ID pattern matching
        assert_eq!(matcher.match_pattern("/api/v1/users/123"), "/api/v*/users/*");
        assert_eq!(matcher.match_pattern("/users/456"), "/users/*");
        
        // Test static paths
        assert_eq!(matcher.match_pattern("/api/health"), "/api/health");
    }
}