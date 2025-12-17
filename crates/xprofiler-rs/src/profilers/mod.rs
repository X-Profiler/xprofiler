//! Profilers module
//!
//! This module provides profiling capabilities for CPU, heap, and GC.

pub mod gc;

use crate::config;
use crate::utils;
use chrono::Local;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// Counter for generating unique file IDs
static FILE_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Get the next diagnostic file ID
pub fn get_next_file_id() -> u64 {
    FILE_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

/// Profiling action types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProfileAction {
    StartCpuProfiling,
    StopCpuProfiling,
    Heapdump,
    StartHeapProfiling,
    StopHeapProfiling,
    StartGcProfiling,
    StopGcProfiling,
    DiagReport,
    Coredump,
}

impl ProfileAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProfileAction::StartCpuProfiling => "start_cpu_profiling",
            ProfileAction::StopCpuProfiling => "stop_cpu_profiling",
            ProfileAction::Heapdump => "heapdump",
            // Use "sampling" naming to match xprofctl expectations
            ProfileAction::StartHeapProfiling => "start_sampling_heap_profiling",
            ProfileAction::StopHeapProfiling => "stop_sampling_heap_profiling",
            ProfileAction::StartGcProfiling => "start_gc_profiling",
            ProfileAction::StopGcProfiling => "stop_gc_profiling",
            ProfileAction::DiagReport => "diag_report",
            ProfileAction::Coredump => "coredump",
        }
    }

    /// Get the dependent action that must be running
    pub fn dependent_action(&self) -> Option<ProfileAction> {
        match self {
            ProfileAction::StopCpuProfiling => Some(ProfileAction::StartCpuProfiling),
            ProfileAction::StopHeapProfiling => Some(ProfileAction::StartHeapProfiling),
            ProfileAction::StopGcProfiling => Some(ProfileAction::StartGcProfiling),
            _ => None,
        }
    }
}

/// Profile state for a thread
#[derive(Debug, Default)]
pub struct ProfileState {
    /// Currently running actions
    pub running_actions: HashMap<ProfileAction, bool>,
    /// File paths for various profiles
    pub cpuprofile_filepath: Option<String>,
    pub heapsnapshot_filepath: Option<String>,
    pub heapprofile_filepath: Option<String>,
    pub gcprofile_filepath: Option<String>,
    pub diagreport_filepath: Option<String>,
    pub coredump_filepath: Option<String>,
}

/// Global profile states per thread
static PROFILE_STATES: Lazy<Mutex<HashMap<i64, ProfileState>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Get or create profile state for a thread
pub fn get_profile_state(thread_id: i64) -> parking_lot::MappedMutexGuard<'static, ProfileState> {
    parking_lot::MutexGuard::map(PROFILE_STATES.lock(), |states| {
        states.entry(thread_id).or_insert_with(ProfileState::default)
    })
}

/// Create a profile file path
pub fn create_filepath(prefix: &str, ext: &str) -> String {
    let cfg = config::get_config();
    let pid = utils::get_pid();
    let date = Local::now().format("%Y%m%d");
    let file_id = get_next_file_id();

    format!(
        "{}/x-{}-{}-{}-{}.{}",
        cfg.log_dir, prefix, pid, date, file_id, ext
    )
}

/// Check if an action is already running
pub fn is_action_running(thread_id: i64, action: ProfileAction) -> bool {
    let state = get_profile_state(thread_id);
    state.running_actions.contains_key(&action)
}

/// Check if dependent action is running (for stop commands)
pub fn check_dependent_action(thread_id: i64, action: ProfileAction) -> Result<(), String> {
    if let Some(dependent) = action.dependent_action() {
        if !is_action_running(thread_id, dependent) {
            return Err(format!(
                "{} dependent action {} is not running.",
                action.as_str(),
                dependent.as_str()
            ));
        }
    }
    Ok(())
}

/// Set an action as running
pub fn set_action_running(thread_id: i64, action: ProfileAction) {
    let mut state = get_profile_state(thread_id);
    state.running_actions.insert(action, true);
}

/// Clear an action from running
pub fn clear_action(thread_id: i64, action: ProfileAction) {
    let mut state = get_profile_state(thread_id);
    state.running_actions.remove(&action);
}
