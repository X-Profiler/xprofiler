//! CPU statistics collection
//!
//! This module collects CPU usage samples at 1-second intervals and
//! calculates usage over different time windows (15s, 30s, 60s).

use crate::platform;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::collections::VecDeque;

/// Maximum number of samples to keep (10 minutes at 1 sample/second)
const MAX_SAMPLES: usize = 600;

/// CPU sample data
#[derive(Debug, Clone, Copy)]
struct CpuSample {
    user_time: f64,
    system_time: f64,
    timestamp: std::time::Instant,
}

/// CPU samples buffer
static CPU_SAMPLES: Lazy<Mutex<VecDeque<CpuSample>>> = Lazy::new(|| Mutex::new(VecDeque::with_capacity(MAX_SAMPLES)));

/// Collected CPU statistics
#[derive(Debug, Clone, Copy, Default)]
pub struct CpuStats {
    pub cpu_now: f64,
    pub cpu_15: f64,
    pub cpu_30: f64,
    pub cpu_60: f64,
}

/// Collect a CPU sample (called every 1 second)
pub fn collect_cpu_sample() {
    let usage = platform::get_cpu_usage();
    let sample = CpuSample {
        user_time: usage.user_time,
        system_time: usage.system_time,
        timestamp: std::time::Instant::now(),
    };

    let mut samples = CPU_SAMPLES.lock();

    // Remove old samples if we're at capacity
    while samples.len() >= MAX_SAMPLES {
        samples.pop_front();
    }

    samples.push_back(sample);
}

/// Get CPU statistics over different time windows
pub fn get_cpu_stats() -> CpuStats {
    let samples = CPU_SAMPLES.lock();

    if samples.len() < 2 {
        return CpuStats::default();
    }

    let now = std::time::Instant::now();
    let latest = samples.back().unwrap();

    // Find samples at different time windows
    let sample_15s = find_sample_at_offset(&samples, now, 15);
    let sample_30s = find_sample_at_offset(&samples, now, 30);
    let sample_60s = find_sample_at_offset(&samples, now, 60);

    // Calculate CPU usage for each window
    let cpu_now = if samples.len() >= 2 {
        let prev = &samples[samples.len() - 2];
        calculate_cpu_usage(prev, latest)
    } else {
        0.0
    };

    let cpu_15 = sample_15s
        .map(|s| calculate_cpu_usage(&s, latest))
        .unwrap_or(cpu_now);

    let cpu_30 = sample_30s
        .map(|s| calculate_cpu_usage(&s, latest))
        .unwrap_or(cpu_15);

    let cpu_60 = sample_60s
        .map(|s| calculate_cpu_usage(&s, latest))
        .unwrap_or(cpu_30);

    CpuStats {
        cpu_now,
        cpu_15,
        cpu_30,
        cpu_60,
    }
}

/// Find a sample approximately `seconds` ago
fn find_sample_at_offset(
    samples: &VecDeque<CpuSample>,
    now: std::time::Instant,
    seconds: u64,
) -> Option<CpuSample> {
    let target_time = now - std::time::Duration::from_secs(seconds);

    // Binary search for the closest sample
    let mut best_sample = None;
    let mut best_diff = std::time::Duration::MAX;

    for sample in samples.iter() {
        let diff = if sample.timestamp > target_time {
            sample.timestamp - target_time
        } else {
            target_time - sample.timestamp
        };

        if diff < best_diff {
            best_diff = diff;
            best_sample = Some(*sample);
        }
    }

    // Only return if we found a sample within 5 seconds of the target
    if best_diff <= std::time::Duration::from_secs(5) {
        best_sample
    } else {
        None
    }
}

/// Calculate CPU usage percentage between two samples
fn calculate_cpu_usage(prev: &CpuSample, curr: &CpuSample) -> f64 {
    let elapsed = curr.timestamp.duration_since(prev.timestamp).as_secs_f64();

    if elapsed <= 0.0 {
        return 0.0;
    }

    let user_diff = curr.user_time - prev.user_time;
    let system_diff = curr.system_time - prev.system_time;
    let total_cpu_time = user_diff + system_diff;

    // CPU usage as percentage (0-100)
    // Note: This is for a single core. For multi-core, this could exceed 100%
    let usage = (total_cpu_time / elapsed) * 100.0;

    // Clamp to reasonable range
    usage.max(0.0).min(100.0 * num_cpus())
}

/// Get the number of CPU cores
fn num_cpus() -> f64 {
    std::thread::available_parallelism()
        .map(|p| p.get() as f64)
        .unwrap_or(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_cpu_sample() {
        // Clear any existing samples
        CPU_SAMPLES.lock().clear();

        // Collect a sample
        collect_cpu_sample();

        assert_eq!(CPU_SAMPLES.lock().len(), 1);
    }

    #[test]
    fn test_get_cpu_stats_empty() {
        CPU_SAMPLES.lock().clear();

        let stats = get_cpu_stats();
        assert_eq!(stats.cpu_now, 0.0);
    }

    #[test]
    fn test_get_cpu_stats_with_samples() {
        CPU_SAMPLES.lock().clear();

        // Collect multiple samples with a small delay
        for _ in 0..3 {
            collect_cpu_sample();
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        let stats = get_cpu_stats();
        // Stats should be calculated (values depend on actual CPU usage)
        assert!(stats.cpu_now >= 0.0);
    }
}
