use napi_derive::napi;
use napi::{Env, JsString, Result};
use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use std::path::Path;
use crate::config::{get_config_string, get_config_bool, get_config_number};

lazy_static::lazy_static! {
    static ref LOGGER_MUTEX: Mutex<()> = Mutex::new(());
}

#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    Info = 0,
    Error = 1,
    Debug = 2,
}

#[derive(PartialEq)]
pub enum LogType {
    File = 0,
    Tty = 1,
}

fn write_to_file(output_level: LogLevel, log: &str) {
    let log_dir = get_config_string("log_dir");
    let log_format_alinode = get_config_bool("log_format_alinode");
    let file_prefix = if log_format_alinode { "node-" } else { "xprofiler-" };
    let time_string_day = Local::now().format("%Y%m%d").to_string();

    let filename = match output_level {
        LogLevel::Info => format!("{}{}.log", file_prefix, time_string_day),
        LogLevel::Error => format!("{}error-{}.log", file_prefix, time_string_day),
        LogLevel::Debug => format!("{}debug-{}.log", file_prefix, time_string_day),
    };

    let filepath = std::path::Path::new(&log_dir).join(filename);

    let _lock = LOGGER_MUTEX.lock().unwrap();
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(filepath) {
        let _ = file.write_all(log.as_bytes());
    }
}

pub fn log_msg(output_level: LogLevel, component: &str, thread_id: u64, message: &str) {
    let level = get_config_number("log_level");
    let level_enum = match level {
        0 => LogLevel::Info,
        1 => LogLevel::Error,
        2 => LogLevel::Debug,
        _ => LogLevel::Info,
    };
    
    if level_enum < output_level {
        return;
    }

    let log_format_alinode = get_config_bool("log_format_alinode");
    let now = Local::now();
    let time_string_ms = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let level_string = match output_level {
        LogLevel::Info => "info",
        LogLevel::Error => "error",
        LogLevel::Debug => "debug",
    };

    let pid = std::process::id();
    let tid = thread_id;

    let tmp_log = if log_format_alinode {
        let time_string_ms_alinode = now.format("%Y-%m-%d %H:%M:%S.%f").to_string();
        let time_string_ms_alinode = if time_string_ms_alinode.len() > 26 {
            &time_string_ms_alinode[..26]
        } else {
            &time_string_ms_alinode
        };
        format!("[{}] [{}] [{}] [{}] {}\n", time_string_ms_alinode, level_string, component, pid, message)
    } else {
        let version = env!("CARGO_PKG_VERSION");
        format!("[{}] [{}] [{}] [{}] [{}] [{}] {}\n", time_string_ms, level_string, component, pid, tid, version, message)
    };

    let log_type = get_config_number("log_type");
    match log_type {
        1 => { // LOG_TO_TTY
            print!("{}", tmp_log);
            write_to_file(output_level, &tmp_log);
        }
        0 => { // LOG_TO_FILE
            write_to_file(output_level, &tmp_log);
        }
        _ => {}
    }
}

#[napi]
pub fn info(component: String, message: String) {
    log_msg(LogLevel::Info, &component, 0, &message);
}

#[napi]
pub fn error(component: String, message: String) {
    log_msg(LogLevel::Error, &component, 0, &message);
}

#[napi]
pub fn debug(component: String, message: String) {
    log_msg(LogLevel::Debug, &component, 0, &message);
}
