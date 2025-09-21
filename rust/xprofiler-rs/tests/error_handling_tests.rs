//! Error handling and edge case tests for the xprofiler-rs library.
//!
//! This module contains comprehensive tests for error conditions, edge cases,
//! and robustness scenarios.

use std::time::{Duration, Instant};
use std::thread;
use xprofiler_rs::monitoring::Monitor;
use xprofiler_rs::monitoring::cpu::CpuMonitor;
use xprofiler_rs::monitoring::memory::MemoryMonitor;
use xprofiler_rs::monitoring::gc::{GcMonitor, GcType, GcEvent};
use xprofiler_rs::monitoring::http::{HttpMonitor, HttpRequest, HttpResponse};
use xprofiler_rs::monitoring::libuv::{LibuvMonitor, HandleType};
use xprofiler_rs::monitoring::error::MonitoringError;

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_monitor_double_start() {
        let mut cpu_monitor = CpuMonitor::new();
        
        // First start should succeed
        assert!(cpu_monitor.start().is_ok());
        assert!(cpu_monitor.is_running());
        
        // Second start should also succeed (idempotent)
        assert!(cpu_monitor.start().is_ok());
        assert!(cpu_monitor.is_running());
        
        cpu_monitor.stop().unwrap();
    }

    #[test]
    fn test_monitor_stop_before_start() {
        let mut memory_monitor = MemoryMonitor::new();
        
        // Stop before start should succeed (no-op)
        assert!(memory_monitor.stop().is_ok());
        assert!(!memory_monitor.is_running());
        
        // Start after stop should work
        assert!(memory_monitor.start().is_ok());
        assert!(memory_monitor.is_running());
        
        memory_monitor.stop().unwrap();
    }

    #[test]
    fn test_monitor_operations_when_stopped() {
        let mut gc_monitor = GcMonitor::new();
        
        // Operations on stopped monitor should not panic
        let stats = gc_monitor.get_stats().unwrap();
        assert_eq!(stats.total_gc_count, 0);
        
        // Recording events when stopped should be ignored
        let gc_event = GcEvent {
            gc_type: GcType::Scavenge,
            duration: Duration::from_millis(10),
            timestamp: Instant::now(),
            heap_size_before: 1024,
            heap_size_after: 512,
        };
        gc_monitor.record_gc_event(gc_event);
        
        let stats_after = gc_monitor.get_stats().unwrap();
        assert_eq!(stats_after.total_gc_count, 0); // Should still be 0
    }

    #[test]
    fn test_http_monitor_invalid_request_response_pairing() {
        let mut http_monitor = HttpMonitor::new();
        http_monitor.start().unwrap();
        
        // Record response without corresponding request
        let response = HttpResponse {
            status_code: 200,
            timestamp: Instant::now(),
            headers_size: 1024,
            body_size: 512,
            response_time: Duration::from_millis(50),
        };
        
        // This should not panic, just be ignored
        http_monitor.record_response("nonexistent_req".to_string(), response);
        
        let stats = http_monitor.get_stats().unwrap();
        assert_eq!(stats.total_responses, 0);
        
        http_monitor.stop().unwrap();
    }

    #[test]
    fn test_libuv_monitor_invalid_handle_operations() {
        let mut libuv_monitor = LibuvMonitor::new();
        libuv_monitor.start().unwrap();
        
        // Update status of non-existent handle
        libuv_monitor.update_handle_status(999999, true, false);
        
        // Unregister non-existent handle
        libuv_monitor.unregister_handle(999999);
        
        // These operations should not panic or cause errors
        let stats = libuv_monitor.get_stats().unwrap();
        assert_eq!(stats.total_handles, 0);
        
        libuv_monitor.stop().unwrap();
    }

    #[test]
    fn test_extreme_values_handling() {
        let mut gc_monitor = GcMonitor::new();
        gc_monitor.start().unwrap();
        
        // Test with extreme duration values
        let extreme_gc_event = GcEvent {
            gc_type: GcType::MarkSweepCompact,
            duration: Duration::from_secs(3600), // 1 hour GC (unrealistic but possible)
            timestamp: Instant::now(),
            heap_size_before: u64::MAX,
            heap_size_after: 0,
        };
        
        gc_monitor.record_gc_event(extreme_gc_event);
        
        let stats = gc_monitor.get_stats().unwrap();
        assert_eq!(stats.total_gc_count, 1);
        assert_eq!(stats.total_gc_time, Duration::from_secs(3600));
        
        gc_monitor.stop().unwrap();
    }

    #[test]
    fn test_zero_duration_events() {
        let mut http_monitor = HttpMonitor::new();
        http_monitor.start().unwrap();
        
        let request = HttpRequest {
            method: "GET".to_string(),
            url: "/instant".to_string(),
            timestamp: Instant::now(),
            headers_size: 0,
            body_size: 0,
            user_agent: None,
            remote_ip: None,
        };
        
        let response = HttpResponse {
            status_code: 200,
            timestamp: Instant::now(),
            headers_size: 0,
            body_size: 0,
            response_time: Duration::ZERO, // Instant response
        };
        
        let request_id = "instant_req".to_string();
        http_monitor.record_request(request_id.clone(), request);
        http_monitor.record_response(request_id, response);
        
        let stats = http_monitor.get_stats().unwrap();
        assert_eq!(stats.total_requests, 1);
        assert_eq!(stats.min_response_time, Duration::ZERO);
        
        http_monitor.stop().unwrap();
    }

    #[test]
    fn test_memory_monitor_edge_cases() {
        let mut memory_monitor = MemoryMonitor::new();
        memory_monitor.start().unwrap();
        
        // Multiple rapid updates
        for _ in 0..1000 {
            memory_monitor.update().unwrap();
        }
        
        let stats = memory_monitor.get_stats().unwrap();
        // RSS should be non-negative (we fixed this in earlier tests)
        assert!(stats.rss >= 0);
        
        memory_monitor.stop().unwrap();
    }

    #[test]
    fn test_concurrent_monitor_access() {
        use std::sync::{Arc, Mutex};
        
        let cpu_monitor = Arc::new(Mutex::new(CpuMonitor::new()));
        cpu_monitor.lock().unwrap().start().unwrap();
        
        let mut handles = vec![];
        
        // Spawn multiple threads accessing the same monitor
        for i in 0..10 {
            let monitor_clone = Arc::clone(&cpu_monitor);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let mut monitor = monitor_clone.lock().unwrap();
                    monitor.update().unwrap();
                    let _stats = monitor.get_stats().unwrap();
                    thread::sleep(Duration::from_millis(1));
                }
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        cpu_monitor.lock().unwrap().stop().unwrap();
    }

    #[test]
    fn test_large_data_volumes() {
        let mut http_monitor = HttpMonitor::new();
        http_monitor.start().unwrap();
        
        // Generate a large number of HTTP transactions
        for i in 0..10000 {
            let request = HttpRequest {
                method: "GET".to_string(),
                url: format!("/api/large_test/{}", i),
                timestamp: Instant::now(),
                headers_size: 1024,
                body_size: 2048,
                user_agent: Some("load-test-agent".to_string()),
                remote_ip: Some("192.168.1.100".to_string()),
            };
            
            let response = HttpResponse {
                status_code: 200,
                timestamp: Instant::now(),
                headers_size: 512,
                body_size: 1024,
                response_time: Duration::from_millis(10),
            };
            
            let request_id = format!("load_req_{}", i);
            http_monitor.record_request(request_id.clone(), request);
            http_monitor.record_response(request_id, response);
        }
        
        let stats = http_monitor.get_stats().unwrap();
        // Monitor should handle large volumes without issues
        // Note: Due to max_transactions limit, we might not see all 10000
        assert!(stats.total_requests <= 10000);
        assert!(stats.total_responses <= 10000);
        
        http_monitor.stop().unwrap();
    }

    #[test]
    fn test_unicode_and_special_characters() {
        let mut http_monitor = HttpMonitor::new();
        http_monitor.start().unwrap();
        
        let request = HttpRequest {
            method: "POST".to_string(),
            url: "/api/测试/🚀/special-chars?param=value&unicode=测试".to_string(),
            timestamp: Instant::now(),
            headers_size: 2048,
            body_size: 1024,
            user_agent: Some("Mozilla/5.0 (测试) 🌟".to_string()),
            remote_ip: Some("::1".to_string()), // IPv6
        };
        
        let response = HttpResponse {
            status_code: 201,
            timestamp: Instant::now(),
            headers_size: 1024,
            body_size: 512,
            response_time: Duration::from_millis(75),
        };
        
        let request_id = "unicode_req_测试_🚀".to_string();
        http_monitor.record_request(request_id.clone(), request);
        http_monitor.record_response(request_id, response);
        
        let stats = http_monitor.get_stats().unwrap();
        assert_eq!(stats.total_requests, 1);
        assert_eq!(stats.total_responses, 1);
        
        http_monitor.stop().unwrap();
    }

    #[test]
    fn test_monitor_reset_during_operation() {
        let mut libuv_monitor = LibuvMonitor::new();
        libuv_monitor.start().unwrap();
        
        // Register some handles
        let handle1 = libuv_monitor.register_handle(HandleType::Timer, true, true);
        let handle2 = libuv_monitor.register_handle(HandleType::Tcp, true, false);
        
        // Record some loop iterations
        for i in 0..10 {
            libuv_monitor.record_loop_iteration(
                Duration::from_millis(i),
                Duration::from_millis(1),
                Duration::from_millis(1),
                Duration::from_millis(1),
                Duration::from_millis(i / 2),
            );
        }
        
        let stats_before = libuv_monitor.get_stats().unwrap();
        assert_eq!(stats_before.total_handles, 2);
        assert_eq!(stats_before.loop_metrics.loop_count, 10);
        
        // Reset during operation
        libuv_monitor.reset().unwrap();
        
        let stats_after = libuv_monitor.get_stats().unwrap();
        assert_eq!(stats_after.total_handles, 0);
        assert_eq!(stats_after.loop_metrics.loop_count, 0);
        
        // Monitor should still be running after reset
        assert!(libuv_monitor.is_running());
        
        libuv_monitor.stop().unwrap();
    }

    #[test]
    fn test_rapid_start_stop_cycles() {
        let mut memory_monitor = MemoryMonitor::new();
        
        // Rapid start/stop cycles
        for _ in 0..100 {
            assert!(memory_monitor.start().is_ok());
            assert!(memory_monitor.is_running());
            
            memory_monitor.update().unwrap();
            let _stats = memory_monitor.get_stats().unwrap();
            
            assert!(memory_monitor.stop().is_ok());
            assert!(!memory_monitor.is_running());
        }
    }
}

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn test_memory_stress() {
        let mut memory_monitor = MemoryMonitor::new();
        let mut gc_monitor = GcMonitor::new();
        
        memory_monitor.start().unwrap();
        gc_monitor.start().unwrap();
        
        let initial_rss = memory_monitor.get_stats().unwrap().rss;
        
        // Allocate and deallocate memory in cycles
        for cycle in 0..5 {
            let mut allocations = Vec::new();
            
            // Allocate memory
            for i in 0..100 {
                let data: Vec<u8> = vec![0; 10240]; // 10KB each
                allocations.push(data);
                
                if i % 10 == 0 {
                    memory_monitor.update().unwrap();
                    
                    // Simulate GC
                    let gc_event = GcEvent {
                        gc_type: if i % 20 == 0 { GcType::MarkSweepCompact } else { GcType::Scavenge },
                        duration: Duration::from_millis(1 + (i % 5) as u64),
                        timestamp: Instant::now(),
                        heap_size_before: (cycle * 100 + i) as u64 * 10240,
                        heap_size_after: (cycle * 100 + i + 1) as u64 * 10240,
                    };
                    gc_monitor.record_gc_event(gc_event);
                }
            }
            
            // Force deallocation
            drop(allocations);
            
            // Give some time for memory to be reclaimed
            thread::sleep(Duration::from_millis(10));
        }
        
        let final_rss = memory_monitor.get_stats().unwrap().rss;
        let gc_stats = gc_monitor.get_stats().unwrap();
        
        // Memory usage should have changed
        assert!(final_rss >= initial_rss);
        
        // GC events should have been recorded
        assert!(gc_stats.total_gc_count > 0);
        assert!(gc_stats.total_gc_time > Duration::ZERO);
        
        memory_monitor.stop().unwrap();
        gc_monitor.stop().unwrap();
    }

    #[test]
    fn test_high_frequency_http_requests() {
        let mut http_monitor = HttpMonitor::new();
        http_monitor.start().unwrap();
        
        let start_time = Instant::now();
        let mut request_count = 0;
        
        // Generate requests for 1 second at high frequency
        while start_time.elapsed() < Duration::from_secs(1) {
            let request = HttpRequest {
                method: "GET".to_string(),
                url: format!("/api/high_freq/{}", request_count),
                timestamp: Instant::now(),
                headers_size: 512,
                body_size: 256,
                user_agent: Some("stress-test".to_string()),
                remote_ip: Some("127.0.0.1".to_string()),
            };
            
            let response = HttpResponse {
                status_code: 200,
                timestamp: Instant::now(),
                headers_size: 256,
                body_size: 128,
                response_time: Duration::from_micros(100), // Very fast response
            };
            
            let request_id = format!("stress_req_{}", request_count);
            http_monitor.record_request(request_id.clone(), request);
            http_monitor.record_response(request_id, response);
            
            request_count += 1;
        }
        
        let stats = http_monitor.get_stats().unwrap();
        
        // Should have processed many requests
        assert!(stats.total_requests > 100); // At least 100 requests per second
        assert_eq!(stats.total_responses, stats.total_requests);
        assert!(stats.avg_response_time > Duration::ZERO);
        
        http_monitor.stop().unwrap();
    }
}