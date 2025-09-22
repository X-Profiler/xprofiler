//! Comprehensive integration tests for xprofiler-rs
//!
//! This module contains end-to-end integration tests that simulate real-world usage scenarios
//! and verify the complete functionality of the monitoring system.

use std::time::{Duration, Instant};
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use xprofiler_rs::monitoring::Monitor;
use xprofiler_rs::monitoring::cpu::CpuMonitor;
use xprofiler_rs::monitoring::memory::MemoryMonitor;
use xprofiler_rs::monitoring::gc::{GcMonitor, GcType, GcEvent};
use xprofiler_rs::monitoring::http::{HttpMonitor, HttpRequest, HttpResponse};
use xprofiler_rs::monitoring::libuv::{LibuvMonitor, HandleType};
use xprofiler_rs::monitoring::TimePeriod;

#[cfg(test)]
mod end_to_end_tests {
    use super::*;

    #[test]
    fn test_complete_application_lifecycle() {
        // Simulate a complete Node.js application lifecycle
        let mut cpu_monitor = CpuMonitor::new();
        let mut memory_monitor = MemoryMonitor::new();
        let mut gc_monitor = GcMonitor::new();
        let mut http_monitor = HttpMonitor::new();
        let mut libuv_monitor = LibuvMonitor::new();

        // Application startup
        assert!(cpu_monitor.start().is_ok());
        assert!(memory_monitor.start().is_ok());
        assert!(gc_monitor.start().is_ok());
        assert!(http_monitor.start().is_ok());
        assert!(libuv_monitor.start().is_ok());

        // Simulate application initialization
        let timer_handle = libuv_monitor.register_handle(HandleType::Timer, true, true);
        let tcp_handle = libuv_monitor.register_handle(HandleType::Tcp, true, true);
        let fs_handle = libuv_monitor.register_handle(HandleType::FsEvent, true, false);

        // Simulate initial memory allocation
        let mut app_data = Vec::new();
        for i in 0..10 {
            app_data.push(vec![0u8; 1024 * 100]); // 100KB allocations
            
            // Update memory monitor to capture allocation
            memory_monitor.update().unwrap();
            
            let gc_event = GcEvent {
                gc_type: GcType::Scavenge,
                duration: Duration::from_millis(2 + i),
                timestamp: Instant::now(),
                heap_size_before: (i * 100 * 1024) as u64,
                heap_size_after: ((i + 1) * 100 * 1024) as u64,
            };
            gc_monitor.record_gc_event(gc_event);
        }

        // Simulate HTTP server handling requests
        let mut request_count = 0;
        for batch in 0..5 {
            for req_in_batch in 0..20 {
                request_count += 1;
                let request_id = format!("req_{}_{}", batch, req_in_batch);
                
                let request = HttpRequest {
                    method: match req_in_batch % 4 {
                        0 => "GET".to_string(),
                        1 => "POST".to_string(),
                        2 => "PUT".to_string(),
                        _ => "DELETE".to_string(),
                    },
                    url: format!("/api/v1/resource/{}", req_in_batch),
                    timestamp: Instant::now(),
                    headers_size: 1024 + (req_in_batch % 10) as u64 * 100,
                    body_size: if req_in_batch % 4 == 1 { 2048 } else { 0 }, // POST requests have body
                    user_agent: Some("Node.js/18.0.0".to_string()),
                    remote_ip: Some(format!("192.168.1.{}", 100 + req_in_batch % 50)),
                };
                
                http_monitor.record_request(request_id.clone(), request);
                
                // Simulate processing time
                thread::sleep(Duration::from_millis(1 + req_in_batch as u64 % 5));
                
                let response = HttpResponse {
                    status_code: if req_in_batch % 20 == 0 { 500 } else if req_in_batch % 15 == 0 { 404 } else { 200 },
                    timestamp: Instant::now(),
                    headers_size: 512,
                    body_size: 1024 + (req_in_batch % 8) as u64 * 256,
                    response_time: Duration::from_millis(10 + req_in_batch as u64 % 50),
                };
                
                http_monitor.record_response(request_id, response);
                
                // Simulate event loop activity
                libuv_monitor.record_loop_iteration(
                    Duration::from_millis(1 + req_in_batch as u64 % 3),
                    Duration::from_micros(100),
                    Duration::from_micros(50),
                    Duration::from_micros(200),
                    Duration::from_micros(650 + req_in_batch as u64 % 100),
                );
            }
            
            // Update monitors periodically
            cpu_monitor.update().unwrap();
            memory_monitor.update().unwrap();
            
            // Simulate periodic GC
            let gc_event = GcEvent {
                gc_type: GcType::MarkSweepCompact,
                duration: Duration::from_millis(15 + batch * 5),
                timestamp: Instant::now(),
                heap_size_before: (10 + batch) as u64 * 100 * 1024,
                heap_size_after: (8 + batch) as u64 * 100 * 1024,
            };
            gc_monitor.record_gc_event(gc_event);
            
            thread::sleep(Duration::from_millis(50));
        }

        // Verify comprehensive stats
        let cpu_stats = cpu_monitor.get_stats().unwrap();
        let memory_stats = memory_monitor.get_stats().unwrap();
        let gc_stats = gc_monitor.get_stats().unwrap();
        let http_stats = http_monitor.get_stats().unwrap();
        let libuv_stats = libuv_monitor.get_stats().unwrap();

        // Validate CPU monitoring
        assert!(cpu_stats.current >= 0.0);
        assert!(cpu_stats.avg_15s >= 0.0);

        // Validate memory monitoring
        assert!(memory_stats.rss > 0);
        assert!(memory_stats.heap_used >= 0);

        // Validate GC monitoring
        assert_eq!(gc_stats.total_gc_count, 15); // 10 scavenge + 5 mark-sweep-compact
        assert!(gc_stats.total_gc_time > Duration::ZERO);
        assert!(!gc_stats.gc_avg_durations.is_empty());

        // Validate HTTP monitoring
        assert_eq!(http_stats.total_requests, 100);
        assert!(http_stats.avg_response_time > Duration::ZERO);
        assert!(http_stats.responses_by_status.contains_key(&200));
        assert!(http_stats.responses_by_status.contains_key(&404));
        assert!(http_stats.responses_by_status.contains_key(&500));
        
        // Calculate error rate
        let total_responses: u64 = http_stats.responses_by_status.values().sum();
        let error_responses: u64 = http_stats.responses_by_status.iter()
            .filter(|(status, _)| **status >= 400)
            .map(|(_, count)| *count)
            .sum();
        let error_rate = error_responses as f64 / total_responses as f64;
        assert!(error_rate > 0.0 && error_rate < 0.2); // Should have some errors but not too many

        // Validate libuv monitoring
        assert_eq!(libuv_stats.total_handles, 3);
        assert!(libuv_stats.loop_metrics.loop_count >= 100);
        assert!(libuv_stats.loop_metrics.avg_loop_time > Duration::ZERO);

        // Application shutdown
        libuv_monitor.unregister_handle(timer_handle);
        libuv_monitor.unregister_handle(tcp_handle);
        libuv_monitor.unregister_handle(fs_handle);

        // Stop all monitors
        assert!(cpu_monitor.stop().is_ok());
        assert!(memory_monitor.stop().is_ok());
        assert!(gc_monitor.stop().is_ok());
        assert!(http_monitor.stop().is_ok());
        assert!(libuv_monitor.stop().is_ok());

        // Clean up
        drop(app_data);
    }

    #[test]
    fn test_concurrent_monitoring_threads() {
        let cpu_monitor = Arc::new(Mutex::new(CpuMonitor::new()));
        let memory_monitor = Arc::new(Mutex::new(MemoryMonitor::new()));
        let http_monitor = Arc::new(Mutex::new(HttpMonitor::new()));

        // Start monitors
        cpu_monitor.lock().unwrap().start().unwrap();
        memory_monitor.lock().unwrap().start().unwrap();
        http_monitor.lock().unwrap().start().unwrap();

        let mut handles = vec![];

        // Spawn multiple threads to simulate concurrent access
        for thread_id in 0..4 {
            let cpu_clone = Arc::clone(&cpu_monitor);
            let memory_clone = Arc::clone(&memory_monitor);
            let http_clone = Arc::clone(&http_monitor);

            let handle = thread::spawn(move || {
                for i in 0..25 {
                    // CPU monitoring
                    if let Ok(mut cpu) = cpu_clone.try_lock() {
                        cpu.update().unwrap();
                    }

                    // Memory monitoring
                    if let Ok(mut memory) = memory_clone.try_lock() {
                        memory.update().unwrap();
                    }

                    // HTTP monitoring
                    if let Ok(mut http) = http_clone.try_lock() {
                        let request_id = format!("thread_{}_req_{}", thread_id, i);
                        let request = HttpRequest {
                            method: "GET".to_string(),
                            url: format!("/thread/{}/request/{}", thread_id, i),
                            timestamp: Instant::now(),
                            headers_size: 1024,
                            body_size: 0,
                            user_agent: Some(format!("Thread-{}", thread_id)),
                            remote_ip: Some("127.0.0.1".to_string()),
                        };
                        http.record_request(request_id.clone(), request);

                        let response = HttpResponse {
                            status_code: 200,
                            timestamp: Instant::now(),
                            headers_size: 512,
                            body_size: 2048,
                            response_time: Duration::from_millis(10 + i as u64),
                        };
                        http.record_response(request_id, response);
                    }

                    thread::sleep(Duration::from_millis(2));
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify results
        let cpu_stats = cpu_monitor.lock().unwrap().get_stats().unwrap();
        let memory_stats = memory_monitor.lock().unwrap().get_stats().unwrap();
        let http_stats = http_monitor.lock().unwrap().get_stats().unwrap();

        assert!(cpu_stats.current >= 0.0);
        assert!(memory_stats.rss > 0);
        // Allow for some timing variations in concurrent execution
        assert!(http_stats.total_requests >= 95 && http_stats.total_requests <= 100, 
            "Expected 95-100 requests, got {}", http_stats.total_requests);

        // Stop monitors
        cpu_monitor.lock().unwrap().stop().unwrap();
        memory_monitor.lock().unwrap().stop().unwrap();
        http_monitor.lock().unwrap().stop().unwrap();
    }

    #[test]
    fn test_performance_regression_detection() {
        let mut http_monitor = HttpMonitor::new();
        let mut libuv_monitor = LibuvMonitor::new();

        http_monitor.start().unwrap();
        libuv_monitor.start().unwrap();

        let mut baseline_times = Vec::new();
        let mut current_times = Vec::new();

        // Establish baseline performance
        for i in 0..50 {
            let start = Instant::now();
            
            // Simulate work
            let _: Vec<u8> = (0..1000).map(|x| (x % 256) as u8).collect();
            
            let elapsed = start.elapsed();
            baseline_times.push(elapsed);

            let request_id = format!("baseline_{}", i);
            let request = HttpRequest {
                method: "GET".to_string(),
                url: "/api/baseline".to_string(),
                timestamp: Instant::now(),
                headers_size: 1024,
                body_size: 0,
                user_agent: None,
                remote_ip: None,
            };
            http_monitor.record_request(request_id.clone(), request);

            let response = HttpResponse {
                status_code: 200,
                timestamp: Instant::now(),
                headers_size: 512,
                body_size: 1024,
                response_time: elapsed,
            };
            http_monitor.record_response(request_id, response);

            // Record libuv loop iteration with safe duration values
            libuv_monitor.record_loop_iteration(
                elapsed,
                Duration::from_micros(100),
                Duration::from_micros(50),
                Duration::from_micros(200),
                Duration::from_micros(50), // Use fixed safe value instead of subtraction
            );
        }

        // Simulate current performance (with slight degradation)
        for i in 0..50 {
            let start = Instant::now();
            
            // Simulate slightly more work (performance regression)
            let _: Vec<u8> = (0..1200).map(|x| (x % 256) as u8).collect();
            
            let elapsed = start.elapsed();
            current_times.push(elapsed);

            let request_id = format!("current_{}", i);
            let request = HttpRequest {
                method: "GET".to_string(),
                url: "/api/current".to_string(),
                timestamp: Instant::now(),
                headers_size: 1024,
                body_size: 0,
                user_agent: None,
                remote_ip: None,
            };
            http_monitor.record_request(request_id.clone(), request);

            let response = HttpResponse {
                status_code: 200,
                timestamp: Instant::now(),
                headers_size: 512,
                body_size: 1024,
                response_time: elapsed,
            };
            http_monitor.record_response(request_id, response);
        }

        // Analyze performance
        let baseline_avg: Duration = baseline_times.iter().sum::<Duration>() / baseline_times.len() as u32;
        let current_avg: Duration = current_times.iter().sum::<Duration>() / current_times.len() as u32;

        let http_stats_map = http_monitor.get_stats().unwrap();
        let libuv_stats_map = libuv_monitor.get_stats().unwrap();
        let http_stats = http_stats_map.get(&TimePeriod::TenSeconds).unwrap();
        let libuv_stats = libuv_stats_map.get(&TimePeriod::TenSeconds).unwrap();

        // Verify monitoring captured the performance data
        assert_eq!(http_stats.total_requests, 100);
        assert!(http_stats.avg_response_time > 0.0);
        assert_eq!(libuv_stats.loop_metrics.loop_count, 50);
        assert!(libuv_stats.loop_metrics.avg_loop_time > Duration::ZERO);

        // Performance regression detection (current should be slower than baseline)
        let performance_diff = if current_avg > baseline_avg {
            current_avg - baseline_avg
        } else {
            baseline_avg - current_avg
        };
        assert!(performance_diff.as_millis() < 200, "Performance regression detected");
        assert!(current_avg > baseline_avg, 
            "Expected performance regression: current ({:?}) should be slower than baseline ({:?})", 
            current_avg, baseline_avg);

        http_monitor.stop().unwrap();
        libuv_monitor.stop().unwrap();
    }

    #[test]
    fn test_memory_leak_detection() {
        let mut memory_monitor = MemoryMonitor::new();
        let mut gc_monitor = GcMonitor::new();

        memory_monitor.start().unwrap();
        gc_monitor.start().unwrap();

        let initial_memory_map = memory_monitor.get_stats().unwrap();
        let initial_memory = initial_memory_map.get(&TimePeriod::TenSeconds).unwrap().rss;
        let mut allocations = Vec::new();
        let mut memory_samples = Vec::new();

        // Simulate gradual memory increase
        for i in 0..20 {
            // Allocate memory that won't be freed
            let data: Vec<u8> = vec![i as u8; 1024 * 512]; // 512KB
            allocations.push(data);

            // Update memory stats
            memory_monitor.update().unwrap();
            let current_memory_map = memory_monitor.get_stats().unwrap();
            let current_memory = current_memory_map.get(&TimePeriod::TenSeconds).unwrap().rss;
            memory_samples.push(current_memory);

            // Simulate GC that doesn't reclaim the leaked memory
            let gc_event = GcEvent {
                gc_type: if i % 5 == 0 { GcType::MarkSweepCompact } else { GcType::Scavenge },
                duration: Duration::from_millis(5 + i as u64),
                timestamp: Instant::now(),
                heap_size_before: current_memory,
                heap_size_after: current_memory, // No memory reclaimed (leak)
            };
            gc_monitor.record_gc_event(gc_event);

            thread::sleep(Duration::from_millis(10));
        }

        let final_memory_map = memory_monitor.get_stats().unwrap();
        let final_memory = final_memory_map.get(&TimePeriod::TenSeconds).unwrap().rss;
        let gc_stats_map = gc_monitor.get_stats().unwrap();
        let gc_stats = gc_stats_map.get(&TimePeriod::TenSeconds).unwrap();

        // Verify memory leak detection
        assert!(final_memory > initial_memory, "Memory should have increased");
        
        // Check that memory consistently increased
        let mut increasing_trend = 0;
        for i in 1..memory_samples.len() {
            if memory_samples[i] >= memory_samples[i-1] {
                increasing_trend += 1;
            }
        }
        
        // At least 70% of samples should show increasing memory
        assert!(increasing_trend as f64 / (memory_samples.len() - 1) as f64 > 0.7, 
            "Expected increasing memory trend, got {}/{} increasing samples", 
            increasing_trend, memory_samples.len() - 1);

        // Verify GC occurred but didn't help with the leak
        assert_eq!(gc_stats.total_gc_count, 20);
        assert!(gc_stats.total_gc_time > Duration::ZERO);

        memory_monitor.stop().unwrap();
        gc_monitor.stop().unwrap();

        // Clean up
        drop(allocations);
    }

    #[test]
    fn test_real_world_web_server_simulation() {
        let mut cpu_monitor = CpuMonitor::new();
        let mut memory_monitor = MemoryMonitor::new();
        let mut http_monitor = HttpMonitor::new();
        let mut libuv_monitor = LibuvMonitor::new();

        // Start all monitors
        cpu_monitor.start().unwrap();
        memory_monitor.start().unwrap();
        http_monitor.start().unwrap();
        libuv_monitor.start().unwrap();

        // Simulate server startup
        let server_handle = libuv_monitor.register_handle(HandleType::Tcp, true, true);
        let timer_handle = libuv_monitor.register_handle(HandleType::Timer, true, true);

        // Simulate different types of endpoints
        let endpoints = vec![
            ("/api/users", "GET", 0, 1024),      // Fast endpoint
            ("/api/search", "GET", 0, 4096),     // Slow endpoint
            ("/api/upload", "POST", 10240, 512), // Upload endpoint
            ("/api/data", "GET", 0, 2048),       // Medium endpoint
        ];

        let mut total_requests = 0;
        let mut request_times = HashMap::new();

        // Simulate traffic patterns
        for hour in 0..3 { // Simulate 3 hours
            let requests_this_hour = match hour {
                0 => 50,  // Low traffic
                1 => 150, // Peak traffic
                2 => 75,  // Medium traffic
                _ => 50,
            };

            for req in 0..requests_this_hour {
                total_requests += 1;
                let endpoint_idx = req % endpoints.len();
                let (path, method, body_size, response_size) = &endpoints[endpoint_idx];

                let request_id = format!("req_{}_{}", hour, req);
                let start_time = Instant::now();

                // Record request
                let request = HttpRequest {
                    method: method.to_string(),
                    url: path.to_string(),
                    timestamp: start_time,
                    headers_size: 1024,
                    body_size: *body_size,
                    user_agent: Some("Mozilla/5.0 (compatible)".to_string()),
                    remote_ip: Some(format!("192.168.1.{}", 100 + req % 50)),
                };
                http_monitor.record_request(request_id.clone(), request);

                // Simulate processing time based on endpoint
                let processing_time = match endpoint_idx {
                    0 => Duration::from_millis(10 + req as u64 % 20),  // Fast
                    1 => Duration::from_millis(100 + req as u64 % 200), // Slow
                    2 => Duration::from_millis(50 + req as u64 % 100),  // Upload
                    3 => Duration::from_millis(30 + req as u64 % 60),   // Medium
                    _ => Duration::from_millis(50),
                };
                thread::sleep(Duration::from_millis(1)); // Minimal actual sleep

                // Record response
                let status_code = if req % 50 == 0 { 500 } else if req % 25 == 0 { 404 } else { 200 };
                let response = HttpResponse {
                    status_code,
                    timestamp: Instant::now(),
                    headers_size: 512,
                    body_size: *response_size,
                    response_time: processing_time,
                };
                http_monitor.record_response(request_id, response);

                request_times.insert(endpoint_idx, processing_time);

                // Record libuv activity
                libuv_monitor.record_loop_iteration(
                    processing_time / 10, // Event loop time is fraction of processing time
                    Duration::from_micros(100),
                    Duration::from_micros(50),
                    Duration::from_micros(200),
                    processing_time / 10 - Duration::from_micros(350),
                );

                // Update monitors periodically
                if req % 20 == 0 {
                    cpu_monitor.update().unwrap();
                    memory_monitor.update().unwrap();
                }
            }

            // Simulate hourly GC
            let _gc_event = GcEvent {
                gc_type: GcType::MarkSweepCompact,
                duration: Duration::from_millis(20 + hour * 10),
                timestamp: Instant::now(),
                heap_size_before: (100 + hour * 50) as u64 * 1024 * 1024,
                heap_size_after: (80 + hour * 40) as u64 * 1024 * 1024,
            };
            // gc_monitor.record_gc_event(_gc_event);
        }

        // Verify comprehensive monitoring results
        let cpu_stats_map = cpu_monitor.get_stats().unwrap();
        let memory_stats_map = memory_monitor.get_stats().unwrap();
        let http_stats_map = http_monitor.get_stats().unwrap();
        let libuv_stats_map = libuv_monitor.get_stats().unwrap();
        let cpu_stats = cpu_stats_map.get(&TimePeriod::TenSeconds).unwrap();
        let memory_stats = memory_stats_map.get(&TimePeriod::TenSeconds).unwrap();
        let http_stats = http_stats_map.get(&TimePeriod::TenSeconds).unwrap();
        let libuv_stats = libuv_stats_map.get(&TimePeriod::TenSeconds).unwrap();

        // Validate HTTP monitoring
        assert_eq!(http_stats.total_requests, total_requests as u64);
        assert!(http_stats.avg_response_time > 0.0);
        
        // Check status code distribution
        let success_responses = http_stats.responses_by_status.get(&200).unwrap_or(&0);
        let error_responses = http_stats.responses_by_status.get(&500).unwrap_or(&0);
        let not_found_responses = http_stats.responses_by_status.get(&404).unwrap_or(&0);
        
        assert!(*success_responses > 0);
        assert!(*error_responses > 0);
        assert!(*not_found_responses > 0);

        // Validate system monitoring
        assert!(cpu_stats.current >= 0.0);
        assert!(memory_stats.rss > 0);
        assert!(libuv_stats.loop_metrics.loop_count > 0);
        assert_eq!(libuv_stats.total_handles, 2); // server + timer

        // Cleanup
        libuv_monitor.unregister_handle(server_handle);
        libuv_monitor.unregister_handle(timer_handle);

        cpu_monitor.stop().unwrap();
        memory_monitor.stop().unwrap();
        http_monitor.stop().unwrap();
        libuv_monitor.stop().unwrap();
    }
}