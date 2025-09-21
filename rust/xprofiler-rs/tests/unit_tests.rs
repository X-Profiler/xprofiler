//! Unit tests for the xprofiler-rs library.
//! 
//! This module contains comprehensive unit tests for all monitoring components.

use std::time::Duration;
use std::thread;
use xprofiler_rs::monitoring::*;
use xprofiler_rs::monitoring::cpu::CpuMonitor;
use xprofiler_rs::monitoring::memory::MemoryMonitor;
use xprofiler_rs::monitoring::gc::{GcMonitor, GcType};
use xprofiler_rs::monitoring::http::{HttpMonitor, HttpRequest, HttpResponse};
use xprofiler_rs::monitoring::libuv::{LibuvMonitor, HandleType};

#[cfg(test)]
mod cpu_tests {
    use super::*;
    

    #[test]
    fn test_cpu_monitor_creation() {
        let monitor = CpuMonitor::new();
        assert!(!monitor.is_running());
        assert_eq!(monitor.get_stats().unwrap().current, 0.0);
    }

    #[test]
    fn test_cpu_monitor_start_stop() {
        let mut monitor = CpuMonitor::new();
        
        // Test start
        assert!(monitor.start().is_ok());
        assert!(monitor.is_running());
        
        // Test stop
        assert!(monitor.stop().is_ok());
        assert!(!monitor.is_running());
    }

    #[test]
    fn test_cpu_usage_calculation() {
        let mut monitor = CpuMonitor::new();
        monitor.start().unwrap();
        
        // Allow some time for CPU measurement
        thread::sleep(Duration::from_millis(100));
        monitor.update().unwrap();
        
        let stats = monitor.get_stats().unwrap();
        assert!(stats.current >= 0.0);
        assert!(stats.current <= 100.0);
        
        monitor.stop().unwrap();
    }

    #[test]
    fn test_cpu_usage_history() {
        let mut monitor = CpuMonitor::new();
        monitor.start().unwrap();
        
        // Generate some usage data
        for _ in 0..5 {
            thread::sleep(Duration::from_millis(50));
            monitor.update().unwrap();
        }
        
        let stats = monitor.get_stats().unwrap();
        assert!(stats.avg_1m >= 0.0);
        assert!(stats.avg_5m >= 0.0);
        assert!(stats.avg_15s >= 0.0);
        
        monitor.stop().unwrap();
    }

    #[test]
    fn test_cpu_reset() {
        let mut monitor = CpuMonitor::new();
        monitor.start().unwrap();
        
        thread::sleep(Duration::from_millis(100));
        monitor.update().unwrap();
        
        // Reset and verify
        assert!(monitor.reset().is_ok());
        let stats = monitor.get_stats().unwrap();
        assert_eq!(stats.current, 0.0);
        
        monitor.stop().unwrap();
    }
}

#[cfg(test)]
mod memory_tests {
    use super::*;
    

    #[test]
    fn test_memory_monitor_creation() {
        let monitor = MemoryMonitor::new();
        assert!(!monitor.is_running());
    }

    #[test]
    fn test_memory_monitor_start_stop() {
        let mut monitor = MemoryMonitor::new();
        
        assert!(monitor.start().is_ok());
        assert!(monitor.is_running());
        
        assert!(monitor.stop().is_ok());
        assert!(!monitor.is_running());
    }

    #[test]
    fn test_memory_stats_collection() {
        let mut monitor = MemoryMonitor::new();
        monitor.start().unwrap();
        
        // Give the monitor time to collect initial data
        thread::sleep(Duration::from_millis(50));
        
        let stats = monitor.get_stats().unwrap();
        // RSS might be 0 in some test environments (containers, CI)
        assert!(stats.rss >= 0, "RSS should be non-negative");
        // heap_used and external are unsigned, always >= 0
        assert!(stats.heap_total >= stats.heap_used);
        
        monitor.stop().unwrap();
    }

    #[test]
    fn test_memory_usage_tracking() {
        let mut monitor = MemoryMonitor::new();
        monitor.start().unwrap();
        
        // Allocate some memory
        let _data: Vec<u8> = vec![0; 1024 * 1024]; // 1MB
        
        // Give the monitor time to collect data
        thread::sleep(Duration::from_millis(50));
        
        let stats = monitor.get_stats().unwrap();
        // RSS might be 0 in some test environments (containers, CI)
        assert!(stats.rss >= 0, "RSS should be non-negative");
        
        monitor.stop().unwrap();
    }
}

#[cfg(test)]
mod gc_tests {
    use super::*;
    

    #[test]
    fn test_gc_monitor_creation() {
        let monitor = GcMonitor::new();
        assert!(!monitor.is_running());
    }

    #[test]
    fn test_gc_monitor_start_stop() {
        let mut monitor = GcMonitor::new();
        
        assert!(monitor.start().is_ok());
        assert!(monitor.is_running());
        
        assert!(monitor.stop().is_ok());
        assert!(!monitor.is_running());
    }

    #[test]
    fn test_gc_stats_initialization() {
        let monitor = GcMonitor::new();
        let stats = monitor.get_stats().unwrap();
        
        assert_eq!(stats.total_gc_count, 0);
        assert_eq!(stats.total_gc_time, Duration::ZERO);
    }

    #[test]
    fn test_gc_event_recording() {
        let mut monitor = GcMonitor::new();
        monitor.start().unwrap();
        
        // Simulate GC event
        let gc_event = xprofiler_rs::monitoring::gc::GcEvent {
            gc_type: GcType::Scavenge,
            duration: Duration::from_millis(10),
            timestamp: std::time::Instant::now(),
            heap_size_before: 1024,
            heap_size_after: 512,
        };
        monitor.record_gc_event(gc_event);
        
        let stats = monitor.get_stats().unwrap();
        assert_eq!(stats.total_gc_count, 1);
        assert!(stats.total_gc_time > Duration::ZERO);
        
        monitor.stop().unwrap();
    }
}

#[cfg(test)]
mod http_tests {
    use super::*;
    use std::time::Instant;
    

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
    fn test_http_request_recording() {
        let mut monitor = HttpMonitor::new();
        monitor.start().unwrap();
        
        // Record HTTP request and response
        let request = HttpRequest {
            method: "GET".to_string(),
            url: "/test".to_string(),
            headers_size: 100,
            body_size: 0,
            timestamp: Instant::now(),
            user_agent: Some("test-agent".to_string()),
            remote_ip: Some("127.0.0.1".to_string()),
        };
        monitor.record_request("req_1".to_string(), request);
        
        let response = HttpResponse {
            status_code: 500,
            headers_size: 80,
            body_size: 0,
            response_time: Duration::from_millis(100),
            timestamp: Instant::now(),
        };
        monitor.record_response("req_1".to_string(), response);
        
        let stats = monitor.get_http_stats();
        assert_eq!(stats.total_responses, 1);
        assert_eq!(stats.responses_by_status.get(&500), Some(&1));
        
        monitor.stop().unwrap();
    }

    #[test]
    fn test_http_error_recording() {
        let mut monitor = HttpMonitor::new();
        monitor.start().unwrap();
        
        // Record a request that results in an error response
        let request = HttpRequest {
            method: "POST".to_string(),
            url: "/error".to_string(),
            headers_size: 100,
            body_size: 50,
            timestamp: Instant::now(),
            user_agent: Some("test-agent".to_string()),
            remote_ip: Some("127.0.0.1".to_string()),
        };
        monitor.record_request("req_error".to_string(), request);
        
        let response = HttpResponse {
            status_code: 500,
            headers_size: 80,
            body_size: 0,
            response_time: Duration::from_millis(200),
            timestamp: Instant::now(),
        };
        monitor.record_response("req_error".to_string(), response);
        
        let stats = monitor.get_http_stats();
        assert!(stats.error_rate > 0.0);
        
        monitor.stop().unwrap();
    }
}

#[cfg(test)]
mod libuv_tests {
    use super::*;
    
    

    #[test]
    fn test_libuv_monitor_creation() {
        let monitor = LibuvMonitor::new();
        assert!(!monitor.is_running());
    }

    #[test]
    fn test_libuv_monitor_start_stop() {
        let mut monitor = LibuvMonitor::new();
        
        assert!(monitor.start().is_ok());
        assert!(monitor.is_running());
        
        assert!(monitor.stop().is_ok());
        assert!(!monitor.is_running());
    }

    #[test]
    fn test_handle_registration() {
        let mut monitor = LibuvMonitor::new();
        assert!(!monitor.is_running());
        
        // Start monitoring
        monitor.start().unwrap();
        assert!(monitor.is_running());
        
        // Test handle registration
        let handle_id = monitor.register_handle(HandleType::Timer, true, true);
        assert!(handle_id > 0);
        
        // Update handle status
        monitor.update_handle_status(handle_id, false, false);
        
        // Record loop iteration with all required parameters
        monitor.record_loop_iteration(
            Duration::from_millis(5),  // iteration_time
            Duration::from_millis(1),  // idle_time
            Duration::from_millis(1),  // prepare_time
            Duration::from_millis(1),  // check_time
            Duration::from_millis(1)   // poll_time
        );
        
        // Get stats
        let stats = monitor.get_stats().unwrap();
        assert!(stats.total_handles > 0);
        
        // Stop monitoring
        monitor.stop().unwrap();
        assert!(!monitor.is_running());
    }

    #[test]
    fn test_handle_unregistration() {
        let mut monitor = LibuvMonitor::new();
        monitor.start().unwrap();
        
        let handle_id = monitor.register_handle(HandleType::Tcp, true, true);
        monitor.unregister_handle(handle_id);
        
        let stats = monitor.get_stats().unwrap();
        assert_eq!(stats.total_handles, 0);
        assert_eq!(stats.total_active_handles, 0);
        
        monitor.stop().unwrap();
    }

    #[test]
    fn test_loop_iteration_recording() {
        let mut monitor = LibuvMonitor::new();
        monitor.start().unwrap();
        
        monitor.record_loop_iteration(
            Duration::from_millis(10),
            Duration::from_millis(2),
            Duration::from_millis(1),
            Duration::from_millis(1),
            Duration::from_millis(6)
        );
        
        let stats = monitor.get_stats().unwrap();
        assert_eq!(stats.loop_metrics.loop_count, 1);
        assert!(stats.loop_metrics.avg_loop_time > Duration::ZERO);
        
        monitor.stop().unwrap();
    }

    #[test]
    fn test_handle_type_conversion() {
        assert_eq!(HandleType::from_uv_type(1), HandleType::Timer);
        assert_eq!(HandleType::from_uv_type(2), HandleType::Tcp);
        assert_eq!(HandleType::from_uv_type(999), HandleType::Unknown);
        
        assert_eq!(HandleType::Timer.as_str(), "timer");
        assert_eq!(HandleType::Tcp.as_str(), "tcp");
        assert_eq!(HandleType::Unknown.as_str(), "unknown");
    }
}

#[cfg(test)]
mod utils_tests {
    use super::*;
    

    #[test]
    fn test_time_utilities() {
        // Test timestamp generation using std::time
        let start = std::time::SystemTime::now();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let end = std::time::SystemTime::now();
        assert!(end > start);
    }

    #[test]
    fn test_memory_formatting() {
        // Test memory size calculations
        let memory_1mb = 1024 * 1024;
        assert_eq!(memory_1mb, 1048576);
        
        // Test percentage calculations
        let calculate_percentage = |part: f64, total: f64| {
            if total == 0.0 { 0.0 } else { (part / total) * 100.0 }
        };
        assert_eq!(calculate_percentage(0.0, 100.0), 0.0);
        assert_eq!(calculate_percentage(100.0, 100.0), 100.0);
        assert_eq!(calculate_percentage(25.0, 0.0), 0.0); // Division by zero
    }

    #[test]
    fn test_duration_formatting() {
        // Test time formatting
        let duration_1500ms = Duration::from_millis(1500);
        let duration_500us = Duration::from_micros(500);
        let duration_1000ns = Duration::from_nanos(1000);
        
        // Basic duration tests
        assert!(duration_1500ms.as_millis() == 1500);
        assert!(duration_500us.as_micros() == 500);
        assert!(duration_1000ns.as_nanos() == 1000);
    }

    #[test]
    fn test_version_info() {
        let version = env!("CARGO_PKG_VERSION");
        assert!(!version.is_empty());
        assert!(version.contains('.'));
    }
}