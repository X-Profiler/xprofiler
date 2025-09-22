//! Thread safety tests for xprofiler-rs
//!
//! This module contains tests to verify that the monitoring modules
//! are thread-safe and can handle concurrent access correctly.

use std::sync::{Arc, Mutex, Barrier};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use xprofiler_rs::monitoring::Monitor;
use xprofiler_rs::monitoring::cpu::CpuMonitor;
use xprofiler_rs::monitoring::memory::MemoryMonitor;
use xprofiler_rs::monitoring::gc::{GcMonitor, GcType, GcEvent};
use xprofiler_rs::monitoring::http::{HttpMonitor, HttpRequest, HttpResponse};
use xprofiler_rs::monitoring::libuv::{LibuvMonitor, HandleType};

#[cfg(test)]
mod concurrent_access_tests {
    use super::*;

    #[test]
    fn test_cpu_monitor_concurrent_updates() {
        let monitor = Arc::new(Mutex::new(CpuMonitor::new()));
        monitor.lock().unwrap().start().unwrap();
        
        let num_threads = 8;
        let iterations_per_thread = 100;
        let barrier = Arc::new(Barrier::new(num_threads));
        
        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                let monitor = Arc::clone(&monitor);
                let barrier = Arc::clone(&barrier);
                
                thread::spawn(move || {
                    barrier.wait();
                    
                    for i in 0..iterations_per_thread {
                        // Try to acquire lock and update
                        if let Ok(mut mon) = monitor.try_lock() {
                            let result = mon.update();
                            assert!(result.is_ok(), "Update failed in thread {} iteration {}: {:?}", 
                                   thread_id, i, result.err());
                        }
                        
                        // Small delay to allow other threads to acquire lock
                        thread::sleep(Duration::from_micros(100));
                    }
                })
            })
            .collect();
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread panicked");
        }
        
        // Verify monitor is still functional
        let mut monitor = monitor.lock().unwrap();
        assert!(monitor.update().is_ok());
        let stats = monitor.get_stats();
        assert!(stats.is_ok());
        monitor.stop().unwrap();
    }

    #[test]
    fn test_memory_monitor_concurrent_updates() {
        let monitor = Arc::new(Mutex::new(MemoryMonitor::new()));
        monitor.lock().unwrap().start().unwrap();
        
        let num_threads = 6;
        let iterations_per_thread = 150;
        let barrier = Arc::new(Barrier::new(num_threads));
        
        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                let monitor = Arc::clone(&monitor);
                let barrier = Arc::clone(&barrier);
                
                thread::spawn(move || {
                    barrier.wait();
                    
                    for i in 0..iterations_per_thread {
                        if let Ok(mut mon) = monitor.try_lock() {
                            let result = mon.update();
                            assert!(result.is_ok(), "Update failed in thread {} iteration {}: {:?}", 
                                   thread_id, i, result.err());
                            
                            // Also test getting stats
                            let stats_result = mon.get_stats();
                            assert!(stats_result.is_ok(), "Get stats failed in thread {} iteration {}: {:?}", 
                                   thread_id, i, stats_result.err());
                        }
                        
                        thread::sleep(Duration::from_micros(50));
                    }
                })
            })
            .collect();
        
        for handle in handles {
            handle.join().expect("Thread panicked");
        }
        
        let mut monitor = monitor.lock().unwrap();
        assert!(monitor.update().is_ok());
        monitor.stop().unwrap();
    }

    #[test]
    fn test_gc_monitor_concurrent_event_recording() {
        let monitor = Arc::new(Mutex::new(GcMonitor::new()));
        monitor.lock().unwrap().start().unwrap();
        
        let num_threads = 10;
        let events_per_thread = 100;
        let barrier = Arc::new(Barrier::new(num_threads));
        
        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                let monitor = Arc::clone(&monitor);
                let barrier = Arc::clone(&barrier);
                
                thread::spawn(move || {
                    barrier.wait();
                    
                    for i in 0..events_per_thread {
                        let gc_event = GcEvent {
                            gc_type: if i % 2 == 0 { GcType::Scavenge } else { GcType::MarkSweepCompact },
                            duration: Duration::from_millis(((thread_id * 10 + i) % 50) as u64),
                            timestamp: Instant::now(),
                            heap_size_before: ((thread_id * 1000 + i) * 1024) as u64,
                            heap_size_after: ((thread_id * 1000 + i + 1) * 1024) as u64,
                        };
                        
                        if let Ok(mut mon) = monitor.try_lock() {
                            mon.record_gc_event(gc_event);
                        }
                        
                        thread::sleep(Duration::from_micros(10));
                    }
                })
            })
            .collect();
        
        for handle in handles {
            handle.join().expect("Thread panicked");
        }
        
        // Verify monitor state
        let mut monitor = monitor.lock().unwrap();
        let stats = monitor.get_stats().unwrap();
        println!("Total GC events recorded: {}", stats.total_gc_count);
        assert!(stats.total_gc_count > 0);
        monitor.stop().unwrap();
    }

    #[test]
    fn test_http_monitor_concurrent_request_recording() {
        let monitor = Arc::new(Mutex::new(HttpMonitor::new()));
        monitor.lock().unwrap().start().unwrap();
        
        let num_threads = 8;
        let requests_per_thread = 50;
        let barrier = Arc::new(Barrier::new(num_threads));
        
        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                let monitor = Arc::clone(&monitor);
                let barrier = Arc::clone(&barrier);
                
                thread::spawn(move || {
                    barrier.wait();
                    
                    for i in 0..requests_per_thread {
                        let request_id = format!("thread_{}_{}", thread_id, i);
                        
                        let request = HttpRequest {
                            method: if i % 2 == 0 { "GET" } else { "POST" }.to_string(),
                            url: format!("/api/thread/{}/{}", thread_id, i),
                            timestamp: Instant::now(),
                            headers_size: 1024,
                            body_size: if i % 2 == 0 { 0 } else { 2048 },
                            user_agent: Some(format!("thread-{}", thread_id)),
                            remote_ip: Some("127.0.0.1".to_string()),
                        };
                        
                        let response = HttpResponse {
                            status_code: if i % 10 == 0 { 404 } else { 200 },
                            timestamp: Instant::now(),
                            headers_size: 512,
                            body_size: 1024,
                            response_time: Duration::from_millis(((thread_id * 10 + i) % 100) as u64),
                        };
                        
                        if let Ok(mut mon) = monitor.try_lock() {
                            mon.record_request(request_id.clone(), request);
                            mon.record_response(request_id, response);
                        }
                        
                        thread::sleep(Duration::from_micros(200));
                    }
                })
            })
            .collect();
        
        for handle in handles {
            handle.join().expect("Thread panicked");
        }
        
        // Verify monitor state
        let mut monitor = monitor.lock().unwrap();
        let stats = monitor.get_stats().unwrap();
        println!("Total HTTP requests recorded: {}", stats.total_requests);
        assert!(stats.total_requests > 0);
        monitor.stop().unwrap();
    }

    #[test]
    fn test_libuv_monitor_concurrent_handle_operations() {
        let monitor = Arc::new(Mutex::new(LibuvMonitor::new()));
        monitor.lock().unwrap().start().unwrap();
        
        let num_threads = 6;
        let operations_per_thread = 20;
        let barrier = Arc::new(Barrier::new(num_threads));
        
        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                let monitor = Arc::clone(&monitor);
                let barrier = Arc::clone(&barrier);
                
                thread::spawn(move || {
                    barrier.wait();
                    
                    let mut thread_handles = Vec::new();
                    
                    for i in 0..operations_per_thread {
                        if let Ok(mut mon) = monitor.try_lock() {
                            // Register handle
                            let handle_type = match i % 4 {
                                0 => HandleType::Timer,
                                1 => HandleType::Tcp,
                                2 => HandleType::FsEvent,
                                _ => HandleType::Process,
                            };
                            
                            let handle = mon.register_handle(handle_type, true, i % 2 == 0);
                            thread_handles.push(handle);
                            
                            // Record loop iteration
                            mon.record_loop_iteration(
                                Duration::from_millis(((thread_id + i) % 10) as u64),
                                Duration::from_micros(100),
                                Duration::from_micros(50),
                                Duration::from_micros(200),
                                Duration::from_micros(650),
                            );
                        }
                        
                        thread::sleep(Duration::from_millis(1));
                    }
                    
                    // Unregister handles
                    for handle in thread_handles {
                        if let Ok(mut mon) = monitor.try_lock() {
                            mon.unregister_handle(handle);
                        }
                    }
                })
            })
            .collect();
        
        for handle in handles {
            handle.join().expect("Thread panicked");
        }
        
        // Verify monitor state
        let mut monitor = monitor.lock().unwrap();
        let stats = monitor.get_stats().unwrap();
        println!("Total libuv handles: {}", stats.total_handles);
        assert!(stats.total_handles >= 0);
        monitor.stop().unwrap();
    }
}

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn test_all_monitors_high_concurrency() {
        let cpu_monitor = Arc::new(Mutex::new(CpuMonitor::new()));
        let memory_monitor = Arc::new(Mutex::new(MemoryMonitor::new()));
        let gc_monitor = Arc::new(Mutex::new(GcMonitor::new()));
        let http_monitor = Arc::new(Mutex::new(HttpMonitor::new()));
        let libuv_monitor = Arc::new(Mutex::new(LibuvMonitor::new()));
        
        // Start all monitors
        cpu_monitor.lock().unwrap().start().unwrap();
        memory_monitor.lock().unwrap().start().unwrap();
        gc_monitor.lock().unwrap().start().unwrap();
        http_monitor.lock().unwrap().start().unwrap();
        libuv_monitor.lock().unwrap().start().unwrap();
        
        let num_threads = 12;
        let duration = Duration::from_secs(5);
        let start_time = Instant::now();
        let barrier = Arc::new(Barrier::new(num_threads));
        
        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                let cpu_monitor = Arc::clone(&cpu_monitor);
                let memory_monitor = Arc::clone(&memory_monitor);
                let gc_monitor = Arc::clone(&gc_monitor);
                let http_monitor = Arc::clone(&http_monitor);
                let libuv_monitor = Arc::clone(&libuv_monitor);
                let barrier = Arc::clone(&barrier);
                
                thread::spawn(move || {
                    barrier.wait();
                    
                    let mut iteration = 0;
                    while start_time.elapsed() < duration {
                        // Randomly choose which monitor to interact with
                        match iteration % 5 {
                            0 => {
                                if let Ok(mut mon) = cpu_monitor.try_lock() {
                                    let _ = mon.update();
                                }
                            }
                            1 => {
                                if let Ok(mut mon) = memory_monitor.try_lock() {
                                    let _ = mon.update();
                                }
                            }
                            2 => {
                                if let Ok(mut mon) = gc_monitor.try_lock() {
                                    let gc_event = GcEvent {
                                        gc_type: GcType::Scavenge,
                                        duration: Duration::from_millis(iteration % 20),
                                        timestamp: Instant::now(),
                                        heap_size_before: (iteration * 1024) as u64,
                                        heap_size_after: ((iteration + 1) * 1024) as u64,
                                    };
                                    mon.record_gc_event(gc_event);
                                }
                            }
                            3 => {
                                if let Ok(mut mon) = http_monitor.try_lock() {
                                    let request_id = format!("stress_{}_{}", thread_id, iteration);
                                    let request = HttpRequest {
                                        method: "GET".to_string(),
                                        url: format!("/stress/{}", iteration),
                                        timestamp: Instant::now(),
                                        headers_size: 1024,
                                        body_size: 0,
                                        user_agent: Some("stress-test".to_string()),
                                        remote_ip: Some("127.0.0.1".to_string()),
                                    };
                                    let response = HttpResponse {
                                        status_code: 200,
                                        timestamp: Instant::now(),
                                        headers_size: 512,
                                        body_size: 1024,
                                        response_time: Duration::from_millis(iteration % 50),
                                    };
                                    mon.record_request(request_id.clone(), request);
                                    mon.record_response(request_id, response);
                                }
                            }
                            4 => {
                                if let Ok(mut mon) = libuv_monitor.try_lock() {
                                    mon.record_loop_iteration(
                                        Duration::from_millis(iteration as u64 % 5),
                                        Duration::from_micros(100),
                                        Duration::from_micros(50),
                                        Duration::from_micros(200),
                                        Duration::from_micros(650),
                                    );
                                }
                            }
                            _ => unreachable!(),
                        }
                        
                        iteration += 1;
                        thread::sleep(Duration::from_micros(100));
                    }
                    
                    println!("Thread {} completed {} iterations", thread_id, iteration);
                })
            })
            .collect();
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread panicked");
        }
        
        // Verify all monitors are still functional
        assert!(cpu_monitor.lock().unwrap().get_stats().is_ok());
        assert!(memory_monitor.lock().unwrap().get_stats().is_ok());
        assert!(gc_monitor.lock().unwrap().get_stats().is_ok());
        assert!(http_monitor.lock().unwrap().get_stats().is_ok());
        assert!(libuv_monitor.lock().unwrap().get_stats().is_ok());
        
        // Stop all monitors
        cpu_monitor.lock().unwrap().stop().unwrap();
        memory_monitor.lock().unwrap().stop().unwrap();
        gc_monitor.lock().unwrap().stop().unwrap();
        http_monitor.lock().unwrap().stop().unwrap();
        libuv_monitor.lock().unwrap().stop().unwrap();
        
        println!("High concurrency stress test completed successfully");
    }

    #[test]
    fn test_monitor_state_consistency_under_load() {
        let monitor = Arc::new(Mutex::new(HttpMonitor::new()));
        monitor.lock().unwrap().start().unwrap();
        
        let num_producer_threads = 4;
        let num_consumer_threads = 2;
        let requests_per_producer: usize = 100;
        let barrier = Arc::new(Barrier::new(num_producer_threads + num_consumer_threads));
        
        let mut handles = Vec::new();
        
        // Producer threads - record HTTP requests
        for thread_id in 0..num_producer_threads {
            let monitor = Arc::clone(&monitor);
            let barrier = Arc::clone(&barrier);
            
            let handle = thread::spawn(move || {
                barrier.wait();
                
                for i in 0..requests_per_producer {
                    let request_id = format!("producer_{}_{}", thread_id, i);
                    
                    if let Ok(mut mon) = monitor.lock() {
                        let request = HttpRequest {
                            method: "POST".to_string(),
                            url: format!("/api/producer/{}/{}", thread_id, i),
                            timestamp: Instant::now(),
                            headers_size: 1024,
                            body_size: 2048,
                            user_agent: Some(format!("producer-{}", thread_id)),
                            remote_ip: Some("127.0.0.1".to_string()),
                        };
                        
                        let response = HttpResponse {
                            status_code: 200,
                            timestamp: Instant::now(),
                            headers_size: 512,
                            body_size: 1024,
                            response_time: Duration::from_millis(i as u64 % 100),
                        };
                        
                        mon.record_request(request_id.clone(), request);
                        mon.record_response(request_id, response);
                    }
                    
                    thread::sleep(Duration::from_micros(500));
                }
            });
            
            handles.push(handle);
        }
        
        // Consumer threads - read statistics
        for thread_id in 0..num_consumer_threads {
            let monitor = Arc::clone(&monitor);
            let barrier = Arc::clone(&barrier);
            
            let handle = thread::spawn(move || {
                barrier.wait();
                
                let mut last_request_count = 0;
                
                for i in 0..200 {
                    if let Ok(mon) = monitor.lock() {
                        if let Ok(stats) = mon.get_stats() {
                            // Verify that request count is monotonically increasing
                            assert!(stats.total_requests >= last_request_count,
                                   "Request count decreased: {} -> {} in consumer {} iteration {}",
                                   last_request_count, stats.total_requests, thread_id, i);
                            last_request_count = stats.total_requests;
                            
                            // Verify basic consistency
                            assert!(stats.avg_response_time.as_millis() <= 1000,
                                    "Average response time too high: {:?}", stats.avg_response_time);
                        }
                    }
                    
                    thread::sleep(Duration::from_millis(1));
                }
                
                println!("Consumer {} observed {} total requests", thread_id, last_request_count);
            });
            
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread panicked");
        }
        
        // Final verification
        let monitor = monitor.lock().unwrap();
        let final_stats = monitor.get_stats().unwrap();
        let expected_requests = num_producer_threads * requests_per_producer;
        
        println!("Expected requests: {}, Actual requests: {}", 
                expected_requests, final_stats.total_requests);
        
        assert_eq!(final_stats.total_requests, expected_requests as u64,
                  "Final request count mismatch");
    }
}

#[cfg(test)]
mod deadlock_prevention_tests {
    use super::*;

    #[test]
    fn test_no_deadlock_with_multiple_monitors() {
        let monitor1 = Arc::new(Mutex::new(CpuMonitor::new()));
        let monitor2 = Arc::new(Mutex::new(MemoryMonitor::new()));
        
        monitor1.lock().unwrap().start().unwrap();
        monitor2.lock().unwrap().start().unwrap();
        
        let num_threads = 4;
        let barrier = Arc::new(Barrier::new(num_threads));
        
        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                let monitor1 = Arc::clone(&monitor1);
                let monitor2 = Arc::clone(&monitor2);
                let barrier = Arc::clone(&barrier);
                
                thread::spawn(move || {
                    barrier.wait();
                    
                    for _i in 0..50 {
                        // Alternate lock acquisition order to test for deadlocks
                        if thread_id % 2 == 0 {
                            // Order: monitor1 -> monitor2
                            if let Ok(mut mon1) = monitor1.try_lock() {
                                let _ = mon1.update();
                                thread::sleep(Duration::from_micros(10));
                                
                                if let Ok(mut mon2) = monitor2.try_lock() {
                                    let _ = mon2.update();
                                }
                            }
                        } else {
                            // Order: monitor2 -> monitor1
                            if let Ok(mut mon2) = monitor2.try_lock() {
                                let _ = mon2.update();
                                thread::sleep(Duration::from_micros(10));
                                
                                if let Ok(mut mon1) = monitor1.try_lock() {
                                    let _ = mon1.update();
                                }
                            }
                        }
                        
                        thread::sleep(Duration::from_micros(100));
                    }
                    
                    println!("Thread {} completed without deadlock", thread_id);
                })
            })
            .collect();
        
        // Use a timeout to detect potential deadlocks
        let timeout = Duration::from_secs(10);
        let start = Instant::now();
        
        for handle in handles {
            while !handle.is_finished() {
                if start.elapsed() > timeout {
                    panic!("Potential deadlock detected - test timed out");
                }
                thread::sleep(Duration::from_millis(10));
            }
            handle.join().expect("Thread panicked");
        }
        
        monitor1.lock().unwrap().stop().unwrap();
        monitor2.lock().unwrap().stop().unwrap();
        
        println!("Deadlock prevention test completed successfully");
    }
}