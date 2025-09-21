//! HTTP monitoring module
//!
//! This module provides HTTP request/response monitoring capabilities
//! including request counting, response time tracking, and status code analysis.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use super::{Monitor, MonitoringResult, MonitoringError};

/// HTTP method enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    TRACE,
    CONNECT,
    Other(String),
}

impl From<String> for HttpMethod {
    fn from(method: String) -> Self {
        match method.to_uppercase().as_str() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "PATCH" => HttpMethod::PATCH,
            "HEAD" => HttpMethod::HEAD,
            "OPTIONS" => HttpMethod::OPTIONS,
            "TRACE" => HttpMethod::TRACE,
            "CONNECT" => HttpMethod::CONNECT,
            _ => HttpMethod::Other(method),
        }
    }
}

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        match self {
            HttpMethod::GET => "GET".to_string(),
            HttpMethod::POST => "POST".to_string(),
            HttpMethod::PUT => "PUT".to_string(),
            HttpMethod::DELETE => "DELETE".to_string(),
            HttpMethod::PATCH => "PATCH".to_string(),
            HttpMethod::HEAD => "HEAD".to_string(),
            HttpMethod::OPTIONS => "OPTIONS".to_string(),
            HttpMethod::TRACE => "TRACE".to_string(),
            HttpMethod::CONNECT => "CONNECT".to_string(),
            HttpMethod::Other(method) => method.clone(),
        }
    }
}

impl HttpMethod {
    /// Convert HttpMethod to string slice
    pub fn as_str(&self) -> &str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::TRACE => "TRACE",
            HttpMethod::CONNECT => "CONNECT",
            HttpMethod::Other(method) => method,
        }
    }
    
    /// Parse HttpMethod from string
    pub fn from_str(method: &str) -> Self {
        HttpMethod::from(method.to_string())
    }
}

/// HTTP request information
#[derive(Debug, Clone)]
pub struct HttpRequest {
    /// HTTP method (GET, POST, etc.)
    pub method: String,
    /// Request URL or path
    pub url: String,
    /// Request timestamp
    pub timestamp: Instant,
    /// Request headers size
    pub headers_size: u64,
    /// Request body size
    pub body_size: u64,
    /// User agent
    pub user_agent: Option<String>,
    /// Remote IP address
    pub remote_ip: Option<String>,
}

/// HTTP response information
#[derive(Debug, Clone)]
pub struct HttpResponse {
    /// HTTP status code
    pub status_code: u16,
    /// Response timestamp
    pub timestamp: Instant,
    /// Response headers size
    pub headers_size: u64,
    /// Response body size
    pub body_size: u64,
    /// Response time (duration from request to response)
    pub response_time: Duration,
}

/// Complete HTTP transaction
#[derive(Debug, Clone)]
pub struct HttpTransaction {
    /// Request information
    pub request: HttpRequest,
    /// Response information
    pub response: HttpResponse,
    /// Total transaction time
    pub total_time: Duration,
}

/// HTTP monitoring statistics
#[derive(Debug, Clone)]
pub struct HttpStats {
    /// Total number of requests
    pub total_requests: u64,
    /// Total number of responses
    pub total_responses: u64,
    /// Requests by HTTP method
    pub requests_by_method: HashMap<String, u64>,
    /// Responses by status code
    pub responses_by_status: HashMap<u16, u64>,
    /// Average response time
    pub avg_response_time: Duration,
    /// Minimum response time
    pub min_response_time: Duration,
    /// Maximum response time
    pub max_response_time: Duration,
    /// Total bytes sent (request bodies + headers)
    pub total_bytes_sent: u64,
    /// Total bytes received (response bodies + headers)
    pub total_bytes_received: u64,
    /// Requests per second (based on monitoring period)
    pub requests_per_second: f64,
    /// Error rate (4xx and 5xx responses)
    pub error_rate: f64,
    /// Recent transactions (last 100)
    pub recent_transactions: Vec<HttpTransaction>,
}

/// HTTP monitor implementation
pub struct HttpMonitor {
    /// HTTP transactions history
    transactions: Vec<HttpTransaction>,
    /// Pending requests (waiting for response)
    pending_requests: HashMap<String, HttpRequest>,
    /// Maximum number of transactions to keep
    max_transactions: usize,
    /// Whether monitoring is active
    is_monitoring: bool,
    /// Start time for monitoring
    start_time: Option<Instant>,
}

impl HttpMonitor {
    /// Create a new HTTP monitor
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
            pending_requests: HashMap::new(),
            max_transactions: 10000, // Keep last 10000 transactions
            is_monitoring: false,
            start_time: None,
        }
    }
    
    /// Record an HTTP request
    pub fn record_request(&mut self, request_id: String, request: HttpRequest) {
        if !self.is_monitoring {
            return;
        }
        
        self.pending_requests.insert(request_id, request);
    }
    
    /// Record an HTTP response and complete the transaction
    pub fn record_response(&mut self, request_id: String, response: HttpResponse) {
        if !self.is_monitoring {
            return;
        }
        
        if let Some(request) = self.pending_requests.remove(&request_id) {
            let total_time = response.timestamp.duration_since(request.timestamp);
            
            let transaction = HttpTransaction {
                request,
                response,
                total_time,
            };
            
            self.transactions.push(transaction);
            
            // Keep only the most recent transactions
            if self.transactions.len() > self.max_transactions {
                self.transactions.remove(0);
            }
        }
    }
    
    /// Get HTTP statistics
    pub fn get_http_stats(&self) -> HttpStats {
        let mut requests_by_method = HashMap::new();
        let mut responses_by_status = HashMap::new();
        let mut response_times = Vec::new();
        let mut total_bytes_sent = 0;
        let mut total_bytes_received = 0;
        let mut error_count = 0;
        
        for transaction in &self.transactions {
            // Count by method
            *requests_by_method.entry(transaction.request.method.clone()).or_insert(0) += 1;
            
            // Count by status
            *responses_by_status.entry(transaction.response.status_code).or_insert(0) += 1;
            
            // Collect response times
            response_times.push(transaction.response.response_time);
            
            // Sum bytes
            total_bytes_sent += transaction.request.headers_size + transaction.request.body_size;
            total_bytes_received += transaction.response.headers_size + transaction.response.body_size;
            
            // Count errors (4xx and 5xx)
            if transaction.response.status_code >= 400 {
                error_count += 1;
            }
        }
        
        // Calculate response time statistics
        let (avg_response_time, min_response_time, max_response_time) = if response_times.is_empty() {
            (Duration::ZERO, Duration::ZERO, Duration::ZERO)
        } else {
            let total: Duration = response_times.iter().sum();
            let avg = total / response_times.len() as u32;
            let min = *response_times.iter().min().unwrap();
            let max = *response_times.iter().max().unwrap();
            (avg, min, max)
        };
        
        // Calculate requests per second
        let requests_per_second = if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed().as_secs_f64();
            if elapsed > 0.0 {
                self.transactions.len() as f64 / elapsed
            } else {
                0.0
            }
        } else {
            0.0
        };
        
        // Calculate error rate
        let error_rate = if self.transactions.is_empty() {
            0.0
        } else {
            error_count as f64 / self.transactions.len() as f64 * 100.0
        };
        
        // Get recent transactions (last 100)
        let recent_transactions = if self.transactions.len() > 100 {
            self.transactions[self.transactions.len() - 100..].to_vec()
        } else {
            self.transactions.clone()
        };
        
        HttpStats {
            total_requests: self.transactions.len() as u64 + self.pending_requests.len() as u64,
            total_responses: self.transactions.len() as u64,
            requests_by_method,
            responses_by_status,
            avg_response_time,
            min_response_time,
            max_response_time,
            total_bytes_sent,
            total_bytes_received,
            requests_per_second,
            error_rate,
            recent_transactions,
        }
    }
    
    /// Format HTTP statistics for logging
    pub fn format_http_stats(&self) -> String {
        let stats = self.get_http_stats();
        
        let status_2xx = stats.responses_by_status.iter()
            .filter(|(code, _)| **code >= 200 && **code < 300)
            .map(|(_, count)| *count)
            .sum::<u64>();
            
        let status_3xx = stats.responses_by_status.iter()
            .filter(|(code, _)| **code >= 300 && **code < 400)
            .map(|(_, count)| *count)
            .sum::<u64>();
            
        let status_4xx = stats.responses_by_status.iter()
            .filter(|(code, _)| **code >= 400 && **code < 500)
            .map(|(_, count)| *count)
            .sum::<u64>();
            
        let status_5xx = stats.responses_by_status.iter()
            .filter(|(code, _)| **code >= 500)
            .map(|(_, count)| *count)
            .sum::<u64>();
        
        format!(
            "http total_requests: {}, total_responses: {}, status_2xx: {}, status_3xx: {}, \
             status_4xx: {}, status_5xx: {}, avg_response_time: {}ms, error_rate: {:.2}%, \
             requests_per_second: {:.2}, total_bytes_sent: {}, total_bytes_received: {}",
            stats.total_requests,
            stats.total_responses,
            status_2xx,
            status_3xx,
            status_4xx,
            status_5xx,
            stats.avg_response_time.as_millis(),
            stats.error_rate,
            stats.requests_per_second,
            stats.total_bytes_sent,
            stats.total_bytes_received
        )
    }
    
    /// Clear all recorded transactions
    pub fn clear_transactions(&mut self) {
        self.transactions.clear();
        self.pending_requests.clear();
    }
}

impl Default for HttpMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Monitor for HttpMonitor {
    type Stats = HttpStats;
    
    fn start(&mut self) -> MonitoringResult<()> {
        self.is_monitoring = true;
        self.start_time = Some(Instant::now());
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
        Ok(self.get_http_stats())
    }
    
    fn reset(&mut self) -> MonitoringResult<()> {
        self.transactions.clear();
        self.pending_requests.clear();
        self.start_time = None;
        Ok(())
    }
    
    fn update(&mut self) -> MonitoringResult<()> {
        // HTTP monitoring doesn't need periodic updates
        Ok(())
    }
    
    fn module_name(&self) -> &'static str {
        "http"
    }
}

/// Global HTTP monitor instance
static HTTP_MONITOR: Mutex<Option<HttpMonitor>> = Mutex::new(None);

/// Initialize global HTTP monitor
pub fn init_http_monitor() -> MonitoringResult<()> {
    let mut monitor = HTTP_MONITOR.lock()
        .map_err(|_| MonitoringError::LockFailed {
            resource: "HTTP monitor".to_string(),
            details: "Failed to acquire lock".to_string(),
        })?;
    *monitor = Some(HttpMonitor::new());
    Ok(())
}

/// Record an HTTP request
pub fn record_http_request(request_id: String, method: String, url: String, headers_size: u64, body_size: u64, user_agent: Option<String>, remote_ip: Option<String>) -> MonitoringResult<()> {
    let mut monitor = HTTP_MONITOR.lock()
        .map_err(|_| MonitoringError::LockFailed {
            resource: "HTTP monitor".to_string(),
            details: "Failed to acquire lock".to_string(),
        })?;
    if let Some(ref mut http_monitor) = monitor.as_mut() {
        let request = HttpRequest {
            method,
            url,
            timestamp: Instant::now(),
            headers_size,
            body_size,
            user_agent,
            remote_ip,
        };
        http_monitor.record_request(request_id, request);
    }
    Ok(())
}

/// Record an HTTP response
pub fn record_http_response(request_id: String, status_code: u16, headers_size: u64, body_size: u64, response_time: Duration) -> MonitoringResult<()> {
    let mut monitor = HTTP_MONITOR.lock()
        .map_err(|_| MonitoringError::LockFailed {
            resource: "HTTP monitor".to_string(),
            details: "Failed to acquire lock".to_string(),
        })?;
    if let Some(ref mut http_monitor) = monitor.as_mut() {
        let response = HttpResponse {
            status_code,
            timestamp: Instant::now(),
            headers_size,
            body_size,
            response_time,
        };
        http_monitor.record_response(request_id, response);
    }
    Ok(())
}

/// Get HTTP statistics
pub fn get_http_stats() -> MonitoringResult<HttpStats> {
    let monitor = HTTP_MONITOR.lock()
        .map_err(|_| MonitoringError::LockFailed {
            resource: "HTTP monitor".to_string(),
            details: "Failed to acquire lock".to_string(),
        })?;
    monitor.as_ref()
        .map(|m| m.get_http_stats())
        .ok_or_else(|| MonitoringError::NotInitialized {
            module: "HTTP monitor".to_string(),
        })
}

/// Format HTTP statistics for logging
pub fn format_http_stats() -> String {
    let monitor = HTTP_MONITOR.lock().unwrap();
    monitor
        .as_ref()
        .map(|m| m.format_http_stats())
        .unwrap_or_else(|| "HTTP monitor not initialized".to_string())
}

/// Start HTTP monitoring
pub fn start_http_monitoring() -> MonitoringResult<()> {
    let mut monitor = HTTP_MONITOR.lock()
        .map_err(|_| MonitoringError::LockFailed {
            resource: "HTTP monitor".to_string(),
            details: "Failed to acquire lock".to_string(),
        })?;
    if let Some(ref mut http_monitor) = monitor.as_mut() {
        http_monitor.start()?;
    }
    Ok(())
}

/// Stop HTTP monitoring
pub fn stop_http_monitoring() -> MonitoringResult<()> {
    let mut monitor = HTTP_MONITOR.lock()
        .map_err(|_| MonitoringError::LockFailed {
            resource: "HTTP monitor".to_string(),
            details: "Failed to acquire lock".to_string(),
        })?;
    if let Some(ref mut http_monitor) = monitor.as_mut() {
        http_monitor.stop()?;
    }
    Ok(())
}

/// Record an HTTP request (alias for record_http_request)
pub fn record_request(request_id: String, method: String, url: String, headers_size: u64, body_size: u64, user_agent: Option<String>, remote_ip: Option<String>) -> MonitoringResult<()> {
    record_http_request(request_id, method, url, headers_size, body_size, user_agent, remote_ip)
}

/// Record an HTTP response (alias for record_http_response)
pub fn record_response(request_id: String, status_code: u16, headers_size: u64, body_size: u64, response_time: Duration) -> MonitoringResult<()> {
    record_http_response(request_id, status_code, headers_size, body_size, response_time)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_http_monitor_creation() {
        let monitor = HttpMonitor::new();
        assert!(!monitor.is_monitoring);
        assert_eq!(monitor.transactions.len(), 0);
    }
    
    #[test]
    fn test_http_transaction_recording() {
        let mut monitor = HttpMonitor::new();
        monitor.start().unwrap();
        
        let request = HttpRequest {
            method: "GET".to_string(),
            url: "/api/test".to_string(),
            timestamp: Instant::now(),
            headers_size: 256,
            body_size: 0,
            user_agent: Some("test-agent".to_string()),
            remote_ip: Some("127.0.0.1".to_string()),
        };
        
        let response = HttpResponse {
            status_code: 200,
            timestamp: Instant::now(),
            headers_size: 128,
            body_size: 1024,
            response_time: Duration::from_millis(50),
        };
        
        monitor.record_request("req1".to_string(), request);
        monitor.record_response("req1".to_string(), response);
        
        assert_eq!(monitor.transactions.len(), 1);
        
        let stats = monitor.get_http_stats();
        assert_eq!(stats.total_requests, 1);
        assert_eq!(stats.total_responses, 1);
        assert_eq!(*stats.requests_by_method.get("GET").unwrap(), 1);
        assert_eq!(*stats.responses_by_status.get(&200).unwrap(), 1);
    }
}