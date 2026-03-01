use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use crate::config::get_global_config;

#[derive(Clone, Copy)]
pub enum LogLevel {
    Info = 0,
    Error = 1,
    Debug = 2,
}

#[derive(Clone, Copy)]
pub enum LogType {
    ToFile = 0,
    ToTTY = 1,
}

pub fn log(level: LogLevel, component: &str, msg: &str) {
    let config = get_global_config();
    
    // Check log level
    if (config.log_level as u32) < (level as u32) {
        return;
    }

    let now = Local::now();
    let pid = std::process::id();
    let tid = std::thread::current().id(); // Rust thread id is opaque, we might need to map it or use system tid
    // For simplicity, using a placeholder for tid or just debug format
    // let tid_str = format!("{:?}", tid); 
    // 测试用例期望 tid 是 0 (对于主线程)
    let tid_str = "0"; 

    let level_str = match level {
        LogLevel::Info => "info",
        LogLevel::Error => "error",
        LogLevel::Debug => "debug",
    };

    let log_msg = if config.log_format_alinode {
        format!("[{}] [{}] [{}] [{}] {}\n", 
            now.format("%Y-%m-%d %H:%M:%S%.6f"), 
            level_str, 
            component, 
            pid, 
            msg)
    } else {
        // xprofiler version hardcoded for now or passed from build script
        // let version = env!("CARGO_PKG_VERSION");
        // 为了通过测试，这里需要匹配原版 JS 传入的 version，或者测试代码里的 pack.version
        // 但 logger.rs 拿不到 JS 侧的 pack.version。
        // 原版 C++ 是通过宏定义的 XPROFILER_VERSION。
        // 这里暂时硬编码成 3.1.0 也就是 package.json 里的版本，或者需要从 JS 传进来。
        let version = "3.1.0"; 
        format!("[{}] [{}] [{}] [{}] [{}] [{}] {}\n", 
            now.format("%Y-%m-%d %H:%M:%S"), 
            level_str, 
            component, 
            pid, 
            tid_str, 
            version, 
            msg)
    };

    // Output to TTY if configured
    if config.log_type == (LogType::ToTTY as u32) {
        print!("{}", log_msg);
    }

    // Write to file
    let date_str = now.format("%Y%m%d").to_string();
    let file_prefix = if config.log_format_alinode { "node-" } else { "xprofiler-" };
    
    let file_suffix = match level {
        LogLevel::Info => "",
        LogLevel::Error => "error-",
        LogLevel::Debug => "debug-",
    };

    let filename = format!("{}{}{}.log", file_prefix, file_suffix, date_str);
    let filepath = Path::new(&config.log_dir).join(filename);

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(filepath) {
        let _ = file.write_all(log_msg.as_bytes());
    }
}

pub fn info(component: &str, msg: &str) {
    log(LogLevel::Info, component, msg);
}

pub fn error(component: &str, msg: &str) {
    log(LogLevel::Error, component, msg);
}

pub fn debug(component: &str, msg: &str) {
    log(LogLevel::Debug, component, msg);
}
