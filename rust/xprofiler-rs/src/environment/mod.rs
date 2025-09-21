//! Environment data management module for XProfiler
//!
//! This module manages thread-specific environment data and process information
//! that is compatible with the original C++ implementation.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;

pub mod data;
pub mod registry;

use data::EnvironmentData;
use registry::EnvironmentRegistry;

/// Global environment registry instance
static ENV_REGISTRY: Lazy<Arc<RwLock<EnvironmentRegistry>>> = Lazy::new(|| {
    Arc::new(RwLock::new(EnvironmentRegistry::new()))
});

// Thread-local environment data
thread_local! {
    static THREAD_ENV_DATA: std::cell::RefCell<Option<EnvironmentData>> = std::cell::RefCell::new(None);
}

/// Initialize environment data for the current thread
pub fn setup_environment_data() -> Result<(), Box<dyn std::error::Error>> {
    let thread_id = get_thread_id();
    let is_main_thread = is_main_thread();
    let node_version = get_node_version();
    
    let env_data = EnvironmentData::new(thread_id, is_main_thread, node_version)?;
    
    // Store in thread-local storage
    THREAD_ENV_DATA.with(|data| {
        *data.borrow_mut() = Some(env_data.clone());
    });
    
    // Register in global registry
    let mut registry = ENV_REGISTRY.write().map_err(|e| format!("Failed to acquire write lock: {}", e))?;
    registry.register_thread(thread_id, env_data)?;
    
    Ok(())
}

/// Get environment data for the current thread
pub fn get_current_environment_data() -> Result<Option<EnvironmentData>, Box<dyn std::error::Error>> {
    THREAD_ENV_DATA.with(|data| {
        Ok(data.borrow().clone())
    })
}

/// Get environment data for a specific thread
pub fn get_thread_environment_data(thread_id: u32) -> Result<Option<EnvironmentData>, Box<dyn std::error::Error>> {
    let registry = ENV_REGISTRY.read().map_err(|e| format!("Failed to acquire read lock: {}", e))?;
    Ok(registry.get_thread_data(thread_id))
}

/// Get all registered thread environment data
pub fn get_all_environment_data() -> Result<HashMap<u32, EnvironmentData>, Box<dyn std::error::Error>> {
    let registry = ENV_REGISTRY.read().map_err(|e| format!("Failed to acquire read lock: {}", e))?;
    Ok(registry.get_all_thread_data())
}

/// Cleanup environment data for the current thread
pub fn cleanup_environment_data() -> Result<(), Box<dyn std::error::Error>> {
    let thread_id = get_thread_id();
    
    // Clear thread-local storage
    THREAD_ENV_DATA.with(|data| {
        *data.borrow_mut() = None;
    });
    
    // Unregister from global registry
    let mut registry = ENV_REGISTRY.write().map_err(|e| format!("Failed to acquire write lock: {}", e))?;
    registry.unregister_thread(thread_id);
    
    Ok(())
}

/// Get the current thread ID
fn get_thread_id() -> u32 {
    // Use a simple hash of the thread ID for now
    // In a real implementation, this might use platform-specific APIs
    let thread_id = thread::current().id();
    let thread_id_str = format!("{:?}", thread_id);
    let mut hash = 0u32;
    for byte in thread_id_str.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
    }
    hash
}

/// Check if the current thread is the main thread
fn is_main_thread() -> bool {
    // This is a simplified implementation
    // In a real implementation, this would check against the main thread ID
    thread::current().name() == Some("main") || thread::current().name().is_none()
}

/// Get the Node.js version
fn get_node_version() -> String {
    // This would typically be obtained from Node.js runtime
    // For now, return a placeholder
    std::env::var("NODE_VERSION").unwrap_or_else(|_| "unknown".to_string())
}

/// Process information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub platform: String,
    pub arch: String,
    pub node_version: String,
    pub v8_version: String,
    pub uv_version: String,
    pub openssl_version: String,
}

/// Get current process information
pub fn get_process_info() -> Result<ProcessInfo, Box<dyn std::error::Error>> {
    Ok(ProcessInfo {
        pid: std::process::id(),
        ppid: get_parent_process_id()?,
        platform: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        node_version: get_node_version(),
        v8_version: "unknown".to_string(), // Would be obtained from Node.js
        uv_version: "unknown".to_string(), // Would be obtained from Node.js
        openssl_version: "unknown".to_string(), // Would be obtained from Node.js
    })
}

/// Get parent process ID (platform-specific implementation)
fn get_parent_process_id() -> Result<u32, Box<dyn std::error::Error>> {
    #[cfg(unix)]
    {
        Ok(unsafe { libc::getppid() as u32 })
    }
    
    #[cfg(windows)]
    {
        // Windows implementation would use WinAPI
        Ok(0) // Placeholder
    }
    
    #[cfg(not(any(unix, windows)))]
    {
        Ok(0) // Fallback
    }
}