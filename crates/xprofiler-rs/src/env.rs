//! Environment data management for xprofiler-rs
//!
//! This module provides per-thread/isolate profiling state management,
//! similar to the C++ EnvironmentData and EnvironmentRegistry.

use napi::bindgen_prelude::*;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::sync::Arc;

/// GC statistics for an environment
#[derive(Debug, Default)]
pub struct GcStatistics {
    pub total_gc_times: AtomicU64,
    pub total_gc_duration: AtomicU64,
    pub total_scavenge_duration: AtomicU64,
    pub total_marksweep_duration: AtomicU64,
    pub total_incremental_marking_duration: AtomicU64,
    pub gc_time_during_last_record: AtomicU64,
    pub scavenge_duration_last_record: AtomicU64,
    pub marksweep_duration_last_record: AtomicU64,
    pub incremental_marking_duration_last_record: AtomicU64,
}

impl GcStatistics {
    pub fn reset_last_record(&self) {
        self.gc_time_during_last_record.store(0, Ordering::SeqCst);
        self.scavenge_duration_last_record.store(0, Ordering::SeqCst);
        self.marksweep_duration_last_record.store(0, Ordering::SeqCst);
        self.incremental_marking_duration_last_record
            .store(0, Ordering::SeqCst);
    }
}

/// HTTP statistics for an environment
#[derive(Debug, Default)]
pub struct HttpStatistics {
    pub live_http_request: AtomicI64,
    pub http_response_close: AtomicU64,
    pub http_response_sent: AtomicU64,
    pub http_request_timeout: AtomicU64,
    pub http_rt: AtomicU64,
    pub http_rt_count: AtomicU64,
    status_codes: RwLock<HashMap<u16, u64>>,
    profiling_details: RwLock<Vec<String>>,
}

impl HttpStatistics {
    pub fn add_live_request(&self, _request_id: &str) {
        self.live_http_request.fetch_add(1, Ordering::SeqCst);
    }

    pub fn add_close_request(&self, _request_id: &str, rt: f64) {
        self.live_http_request.fetch_sub(1, Ordering::SeqCst);
        self.http_response_close.fetch_add(1, Ordering::SeqCst);
        self.http_rt.fetch_add(rt as u64, Ordering::SeqCst);
        self.http_rt_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn add_sent_request(&self, _request_id: &str, rt: f64) {
        self.http_response_sent.fetch_add(1, Ordering::SeqCst);
        self.http_rt.fetch_add(rt as u64, Ordering::SeqCst);
        self.http_rt_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn add_request_timeout(&self, _request_id: &str) {
        self.live_http_request.fetch_sub(1, Ordering::SeqCst);
        self.http_request_timeout.fetch_add(1, Ordering::SeqCst);
    }

    pub fn add_status_code(&self, status_code: u16) {
        let mut codes = self.status_codes.write();
        *codes.entry(status_code).or_insert(0) += 1;
    }

    pub fn add_profiling_detail(&self, detail: &str) {
        let mut details = self.profiling_details.write();
        details.push(detail.to_string());
    }

    pub fn get_status_codes(&self) -> HashMap<u16, u64> {
        self.status_codes.read().clone()
    }

    pub fn get_profiling_details(&self) -> Vec<String> {
        self.profiling_details.read().clone()
    }

    pub fn reset_last_record(&self) {
        self.http_response_close.store(0, Ordering::SeqCst);
        self.http_response_sent.store(0, Ordering::SeqCst);
        self.http_request_timeout.store(0, Ordering::SeqCst);
        self.http_rt.store(0, Ordering::SeqCst);
        self.http_rt_count.store(0, Ordering::SeqCst);
        self.status_codes.write().clear();
        self.profiling_details.write().clear();
    }
}

/// Memory statistics for an environment
#[derive(Debug, Default)]
pub struct MemoryStatistics {
    pub rss: AtomicU64,
    pub heap_used: AtomicU64,
    pub heap_available: AtomicU64,
    pub heap_total: AtomicU64,
    pub heap_limit: AtomicU64,
    pub heap_executable: AtomicU64,
    pub total_physical_size: AtomicU64,
    pub malloced_memory: AtomicU64,
    pub external_memory: AtomicU64,
}

/// libuv handle statistics for an environment
#[derive(Debug, Default)]
pub struct UvHandleStatistics {
    pub active_handles: AtomicU64,
    pub active_file_handles: AtomicU64,
    pub active_tcp_handles: AtomicU64,
    pub active_udp_handles: AtomicU64,
    pub active_timer_handles: AtomicU64,
}

/// Per-thread/isolate environment data
pub struct EnvironmentData {
    pub thread_id: i64,
    pub is_main_thread: bool,
    pub node_version: String,
    pub gc_stats: GcStatistics,
    pub http_stats: HttpStatistics,
    pub memory_stats: MemoryStatistics,
    pub uv_stats: UvHandleStatistics,
}

impl EnvironmentData {
    pub fn new(thread_id: i64, is_main_thread: bool, node_version: &str) -> Self {
        Self {
            thread_id,
            is_main_thread,
            node_version: node_version.to_string(),
            gc_stats: GcStatistics::default(),
            http_stats: HttpStatistics::default(),
            memory_stats: MemoryStatistics::default(),
            uv_stats: UvHandleStatistics::default(),
        }
    }
}

/// Global environment registry
pub struct EnvironmentRegistry {
    environments: RwLock<HashMap<i64, Arc<EnvironmentData>>>,
}

impl EnvironmentRegistry {
    pub fn new() -> Self {
        Self {
            environments: RwLock::new(HashMap::new()),
        }
    }

    pub fn global() -> &'static Self {
        static REGISTRY: Lazy<EnvironmentRegistry> = Lazy::new(EnvironmentRegistry::new);
        &REGISTRY
    }

    pub fn register(&self, data: EnvironmentData) {
        let thread_id = data.thread_id;
        self.environments.write().insert(thread_id, Arc::new(data));
    }

    pub fn unregister(&self, thread_id: i64) {
        self.environments.write().remove(&thread_id);
    }

    pub fn get(&self, thread_id: i64) -> Option<Arc<EnvironmentData>> {
        self.environments.read().get(&thread_id).cloned()
    }

    pub fn get_main_thread(&self) -> Option<Arc<EnvironmentData>> {
        self.environments
            .read()
            .values()
            .find(|e| e.is_main_thread)
            .cloned()
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&EnvironmentData),
    {
        for env in self.environments.read().values() {
            f(env);
        }
    }

    pub fn count(&self) -> usize {
        self.environments.read().len()
    }
}

impl Default for EnvironmentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Thread-local storage for current thread's environment
thread_local! {
    static CURRENT_THREAD_ID: std::cell::Cell<Option<i64>> = const { std::cell::Cell::new(None) };
}

/// Setup environment data for the current thread
pub fn setup_environment(thread_id: i64, is_main_thread: bool, node_version: &str) -> Result<()> {
    let data = EnvironmentData::new(thread_id, is_main_thread, node_version);
    EnvironmentRegistry::global().register(data);

    CURRENT_THREAD_ID.with(|id| {
        id.set(Some(thread_id));
    });

    Ok(())
}

/// Get the current thread's environment data
pub fn get_current_env() -> Option<Arc<EnvironmentData>> {
    CURRENT_THREAD_ID.with(|id| id.get().and_then(|tid| EnvironmentRegistry::global().get(tid)))
}

/// Execute a closure with the current environment data
pub fn with_current_env<F, R>(f: F) -> Result<R>
where
    F: FnOnce(&EnvironmentData) -> Result<R>,
{
    match get_current_env() {
        Some(env) => f(&env),
        None => Err(Error::from_reason("Environment not initialized")),
    }
}

/// Check if current thread is the main thread
pub fn is_main_thread() -> bool {
    get_current_env().map_or(false, |e| e.is_main_thread)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_registry() {
        let registry = EnvironmentRegistry::new();

        let data = EnvironmentData::new(1, true, "v18.0.0");
        registry.register(data);

        assert_eq!(registry.count(), 1);

        let env = registry.get(1).unwrap();
        assert!(env.is_main_thread);
        assert_eq!(env.node_version, "v18.0.0");

        registry.unregister(1);
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_http_statistics() {
        let stats = HttpStatistics::default();

        stats.add_live_request("req1");
        assert_eq!(stats.live_http_request.load(Ordering::SeqCst), 1);

        stats.add_close_request("req1", 100.0);
        assert_eq!(stats.live_http_request.load(Ordering::SeqCst), 0);
        assert_eq!(stats.http_response_close.load(Ordering::SeqCst), 1);

        stats.add_status_code(200);
        stats.add_status_code(200);
        stats.add_status_code(404);

        let codes = stats.get_status_codes();
        assert_eq!(codes.get(&200), Some(&2));
        assert_eq!(codes.get(&404), Some(&1));
    }
}
