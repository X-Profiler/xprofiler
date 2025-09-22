//! Performance benchmark tests for xprofiler-rs
//!
//! This module contains benchmarks to measure the performance overhead
//! of the monitoring modules and ensure they meet performance requirements.

use std::time::{Duration, Instant};
use std::thread;
use std::sync::{Arc, Mutex};
use xprofiler_rs::monitoring::Monitor;
use xprofiler_rs::monitoring::cpu::CpuMonitor;
use xprofiler_rs::monitoring::memory::MemoryMonitor;
use xprofiler_rs::monitoring::gc::{GcMonitor, GcType, GcEvent};
use xprofiler_rs::monitoring::http::{HttpMonitor, HttpRequest, HttpResponse};
use xprofiler_rs::monitoring::libuv::{LibuvMonitor, HandleType};

/// Benchmark result structure
#[derive(Debug, Clone)]
struct BenchmarkResult {
    operation: String,
    total_operations: u64,
    total_duration: Duration,
    operations_per_second: f64,
    average_latency: Duration,
    min_latency: Duration,
    max_latency: Duration,
    p95_latency: Duration,
    p99_latency: Duration,
}

impl BenchmarkResult {
    fn new(operation: String, durations: Vec<Duration>) -> Self {
        let total_operations = durations.len() as u64;
        let total_duration: Duration = durations.iter().sum();
        let operations_per_second = if total_duration.as_secs_f64() > 0.0 {
            total_operations as f64 / total_duration.as_secs_f64()
        } else {
            0.0
        };
        
        let average_latency = if total_operations > 0 {
            total_duration / total_operations as u32
        } else {
            Duration::ZERO
        };
        
        let mut sorted_durations = durations.clone();
        sorted_durations.sort();
        
        let min_latency = sorted_durations.first().copied().unwrap_or(Duration::ZERO);
        let max_latency = sorted_durations.last().copied().unwrap_or(Duration::ZERO);
        
        let p95_index = (total_operations as f64 * 0.95) as usize;
        let p99_index = (total_operations as f64 * 0.99) as usize;
        
        let p95_latency = sorted_durations.get(p95_index.min(sorted_durations.len() - 1))
            .copied().unwrap_or(Duration::ZERO);
        let p99_latency = sorted_durations.get(p99_index.min(sorted_durations.len() - 1))
            .copied().unwrap_or(Duration::ZERO);
        
        BenchmarkResult {
            operation,
            total_operations,
            total_duration,
            operations_per_second,
            average_latency,
            min_latency,
            max_latency,
            p95_latency,
            p99_latency,
        }
    }
    
    fn print_summary(&self) {
        println!("\n=== {} Benchmark Results ===", self.operation);
        println!("Total Operations: {}", self.total_operations);
        println!("Total Duration: {:?}", self.total_duration);
        println!("Operations/sec: {:.2}", self.operations_per_second);
        println!("Average Latency: {:?}", self.average_latency);
        println!("Min Latency: {:?}", self.min_latency);
        println!("Max Latency: {:?}", self.max_latency);
        println!("P95 Latency: {:?}", self.p95_latency);
        println!("P99 Latency: {:?}", self.p99_latency);
    }
}

/// Helper function to benchmark a closure
fn benchmark_operation<F>(operation_name: &str, iterations: usize, mut operation: F) -> BenchmarkResult
where
    F: FnMut() -> (),
{
    let mut durations = Vec::with_capacity(iterations);
    
    // Warm up
    for _ in 0..10 {
        operation();
    }
    
    // Actual benchmark
    for _ in 0..iterations {
        let start = Instant::now();
        operation();
        let duration = start.elapsed();
        durations.push(duration);
    }
    
    BenchmarkResult::new(operation_name.to_string(), durations)
}

#[cfg(test)]
mod monitor_performance_tests {
    use super::*;

    #[test]
    fn benchmark_cpu_monitor_update() {
        let mut monitor = CpuMonitor::new();
        monitor.start().unwrap();
        
        let result = benchmark_operation("CPU Monitor Update", 1000, || {
            monitor.update().unwrap();
        });
        
        result.print_summary();
        
        // Performance assertions
        assert!(result.operations_per_second > 1000.0, 
                "CPU monitor update too slow: {:.2} ops/sec", result.operations_per_second);
        assert!(result.p99_latency < Duration::from_millis(10), 
                "CPU monitor P99 latency too high: {:?}", result.p99_latency);
        
        monitor.stop().unwrap();
    }

    #[test]
    fn benchmark_memory_monitor_update() {
        let mut monitor = MemoryMonitor::new();
        monitor.start().unwrap();
        
        let result = benchmark_operation("Memory Monitor Update", 1000, || {
            monitor.update().unwrap();
        });
        
        result.print_summary();
        
        // Performance assertions
        assert!(result.operations_per_second > 500.0, 
                "Memory monitor update too slow: {:.2} ops/sec", result.operations_per_second);
        assert!(result.p99_latency < Duration::from_millis(20), 
                "Memory monitor P99 latency too high: {:?}", result.p99_latency);
        
        monitor.stop().unwrap();
    }

    #[test]
    fn benchmark_gc_monitor_event_recording() {
        let mut monitor = GcMonitor::new();
        monitor.start().unwrap();
        
        let mut event_counter = 0;
        let result = benchmark_operation("GC Event Recording", 10000, || {
            let gc_event = GcEvent {
                gc_type: if event_counter % 2 == 0 { GcType::Scavenge } else { GcType::MarkSweepCompact },
                duration: Duration::from_millis(event_counter % 50),
                timestamp: Instant::now(),
                heap_size_before: (event_counter * 1024) as u64,
                heap_size_after: ((event_counter + 1) * 1024) as u64,
            };
            monitor.record_gc_event(gc_event);
            event_counter += 1;
        });
        
        result.print_summary();
        
        // Performance assertions
        assert!(result.operations_per_second > 10000.0, 
                "GC event recording too slow: {:.2} ops/sec", result.operations_per_second);
        assert!(result.p99_latency < Duration::from_millis(1), 
                "GC event recording P99 latency too high: {:?}", result.p99_latency);
        
        monitor.stop().unwrap();
    }

    #[test]
    fn benchmark_http_monitor_request_recording() {
        let mut monitor = HttpMonitor::new();
        monitor.start().unwrap();
        
        let mut request_counter = 0;
        let result = benchmark_operation("HTTP Request Recording", 5000, || {
            let request_id = format!("bench_req_{}", request_counter);
            
            let request = HttpRequest {
                method: "GET".to_string(),
                url: format!("/api/bench/{}", request_counter),
                timestamp: Instant::now(),
                headers_size: 1024,
                body_size: 0,
                user_agent: Some("benchmark".to_string()),
                remote_ip: Some("127.0.0.1".to_string()),
            };
            
            let response = HttpResponse {
                status_code: 200,
                timestamp: Instant::now(),
                headers_size: 512,
                body_size: 1024,
                response_time: Duration::from_millis(request_counter % 100),
            };
            
            monitor.record_request(request_id.clone(), request);
            monitor.record_response(request_id, response);
            request_counter += 1;
        });
        
        result.print_summary();
        
        // Performance assertions
        assert!(result.operations_per_second > 2000.0, 
                "HTTP request recording too slow: {:.2} ops/sec", result.operations_per_second);
        assert!(result.p99_latency < Duration::from_millis(5), 
                "HTTP request recording P99 latency too high: {:?}", result.p99_latency);
        
        monitor.stop().unwrap();
    }

    #[test]
    fn benchmark_libuv_monitor_loop_recording() {
        let mut monitor = LibuvMonitor::new();
        monitor.start().unwrap();
        
        let mut iteration_counter = 0;
        let result = benchmark_operation("Libuv Loop Recording", 10000, || {
            monitor.record_loop_iteration(
                Duration::from_millis(iteration_counter % 10),
                Duration::from_micros(100),
                Duration::from_micros(50),
                Duration::from_micros(200),
                Duration::from_micros(650),
            );
            iteration_counter += 1;
        });
        
        result.print_summary();
        
        // Performance assertions
        assert!(result.operations_per_second > 50000.0, 
                "Libuv loop recording too slow: {:.2} ops/sec", result.operations_per_second);
        assert!(result.p99_latency < Duration::from_micros(500), 
                "Libuv loop recording P99 latency too high: {:?}", result.p99_latency);
        
        monitor.stop().unwrap();
    }
}

#[cfg(test)]
mod stats_retrieval_performance_tests {
    use super::*;

    #[test]
    fn benchmark_cpu_stats_retrieval() {
        let mut monitor = CpuMonitor::new();
        monitor.start().unwrap();
        
        // Generate some data
        for _ in 0..100 {
            monitor.update().unwrap();
            thread::sleep(Duration::from_millis(1));
        }
        
        let result = benchmark_operation("CPU Stats Retrieval", 1000, || {
            let _ = monitor.get_stats().unwrap();
        });
        
        result.print_summary();
        
        // Performance assertions
        assert!(result.operations_per_second > 10000.0, 
                "CPU stats retrieval too slow: {:.2} ops/sec", result.operations_per_second);
        assert!(result.p99_latency < Duration::from_millis(1), 
                "CPU stats retrieval P99 latency too high: {:?}", result.p99_latency);
        
        monitor.stop().unwrap();
    }

    #[test]
    fn benchmark_memory_stats_retrieval() {
        let mut monitor = MemoryMonitor::new();
        monitor.start().unwrap();
        
        // Generate some data
        for _ in 0..100 {
            monitor.update().unwrap();
            thread::sleep(Duration::from_millis(1));
        }
        
        let result = benchmark_operation("Memory Stats Retrieval", 1000, || {
            let _ = monitor.get_stats().unwrap();
        });
        
        result.print_summary();
        
        // Performance assertions
        assert!(result.operations_per_second > 10000.0, 
                "Memory stats retrieval too slow: {:.2} ops/sec", result.operations_per_second);
        assert!(result.p99_latency < Duration::from_millis(1), 
                "Memory stats retrieval P99 latency too high: {:?}", result.p99_latency);
        
        monitor.stop().unwrap();
    }

    #[test]
    fn benchmark_gc_stats_retrieval() {
        let mut monitor = GcMonitor::new();
        monitor.start().unwrap();
        
        // Generate some data
        for i in 0..1000 {
            let gc_event = GcEvent {
                gc_type: if i % 2 == 0 { GcType::Scavenge } else { GcType::MarkSweepCompact },
                duration: Duration::from_millis(i % 50),
                timestamp: Instant::now(),
                heap_size_before: (i * 1024) as u64,
                heap_size_after: ((i + 1) * 1024) as u64,
            };
            monitor.record_gc_event(gc_event);
        }
        
        let result = benchmark_operation("GC Stats Retrieval", 1000, || {
            let _ = monitor.get_stats().unwrap();
        });
        
        result.print_summary();
        
        // Performance assertions
        assert!(result.operations_per_second > 5000.0, 
                "GC stats retrieval too slow: {:.2} ops/sec", result.operations_per_second);
        assert!(result.p99_latency < Duration::from_millis(2), 
                "GC stats retrieval P99 latency too high: {:?}", result.p99_latency);
        
        monitor.stop().unwrap();
    }

    #[test]
    fn benchmark_http_stats_retrieval() {
        let mut monitor = HttpMonitor::new();
        monitor.start().unwrap();
        
        // Generate some data
        for i in 0..1000 {
            let request_id = format!("stats_req_{}", i);
            let request = HttpRequest {
                method: "GET".to_string(),
                url: format!("/api/stats/{}", i),
                timestamp: Instant::now(),
                headers_size: 1024,
                body_size: 0,
                user_agent: Some("stats-test".to_string()),
                remote_ip: Some("127.0.0.1".to_string()),
            };
            let response = HttpResponse {
                status_code: 200,
                timestamp: Instant::now(),
                headers_size: 512,
                body_size: 1024,
                response_time: Duration::from_millis(i % 100),
            };
            monitor.record_request(request_id.clone(), request);
            monitor.record_response(request_id, response);
        }
        
        let result = benchmark_operation("HTTP Stats Retrieval", 1000, || {
            let _ = monitor.get_stats().unwrap();
        });
        
        result.print_summary();
        
        // Performance assertions
        assert!(result.operations_per_second > 2000.0, 
                "HTTP stats retrieval too slow: {:.2} ops/sec", result.operations_per_second);
        assert!(result.p99_latency < Duration::from_millis(5), 
                "HTTP stats retrieval P99 latency too high: {:?}", result.p99_latency);
        
        monitor.stop().unwrap();
    }

    #[test]
    fn benchmark_libuv_stats_retrieval() {
        let mut monitor = LibuvMonitor::new();
        monitor.start().unwrap();
        
        // Generate some data
        for i in 0..1000 {
            monitor.record_loop_iteration(
                Duration::from_millis(i % 10),
                Duration::from_micros(100),
                Duration::from_micros(50),
                Duration::from_micros(200),
                Duration::from_micros(650),
            );
        }
        
        let result = benchmark_operation("Libuv Stats Retrieval", 1000, || {
            let _ = monitor.get_stats().unwrap();
        });
        
        result.print_summary();
        
        // Performance assertions
        assert!(result.operations_per_second > 5000.0, 
                "Libuv stats retrieval too slow: {:.2} ops/sec", result.operations_per_second);
        assert!(result.p99_latency < Duration::from_millis(2), 
                "Libuv stats retrieval P99 latency too high: {:?}", result.p99_latency);
        
        monitor.stop().unwrap();
    }
}

#[cfg(test)]
mod concurrent_performance_tests {
    use super::*;

    #[test]
    fn benchmark_concurrent_cpu_monitor_access() {
        let monitor = Arc::new(Mutex::new(CpuMonitor::new()));
        monitor.lock().unwrap().start().unwrap();
        
        let num_threads = 4;
        let operations_per_thread = 250;
        
        let start_time = Instant::now();
        
        let handles: Vec<_> = (0..num_threads)
            .map(|_| {
                let monitor = Arc::clone(&monitor);
                thread::spawn(move || {
                    for _ in 0..operations_per_thread {
                        if let Ok(mut mon) = monitor.lock() {
                            let _ = mon.update();
                        }
                    }
                })
            })
            .collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let total_duration = start_time.elapsed();
        let total_operations = num_threads * operations_per_thread;
        let ops_per_second = total_operations as f64 / total_duration.as_secs_f64();
        
        println!("\n=== Concurrent CPU Monitor Benchmark ===");
        println!("Threads: {}", num_threads);
        println!("Operations per thread: {}", operations_per_thread);
        println!("Total operations: {}", total_operations);
        println!("Total duration: {:?}", total_duration);
        println!("Operations/sec: {:.2}", ops_per_second);
        
        // Performance assertion
        assert!(ops_per_second > 500.0, 
                "Concurrent CPU monitor access too slow: {:.2} ops/sec", ops_per_second);
        
        monitor.lock().unwrap().stop().unwrap();
    }

    #[test]
    fn benchmark_mixed_monitor_workload() {
        let cpu_monitor = Arc::new(Mutex::new(CpuMonitor::new()));
        let memory_monitor = Arc::new(Mutex::new(MemoryMonitor::new()));
        let gc_monitor = Arc::new(Mutex::new(GcMonitor::new()));
        let http_monitor = Arc::new(Mutex::new(HttpMonitor::new()));
        
        cpu_monitor.lock().unwrap().start().unwrap();
        memory_monitor.lock().unwrap().start().unwrap();
        gc_monitor.lock().unwrap().start().unwrap();
        http_monitor.lock().unwrap().start().unwrap();
        
        let num_threads = 8;
        let operations_per_thread = 100;
        
        let start_time = Instant::now();
        
        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                let cpu_monitor = Arc::clone(&cpu_monitor);
                let memory_monitor = Arc::clone(&memory_monitor);
                let gc_monitor = Arc::clone(&gc_monitor);
                let http_monitor = Arc::clone(&http_monitor);
                
                thread::spawn(move || {
                    for i in 0..operations_per_thread {
                        match i % 4 {
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
                                        duration: Duration::from_millis(i % 20),
                                        timestamp: Instant::now(),
                                        heap_size_before: (i * 1024) as u64,
                                        heap_size_after: ((i + 1) * 1024) as u64,
                                    };
                                    mon.record_gc_event(gc_event);
                                }
                            }
                            3 => {
                                if let Ok(mut mon) = http_monitor.try_lock() {
                                    let request_id = format!("mixed_{}_{}", thread_id, i);
                                    let request = HttpRequest {
                                        method: "GET".to_string(),
                                        url: format!("/mixed/{}", i),
                                        timestamp: Instant::now(),
                                        headers_size: 1024,
                                        body_size: 0,
                                        user_agent: Some("mixed-test".to_string()),
                                        remote_ip: Some("127.0.0.1".to_string()),
                                    };
                                    let response = HttpResponse {
                                        status_code: 200,
                                        timestamp: Instant::now(),
                                        headers_size: 512,
                                        body_size: 1024,
                                        response_time: Duration::from_millis(i % 50),
                                    };
                                    mon.record_request(request_id.clone(), request);
                                    mon.record_response(request_id, response);
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                })
            })
            .collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let total_duration = start_time.elapsed();
        let total_operations = num_threads * operations_per_thread;
        let ops_per_second = total_operations as f64 / total_duration.as_secs_f64();
        
        println!("\n=== Mixed Monitor Workload Benchmark ===");
        println!("Threads: {}", num_threads);
        println!("Operations per thread: {}", operations_per_thread);
        println!("Total operations: {}", total_operations);
        println!("Total duration: {:?}", total_duration);
        println!("Operations/sec: {:.2}", ops_per_second);
        
        // Performance assertion
        assert!(ops_per_second > 200.0, 
                "Mixed monitor workload too slow: {:.2} ops/sec", ops_per_second);
        
        cpu_monitor.lock().unwrap().stop().unwrap();
        memory_monitor.lock().unwrap().stop().unwrap();
        gc_monitor.lock().unwrap().stop().unwrap();
        http_monitor.lock().unwrap().stop().unwrap();
    }
}

#[cfg(test)]
mod overhead_measurement_tests {
    use super::*;

    #[test]
    fn measure_monitoring_overhead() {
        println!("\n=== Monitoring Overhead Measurement ===");
        
        // Baseline: no monitoring
        let baseline_result = benchmark_operation("Baseline (No Monitoring)", 10000, || {
            // Simulate some work
            let _dummy = (0..100).map(|i| i * i).sum::<i32>();
        });
        
        baseline_result.print_summary();
        
        // With CPU monitoring
        let mut cpu_monitor = CpuMonitor::new();
        cpu_monitor.start().unwrap();
        
        let cpu_monitoring_result = benchmark_operation("With CPU Monitoring", 10000, || {
            // Simulate some work
            let _dummy = (0..100).map(|i| i * i).sum::<i32>();
            // Update monitor
            let _ = cpu_monitor.update();
        });
        
        cpu_monitoring_result.print_summary();
        cpu_monitor.stop().unwrap();
        
        // Calculate overhead
        let overhead_percentage = ((cpu_monitoring_result.average_latency.as_nanos() as f64 - 
                                   baseline_result.average_latency.as_nanos() as f64) / 
                                   baseline_result.average_latency.as_nanos() as f64) * 100.0;
        
        println!("\nCPU Monitoring Overhead: {:.2}%", overhead_percentage);
        
        // Overhead should be less than 50%
        assert!(overhead_percentage < 50.0, 
                "CPU monitoring overhead too high: {:.2}%", overhead_percentage);
    }

    #[test]
    fn measure_all_monitors_overhead() {
        println!("\n=== All Monitors Overhead Measurement ===");
        
        // Baseline: no monitoring
        let baseline_result = benchmark_operation("Baseline (No Monitoring)", 1000, || {
            // Simulate application work
            let _dummy = (0..1000).map(|i| i * i).sum::<i32>();
            thread::sleep(Duration::from_micros(100));
        });
        
        baseline_result.print_summary();
        
        // With all monitors
        let mut cpu_monitor = CpuMonitor::new();
        let mut memory_monitor = MemoryMonitor::new();
        let mut gc_monitor = GcMonitor::new();
        let mut http_monitor = HttpMonitor::new();
        let mut libuv_monitor = LibuvMonitor::new();
        
        cpu_monitor.start().unwrap();
        memory_monitor.start().unwrap();
        gc_monitor.start().unwrap();
        http_monitor.start().unwrap();
        libuv_monitor.start().unwrap();
        
        let mut counter = 0;
        let all_monitors_result = benchmark_operation("With All Monitors", 1000, || {
            // Simulate application work
            let _dummy = (0..1000).map(|i| i * i).sum::<i32>();
            thread::sleep(Duration::from_micros(100));
            
            // Update monitors
            let _ = cpu_monitor.update();
            let _ = memory_monitor.update();
            
            // Record events
            let gc_event = GcEvent {
                gc_type: GcType::Scavenge,
                duration: Duration::from_millis(counter % 10),
                timestamp: Instant::now(),
                heap_size_before: (counter * 1024) as u64,
                heap_size_after: ((counter + 1) * 1024) as u64,
            };
            gc_monitor.record_gc_event(gc_event);
            
            let request_id = format!("overhead_req_{}", counter);
            let request = HttpRequest {
                method: "GET".to_string(),
                url: format!("/overhead/{}", counter),
                timestamp: Instant::now(),
                headers_size: 1024,
                body_size: 0,
                user_agent: Some("overhead-test".to_string()),
                remote_ip: Some("127.0.0.1".to_string()),
            };
            let response = HttpResponse {
                status_code: 200,
                timestamp: Instant::now(),
                headers_size: 512,
                body_size: 1024,
                response_time: Duration::from_millis(counter % 50),
            };
            http_monitor.record_request(request_id.clone(), request);
            http_monitor.record_response(request_id, response);
            
            libuv_monitor.record_loop_iteration(
                Duration::from_millis(counter % 5),
                Duration::from_micros(100),
                Duration::from_micros(50),
                Duration::from_micros(200),
                Duration::from_micros(650),
            );
            
            counter += 1;
        });
        
        all_monitors_result.print_summary();
        
        cpu_monitor.stop().unwrap();
        memory_monitor.stop().unwrap();
        gc_monitor.stop().unwrap();
        http_monitor.stop().unwrap();
        libuv_monitor.stop().unwrap();
        
        // Calculate total overhead
        let total_overhead_percentage = ((all_monitors_result.average_latency.as_nanos() as f64 - 
                                         baseline_result.average_latency.as_nanos() as f64) / 
                                         baseline_result.average_latency.as_nanos() as f64) * 100.0;
        
        println!("\nTotal Monitoring Overhead: {:.2}%", total_overhead_percentage);
        
        // Total overhead should be less than 100% (i.e., monitoring shouldn't double the execution time)
        assert!(total_overhead_percentage < 100.0, 
                "Total monitoring overhead too high: {:.2}%", total_overhead_percentage);
    }
}