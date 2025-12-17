//! Log file writer for statistics
//!
//! This module writes performance statistics to log files in a format
//! compatible with the original xprofiler.

use super::EnvironmentStats;
use crate::config::XprofilerConfig;
use crate::utils;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

const XPROFILER_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Write statistics to the log file
pub fn write_stats_log(
    cfg: &XprofilerConfig,
    thread_id: i64,
    _is_main_thread: bool,
    stats: &EnvironmentStats,
) {
    let log_path = get_log_path(cfg);

    let file = match OpenOptions::new().create(true).append(true).open(&log_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[xprofiler] Failed to open log file {}: {}", log_path.display(), e);
            return;
        }
    };

    if cfg.log_format_alinode {
        write_alinode_format(file, cfg, thread_id, stats);
    } else {
        write_xprofiler_format(file, cfg, thread_id, stats);
    }
}

/// Get the log file path
fn get_log_path(cfg: &XprofilerConfig) -> PathBuf {
    let date = chrono::Local::now().format("%Y%m%d");

    if cfg.log_format_alinode {
        // Alinode format: node-{date}.log
        PathBuf::from(&cfg.log_dir).join(format!("node-{}.log", date))
    } else {
        // xprofiler format: xprofiler-{date}.log
        PathBuf::from(&cfg.log_dir).join(format!("xprofiler-{}.log", date))
    }
}

/// Get timestamp string
fn get_timestamp() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Get timestamp string with microseconds for alinode format
fn get_timestamp_alinode() -> String {
    let now = chrono::Local::now();
    now.format("%Y-%m-%d %H:%M:%S%.6f").to_string()
}

/// Write in xprofiler format - each component on separate line
fn write_xprofiler_format(
    mut file: File,
    cfg: &XprofilerConfig,
    thread_id: i64,
    stats: &EnvironmentStats,
) {
    let timestamp = get_timestamp();
    let pid = utils::get_pid();
    let uptime = utils::get_uptime();

    // CPU line
    let cpu_line = format!(
        "[{}] [info] [cpu] [{}] [{}] [{}] cpu_usage(%) cpu_now: {:.2}, cpu_15: {:.2}, cpu_30: {:.2}, cpu_60: {:.2}, cpu_180: {:.2}, cpu_300: {:.2}, cpu_600: {:.2}\n",
        timestamp, pid, thread_id, XPROFILER_VERSION,
        stats.cpu_now, stats.cpu_15, stats.cpu_30, stats.cpu_60,
        stats.cpu_60, stats.cpu_60, stats.cpu_60  // Use cpu_60 for longer periods (not implemented yet)
    );

    // Memory line
    let memory_line = format!(
        "[{}] [info] [memory] [{}] [{}] [{}] memory_usage(byte) rss: {}, heap_used: {}, heap_available: {}, heap_total: {}, heap_limit: {}, heap_executeable: {}, total_physical_size: {}, malloced_memory: {}, amount_of_external_allocated_memory: {}, new_space_size: 0, new_space_used: 0, new_space_available: 0, new_space_committed: 0, old_space_size: 0, old_space_used: 0, old_space_available: 0, old_space_committed: 0, code_space_size: 0, code_space_used: 0, code_space_available: 0, code_space_committed: 0, map_space_size: 0, map_space_used: 0, map_space_available: 0, map_space_committed: 0, lo_space_size: 0, lo_space_used: 0, lo_space_available: 0, lo_space_committed: 0, read_only_space_size: 0, read_only_space_used: 0, read_only_space_available: 0, read_only_space_committed: 0, new_lo_space_size: 0, new_lo_space_used: 0, new_lo_space_available: 0, new_lo_space_committed: 0, code_lo_space_size: 0, code_lo_space_used: 0, code_lo_space_available: 0, code_lo_space_committed: 0\n",
        timestamp, pid, thread_id, XPROFILER_VERSION,
        stats.rss, stats.heap_used, stats.heap_available, stats.heap_total,
        stats.heap_limit, stats.heap_executable, stats.total_physical_size,
        stats.malloced_memory, stats.external_memory
    );

    // GC line
    let gc_line = format!(
        "[{}] [info] [gc] [{}] [{}] [{}] gc_status uptime: {}, total_gc_times: {}, total_gc_duration: {}, total_scavange_duration: {}, total_marksweep_duration: {}, total_incremental_marking_duration: {}, gc_time_during_last_record: {}, scavange_duration_last_record: {}, marksweep_duration_last_record: {}, incremental_marking_duration_last_record: {}\n",
        timestamp, pid, thread_id, XPROFILER_VERSION,
        uptime, stats.total_gc_times, stats.total_gc_duration,
        stats.total_scavenge_duration, stats.total_marksweep_duration,
        stats.total_incremental_marking_duration, stats.gc_time_during_last_record,
        stats.scavenge_duration_last_record, stats.marksweep_duration_last_record,
        stats.incremental_marking_duration_last_record
    );

    // UV handles line
    let uv_line = if cfg.enable_log_uv_handles {
        format!(
            "[{}] [info] [uv] [{}] [{}] [{}] libuv_handles_status active_handles: {}, active_file_handles: 0, active_and_ref_file_handles: 0, active_tcp_handles: 0, active_and_ref_tcp_handles: 0, active_udp_handles: 0, active_and_ref_udp_handles: 0, active_timer_handles: 0, active_and_ref_timer_handles: 0\n",
            timestamp, pid, thread_id, XPROFILER_VERSION,
            stats.active_handles
        )
    } else {
        // When enable_log_uv_handles is false, only output active_handles
        format!(
            "[{}] [info] [uv] [{}] [{}] [{}] libuv_handles_status active_handles: {}\n",
            timestamp, pid, thread_id, XPROFILER_VERSION,
            stats.active_handles
        )
    };

    // HTTP line
    let http_rt = if stats.http_response_sent > 0 {
        stats.http_rt as f64 / stats.http_response_sent as f64
    } else {
        0.0
    };
    let http_line = format!(
        "[{}] [info] [http] [{}] [{}] [{}] http_status live_http_request: {}, http_response_close: {}, http_response_sent: {}, http_request_timeout: {}, http_patch_timeout: {}, http_rt: {:.2}\n",
        timestamp, pid, thread_id, XPROFILER_VERSION,
        stats.live_http_request, stats.http_response_close,
        stats.http_response_sent, stats.http_request_timeout,
        cfg.patch_http_timeout, http_rt
    );

    // Write all lines
    if let Err(e) = file.write_all(cpu_line.as_bytes()) {
        eprintln!("[xprofiler] Failed to write cpu log: {}", e);
    }
    if let Err(e) = file.write_all(memory_line.as_bytes()) {
        eprintln!("[xprofiler] Failed to write memory log: {}", e);
    }
    if let Err(e) = file.write_all(gc_line.as_bytes()) {
        eprintln!("[xprofiler] Failed to write gc log: {}", e);
    }
    if let Err(e) = file.write_all(uv_line.as_bytes()) {
        eprintln!("[xprofiler] Failed to write uv log: {}", e);
    }
    if let Err(e) = file.write_all(http_line.as_bytes()) {
        eprintln!("[xprofiler] Failed to write http log: {}", e);
    }
}

/// Write in Alinode format
fn write_alinode_format(
    mut file: File,
    _cfg: &XprofilerConfig,
    _thread_id: i64,
    stats: &EnvironmentStats,
) {
    let timestamp = get_timestamp_alinode();
    let pid = utils::get_pid();

    // "other" component - CPU
    let cpu_line = format!(
        "[{}] [info] [other] [{}] cpu_usage(%) now: {:.2}, cpu_15: {:.2}, cpu_30: {:.2}, cpu_60: {:.2}, cpu_180: {:.2}, cpu_300: {:.2}, cpu_600: {:.2}\n",
        timestamp, pid,
        stats.cpu_now, stats.cpu_15, stats.cpu_30, stats.cpu_60,
        stats.cpu_60, stats.cpu_60, stats.cpu_60
    );

    // "heap" component - Memory
    let memory_line = format!(
        "[{}] [info] [heap] [{}] rss: {}, heap_used: {}, heap_available: {}, heap_total: {}, heap_limit: {}, heap_executeable: {}, total_physical_size: {}, malloced_memory: {}, amount_of_external_allocated_memory: {}, new_space_size: 0, new_space_used: 0, new_space_available: 0, new_space_committed: 0, old_space_size: 0, old_space_used: 0, old_space_available: 0, old_space_committed: 0, code_space_size: 0, code_space_used: 0, code_space_available: 0, code_space_committed: 0, map_space_size: 0, map_space_used: 0, map_space_available: 0, map_space_committed: 0, lo_space_size: 0, lo_space_used: 0, lo_space_available: 0, lo_space_committed: 0, read_only_space_size: 0, read_only_space_used: 0, read_only_space_available: 0, read_only_space_committed: 0, new_lo_space_size: 0, new_lo_space_used: 0, new_lo_space_available: 0, new_lo_space_committed: 0, code_lo_space_size: 0, code_lo_space_used: 0, code_lo_space_available: 0, code_lo_space_committed: 0\n",
        timestamp, pid,
        stats.rss, stats.heap_used, stats.heap_available, stats.heap_total,
        stats.heap_limit, stats.heap_executable, stats.total_physical_size,
        stats.malloced_memory, stats.external_memory
    );

    // "gc" component
    let gc_line = format!(
        "[{}] [info] [gc] [{}] gc_time_during_last_min: {}, total: {}, scavange_duration: {}, marksweep_duration: {}\n",
        timestamp, pid,
        stats.gc_time_during_last_record, stats.total_gc_times,
        stats.total_scavenge_duration, stats.total_marksweep_duration
    );

    // "timer" component - UV handles
    let uv_line = format!(
        "[{}] [info] [timer] [{}] total_timer: 0, active_handles: {}\n",
        timestamp, pid, stats.active_handles
    );

    // "http" component
    let http_rt = if stats.http_response_sent > 0 {
        stats.http_rt as f64 / stats.http_response_sent as f64
    } else {
        0.0
    };
    let http_line = format!(
        "[{}] [info] [http] [{}] live_http_request: {}, http_request_handled: {}, http_response_sent: {}, http_rt: {:.2}\n",
        timestamp, pid,
        stats.live_http_request, stats.http_response_close,
        stats.http_response_sent, http_rt
    );

    // Write all lines
    if let Err(e) = file.write_all(cpu_line.as_bytes()) {
        eprintln!("[xprofiler] Failed to write cpu log: {}", e);
    }
    if let Err(e) = file.write_all(memory_line.as_bytes()) {
        eprintln!("[xprofiler] Failed to write memory log: {}", e);
    }
    if let Err(e) = file.write_all(gc_line.as_bytes()) {
        eprintln!("[xprofiler] Failed to write gc log: {}", e);
    }
    if let Err(e) = file.write_all(uv_line.as_bytes()) {
        eprintln!("[xprofiler] Failed to write uv log: {}", e);
    }
    if let Err(e) = file.write_all(http_line.as_bytes()) {
        eprintln!("[xprofiler] Failed to write http log: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_log_path_xprofiler_format() {
        let cfg = XprofilerConfig {
            log_dir: "/tmp".to_string(),
            log_format_alinode: false,
            ..Default::default()
        };

        let path = get_log_path(&cfg);
        assert!(path.to_string_lossy().contains("xprofiler-"));
        assert!(path.to_string_lossy().ends_with(".log"));
    }

    #[test]
    fn test_get_log_path_alinode_format() {
        let cfg = XprofilerConfig {
            log_dir: "/tmp".to_string(),
            log_format_alinode: true,
            ..Default::default()
        };

        let path = get_log_path(&cfg);
        assert!(path.to_string_lossy().contains("node-"));
        assert!(path.to_string_lossy().ends_with(".log"));
    }
}
