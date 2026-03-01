use std::thread;
use std::time::Duration;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::collections::HashMap;
use crate::config::get_global_config;
use crate::logger::{info, error};

#[repr(C)]
#[derive(Debug, Default)]
struct HeapStatistics {
    total_heap_size: usize,
    total_heap_size_executable: usize,
    total_physical_size: usize,
    total_available_size: usize,
    used_heap_size: usize,
    heap_size_limit: usize,
    malloced_memory: usize,
    peak_malloced_memory: usize,
    does_zap_garbage: usize,
    number_of_native_contexts: usize,
    number_of_detached_contexts: usize,
}

#[repr(C)]
#[derive(Debug, Default)]
struct HeapSpaceStatistics {
    space_name: *const c_char,
    space_size: usize,
    space_used_size: usize,
    space_available_size: usize,
    physical_space_size: usize,
}

#[repr(C)]
#[derive(Debug, Default)]
struct GcStatistics {
    total_gc_times: u32,
    total_gc_duration: u32,
    total_scavange_duration: u32,
    total_marksweep_duration: u32,
    total_incremental_marking_duration: u32,
    gc_time_during_last_record: u32,
    scavange_duration_last_record: u32,
    marksweep_duration_last_record: u32,
    incremental_marking_duration_last_record: u32,
}

#[repr(C)]
#[derive(Debug, Default)]
struct UvHandleStatistics {
    active_handles: usize,
    active_file_handles: usize,
    active_and_ref_file_handles: usize,
    active_tcp_handles: usize,
    active_and_ref_tcp_handles: usize,
    active_udp_handles: usize,
    active_and_ref_udp_handles: usize,
    active_timer_handles: usize,
    active_and_ref_timer_handles: usize,
}

extern "C" {
    fn bridge_get_rss() -> u64;
    fn bridge_get_cpu_usage() -> f64;
    fn bridge_get_heap_statistics(stats: *mut HeapStatistics);
    fn bridge_get_heap_space_statistics(callback: extern "C" fn(*mut HeapSpaceStatistics, *mut std::ffi::c_void), data: *mut std::ffi::c_void);
    fn bridge_get_gc_statistics(stats: *mut GcStatistics);
    fn bridge_get_uv_handle_statistics(stats: *mut UvHandleStatistics);
}

// Callback for bridge_get_heap_space_statistics
extern "C" fn heap_space_callback(stats: *mut HeapSpaceStatistics, data: *mut std::ffi::c_void) {
    let spaces = unsafe { &mut *(data as *mut HashMap<String, HeapSpaceStatistics>) };
    let stats_ref = unsafe { &*stats };
    
    let name = unsafe { CStr::from_ptr(stats_ref.space_name) }.to_string_lossy().into_owned();
    
    spaces.insert(name, HeapSpaceStatistics {
        space_name: std::ptr::null(),
        space_size: stats_ref.space_size,
        space_used_size: stats_ref.space_used_size,
        space_available_size: stats_ref.space_available_size,
        physical_space_size: stats_ref.physical_space_size,
    });
}

pub fn start_log_bypass_thread() {
    thread::spawn(|| {
        loop {
            let config = get_global_config();
            let interval = config.log_interval;
            
            if interval == 0 {
                break;
            }

            thread::sleep(Duration::from_secs(interval as u64));

            // 1. CPU
            let cpu_usage = unsafe { bridge_get_cpu_usage() };
            if config.log_format_alinode {
                info("other", &format!("cpu_usage(%): {:.2}, cpu_now: {:.2}, cpu_15: 0.00, cpu_30: 0.00, cpu_60: 0.00, cpu_180: 0.00, cpu_300: 0.00, cpu_600: 0.00", cpu_usage, cpu_usage));
            } else {
                info("cpu", &format!("cpu_usage(%) cpu_now: {:.2}, cpu_15: 0.00, cpu_30: 0.00, cpu_60: 0.00, cpu_180: 0.00, cpu_300: 0.00, cpu_600: 0.00", cpu_usage));
            }

            // 2. Memory
            let rss = unsafe { bridge_get_rss() };
            let mut heap_stats = HeapStatistics::default();
            unsafe { bridge_get_heap_statistics(&mut heap_stats) };
            
            let mut heap_spaces = HashMap::new();
            unsafe { bridge_get_heap_space_statistics(heap_space_callback, &mut heap_spaces as *mut _ as *mut std::ffi::c_void) };
            
            // Helper to format space info
            let format_space = |name: &str| -> String {
                if let Some(s) = heap_spaces.get(name) {
                    format!("{}_space_size: {}, {}_space_used: {}, {}_space_available: {}, {}_space_committed: {}, ", 
                        name, s.space_size, name, s.space_used_size, name, s.space_available_size, name, s.physical_space_size)
                } else {
                     format!("{}_space_size: 0, {}_space_used: 0, {}_space_available: 0, {}_space_committed: 0, ", 
                        name, name, name, name)
                }
            };

            let space_info = format!("{}{}{}{}{}{}{}{}",
                format_space("new"),
                format_space("old"),
                format_space("code"),
                format_space("map"),
                format_space("large_object"),
                format_space("read_only"),
                format_space("new_large_object"),
                format_space("code_large_object")
            );

            let common_mem_info = format!("rss: {}, heap_used: {}, heap_available: {}, heap_total: {}, heap_limit: {}, heap_executeable: {}, total_physical_size: {}, malloced_memory: {}, amount_of_external_allocated_memory: {}, ",
                rss, heap_stats.used_heap_size, heap_stats.total_available_size, heap_stats.total_heap_size, heap_stats.heap_size_limit, heap_stats.total_heap_size_executable, heap_stats.total_physical_size, heap_stats.malloced_memory, 0 // external memory not available in simple struct
            );

            if config.log_format_alinode {
                info("heap", &format!("{}{}", common_mem_info, space_info));
            } else {
                info("memory", &format!("memory_usage(byte) {}{}", common_mem_info, space_info));
            }

            // 3. GC
            let mut gc_stats = GcStatistics::default();
            unsafe { bridge_get_gc_statistics(&mut gc_stats) };
            
            if config.log_format_alinode {
                info("gc", &format!("gc_time_during_last_min: {}, total: {}, scavange_duration: {}, marksweep_duration: {}",
                    gc_stats.gc_time_during_last_record, gc_stats.total_gc_duration, gc_stats.scavange_duration_last_record, gc_stats.marksweep_duration_last_record));
            } else {
                info("gc", &format!("uptime: {}, total_gc_times: {}, total_gc_duration: {}, total_scavange_duration: {}, total_marksweep_duration: {}, total_incremental_marking_duration: {}, gc_time_during_last_record: {}, scavange_duration_last_record: {}, marksweep_duration_last_record: {}, incremental_marking_duration_last_record: {}",
                    0, // uptime placeholder
                    gc_stats.total_gc_times, gc_stats.total_gc_duration, gc_stats.total_scavange_duration, gc_stats.total_marksweep_duration, gc_stats.total_incremental_marking_duration,
                    gc_stats.gc_time_during_last_record, gc_stats.scavange_duration_last_record, gc_stats.marksweep_duration_last_record, gc_stats.incremental_marking_duration_last_record));
            }

            // 4. Libuv
            let mut uv_stats = UvHandleStatistics::default();
            unsafe { bridge_get_uv_handle_statistics(&mut uv_stats) };
            
            if config.log_format_alinode {
                info("timer", &format!("total_timer: {}, active_handles: {}", uv_stats.active_timer_handles, uv_stats.active_handles));
            } else if config.enable_log_uv_handles {
                info("uv", &format!("active_handles: {}, active_file_handles: {}, active_and_ref_file_handles: {}, active_tcp_handles: {}, active_and_ref_tcp_handles: {}, active_udp_handles: {}, active_and_ref_udp_handles: {}, active_timer_handles: {}, active_and_ref_timer_handles: {}",
                    uv_stats.active_handles, uv_stats.active_file_handles, uv_stats.active_and_ref_file_handles, uv_stats.active_tcp_handles, uv_stats.active_and_ref_tcp_handles, uv_stats.active_udp_handles, uv_stats.active_and_ref_udp_handles, uv_stats.active_timer_handles, uv_stats.active_and_ref_timer_handles));
            } else {
                 info("uv", &format!("active_handles: {}", uv_stats.active_handles));
            }

            // 5. HTTP (Placeholder)
            if config.log_format_alinode {
                info("http", "live_http_request: 0, http_request_handled: 0, http_response_sent: 0, http_rt: 0.00");
            } else {
                info("http", &format!("live_http_request: 0, http_response_close: 0, http_response_sent: 0, http_request_timeout: 0, http_patch_timeout: {}, http_rt: 0.00, res: 0", config.patch_http_timeout));
            }
        }
    });
}
