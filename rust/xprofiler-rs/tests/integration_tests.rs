//! Integration tests for the xprofiler-rs library.
//! 
//! This module contains integration tests that verify the interaction between different components.

use std::time::{Duration, Instant};
use std::thread;
use xprofiler_rs::monitoring::*;
use xprofiler_rs::monitoring::cpu::{CpuMonitor, CpuUsage};
use xprofiler_rs::monitoring::memory::{MemoryMonitor, MemoryUsage};
use xprofiler_rs::monitoring::gc::{GcMonitor, GcType, GcStats, GcEvent};
use xprofiler_rs::monitoring::http::{HttpMonitor, HttpMethod, HttpStats, HttpRequest, HttpResponse};
use xprofiler_rs::monitoring::libuv::{LibuvMonitor, HandleType};
use xprofiler_rs::utils::*;
use serial_test::serial;
use xprofiler_rs::monitoring::Monitor;

#[cfg(test)]
mod monitoring_integration_tests {
    use super::*;
    use xprofiler_rs::monitoring::{cpu::*, memory::*, gc::*, http::*, libuv::*};

    #[test]
    fn test_multiple_monitors_concurrent() {
        let mut cpu_monitor = CpuMonitor::new();
        let mut memory_monitor = MemoryMonitor::new();
        let mut gc_monitor = GcMonitor::new();
        let mut http_monitor = HttpMonitor::new();
        let mut libuv_monitor = LibuvMonitor::new();

        // Start all monitors
        assert!(cpu_monitor.start().is_ok());
        assert!(memory_monitor.start().is_ok());
        assert!(gc_monitor.start().is_ok());
        assert!(http_monitor.start().is_ok());
        assert!(libuv_monitor.start().is_ok());

        // Verify all are running
        assert!(cpu_monitor.is_running());
        assert!(memory_monitor.is_running());
        assert!(gc_monitor.is_running());
        assert!(http_monitor.is_running());
        assert!(libuv_monitor.is_running());

        // Generate some activity
        thread::sleep(Duration::from_millis(100));
        cpu_monitor.update().unwrap();

        // Simulate HTTP activity
        let request = HttpRequest {
            method: "GET".to_string(),
            url: "/api/test".to_string(),
            timestamp: Instant::now(),
            headers_size: 1024,
            body_size: 512,
            user_agent: None,
            remote_ip: None,
        };
        let request_id = "req1".to_string();
        http_monitor.record_request(request_id.clone(), request);
        
        let response = HttpResponse {
            status_code: 200,
            timestamp: Instant::now(),
            headers_size: 2048,
            body_size: 1024,
            response_time: Duration::from_millis(50),
        };
        http_monitor.record_response(request_id, response);

        // Simulate libuv activity
        let handle_id = libuv_monitor.register_handle(HandleType::Timer, true, true);
        libuv_monitor.record_loop_iteration(
            Duration::from_millis(5),
            Duration::from_millis(1),
            Duration::from_millis(1),
            Duration::from_millis(1),
            Duration::from_millis(2)
        );

        // Simulate GC activity
        let gc_event = GcEvent {
            gc_type: GcType::Major,
            duration: Duration::from_millis(20),
            timestamp: Instant::now(),
            heap_size_before: 2048,
            heap_size_after: 1024,
        };
        gc_monitor.record_gc_event(gc_event);

        // Verify stats collection
        let cpu_stats = cpu_monitor.get_stats();
        let memory_stats = memory_monitor.get_stats();
        let gc_stats = gc_monitor.get_stats();
        let http_stats = http_monitor.get_stats();
        let libuv_stats = libuv_monitor.get_stats();

        assert!(cpu_stats.current >= 0.0);
        assert!(memory_stats.rss > 0);
        assert_eq!(gc_stats.gc_count, 1);
        assert_eq!(http_stats.total_requests, 1);
        assert_eq!(libuv_stats.total_handles, 1);

        // Stop all monitors
        assert!(cpu_monitor.stop().is_ok());
        assert!(memory_monitor.stop().is_ok());
        assert!(gc_monitor.stop().is_ok());
        assert!(http_monitor.stop().is_ok());
        assert!(libuv_monitor.stop().is_ok());
    }

    #[test]
    fn test_monitor_reset_functionality() {
        let mut cpu_monitor = CpuMonitor::new();
        let mut gc_monitor = GcMonitor::new();
        let mut http_monitor = HttpMonitor::new();

        // Start monitors and generate activity
        cpu_monitor.start().unwrap();
        gc_monitor.start().unwrap();
        http_monitor.start().unwrap();

        thread::sleep(Duration::from_millis(50));
        cpu_monitor.update().unwrap();

        let gc_event = GcEvent {
            gc_type: GcType::Scavenge,
            duration: Duration::from_millis(5),
            timestamp: Instant::now(),
            heap_size_before: 1024,
            heap_size_after: 512,
        };
        gc_monitor.record_gc_event(gc_event);

        let request = HttpRequest {
            method: "POST".to_string(),
            url: "/api/data".to_string(),
            timestamp: Instant::now(),
            headers_size: 2048,
            body_size: 1024,
            user_agent: None,
            remote_ip: None,
        };
        let request_id = "req2".to_string();
        http_monitor.record_request(request_id.clone(), request);
        
        let response = HttpResponse {
            status_code: 201,
            timestamp: Instant::now(),
            headers_size: 1024,
            body_size: 512,
            response_time: Duration::from_millis(75),
        };
        http_monitor.record_response(request_id, response);

        // Verify activity was recorded
        assert!(cpu_monitor.get_stats().current >= 0.0);
        assert_eq!(gc_monitor.get_stats().total_gc_count, 1);
        assert_eq!(http_monitor.get_stats().total_requests, 1);

        // Reset all monitors
        assert!(cpu_monitor.reset().is_ok());
        assert!(gc_monitor.reset().is_ok());
        assert!(http_monitor.reset().is_ok());

        // Verify reset worked
        assert_eq!(cpu_monitor.get_stats().current, 0.0);
        assert_eq!(gc_monitor.get_stats().total_gc_count, 0);
        assert_eq!(http_monitor.get_stats().total_requests, 0);

        // Stop monitors
        cpu_monitor.stop().unwrap();
        gc_monitor.stop().unwrap();
        http_monitor.stop().unwrap();
    }

    #[test]
    fn test_high_frequency_monitoring() {
        let mut cpu_monitor = CpuMonitor::new();
        let mut libuv_monitor = LibuvMonitor::new();

        cpu_monitor.start().unwrap();
        libuv_monitor.start().unwrap();

        // Simulate high-frequency updates
        for i in 0..100 {
            thread::sleep(Duration::from_millis(1));
            cpu_monitor.update().unwrap();

            // Register and unregister handles rapidly
            let handle_id = libuv_monitor.register_handle(
                if i % 2 == 0 { HandleType::Timer } else { HandleType::Tcp },
                true,
                true
            );
            
            libuv_monitor.record_loop_iteration(
                Duration::from_micros(500 + (i % 10) * 100),
                Duration::from_micros(100),
                Duration::from_micros(50),
                Duration::from_micros(50),
                Duration::from_micros(300)
            );

            if i % 10 == 0 {
                libuv_monitor.unregister_handle(handle_id);
            }
        }

        let cpu_stats = cpu_monitor.get_stats();
        let libuv_stats = libuv_monitor.get_stats();

        assert!(cpu_stats.current >= 0.0);
        assert!(libuv_stats.loop_metrics.loop_count >= 100);
        assert!(libuv_stats.loop_metrics.avg_loop_time > Duration::ZERO);

        cpu_monitor.stop().unwrap();
        libuv_monitor.stop().unwrap();
    }

    #[test]
    fn test_memory_pressure_simulation() {
        let mut memory_monitor = MemoryMonitor::new();
        let mut gc_monitor = GcMonitor::new();

        memory_monitor.start().unwrap();
        gc_monitor.start().unwrap();

        let initial_memory = memory_monitor.get_stats().rss;

        // Simulate memory allocation and GC cycles
        let mut allocations = Vec::new();
        for i in 0..10 {
            // Allocate memory
            let data: Vec<u8> = vec![0; 1024 * 1024]; // 1MB
            allocations.push(data);

            // Simulate GC event
            let gc_event = GcEvent {
                gc_type: if i % 3 == 0 { GcType::MarkSweepCompact } else { GcType::Scavenge },
                duration: Duration::from_millis(5 + i as u64 * 2),
                timestamp: Instant::now(),
                heap_size_before: i as u64 * 1024 * 1024,
                heap_size_after: (i + 1) as u64 * 1024 * 1024,
            };
            gc_monitor.record_gc_event(gc_event);

            thread::sleep(Duration::from_millis(10));
        }

        let final_memory = memory_monitor.get_stats().rss;
        let gc_stats = gc_monitor.get_stats();

        // Memory should have increased
        assert!(final_memory >= initial_memory);
        
        // GC should have occurred
        assert_eq!(gc_stats.total_gc_count, 10);
        assert!(gc_stats.total_gc_time > Duration::ZERO);

        // Clean up
        drop(allocations);
        
        memory_monitor.stop().unwrap();
        gc_monitor.stop().unwrap();
    }

    #[test]
    fn test_http_load_simulation() {
        let mut http_monitor = HttpMonitor::new();
        http_monitor.start().unwrap();

        let mut request_ids = Vec::new();

        // Simulate concurrent HTTP requests
        for i in 0..50 {
            let method = match i % 4 {
                0 => "GET",
                1 => "POST",
                2 => "PUT",
                _ => "DELETE",
            };

            let request = HttpRequest {
                method: method.to_string(),
                url: format!("/api/endpoint/{}", i),
                headers_size: 1024 + (i % 10) as u64 * 100,
                body_size: 512 + (i % 5) as u64 * 50,
                timestamp: Instant::now(),
                user_agent: Some("test-agent".to_string()),
                remote_ip: Some("127.0.0.1".to_string()),
            };
            let request_id = format!("req_{}", i);
            http_monitor.record_request(request_id.clone(), request);
            request_ids.push(request_id);
        }

        // Simulate responses with varying status codes and timing
        for (i, request_id) in request_ids.iter().enumerate() {
            let status_code = if i % 10 == 0 {
                500 // Simulate some errors
            } else if i % 20 == 0 {
                404 // Simulate not found
            } else {
                200 // Success
            };

            let response = HttpResponse {
                status_code,
                headers_size: 2048 + (i % 15) as u64 * 100,
                body_size: 1024 + (i % 8) as u64 * 75,
                response_time: Duration::from_millis(50 + (i % 30) as u64 * 10),
                timestamp: Instant::now(),
            };
            http_monitor.record_response(request_id.clone(), response);
        }

        // Record some error requests
        for i in 0..5 {
            let request = HttpRequest {
                method: "GET".to_string(),
                url: format!("/api/error/{}", i),
                headers_size: 100,
                body_size: 0,
                timestamp: Instant::now(),
                user_agent: Some("test-agent".to_string()),
                remote_ip: Some("127.0.0.1".to_string()),
            };
            let request_id = format!("error_req_{}", i);
            http_monitor.record_request(request_id.clone(), request);
            
            let response = HttpResponse {
                status_code: 500,
                headers_size: 80,
                body_size: 0,
                response_time: Duration::from_millis(200),
                timestamp: Instant::now(),
            };
            http_monitor.record_response(request_id, response);
        }

        let stats = http_monitor.get_stats();
        
        assert_eq!(stats.total_requests, 50);
        // Check error responses (status codes >= 400)
        let error_count: u64 = stats.responses_by_status.iter()
            .filter(|(status, _)| **status >= 400)
            .map(|(_, count)| *count)
            .sum();
        assert_eq!(error_count, 5);
        assert!(stats.avg_response_time > Duration::ZERO);
        assert!(stats.responses_by_status.contains_key(&200));
        assert!(stats.responses_by_status.contains_key(&500));
        assert!(stats.responses_by_status.contains_key(&404));

        http_monitor.stop().unwrap();
    }
}

#[cfg(test)]
mod napi_integration_tests {
    use super::*;

    #[test]
    fn test_global_monitor_initialization() {
        // Test that global monitors can be initialized without conflicts
        use xprofiler_rs::monitoring::cpu::init_cpu_monitor;
        use xprofiler_rs::monitoring::memory::init_memory_monitor;
        use xprofiler_rs::monitoring::gc::init_gc_monitor;
        use xprofiler_rs::monitoring::http::init_http_monitor;
        use xprofiler_rs::monitoring::libuv::init_libuv_monitor;

        // Initialize all global monitors
        assert!(init_cpu_monitor().is_ok());
        assert!(init_memory_monitor().is_ok());
        init_gc_monitor(); // This function returns ()
        assert!(init_http_monitor().is_ok());
        assert!(init_libuv_monitor().is_ok());
    }

    #[test]
    fn test_cross_module_data_consistency() {
        let mut cpu_monitor = CpuMonitor::new();
        let mut memory_monitor = MemoryMonitor::new();

        cpu_monitor.start().unwrap();
        memory_monitor.start().unwrap();

        // Generate some load
        let start_time = std::time::Instant::now();
        while start_time.elapsed() < Duration::from_millis(100) {
            // Busy loop to generate CPU load
            let _: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
        }

        cpu_monitor.update().unwrap();

        let cpu_stats = cpu_monitor.get_stats();
        let memory_stats = memory_monitor.get_stats();

        // Both monitors should show activity
        assert!(cpu_stats.current >= 0.0);
        assert!(memory_stats.rss > 0);
        assert!(memory_stats.heap_used >= 0);

        cpu_monitor.stop().unwrap();
        memory_monitor.stop().unwrap();
    }
}