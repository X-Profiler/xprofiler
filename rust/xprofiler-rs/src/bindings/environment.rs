//! Environment data bindings for JavaScript interface

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;

use crate::environment::{ENVIRONMENT_REGISTRY, EnvironmentData, HeapStatistics, GcStatistics, UvStatistics};

/// Environment data manager class for JavaScript
#[napi]
pub struct EnvironmentManager {
    initialized: bool,
}

#[napi]
impl EnvironmentManager {
    /// Create a new environment manager
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            initialized: false,
        }
    }
    
    /// Initialize the environment registry
    #[napi]
    pub fn initialize(&mut self) -> Result<()> {
        ENVIRONMENT_REGISTRY.initialize();
        self.initialized = true;
        Ok(())
    }
    
    /// Check if environment manager is initialized
    #[napi]
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    /// Register current thread for monitoring
    #[napi]
    pub fn register_current_thread(&self) -> Result<()> {
        let env_data = EnvironmentData::new();
        ENVIRONMENT_REGISTRY.register_thread(env_data);
        Ok(())
    }
    
    /// Unregister current thread from monitoring
    #[napi]
    pub fn unregister_current_thread(&self) -> Result<()> {
        ENVIRONMENT_REGISTRY.unregister_current_thread();
        Ok(())
    }
    
    /// Get current thread environment data
    #[napi]
    pub fn get_current_thread_data(&self) -> Result<Object> {
        match ENVIRONMENT_REGISTRY.get_current_thread_data() {
            Some(env_data) => environment_data_to_object(env_data),
            None => Err(Error::new(Status::InvalidArg, "No environment data for current thread")),
        }
    }
    
    /// Get all threads environment data
    #[napi]
    pub fn get_all_threads_data(&self) -> Result<Array> {
        let all_data = ENVIRONMENT_REGISTRY.get_all_threads_data();
        let mut result = Array::new(all_data.len() as u32)?;
        
        for (index, env_data) in all_data.iter().enumerate() {
            let obj = environment_data_to_object(env_data.clone())?;
            result.set(index as u32, obj)?;
        }
        
        Ok(result)
    }
    
    /// Update current thread environment data
    #[napi]
    pub fn update_current_thread(&self, heap_stats: Option<Object>, gc_stats: Option<Object>, uv_stats: Option<Object>) -> Result<()> {
        let mut env_data = ENVIRONMENT_REGISTRY.get_current_thread_data()
            .ok_or_else(|| Error::new(Status::InvalidArg, "No environment data for current thread"))?;
        
        // Update heap statistics if provided
        if let Some(heap_obj) = heap_stats {
            env_data.heap_stats = object_to_heap_statistics(heap_obj)?;
        }
        
        // Update GC statistics if provided
        if let Some(gc_obj) = gc_stats {
            env_data.gc_stats = object_to_gc_statistics(gc_obj)?;
        }
        
        // Update UV statistics if provided
        if let Some(uv_obj) = uv_stats {
            env_data.uv_stats = object_to_uv_statistics(uv_obj)?;
        }
        
        ENVIRONMENT_REGISTRY.update_thread_data(env_data);
        Ok(())
    }
    
    /// Get environment statistics
    #[napi]
    pub fn get_statistics(&self) -> Result<Object> {
        let stats = ENVIRONMENT_REGISTRY.get_statistics();
        let mut result = Object::new();
        
        result.set("total_threads", stats.total_threads)?;
        result.set("active_threads", stats.active_threads)?;
        result.set("total_samples", stats.total_samples)?;
        result.set("uptime_ms", stats.uptime_ms)?;
        
        Ok(result)
    }
    
    /// Clear all environment data
    #[napi]
    pub fn clear_all(&self) -> Result<()> {
        ENVIRONMENT_REGISTRY.clear_all();
        Ok(())
    }
    
    /// Check if registry is active
    #[napi]
    pub fn is_active(&self) -> bool {
        ENVIRONMENT_REGISTRY.is_active()
    }
    
    /// Get thread count
    #[napi]
    pub fn get_thread_count(&self) -> u32 {
        ENVIRONMENT_REGISTRY.get_thread_count()
    }
    
    /// Get specific thread data by thread ID
    #[napi]
    pub fn get_thread_data(&self, thread_id: String) -> Result<Object> {
        match ENVIRONMENT_REGISTRY.get_thread_data(&thread_id) {
            Some(env_data) => environment_data_to_object(env_data),
            None => Err(Error::new(Status::InvalidArg, "Thread not found")),
        }
    }
    
    /// Remove specific thread data
    #[napi]
    pub fn remove_thread(&self, thread_id: String) -> bool {
        ENVIRONMENT_REGISTRY.remove_thread(&thread_id)
    }
    
    /// Get aggregated heap statistics across all threads
    #[napi]
    pub fn get_aggregated_heap_stats(&self) -> Result<Object> {
        let all_data = ENVIRONMENT_REGISTRY.get_all_threads_data();
        let mut aggregated = HeapStatistics::default();
        
        for env_data in all_data {
            aggregated.total_heap_size += env_data.heap_stats.total_heap_size;
            aggregated.total_heap_size_executable += env_data.heap_stats.total_heap_size_executable;
            aggregated.total_physical_size += env_data.heap_stats.total_physical_size;
            aggregated.total_available_size += env_data.heap_stats.total_available_size;
            aggregated.used_heap_size += env_data.heap_stats.used_heap_size;
            aggregated.heap_size_limit += env_data.heap_stats.heap_size_limit;
            aggregated.malloced_memory += env_data.heap_stats.malloced_memory;
            aggregated.peak_malloced_memory = aggregated.peak_malloced_memory.max(env_data.heap_stats.peak_malloced_memory);
            aggregated.number_of_native_contexts += env_data.heap_stats.number_of_native_contexts;
            aggregated.number_of_detached_contexts += env_data.heap_stats.number_of_detached_contexts;
        }
        
        heap_statistics_to_object(aggregated)
    }
    
    /// Get aggregated GC statistics across all threads
    #[napi]
    pub fn get_aggregated_gc_stats(&self) -> Result<Object> {
        let all_data = ENVIRONMENT_REGISTRY.get_all_threads_data();
        let mut aggregated = GcStatistics::default();
        
        for env_data in all_data {
            aggregated.gc_count += env_data.gc_stats.gc_count;
            aggregated.gc_time_ms += env_data.gc_stats.gc_time_ms;
            aggregated.scavenge_count += env_data.gc_stats.scavenge_count;
            aggregated.scavenge_time_ms += env_data.gc_stats.scavenge_time_ms;
            aggregated.mark_sweep_count += env_data.gc_stats.mark_sweep_count;
            aggregated.mark_sweep_time_ms += env_data.gc_stats.mark_sweep_time_ms;
        }
        
        gc_statistics_to_object(aggregated)
    }
    
    /// Get aggregated UV statistics across all threads
    #[napi]
    pub fn get_aggregated_uv_stats(&self) -> Result<Object> {
        let all_data = ENVIRONMENT_REGISTRY.get_all_threads_data();
        let mut aggregated = UvStatistics::default();
        
        for env_data in all_data {
            aggregated.handle_count += env_data.uv_stats.handle_count;
            aggregated.request_count += env_data.uv_stats.request_count;
            aggregated.active_handles += env_data.uv_stats.active_handles;
            aggregated.active_requests += env_data.uv_stats.active_requests;
        }
        
        uv_statistics_to_object(aggregated)
    }
    
    /// Start monitoring with custom interval
    #[napi]
    pub fn start_monitoring(&self, interval_ms: Option<u32>) -> Result<()> {
        let interval = interval_ms.unwrap_or(1000); // Default 1 second
        ENVIRONMENT_REGISTRY.start_monitoring(interval);
        Ok(())
    }
    
    /// Stop monitoring
    #[napi]
    pub fn stop_monitoring(&self) -> Result<()> {
        ENVIRONMENT_REGISTRY.stop_monitoring();
        Ok(())
    }
    
    /// Check if monitoring is active
    #[napi]
    pub fn is_monitoring_active(&self) -> bool {
        ENVIRONMENT_REGISTRY.is_monitoring_active()
    }
}

/// Standalone environment functions

/// Get current environment data (standalone function)
#[napi]
pub fn get_environment_data() -> Result<Object> {
    match ENVIRONMENT_REGISTRY.get_current_thread_data() {
        Some(env_data) => environment_data_to_object(env_data),
        None => Err(Error::new(Status::InvalidArg, "No environment data available")),
    }
}

/// Get all environment data (standalone function)
#[napi]
pub fn get_all_environment_data() -> Result<Array> {
    let all_data = ENVIRONMENT_REGISTRY.get_all_threads_data();
    let mut result = Array::new(all_data.len() as u32)?;
    
    for (index, env_data) in all_data.iter().enumerate() {
        let obj = environment_data_to_object(env_data.clone())?;
        result.set(index as u32, obj)?;
    }
    
    Ok(result)
}

/// Register current thread (standalone function)
#[napi]
pub fn register_current_thread() -> Result<()> {
    let env_data = EnvironmentData::new();
    ENVIRONMENT_REGISTRY.register_thread(env_data);
    Ok(())
}

/// Unregister current thread (standalone function)
#[napi]
pub fn unregister_current_thread() -> Result<()> {
    ENVIRONMENT_REGISTRY.unregister_current_thread();
    Ok(())
}

/// Get environment statistics (standalone function)
#[napi]
pub fn get_environment_statistics() -> Result<Object> {
    let stats = ENVIRONMENT_REGISTRY.get_statistics();
    let mut result = Object::new();
    
    result.set("total_threads", stats.total_threads)?;
    result.set("active_threads", stats.active_threads)?;
    result.set("total_samples", stats.total_samples)?;
    result.set("uptime_ms", stats.uptime_ms)?;
    
    Ok(result)
}

/// Utility functions for data conversion

fn environment_data_to_object(env_data: EnvironmentData) -> Result<Object> {
    let mut result = Object::new();
    
    result.set("thread_id", env_data.thread_id)?;
    result.set("is_main_thread", env_data.is_main_thread)?;
    result.set("start_time", env_data.start_time)?;
    result.set("last_update", env_data.last_update)?;
    result.set("sample_count", env_data.sample_count)?;
    
    // Heap statistics
    let heap_stats = heap_statistics_to_object(env_data.heap_stats)?;
    result.set("heap_statistics", heap_stats)?;
    
    // GC statistics
    let gc_stats = gc_statistics_to_object(env_data.gc_stats)?;
    result.set("gc_statistics", gc_stats)?;
    
    // UV statistics
    let uv_stats = uv_statistics_to_object(env_data.uv_stats)?;
    result.set("uv_statistics", uv_stats)?;
    
    Ok(result)
}

fn heap_statistics_to_object(heap_stats: HeapStatistics) -> Result<Object> {
    let mut result = Object::new();
    
    result.set("total_heap_size", heap_stats.total_heap_size)?;
    result.set("total_heap_size_executable", heap_stats.total_heap_size_executable)?;
    result.set("total_physical_size", heap_stats.total_physical_size)?;
    result.set("total_available_size", heap_stats.total_available_size)?;
    result.set("used_heap_size", heap_stats.used_heap_size)?;
    result.set("heap_size_limit", heap_stats.heap_size_limit)?;
    result.set("malloced_memory", heap_stats.malloced_memory)?;
    result.set("peak_malloced_memory", heap_stats.peak_malloced_memory)?;
    result.set("does_zap_garbage", heap_stats.does_zap_garbage)?;
    result.set("number_of_native_contexts", heap_stats.number_of_native_contexts)?;
    result.set("number_of_detached_contexts", heap_stats.number_of_detached_contexts)?;
    
    Ok(result)
}

fn gc_statistics_to_object(gc_stats: GcStatistics) -> Result<Object> {
    let mut result = Object::new();
    
    result.set("gc_count", gc_stats.gc_count)?;
    result.set("gc_time_ms", gc_stats.gc_time_ms)?;
    result.set("scavenge_count", gc_stats.scavenge_count)?;
    result.set("scavenge_time_ms", gc_stats.scavenge_time_ms)?;
    result.set("mark_sweep_count", gc_stats.mark_sweep_count)?;
    result.set("mark_sweep_time_ms", gc_stats.mark_sweep_time_ms)?;
    
    Ok(result)
}

fn uv_statistics_to_object(uv_stats: UvStatistics) -> Result<Object> {
    let mut result = Object::new();
    
    result.set("handle_count", uv_stats.handle_count)?;
    result.set("request_count", uv_stats.request_count)?;
    result.set("active_handles", uv_stats.active_handles)?;
    result.set("active_requests", uv_stats.active_requests)?;
    
    Ok(result)
}

fn object_to_heap_statistics(obj: Object) -> Result<HeapStatistics> {
    Ok(HeapStatistics {
        total_heap_size: obj.get::<u64>("total_heap_size")?.unwrap_or_default(),
        total_heap_size_executable: obj.get::<u64>("total_heap_size_executable")?.unwrap_or_default(),
        total_physical_size: obj.get::<u64>("total_physical_size")?.unwrap_or_default(),
        total_available_size: obj.get::<u64>("total_available_size")?.unwrap_or_default(),
        used_heap_size: obj.get::<u64>("used_heap_size")?.unwrap_or_default(),
        heap_size_limit: obj.get::<u64>("heap_size_limit")?.unwrap_or_default(),
        malloced_memory: obj.get::<u64>("malloced_memory")?.unwrap_or_default(),
        peak_malloced_memory: obj.get::<u64>("peak_malloced_memory")?.unwrap_or_default(),
        does_zap_garbage: obj.get::<bool>("does_zap_garbage")?.unwrap_or_default(),
        number_of_native_contexts: obj.get::<u32>("number_of_native_contexts")?.unwrap_or_default(),
        number_of_detached_contexts: obj.get::<u32>("number_of_detached_contexts")?.unwrap_or_default(),
    })
}

fn object_to_gc_statistics(obj: Object) -> Result<GcStatistics> {
    Ok(GcStatistics {
        gc_count: obj.get::<u64>("gc_count")?.unwrap_or_default(),
        gc_time_ms: obj.get::<f64>("gc_time_ms")?.unwrap_or_default(),
        scavenge_count: obj.get::<u64>("scavenge_count")?.unwrap_or_default(),
        scavenge_time_ms: obj.get::<f64>("scavenge_time_ms")?.unwrap_or_default(),
        mark_sweep_count: obj.get::<u64>("mark_sweep_count")?.unwrap_or_default(),
        mark_sweep_time_ms: obj.get::<f64>("mark_sweep_time_ms")?.unwrap_or_default(),
    })
}

fn object_to_uv_statistics(obj: Object) -> Result<UvStatistics> {
    Ok(UvStatistics {
        handle_count: obj.get::<u32>("handle_count")?.unwrap_or_default(),
        request_count: obj.get::<u32>("request_count")?.unwrap_or_default(),
        active_handles: obj.get::<u32>("active_handles")?.unwrap_or_default(),
        active_requests: obj.get::<u32>("active_requests")?.unwrap_or_default(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_environment_manager_creation() {
        let manager = EnvironmentManager::new();
        assert!(!manager.is_initialized());
    }
    
    #[test]
    fn test_heap_statistics_conversion() {
        let heap_stats = HeapStatistics {
            total_heap_size: 1024,
            total_heap_size_executable: 512,
            total_physical_size: 2048,
            total_available_size: 4096,
            used_heap_size: 800,
            heap_size_limit: 8192,
            malloced_memory: 256,
            peak_malloced_memory: 512,
            does_zap_garbage: true,
            number_of_native_contexts: 2,
            number_of_detached_contexts: 1,
        };
        
        let obj = heap_statistics_to_object(heap_stats).unwrap();
        // In a real test, we would verify the object contents
        // This is just a compilation test
    }
    
    #[test]
    fn test_gc_statistics_conversion() {
        let gc_stats = GcStatistics {
            gc_count: 10,
            gc_time_ms: 15.5,
            scavenge_count: 5,
            scavenge_time_ms: 8.2,
            mark_sweep_count: 3,
            mark_sweep_time_ms: 12.1,
        };
        
        let obj = gc_statistics_to_object(gc_stats).unwrap();
        // In a real test, we would verify the object contents
        // This is just a compilation test
    }
    
    #[test]
    fn test_uv_statistics_conversion() {
        let uv_stats = UvStatistics {
            handle_count: 20,
            request_count: 15,
            active_handles: 5,
            active_requests: 3,
        };
        
        let obj = uv_statistics_to_object(uv_stats).unwrap();
        // In a real test, we would verify the object contents
        // This is just a compilation test
    }
}