//! Configuration management module for XProfiler
//!
//! This module provides configuration storage and management functionality
//! that is compatible with the original C++ implementation.

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub mod store;

use store::ConfigStore;

/// Configuration value types supported by XProfiler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigValue {
  String(String),
  Integer(i64),
  Float(f64),
  Boolean(bool),
}

/// Configuration description for validation and documentation
#[derive(Debug, Clone)]
pub struct ConfigDescription {
  pub name: String,
  pub description: String,
  pub default_value: ConfigValue,
  pub validator: Option<fn(&ConfigValue) -> bool>,
}

/// Global configuration store instance
static CONFIG_STORE: Lazy<Arc<RwLock<ConfigStore>>> =
  Lazy::new(|| Arc::new(RwLock::new(ConfigStore::new())));

/// Initialize configuration with default values
pub fn initialize_config() -> Result<(), Box<dyn std::error::Error>> {
  let mut store = CONFIG_STORE
    .write()
    .map_err(|e| format!("Failed to acquire write lock: {}", e))?;
  store.initialize_defaults()?;
  Ok(())
}

/// Set a configuration value
pub fn set_config(key: &str, value: ConfigValue) -> Result<bool, Box<dyn std::error::Error>> {
  let mut store = CONFIG_STORE
    .write()
    .map_err(|e| format!("Failed to acquire write lock: {}", e))?;
  store.set(key, value)
}

/// Get a configuration value
pub fn get_config(key: &str) -> Result<Option<ConfigValue>, Box<dyn std::error::Error>> {
  let store = CONFIG_STORE
    .read()
    .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
  Ok(store.get(key))
}

/// Get all configuration values
pub fn get_all_config() -> Result<HashMap<String, ConfigValue>, Box<dyn std::error::Error>> {
  let store = CONFIG_STORE
    .read()
    .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
  Ok(store.get_all())
}

/// Validate configuration
pub fn validate_config() -> Result<Vec<String>, Box<dyn std::error::Error>> {
  let store = CONFIG_STORE
    .read()
    .map_err(|e| format!("Failed to acquire read lock: {}", e))?;
  Ok(store.validate())
}
