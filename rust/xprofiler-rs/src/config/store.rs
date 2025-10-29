//! Configuration store implementation

use super::{ConfigDescription, ConfigValue};
use std::collections::HashMap;

/// Configuration store that manages all XProfiler settings
pub struct ConfigStore {
  configs: HashMap<String, ConfigValue>,
  descriptions: HashMap<String, ConfigDescription>,
}

impl ConfigStore {
  /// Create a new configuration store
  pub fn new() -> Self {
    Self {
      configs: HashMap::new(),
      descriptions: HashMap::new(),
    }
  }

  /// Initialize with default configuration values
  pub fn initialize_defaults(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    // Core configuration
    self.register_config(
      "log_dir",
      ConfigValue::String("/tmp".to_string()),
      "Log directory path",
      None,
    )?;
    self.register_config(
      "log_interval",
      ConfigValue::Integer(60),
      "Log interval in seconds",
      Some(validate_positive_integer),
    )?;
    self.register_config(
      "log_format_alinode",
      ConfigValue::Boolean(false),
      "Use alinode log format",
      None,
    )?;
    self.register_config(
      "log_type",
      ConfigValue::Integer(0),
      "Log type (0=file, 1=console)",
      Some(validate_log_type),
    )?;
    self.register_config(
      "log_level",
      ConfigValue::Integer(1),
      "Log level (0=debug, 1=info, 2=warn, 3=error)",
      Some(validate_log_level),
    )?;

    // Monitoring configuration
    self.register_config(
      "enable_log_uv_handles",
      ConfigValue::Boolean(true),
      "Enable UV handles logging",
      None,
    )?;
    self.register_config(
      "log_bypass_cpu",
      ConfigValue::Boolean(true),
      "Enable CPU monitoring",
      None,
    )?;
    self.register_config(
      "log_bypass_memory",
      ConfigValue::Boolean(true),
      "Enable memory monitoring",
      None,
    )?;
    self.register_config(
      "log_bypass_gc",
      ConfigValue::Boolean(true),
      "Enable GC monitoring",
      None,
    )?;
    self.register_config(
      "log_bypass_http",
      ConfigValue::Boolean(true),
      "Enable HTTP monitoring",
      None,
    )?;

    // Performance configuration
    self.register_config(
      "sampling_interval",
      ConfigValue::Integer(1000),
      "Sampling interval in microseconds",
      Some(validate_positive_integer),
    )?;
    self.register_config(
      "sampling_duration",
      ConfigValue::Integer(10000),
      "Sampling duration in microseconds",
      Some(validate_positive_integer),
    )?;

    // Advanced configuration
    self.register_config(
      "enable_fatal_error_hook",
      ConfigValue::Boolean(true),
      "Enable fatal error hook",
      None,
    )?;
    self.register_config(
      "enable_fatal_error_report",
      ConfigValue::Boolean(true),
      "Enable fatal error report",
      None,
    )?;
    self.register_config(
      "fatal_error_report_dir",
      ConfigValue::String("/tmp".to_string()),
      "Fatal error report directory",
      None,
    )?;

    Ok(())
  }

  /// Register a configuration with description and validator
  pub fn register_config(
    &mut self,
    key: &str,
    default_value: ConfigValue,
    description: &str,
    validator: Option<fn(&ConfigValue) -> bool>,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let desc = ConfigDescription {
      name: key.to_string(),
      description: description.to_string(),
      default_value: default_value.clone(),
      validator,
    };

    self.descriptions.insert(key.to_string(), desc);
    self.configs.insert(key.to_string(), default_value);

    Ok(())
  }

  /// Set a configuration value
  pub fn set(&mut self, key: &str, value: ConfigValue) -> Result<bool, Box<dyn std::error::Error>> {
    // Validate if description exists
    if let Some(desc) = self.descriptions.get(key) {
      if let Some(validator) = desc.validator {
        if !validator(&value) {
          return Err(format!("Invalid value for config '{}'", key).into());
        }
      }
    }

    let existed = self.configs.contains_key(key);
    self.configs.insert(key.to_string(), value);
    Ok(existed)
  }

  /// Get a configuration value
  pub fn get(&self, key: &str) -> Option<ConfigValue> {
    self.configs.get(key).cloned()
  }

  /// Get all configuration values
  pub fn get_all(&self) -> HashMap<String, ConfigValue> {
    self.configs.clone()
  }

  /// Validate all configurations
  pub fn validate(&self) -> Vec<String> {
    let mut errors = Vec::new();

    for (key, value) in &self.configs {
      if let Some(desc) = self.descriptions.get(key) {
        if let Some(validator) = desc.validator {
          if !validator(value) {
            errors.push(format!("Invalid value for config '{}': {:?}", key, value));
          }
        }
      }
    }

    errors
  }
}

// Validator functions
fn validate_positive_integer(value: &ConfigValue) -> bool {
  match value {
    ConfigValue::Integer(i) => *i > 0,
    _ => false,
  }
}

fn validate_log_type(value: &ConfigValue) -> bool {
  match value {
    ConfigValue::Integer(i) => *i >= 0 && *i <= 1,
    _ => false,
  }
}

fn validate_log_level(value: &ConfigValue) -> bool {
  match value {
    ConfigValue::Integer(i) => *i >= 0 && *i <= 3,
    _ => false,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_config_store_creation() {
    let store = ConfigStore::new();
    assert!(store.configs.is_empty());
    assert!(store.descriptions.is_empty());
  }

  #[test]
  fn test_config_initialization() {
    let mut store = ConfigStore::new();
    store.initialize_defaults().unwrap();

    assert!(store.get("log_dir").is_some());
    assert!(store.get("log_interval").is_some());
  }

  #[test]
  fn test_config_validation() {
    let mut store = ConfigStore::new();
    store.initialize_defaults().unwrap();

    // Valid value
    assert!(store.set("log_interval", ConfigValue::Integer(30)).is_ok());

    // Invalid value
    assert!(store.set("log_interval", ConfigValue::Integer(-1)).is_err());
  }
}
