use napi_derive::napi;

pub mod config;
pub mod library;
pub mod commands;
pub mod logger;
pub mod log_bypass;
pub mod hooks;

#[napi]
pub fn setup() {}

#[napi]
pub fn init_mallopt() {}

#[napi]
pub fn check_socket_path() -> bool {
    true
}

#[napi]
pub fn set_http_config() {}

#[napi]
pub fn add_live_request() {}

#[napi]
pub fn add_close_request() {}

#[napi]
pub fn add_sent_request() {}

#[napi]
pub fn add_request_timeout() {}

#[napi]
pub fn add_http_status_code() {}

#[napi]
pub fn add_http_profiling_detail() {}

