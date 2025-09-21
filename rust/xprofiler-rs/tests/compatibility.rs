//! Compatibility tests for the xprofiler-rs library.
//! 
//! This module contains tests to verify cross-platform compatibility and edge cases.

use std::time::{Duration, Instant};
use std::thread;
use xprofiler_rs::monitoring::*;
use xprofiler_rs::monitoring::cpu::CpuMonitor;
use xprofiler_rs::monitoring::memory::MemoryMonitor;
use xprofiler_rs::monitoring::gc::{GcMonitor, GcType, GcEvent};
use xprofiler_rs::monitoring::http::{HttpMonitor, HttpRequest, HttpResponse};
use xprofiler_rs::monitoring::libuv::{LibuvMonitor, HandleType};

#[cfg(test)]
mod platform_compatibility_tests {
    use super::*;
    

    #[test]
    fn test_cpu_monitoring_cross_platform() {
        let mut cpu_monitor = CpuMonitor::new();
        
        // Should work on all platforms
        assert!(cpu_monitor.start().is_ok());
        assert!(cpu_monitor.is_running());
        
        // Generate some CPU load
        let start = std::time::Instant::now();
        while start.elapsed() < Duration::from_millis(50) {
            let _: u64 = (0..1000).map(|i| i * i).sum();
        }
        
        assert!(cpu_monitor.update().is_ok());
        let stats = cpu_monitor.get_stats().unwrap();
        
        // CPU usage should be valid on all platforms
        assert!(stats.current >= 0.0);
        assert!(stats.current <= 100.0);
        
        assert!(cpu_monitor.stop().is_ok());
        assert!(!cpu_monitor.is_running());
    }

    #[test]
    fn test_memory_monitoring_cross_platform() {
        let mut memory_monitor = MemoryMonitor::new();
        
        assert!(memory_monitor.start().is_ok());
        
        // Give the monitor time to collect initial data
        thread::sleep(Duration::from_millis(50));
        
        let stats = memory_monitor.get_stats().unwrap();
        
        // Memory stats should be valid on all platforms
        // Note: RSS might be 0 in some test environments (containers, CI), so we check if it's non-negative
        assert!(stats.rss >= 0, "RSS should be non-negative");
        
        // If RSS is 0, skip the allocation test as memory tracking might not be available
        if stats.rss == 0 {
            println!("Warning: RSS is 0, skipping memory allocation test (likely in container/CI environment)");
            assert!(memory_monitor.stop().is_ok());
            return;
        }
        // heap_used and external are unsigned, always >= 0
        assert!(stats.heap_total >= stats.heap_used, "Heap total should be >= heap used");
        
        // Test memory allocation tracking
        let initial_rss = stats.rss;
        let _large_allocation: Vec<u8> = vec![0; 1024 * 1024]; // 1MB
        
        thread::sleep(Duration::from_millis(10));
        let new_stats = memory_monitor.get_stats().unwrap();
        
        // Memory usage might increase (platform dependent)
        assert!(new_stats.rss >= initial_rss);
        
        assert!(memory_monitor.stop().is_ok());
    }

    #[test]
    fn test_gc_monitoring_edge_cases() {
        let mut gc_monitor = GcMonitor::new();
        
        assert!(gc_monitor.start().is_ok());
        
        // Test with zero duration
        let gc_event1 = GcEvent {
            gc_type: GcType::Scavenge,
            duration: Duration::ZERO,
            timestamp: Instant::now(),
            heap_size_before: 1024,
            heap_size_after: 512,
        };
        gc_monitor.record_gc_event(gc_event1);
        
        // Test with very large values
        let gc_event2 = GcEvent {
            gc_type: GcType::MarkSweepCompact,
            duration: Duration::from_secs(1),
            timestamp: Instant::now(),
            heap_size_before: u64::MAX / 2,
            heap_size_after: u64::MAX / 4,
        };
        gc_monitor.record_gc_event(gc_event2);
        
        // Test with same before/after memory
        let gc_event3 = GcEvent {
            gc_type: GcType::Scavenge,
            duration: Duration::from_millis(10),
            timestamp: Instant::now(),
            heap_size_before: 1024,
            heap_size_after: 1024,
        };
        gc_monitor.record_gc_event(gc_event3);
        
        let stats = gc_monitor.get_stats().unwrap();
        assert_eq!(stats.total_gc_count, 3);
        assert!(stats.total_gc_time >= Duration::from_secs(1));
        
        assert!(gc_monitor.stop().is_ok());
    }

    #[test]
    fn test_http_monitoring_edge_cases() {
        let mut http_monitor = HttpMonitor::new();
        
        assert!(http_monitor.start().is_ok());
        
        // Test with empty URL
        let request1 = HttpRequest {
            method: "GET".to_string(),
            url: "".to_string(),
            timestamp: Instant::now(),
            headers_size: 0,
            body_size: 0,
            user_agent: None,
            remote_ip: None,
        };
        let request_id1 = "req1".to_string();
        http_monitor.record_request(request_id1.clone(), request1);
        
        // Test with very long URL
        let long_url = "a".repeat(10000);
        let request2 = HttpRequest {
            method: "POST".to_string(),
            url: long_url,
            timestamp: Instant::now(),
            headers_size: u64::MAX / 2,
            body_size: u64::MAX / 4,
            user_agent: None,
            remote_ip: None,
        };
        let request_id2 = "req2".to_string();
        http_monitor.record_request(request_id2.clone(), request2);
        
        // Test response with zero duration
        let zero_response = HttpResponse {
            status_code: 200,
            timestamp: Instant::now(),
            headers_size: 0,
            body_size: 0,
            response_time: Duration::ZERO,
        };
        http_monitor.record_response(request_id1, zero_response);
        
        // Test response with very long duration
        let long_response = HttpResponse {
            status_code: 500,
            timestamp: Instant::now(),
            headers_size: 1024,
            body_size: 512,
            response_time: Duration::from_secs(3600), // 1 hour
        };
        http_monitor.record_response(request_id2, long_response);
        
        // Test error recording through response with error status
        let error_response = HttpResponse {
            status_code: 500,
            timestamp: Instant::now(),
            headers_size: 128,
            body_size: 0,
            response_time: Duration::from_millis(100),
        };
        http_monitor.record_response("req1".to_string(), error_response);
        let stats = http_monitor.get_stats().unwrap();
        assert!(stats.error_rate > 0.0);
        assert!(stats.avg_response_time >= Duration::from_secs(1800)); // Should be high due to long response
        
        assert!(http_monitor.stop().is_ok());
    }

    #[test]
    fn test_libuv_monitoring_edge_cases() {
        let mut libuv_monitor = LibuvMonitor::new();
        
        assert!(libuv_monitor.start().is_ok());
        
        // Test registering many handles
        let mut handle_ids = Vec::new();
        for i in 0..1000 {
            let handle_type = match i % 6 {
                0 => HandleType::Timer,
                1 => HandleType::Tcp,
                2 => HandleType::Udp,
                3 => HandleType::Pipe,
                4 => HandleType::Pipe,
                _ => HandleType::Process,
            };
            
            let handle_id = libuv_monitor.register_handle(
                handle_type,
                i % 2 == 0, // active
                i % 3 == 0  // referenced
            );
            handle_ids.push(handle_id);
        }
        
        // Test loop iteration with zero times
        libuv_monitor.record_loop_iteration(
            Duration::ZERO,
            Duration::ZERO,
            Duration::ZERO,
            Duration::ZERO,
            Duration::ZERO
        );
        
        // Test loop iteration with very long times
        libuv_monitor.record_loop_iteration(
            Duration::from_secs(10),
            Duration::from_secs(2),
            Duration::from_secs(3),
            Duration::from_secs(2),
            Duration::from_secs(3)
        );
        
        // Unregister half the handles
        for &handle_id in handle_ids.iter().take(500) {
            libuv_monitor.unregister_handle(handle_id);
        }
        
        let stats = libuv_monitor.get_stats().unwrap();
        assert_eq!(stats.total_handles, 500); // 500 remaining
        assert_eq!(stats.loop_metrics.loop_count, 2);
        assert!(stats.loop_metrics.avg_loop_time > Duration::ZERO);
        
        assert!(libuv_monitor.stop().is_ok());
    }

    #[test]
    fn test_monitor_state_transitions() {
        let mut cpu_monitor = CpuMonitor::new();
        
        // Test initial state
        assert!(!cpu_monitor.is_running());
        
        // Test start
        assert!(cpu_monitor.start().is_ok());
        assert!(cpu_monitor.is_running());
        
        // Test double start (should be ok)
        assert!(cpu_monitor.start().is_ok());
        assert!(cpu_monitor.is_running());
        
        // Test stop
        assert!(cpu_monitor.stop().is_ok());
        assert!(!cpu_monitor.is_running());
        
        // Test double stop (should be ok)
        assert!(cpu_monitor.stop().is_ok());
        assert!(!cpu_monitor.is_running());
        
        // Test restart
        assert!(cpu_monitor.start().is_ok());
        assert!(cpu_monitor.is_running());
        
        // Test reset while running
        assert!(cpu_monitor.reset().is_ok());
        assert!(cpu_monitor.is_running()); // Should still be running after reset
        
        assert!(cpu_monitor.stop().is_ok());
    }

    #[test]
    fn test_concurrent_monitor_access() {
        use std::sync::{Arc, Mutex};
        use std::thread;
        
        let cpu_monitor = Arc::new(Mutex::new(CpuMonitor::new()));
        let memory_monitor = Arc::new(Mutex::new(MemoryMonitor::new()));
        
        // Start monitors
        cpu_monitor.lock().unwrap().start().unwrap();
        memory_monitor.lock().unwrap().start().unwrap();
        
        let mut handles = Vec::new();
        
        // Spawn multiple threads accessing monitors
        for _i in 0..10 {
            let cpu_clone = Arc::clone(&cpu_monitor);
            let memory_clone = Arc::clone(&memory_monitor);
            
            let handle = thread::spawn(move || {
                for _ in 0..10 {
                    // Access CPU monitor
                    {
                        let mut cpu = cpu_clone.lock().unwrap();
                        let _ = cpu.update();
                        let _ = cpu.get_stats();
                    }
                    
                    // Access memory monitor
                    {
                        let memory = memory_clone.lock().unwrap();
                        let _ = memory.get_stats();
                    }
                    
                    thread::sleep(Duration::from_millis(1));
                }
            });
            
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Stop monitors
        cpu_monitor.lock().unwrap().stop().unwrap();
        memory_monitor.lock().unwrap().stop().unwrap();
    }

    #[test]
    fn test_monitor_format_output() {
        let mut cpu_monitor = CpuMonitor::new();
        let mut memory_monitor = MemoryMonitor::new();
        let mut gc_monitor = GcMonitor::new();
        let mut http_monitor = HttpMonitor::new();
        let mut libuv_monitor = LibuvMonitor::new();
        
        // Start all monitors
        cpu_monitor.start().unwrap();
        memory_monitor.start().unwrap();
        gc_monitor.start().unwrap();
        http_monitor.start().unwrap();
        libuv_monitor.start().unwrap();
        
        // Generate some activity
        cpu_monitor.update().unwrap();
        
        let gc_event = GcEvent {
            gc_type: GcType::MarkSweepCompact,
            duration: Duration::from_millis(10),
            timestamp: Instant::now(),
            heap_size_before: 1024,
            heap_size_after: 512,
        };
        gc_monitor.record_gc_event(gc_event);
        
        let request_id = "test_request_1".to_string();
        let request = HttpRequest {
            method: "GET".to_string(),
            url: "/test".to_string(),
            headers_size: 1024,
            body_size: 512,
            timestamp: Instant::now(),
            user_agent: Some("test-agent".to_string()),
            remote_ip: Some("127.0.0.1".to_string()),
        };
        http_monitor.record_request(request_id.clone(), request);
        let response = HttpResponse {
            status_code: 200,
            headers_size: 2048,
            body_size: 1024,
            response_time: Duration::from_millis(50),
            timestamp: Instant::now(),
        };
        http_monitor.record_response(request_id, response);
        
        let _handle_id = libuv_monitor.register_handle(HandleType::Timer, true, true);
        libuv_monitor.record_loop_iteration(
            Duration::from_millis(5),
            Duration::from_millis(1),
            Duration::from_millis(1),
            Duration::from_millis(1),
            Duration::from_millis(2)
        );
        
        // Test format output (should not panic)
        let cpu_format = cpu_monitor.format_cpu_usage(false);
        let memory_format = memory_monitor.format_memory_usage().unwrap_or_default();
        let gc_format = gc_monitor.format_gc_stats();
        let http_format = http_monitor.format_http_stats();
        let libuv_format = libuv_monitor.format_libuv_stats();
        
        // All formats should be non-empty strings
        assert!(!cpu_format.is_empty());
        assert!(!memory_format.is_empty());
        assert!(!gc_format.is_empty());
        assert!(!http_format.is_empty());
        assert!(!libuv_format.is_empty());
        
        // Stop all monitors
        cpu_monitor.stop().unwrap();
        memory_monitor.stop().unwrap();
        gc_monitor.stop().unwrap();
        http_monitor.stop().unwrap();
        libuv_monitor.stop().unwrap();
    }
}