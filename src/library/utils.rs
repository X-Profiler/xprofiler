use std::thread;
use std::time::Duration;
use chrono::Local;

pub fn sleep(seconds: u64) {
    thread::sleep(Duration::from_secs(seconds));
}

pub fn convert_time(format: &str) -> String {
    // A simplified time converter, assuming format is standard strftime
    Local::now().format(format).to_string()
}
