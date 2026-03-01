mod config;
mod logger;
mod logbypass;

use napi::bindgen_prelude::*;
use napi::{Error, Result};
use napi_derive::napi;
use crate::config::{XProfilerConfig, set_global_config, get_global_config, check_configured};
use crate::logger::{info as log_info, error as log_error, debug as log_debug};
use crate::logbypass::start_log_bypass_thread;

#[napi]
pub fn start() {
  println!("xprofiler-rs started successfully!");
}

#[napi(object)]
#[derive(Debug)]
pub struct SetupOptions {
    pub is_main_thread: bool,
    pub thread_id: i32,
    pub node_version: String,
}

#[napi]
pub fn setup(options: SetupOptions) {
    // 暂时只打印日志，后续可能需要保存这些信息
    // println!("xprofiler setup: {:?}", options);
}

#[napi]
pub fn configure(config: XProfilerConfig) {
    set_global_config(config);
}

#[napi]
pub fn get_config() -> Result<XProfilerConfig> {
     if !check_configured() {
        return Err(Error::from_reason("must run \"require('xprofiler')()\" to set xprofiler config first!"));
    }
    Ok(get_global_config())
}

// 兼容旧接口名，虽然 JS 端并没有直接调用这个，但为了方便测试和一致性
#[napi(js_name = "getXprofilerConfig")]
pub fn get_xprofiler_config_js() -> Result<XProfilerConfig> {
    if !check_configured() {
        return Err(Error::from_reason("must run \"require('xprofiler')()\" to set xprofiler config first!"));
    }
    Ok(get_global_config())
}

#[napi]
pub fn info(component: String, msg: String) {
    log_info(&component, &msg);
}

#[napi]
pub fn error(component: String, msg: String) {
    log_error(&component, &msg);
}

#[napi]
pub fn debug(component: String, msg: String) {
    log_debug(&component, &msg);
}

// 还有 checkSocketPath, runLogBypass 等占位符
#[napi]
pub fn check_socket_path(_force: bool) -> bool {
    true
}

#[napi]
pub fn run_log_bypass() {
    start_log_bypass_thread();
}

#[napi]
pub fn run_commands_listener() {
    println!("run_commands_listener called (placeholder)");
}

#[napi]
pub fn set_hooks() {
    println!("set_hooks called (placeholder)");
}

#[napi]
pub fn init_mallopt() {
    println!("init_mallopt called (placeholder)");
}

// patch http methods placeholder
#[napi]
pub fn set_http_config(_config: serde_json::Value) {}

#[napi]
pub fn add_live_request(_start_time: u32) {}

#[napi]
pub fn add_close_request(_start_time: u32) {}

#[napi]
pub fn add_sent_request(_start_time: u32) {}

#[napi]
pub fn add_request_timeout(_start_time: u32) {}

#[napi]
pub fn add_http_status_code(_status: u32) {}

#[napi]
pub fn add_http_profiling_detail(_detail: serde_json::Value) {}
