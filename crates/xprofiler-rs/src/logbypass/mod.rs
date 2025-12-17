//! LogByPass - Statistics collection and logging thread
//!
//! This module implements a background thread that periodically collects
//! performance statistics and writes them to log files.

pub mod cpu;
pub mod writer;

use crate::config;
use crate::env::EnvironmentRegistry;
use crate::platform;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::mpsc;

/// Global LogByPass instance
static LOG_BYPASS: Lazy<Mutex<Option<LogByPass>>> = Lazy::new(|| Mutex::new(None));

/// LogByPass thread state
struct LogByPass {
    shutdown_tx: Option<mpsc::Sender<()>>,
    thread_handle: Option<std::thread::JoinHandle<()>>,
    is_running: AtomicBool,
}

impl LogByPass {
    fn new() -> Self {
        Self {
            shutdown_tx: None,
            thread_handle: None,
            is_running: AtomicBool::new(false),
        }
    }
}

/// Start the LogByPass thread
pub fn start_log_bypass() -> Result<(), String> {
    let mut guard = LOG_BYPASS.lock();

    if guard.is_some() {
        return Err("LogByPass already started".to_string());
    }

    let cfg = config::get_config();
    let log_interval = cfg.log_interval as u64;

    let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);

    let mut log_bypass = LogByPass::new();
    log_bypass.shutdown_tx = Some(shutdown_tx);
    log_bypass.is_running.store(true, Ordering::SeqCst);

    let thread_handle = std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to create tokio runtime for LogByPass");

        rt.block_on(async {
            // CPU sampling interval (1 second)
            let mut cpu_interval = tokio::time::interval(Duration::from_secs(1));
            // Log writing interval (configurable, default 60 seconds)
            let mut log_interval_timer = tokio::time::interval(Duration::from_secs(log_interval));

            // Skip the first immediate tick
            cpu_interval.tick().await;
            log_interval_timer.tick().await;

            loop {
                tokio::select! {
                    _ = shutdown_rx.recv() => {
                        break;
                    }
                    _ = cpu_interval.tick() => {
                        // Collect CPU statistics
                        cpu::collect_cpu_sample();
                    }
                    _ = log_interval_timer.tick() => {
                        // Write statistics to log
                        write_statistics();
                    }
                }
            }
        });
    });

    log_bypass.thread_handle = Some(thread_handle);
    *guard = Some(log_bypass);

    Ok(())
}

/// Stop the LogByPass thread
pub fn stop_log_bypass() -> Result<(), String> {
    let mut guard = LOG_BYPASS.lock();

    if let Some(mut log_bypass) = guard.take() {
        if let Some(tx) = log_bypass.shutdown_tx.take() {
            let _ = tx.blocking_send(());
        }
        if let Some(handle) = log_bypass.thread_handle.take() {
            let _ = handle.join();
        }
        log_bypass.is_running.store(false, Ordering::SeqCst);
    }

    Ok(())
}

/// Check if LogByPass is running
pub fn is_log_bypass_running() -> bool {
    LOG_BYPASS
        .lock()
        .as_ref()
        .map_or(false, |l| l.is_running.load(Ordering::SeqCst))
}

/// Collect and write all statistics
fn write_statistics() {
    let cfg = config::get_config();
    let registry = EnvironmentRegistry::global();

    // Collect CPU usage
    let cpu_usage = platform::get_cpu_usage();
    let cpu_stats = cpu::get_cpu_stats();

    // Write stats for each environment (thread)
    registry.for_each(|env| {
        let stats = collect_environment_stats(env, &cpu_usage, &cpu_stats);
        writer::write_stats_log(&cfg, env.thread_id, env.is_main_thread, &stats);
    });
}

/// Collected statistics for an environment
pub struct EnvironmentStats {
    // CPU
    pub cpu_now: f64,
    pub cpu_15: f64,
    pub cpu_30: f64,
    pub cpu_60: f64,

    // Memory
    pub rss: u64,
    pub heap_used: u64,
    pub heap_available: u64,
    pub heap_total: u64,
    pub heap_limit: u64,
    pub heap_executable: u64,
    pub total_physical_size: u64,
    pub malloced_memory: u64,
    pub external_memory: u64,

    // GC
    pub total_gc_times: u64,
    pub total_gc_duration: u64,
    pub total_scavenge_duration: u64,
    pub total_marksweep_duration: u64,
    pub total_incremental_marking_duration: u64,
    pub gc_time_during_last_record: u64,
    pub scavenge_duration_last_record: u64,
    pub marksweep_duration_last_record: u64,
    pub incremental_marking_duration_last_record: u64,

    // HTTP
    pub live_http_request: i64,
    pub http_response_close: u64,
    pub http_response_sent: u64,
    pub http_request_timeout: u64,
    pub http_rt: u64,

    // UV handles
    pub active_handles: u64,
}

impl Default for EnvironmentStats {
    fn default() -> Self {
        Self {
            cpu_now: 0.0,
            cpu_15: 0.0,
            cpu_30: 0.0,
            cpu_60: 0.0,
            rss: 0,
            heap_used: 0,
            heap_available: 0,
            heap_total: 0,
            heap_limit: 0,
            heap_executable: 0,
            total_physical_size: 0,
            malloced_memory: 0,
            external_memory: 0,
            total_gc_times: 0,
            total_gc_duration: 0,
            total_scavenge_duration: 0,
            total_marksweep_duration: 0,
            total_incremental_marking_duration: 0,
            gc_time_during_last_record: 0,
            scavenge_duration_last_record: 0,
            marksweep_duration_last_record: 0,
            incremental_marking_duration_last_record: 0,
            live_http_request: 0,
            http_response_close: 0,
            http_response_sent: 0,
            http_request_timeout: 0,
            http_rt: 0,
            active_handles: 0,
        }
    }
}

/// Collect statistics from an environment
fn collect_environment_stats(
    env: &crate::env::EnvironmentData,
    _cpu_usage: &platform::CpuUsage,
    cpu_stats: &cpu::CpuStats,
) -> EnvironmentStats {
    use std::sync::atomic::Ordering;

    let mut stats = EnvironmentStats::default();

    // CPU stats from our sampler
    stats.cpu_now = cpu_stats.cpu_now;
    stats.cpu_15 = cpu_stats.cpu_15;
    stats.cpu_30 = cpu_stats.cpu_30;
    stats.cpu_60 = cpu_stats.cpu_60;

    // GC stats
    stats.total_gc_times = env.gc_stats.total_gc_times.load(Ordering::SeqCst);
    stats.total_gc_duration = env.gc_stats.total_gc_duration.load(Ordering::SeqCst);
    stats.total_scavenge_duration = env.gc_stats.total_scavenge_duration.load(Ordering::SeqCst);
    stats.total_marksweep_duration = env.gc_stats.total_marksweep_duration.load(Ordering::SeqCst);
    stats.total_incremental_marking_duration = env
        .gc_stats
        .total_incremental_marking_duration
        .load(Ordering::SeqCst);
    stats.gc_time_during_last_record = env
        .gc_stats
        .gc_time_during_last_record
        .swap(0, Ordering::SeqCst);
    stats.scavenge_duration_last_record = env
        .gc_stats
        .scavenge_duration_last_record
        .swap(0, Ordering::SeqCst);
    stats.marksweep_duration_last_record = env
        .gc_stats
        .marksweep_duration_last_record
        .swap(0, Ordering::SeqCst);
    stats.incremental_marking_duration_last_record = env
        .gc_stats
        .incremental_marking_duration_last_record
        .swap(0, Ordering::SeqCst);

    // HTTP stats
    stats.live_http_request = env.http_stats.live_http_request.load(Ordering::SeqCst);
    stats.http_response_close = env.http_stats.http_response_close.swap(0, Ordering::SeqCst);
    stats.http_response_sent = env.http_stats.http_response_sent.swap(0, Ordering::SeqCst);
    stats.http_request_timeout = env.http_stats.http_request_timeout.swap(0, Ordering::SeqCst);
    stats.http_rt = env.http_stats.http_rt.swap(0, Ordering::SeqCst);

    // Memory stats (from env, may need to be populated from V8)
    stats.rss = env.memory_stats.rss.load(Ordering::SeqCst);
    stats.heap_used = env.memory_stats.heap_used.load(Ordering::SeqCst);
    stats.heap_available = env.memory_stats.heap_available.load(Ordering::SeqCst);
    stats.heap_total = env.memory_stats.heap_total.load(Ordering::SeqCst);
    stats.heap_limit = env.memory_stats.heap_limit.load(Ordering::SeqCst);

    // UV stats
    stats.active_handles = env.uv_stats.active_handles.load(Ordering::SeqCst);

    stats
}
