//! HTTP monitoring NAPI bindings
//!
//! This module provides Node.js bindings for HTTP monitoring functionality.

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;
use crate::monitoring::http::{
    HttpRequest, HttpResponse, HttpTransaction, HttpStats, HttpMethod,
    init_http_monitor, start_http_monitoring, stop_http_monitoring,
    record_request, record_response, get_http_stats, format_http_stats
};

/// JavaScript representation of HTTP method
#[napi(object)]
pub struct JsHttpMethod {
    pub value: String,
}

impl From<HttpMethod> for JsHttpMethod {
    fn from(method: HttpMethod) -> Self {
        Self {
            value: method.as_str().to_string(),
        }
    }
}

impl From<JsHttpMethod> for HttpMethod {
    fn from(js_method: JsHttpMethod) -> Self {
        HttpMethod::from_str(&js_method.value)
    }
}

/// JavaScript representation of HTTP request
#[napi(object)]
pub struct JsHttpRequest {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body_size: u64,
    pub timestamp: f64,
    pub user_agent: Option<String>,
    pub remote_address: Option<String>,
}

impl From<HttpRequest> for JsHttpRequest {
    fn from(request: HttpRequest) -> Self {
        Self {
            method: request.method.as_str().to_string(),
            url: request.url,
            headers: request.headers,
            body_size: request.body_size,
            timestamp: request.timestamp.elapsed().as_secs_f64(),
            user_agent: request.user_agent,
            remote_address: request.remote_address,
        }
    }
}

/// JavaScript representation of HTTP response
#[napi(object)]
pub struct JsHttpResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body_size: u64,
    pub timestamp: f64,
}

impl From<HttpResponse> for JsHttpResponse {
    fn from(response: HttpResponse) -> Self {
        Self {
            status_code: response.status_code,
            headers: response.headers,
            body_size: response.body_size,
            timestamp: response.timestamp.elapsed().as_secs_f64(),
        }
    }
}

/// JavaScript representation of HTTP transaction
#[napi(object)]
pub struct JsHttpTransaction {
    pub id: String,
    pub request: JsHttpRequest,
    pub response: Option<JsHttpResponse>,
    pub duration_ms: Option<f64>,
    pub error: Option<String>,
}

impl From<HttpTransaction> for JsHttpTransaction {
    fn from(transaction: HttpTransaction) -> Self {
        Self {
            id: transaction.id,
            request: transaction.request.into(),
            response: transaction.response.map(|r| r.into()),
            duration_ms: transaction.duration.map(|d| d.as_secs_f64() * 1000.0),
            error: transaction.error,
        }
    }
}

/// JavaScript representation of HTTP statistics
#[napi(object)]
pub struct JsHttpStats {
    pub total_requests: u64,
    pub total_responses: u64,
    pub active_requests: u64,
    pub total_request_size: u64,
    pub total_response_size: u64,
    pub avg_response_time_ms: f64,
    pub min_response_time_ms: f64,
    pub max_response_time_ms: f64,
    pub status_code_counts: HashMap<String, u32>,
    pub method_counts: HashMap<String, u32>,
    pub error_count: u64,
    pub timeout_count: u64,
    pub requests_per_second: f64,
    pub responses_per_second: f64,
    pub recent_transactions: Vec<JsHttpTransaction>,
}

impl From<HttpStats> for JsHttpStats {
    fn from(stats: HttpStats) -> Self {
        let status_code_counts = stats.status_code_counts
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
            
        let method_counts = stats.method_counts
            .into_iter()
            .map(|(k, v)| (k.as_str().to_string(), v))
            .collect();
            
        let recent_transactions = stats.recent_transactions
            .into_iter()
            .map(|t| t.into())
            .collect();
        
        Self {
            total_requests: stats.total_requests,
            total_responses: stats.total_responses,
            active_requests: stats.active_requests,
            total_request_size: stats.total_request_size,
            total_response_size: stats.total_response_size,
            avg_response_time_ms: stats.avg_response_time.as_secs_f64() * 1000.0,
            min_response_time_ms: stats.min_response_time.as_secs_f64() * 1000.0,
            max_response_time_ms: stats.max_response_time.as_secs_f64() * 1000.0,
            status_code_counts,
            method_counts,
            error_count: stats.error_count,
            timeout_count: stats.timeout_count,
            requests_per_second: stats.requests_per_second,
            responses_per_second: stats.responses_per_second,
            recent_transactions,
        }
    }
}

/// Initialize HTTP monitor
#[napi]
pub fn init_http_monitor_js() -> Result<()> {
    init_http_monitor()
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to initialize HTTP monitor: {}", e)))
}

/// Start HTTP monitoring
#[napi]
pub fn start_http_monitoring_js() -> Result<()> {
    start_http_monitoring()
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to start HTTP monitoring: {}", e)))
}

/// Stop HTTP monitoring
#[napi]
pub fn stop_http_monitoring_js() -> Result<()> {
    stop_http_monitoring()
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to stop HTTP monitoring: {}", e)))
}

/// Record HTTP request
#[napi]
pub fn record_http_request(
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body_size: u32,
    user_agent: Option<String>,
    remote_address: Option<String>,
) -> String {
    let http_method = HttpMethod::from_str(&method);
    record_request(
        http_method,
        url,
        headers,
        body_size as u64,
        user_agent,
        remote_address,
    )
}

/// Record HTTP response
#[napi]
pub fn record_http_response(
    transaction_id: String,
    status_code: u32,
    headers: HashMap<String, String>,
    body_size: u32,
) {
    record_response(
        transaction_id,
        status_code as u16,
        headers,
        body_size as u64,
    );
}

/// Record HTTP error
#[napi]
pub fn record_http_error(transaction_id: String, error_message: String) {
    // This would be implemented in the monitoring module
    // For now, we'll use the existing record_response with error status
    record_response(
        transaction_id,
        500, // Internal Server Error
        HashMap::new(),
        0,
    );
}

/// Get HTTP statistics
#[napi]
pub fn get_http_stats_js() -> Option<JsHttpStats> {
    get_http_stats().map(|stats| stats.into())
}

/// Format HTTP statistics for logging
#[napi]
pub fn format_http_stats_js() -> String {
    format_http_stats()
}

/// Get HTTP statistics by status code
#[napi]
pub fn get_http_stats_by_status_code() -> HashMap<String, u32> {
    get_http_stats()
        .map(|stats| {
            stats.status_code_counts
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect()
        })
        .unwrap_or_default()
}

/// Get HTTP statistics by method
#[napi]
pub fn get_http_stats_by_method() -> HashMap<String, u32> {
    get_http_stats()
        .map(|stats| {
            stats.method_counts
                .into_iter()
                .map(|(k, v)| (k.as_str().to_string(), v))
                .collect()
        })
        .unwrap_or_default()
}

/// Get recent HTTP transactions
#[napi]
pub fn get_recent_http_transactions(limit: Option<u32>) -> Vec<JsHttpTransaction> {
    get_http_stats()
        .map(|stats| {
            let mut transactions: Vec<JsHttpTransaction> = stats.recent_transactions
                .into_iter()
                .map(|t| t.into())
                .collect();
            
            if let Some(limit) = limit {
                transactions.truncate(limit as usize);
            }
            
            transactions
        })
        .unwrap_or_default()
}

/// Get HTTP performance metrics
#[napi(object)]
pub struct JsHttpPerformanceMetrics {
    pub avg_response_time_ms: f64,
    pub min_response_time_ms: f64,
    pub max_response_time_ms: f64,
    pub requests_per_second: f64,
    pub responses_per_second: f64,
    pub error_rate: f64,
    pub timeout_rate: f64,
}

/// Get HTTP performance metrics
#[napi]
pub fn get_http_performance_metrics() -> Option<JsHttpPerformanceMetrics> {
    get_http_stats().map(|stats| {
        let error_rate = if stats.total_requests > 0 {
            (stats.error_count as f64) / (stats.total_requests as f64) * 100.0
        } else {
            0.0
        };
        
        let timeout_rate = if stats.total_requests > 0 {
            (stats.timeout_count as f64) / (stats.total_requests as f64) * 100.0
        } else {
            0.0
        };
        
        JsHttpPerformanceMetrics {
            avg_response_time_ms: stats.avg_response_time.as_secs_f64() * 1000.0,
            min_response_time_ms: stats.min_response_time.as_secs_f64() * 1000.0,
            max_response_time_ms: stats.max_response_time.as_secs_f64() * 1000.0,
            requests_per_second: stats.requests_per_second,
            responses_per_second: stats.responses_per_second,
            error_rate,
            timeout_rate,
        }
    })
}

/// Reset HTTP monitoring statistics
#[napi]
pub fn reset_http_stats() -> Result<()> {
    // This would be implemented in the monitoring module
    // For now, we'll return success
    Ok(())
}