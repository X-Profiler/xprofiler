//! Environment data registry for managing multiple threads

use std::collections::HashMap;
use super::data::EnvironmentData;

/// Registry for managing environment data across multiple threads
pub struct EnvironmentRegistry {
    thread_data: HashMap<u32, EnvironmentData>,
    main_thread_id: Option<u32>,
}

impl EnvironmentRegistry {
    /// Create a new environment registry
    pub fn new() -> Self {
        Self {
            thread_data: HashMap::new(),
            main_thread_id: None,
        }
    }

    /// Register environment data for a thread
    pub fn register_thread(
        &mut self,
        thread_id: u32,
        env_data: EnvironmentData,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Set main thread ID if this is the main thread
        if env_data.is_main_thread {
            self.main_thread_id = Some(thread_id);
        }

        self.thread_data.insert(thread_id, env_data);
        Ok(())
    }

    /// Unregister a thread
    pub fn unregister_thread(&mut self, thread_id: u32) -> Option<EnvironmentData> {
        // Clear main thread ID if this was the main thread
        if self.main_thread_id == Some(thread_id) {
            self.main_thread_id = None;
        }

        self.thread_data.remove(&thread_id)
    }

    /// Get environment data for a specific thread
    pub fn get_thread_data(&self, thread_id: u32) -> Option<EnvironmentData> {
        self.thread_data.get(&thread_id).cloned()
    }

    /// Get mutable reference to environment data for a specific thread
    pub fn get_thread_data_mut(&mut self, thread_id: u32) -> Option<&mut EnvironmentData> {
        self.thread_data.get_mut(&thread_id)
    }

    /// Get environment data for the main thread
    pub fn get_main_thread_data(&self) -> Option<EnvironmentData> {
        self.main_thread_id
            .and_then(|id| self.thread_data.get(&id).cloned())
    }

    /// Get all thread environment data
    pub fn get_all_thread_data(&self) -> HashMap<u32, EnvironmentData> {
        self.thread_data.clone()
    }

    /// Get all worker thread environment data (excluding main thread)
    pub fn get_worker_thread_data(&self) -> HashMap<u32, EnvironmentData> {
        self.thread_data
            .iter()
            .filter(|(_, env_data)| !env_data.is_main_thread)
            .map(|(id, env_data)| (*id, env_data.clone()))
            .collect()
    }

    /// Get the number of registered threads
    pub fn thread_count(&self) -> usize {
        self.thread_data.len()
    }

    /// Get the number of worker threads
    pub fn worker_thread_count(&self) -> usize {
        self.thread_data
            .values()
            .filter(|env_data| !env_data.is_main_thread)
            .count()
    }

    /// Check if a thread is registered
    pub fn is_thread_registered(&self, thread_id: u32) -> bool {
        self.thread_data.contains_key(&thread_id)
    }

    /// Get the main thread ID
    pub fn get_main_thread_id(&self) -> Option<u32> {
        self.main_thread_id
    }

    /// Update environment data for a thread
    pub fn update_thread_data(
        &mut self,
        thread_id: u32,
        updater: impl FnOnce(&mut EnvironmentData),
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(env_data) = self.thread_data.get_mut(&thread_id) {
            updater(env_data);
            Ok(())
        } else {
            Err(format!("Thread {} not found in registry", thread_id).into())
        }
    }

    /// Clean up stale thread data
    pub fn cleanup_stale_threads(&mut self, threshold_ms: u64) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
        let mut stale_threads = Vec::new();
        
        // Find stale threads
        for (thread_id, env_data) in &self.thread_data {
            if env_data.is_stale(threshold_ms)? {
                stale_threads.push(*thread_id);
            }
        }
        
        // Remove stale threads
        for thread_id in &stale_threads {
            self.unregister_thread(*thread_id);
        }
        
        Ok(stale_threads)
    }

    /// Get registry statistics
    pub fn get_statistics(&self) -> RegistryStatistics {
        let total_threads = self.thread_count();
        let worker_threads = self.worker_thread_count();
        let main_threads = if self.main_thread_id.is_some() { 1 } else { 0 };
        
        let total_heap_size: u64 = self.thread_data
            .values()
            .map(|env_data| env_data.heap_statistics.used_heap_size)
            .sum();
        
        let total_gc_count: u32 = self.thread_data
            .values()
            .map(|env_data| env_data.gc_statistics.gc_count_full + env_data.gc_statistics.gc_count_incremental)
            .sum();
        
        let total_uv_handles: u32 = self.thread_data
            .values()
            .map(|env_data| env_data.uv_statistics.active_handles)
            .sum();

        RegistryStatistics {
            total_threads,
            main_threads,
            worker_threads,
            total_heap_size,
            total_gc_count,
            total_uv_handles,
        }
    }

    /// Get a summary of all registered threads
    pub fn summary(&self) -> String {
        let stats = self.get_statistics();
        let mut summary = format!(
            "Registry: {} threads ({} main, {} workers)\n",
            stats.total_threads, stats.main_threads, stats.worker_threads
        );
        
        summary.push_str(&format!(
            "Total: heap={:.2}MB, gc_count={}, uv_handles={}\n",
            stats.total_heap_size as f64 / 1024.0 / 1024.0,
            stats.total_gc_count,
            stats.total_uv_handles
        ));
        
        for (thread_id, env_data) in &self.thread_data {
            summary.push_str(&format!("  {}: {}\n", thread_id, env_data.summary()));
        }
        
        summary
    }
}

/// Registry statistics
#[derive(Debug, Clone)]
pub struct RegistryStatistics {
    pub total_threads: usize,
    pub main_threads: usize,
    pub worker_threads: usize,
    pub total_heap_size: u64,
    pub total_gc_count: u32,
    pub total_uv_handles: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::data::EnvironmentData;

    #[test]
    fn test_registry_creation() {
        let registry = EnvironmentRegistry::new();
        assert_eq!(registry.thread_count(), 0);
        assert!(registry.get_main_thread_id().is_none());
    }

    #[test]
    fn test_thread_registration() {
        let mut registry = EnvironmentRegistry::new();
        let env_data = EnvironmentData::new(1, true, "v18.0.0".to_string()).unwrap();
        
        registry.register_thread(1, env_data).unwrap();
        
        assert_eq!(registry.thread_count(), 1);
        assert_eq!(registry.get_main_thread_id(), Some(1));
        assert!(registry.is_thread_registered(1));
    }

    #[test]
    fn test_thread_unregistration() {
        let mut registry = EnvironmentRegistry::new();
        let env_data = EnvironmentData::new(1, true, "v18.0.0".to_string()).unwrap();
        
        registry.register_thread(1, env_data).unwrap();
        let removed = registry.unregister_thread(1);
        
        assert!(removed.is_some());
        assert_eq!(registry.thread_count(), 0);
        assert!(registry.get_main_thread_id().is_none());
    }

    #[test]
    fn test_worker_thread_count() {
        let mut registry = EnvironmentRegistry::new();
        
        // Add main thread
        let main_env = EnvironmentData::new(1, true, "v18.0.0".to_string()).unwrap();
        registry.register_thread(1, main_env).unwrap();
        
        // Add worker threads
        let worker1_env = EnvironmentData::new(2, false, "v18.0.0".to_string()).unwrap();
        let worker2_env = EnvironmentData::new(3, false, "v18.0.0".to_string()).unwrap();
        registry.register_thread(2, worker1_env).unwrap();
        registry.register_thread(3, worker2_env).unwrap();
        
        assert_eq!(registry.thread_count(), 3);
        assert_eq!(registry.worker_thread_count(), 2);
    }
}