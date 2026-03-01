use napi_derive::napi;
use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};
use lazy_static::lazy_static;

#[napi(object)]
#[derive(Debug, Clone)]
pub struct XProfilerConfig {
    #[napi(js_name = "log_dir")]
    pub log_dir: String,
    #[napi(js_name = "log_interval")]
    pub log_interval: u32,
    #[napi(js_name = "log_level")]
    pub log_level: u32,
    #[napi(js_name = "log_type")]
    pub log_type: u32,
    #[napi(js_name = "log_format_alinode")]
    pub log_format_alinode: bool,
    #[napi(js_name = "patch_http")]
    pub patch_http: bool,
    #[napi(js_name = "patch_http_timeout")]
    pub patch_http_timeout: u32,
    #[napi(js_name = "check_throw")]
    pub check_throw: bool,
    #[napi(js_name = "auto_incr_heap_limit_size")]
    pub auto_incr_heap_limit_size: u32,
    #[napi(js_name = "enable_log_uv_handles")]
    pub enable_log_uv_handles: bool,
    #[napi(js_name = "enable_fatal_error_hook")]
    pub enable_fatal_error_hook: bool,
    #[napi(js_name = "enable_fatal_error_report")]
    pub enable_fatal_error_report: bool,
    #[napi(js_name = "enable_fatal_error_coredump")]
    pub enable_fatal_error_coredump: bool,
    #[napi(js_name = "enable_http_profiling")]
    pub enable_http_profiling: bool,
    #[napi(js_name = "enable_auto_incr_heap_limit")]
    pub enable_auto_incr_heap_limit: bool,
    #[napi(js_name = "enable_avoid_rss_leak")]
    pub enable_avoid_rss_leak: bool,
    #[napi(js_name = "m_mmap_threshold")]
    pub m_mmap_threshold: u32,
}

impl Default for XProfilerConfig {
    fn default() -> Self {
        XProfilerConfig {
            log_dir: "/tmp".to_string(),
            log_interval: 60,
            log_level: 1,
            log_type: 0,
            log_format_alinode: false,
            patch_http: true,
            patch_http_timeout: 30,
            check_throw: true,
            auto_incr_heap_limit_size: 256,
            enable_log_uv_handles: true,
            enable_fatal_error_hook: true,
            enable_fatal_error_report: true,
            enable_fatal_error_coredump: false,
            enable_http_profiling: false,
            enable_auto_incr_heap_limit: false,
            enable_avoid_rss_leak: false,
            m_mmap_threshold: 128,
        }
    }
}

lazy_static! {
    pub static ref GLOBAL_CONFIG: RwLock<XProfilerConfig> = RwLock::new(XProfilerConfig::default());
    pub static ref CONFIGURED: AtomicBool = AtomicBool::new(false);
}

pub fn set_global_config(config: XProfilerConfig) {
    let mut w = GLOBAL_CONFIG.write().unwrap();
    *w = config;
    CONFIGURED.store(true, Ordering::SeqCst);
}

pub fn get_global_config() -> XProfilerConfig {
    GLOBAL_CONFIG.read().unwrap().clone()
}

pub fn check_configured() -> bool {
    CONFIGURED.load(Ordering::SeqCst)
}
