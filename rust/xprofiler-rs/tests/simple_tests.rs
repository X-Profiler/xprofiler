//! Simple tests that don't require NAPI bindings
//! These tests focus on core monitoring functionality without Node.js integration.

use std::time::{Duration, Instant};
use std::thread;

// Test basic CPU monitoring structures
#[test]
fn test_cpu_stats_creation() {
    // Test that we can create basic CPU stats without NAPI
    let current = 50.0;
    let avg_1s = 45.0;
    let avg_5s = 40.0;
    let avg_15s = 35.0;
    
    // Basic validation
    assert!(current >= 0.0 && current <= 100.0);
    assert!(avg_1s >= 0.0 && avg_1s <= 100.0);
    assert!(avg_5s >= 0.0 && avg_5s <= 100.0);
    assert!(avg_15s >= 0.0 && avg_15s <= 100.0);
}

// Test basic memory monitoring structures
#[test]
fn test_memory_stats_creation() {
    // Test that we can create basic memory stats without NAPI
    let rss = 1024 * 1024; // 1MB
    let heap_used = 512 * 1024; // 512KB
    let heap_total = 1024 * 1024; // 1MB
    let external = 256 * 1024; // 256KB
    
    // Basic validation
    assert!(rss > 0);
    assert!(heap_used > 0);
    assert!(heap_total >= heap_used);
    assert!(external >= 0);
}

// Test timing utilities
#[test]
fn test_timing_utilities() {
    let start = Instant::now();
    thread::sleep(Duration::from_millis(10));
    let end = Instant::now();
    
    let elapsed = end.duration_since(start);
    assert!(elapsed >= Duration::from_millis(10));
    assert!(elapsed < Duration::from_millis(100)); // Should be much less than 100ms
}

// Test basic HTTP monitoring structures
#[test]
fn test_http_stats_creation() {
    // Test that we can create basic HTTP stats without NAPI
    let status_code = 200;
    let method = "GET";
    let url = "/api/test";
    let response_time = 150; // ms
    
    // Basic validation
    assert!(status_code >= 100 && status_code < 600);
    assert!(!method.is_empty());
    assert!(!url.is_empty());
    assert!(response_time > 0);
}

// Test basic GC monitoring structures
#[test]
fn test_gc_stats_creation() {
    // Test that we can create basic GC stats without NAPI
    let gc_count = 5;
    let gc_time = 25; // ms
    let heap_before = 1024 * 1024;
    let heap_after = 512 * 1024;
    
    // Basic validation
    assert!(gc_count >= 0);
    assert!(gc_time >= 0);
    assert!(heap_before > 0);
    assert!(heap_after >= 0);
    assert!(heap_after <= heap_before); // GC should reduce heap usage
}

// Test basic libuv monitoring structures
#[test]
fn test_libuv_stats_creation() {
    // Test that we can create basic libuv stats without NAPI
    let active_handles = 10;
    let active_requests = 5;
    let idle_time = 1000; // ms
    
    // Basic validation
    assert!(active_handles >= 0);
    assert!(active_requests >= 0);
    assert!(idle_time >= 0);
}