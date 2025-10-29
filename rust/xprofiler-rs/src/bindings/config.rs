//! Configuration bindings for JavaScript interface

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;
use serde_json::Value;

use crate::config::{CONFIG_STORE, ConfigValue, ConfigDescription};

/// Configuration management class for JavaScript
#[napi]
pub struct ConfigManager {
    initialized: bool,
}

#[napi]
impl ConfigManager {
    /// Create a new configuration manager
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            initialized: true,
        }
    }
    
    /// Initialize configuration with default values
    #[napi]
    pub fn initialize(&mut self) -> Result<()> {
        CONFIG_STORE.initialize();
        self.initialized = true;
        Ok(())
    }
    
    /// Check if configuration is initialized
    #[napi]
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    /// Set a configuration value
    #[napi]
    pub fn set(&self, key: String, value: Unknown) -> Result<()> {
        let config_value = js_value_to_config_value(value)?;
        CONFIG_STORE.set(&key, config_value)
            .map_err(|e| Error::new(Status::InvalidArg, format!("Config error: {}", e)))?;
        Ok(())
    }
    
    /// Get a configuration value
    #[napi]
    pub fn get(&self, key: String) -> Result<Unknown> {
        match CONFIG_STORE.get(&key) {
            Some(value) => config_value_to_js_value(value),
            None => Ok(Unknown::from_undefined()),
        }
    }
    
    /// Get all configuration values
    #[napi]
    pub fn get_all(&self) -> Result<Object> {
        let config = CONFIG_STORE.get_all();
        let mut result = Object::new();
        
        for (key, value) in config {
            let js_value = config_value_to_js_value(value)?;
            result.set(&key, js_value)?;
        }
        
        Ok(result)
    }
    
    /// Check if a configuration key exists
    #[napi]
    pub fn has(&self, key: String) -> bool {
        CONFIG_STORE.get(&key).is_some()
    }
    
    /// Remove a configuration key
    #[napi]
    pub fn remove(&self, key: String) -> bool {
        CONFIG_STORE.remove(&key)
    }
    
    /// Clear all configuration
    #[napi]
    pub fn clear(&self) -> Result<()> {
        CONFIG_STORE.clear();
        Ok(())
    }
    
    /// Register a new configuration option
    #[napi]
    pub fn register(
        &self,
        key: String,
        default_value: Unknown,
        description: Option<String>,
        validator: Option<String>,
    ) -> Result<()> {
        let config_value = js_value_to_config_value(default_value)?;
        let desc = ConfigDescription {
            description: description.unwrap_or_default(),
            validator: validator.unwrap_or_default(),
        };
        
        CONFIG_STORE.register(&key, config_value, desc)
            .map_err(|e| Error::new(Status::InvalidArg, format!("Registration error: {}", e)))?;
        
        Ok(())
    }
    
    /// Get configuration description
    #[napi]
    pub fn get_description(&self, key: String) -> Result<Object> {
        match CONFIG_STORE.get_description(&key) {
            Some(desc) => {
                let mut result = Object::new();
                result.set("description", desc.description)?;
                result.set("validator", desc.validator)?;
                Ok(result)
            }
            None => Err(Error::new(Status::InvalidArg, "Configuration key not found")),
        }
    }
    
    /// Validate a configuration value
    #[napi]
    pub fn validate(&self, key: String, value: Unknown) -> Result<bool> {
        let config_value = js_value_to_config_value(value)?;
        Ok(CONFIG_STORE.validate(&key, &config_value))
    }
    
    /// Get all registered configuration keys
    #[napi]
    pub fn get_keys(&self) -> Vec<String> {
        CONFIG_STORE.get_keys()
    }
    
    /// Load configuration from JSON string
    #[napi]
    pub fn load_from_json(&self, json_str: String) -> Result<()> {
        let config: HashMap<String, Value> = serde_json::from_str(&json_str)
            .map_err(|e| Error::new(Status::InvalidArg, format!("JSON parse error: {}", e)))?;
        
        for (key, value) in config {
            let config_value = json_value_to_config_value(value)?;
            CONFIG_STORE.set(&key, config_value)
                .map_err(|e| Error::new(Status::InvalidArg, format!("Config error: {}", e)))?;
        }
        
        Ok(())
    }
    
    /// Export configuration to JSON string
    #[napi]
    pub fn export_to_json(&self) -> Result<String> {
        let config = CONFIG_STORE.get_all();
        let mut json_config = HashMap::new();
        
        for (key, value) in config {
            let json_value = config_value_to_json_value(value);
            json_config.insert(key, json_value);
        }
        
        serde_json::to_string_pretty(&json_config)
            .map_err(|e| Error::new(Status::GenericFailure, format!("JSON serialize error: {}", e)))
    }
    
    /// Set multiple configuration values at once
    #[napi]
    pub fn set_multiple(&self, config: Object) -> Result<()> {
        let config_map = object_to_hashmap(config)?;
        
        for (key, value) in config_map {
            let config_value = js_value_to_config_value(value)?;
            CONFIG_STORE.set(&key, config_value)
                .map_err(|e| Error::new(Status::InvalidArg, format!("Config error for key '{}': {}", key, e)))?;
        }
        
        Ok(())
    }
    
    /// Get configuration values matching a pattern
    #[napi]
    pub fn get_matching(&self, pattern: String) -> Result<Object> {
        let config = CONFIG_STORE.get_all();
        let mut result = Object::new();
        
        for (key, value) in config {
            if key.contains(&pattern) {
                let js_value = config_value_to_js_value(value)?;
                result.set(&key, js_value)?;
            }
        }
        
        Ok(result)
    }
    
    /// Reset configuration to defaults
    #[napi]
    pub fn reset_to_defaults(&self) -> Result<()> {
        CONFIG_STORE.reset_to_defaults();
        Ok(())
    }
    
    /// Get configuration statistics
    #[napi]
    pub fn get_stats(&self) -> Result<Object> {
        let config = CONFIG_STORE.get_all();
        let mut result = Object::new();
        
        result.set("total_keys", config.len() as u32)?;
        
        let mut type_counts = HashMap::new();
        for (_, value) in config {
            let type_name = match value {
                ConfigValue::String(_) => "string",
                ConfigValue::Integer(_) => "integer",
                ConfigValue::Float(_) => "float",
                ConfigValue::Boolean(_) => "boolean",
            };
            *type_counts.entry(type_name).or_insert(0) += 1;
        }
        
        let mut types_obj = Object::new();
        for (type_name, count) in type_counts {
            types_obj.set(type_name, count)?;
        }
        result.set("types", types_obj)?;
        
        Ok(result)
    }
}

/// Standalone configuration functions

/// Set configuration value (standalone function)
#[napi]
pub fn set_config(key: String, value: Unknown) -> Result<()> {
    let config_value = js_value_to_config_value(value)?;
    CONFIG_STORE.set(&key, config_value)
        .map_err(|e| Error::new(Status::InvalidArg, format!("Config error: {}", e)))?;
    Ok(())
}

/// Get configuration value (standalone function)
#[napi]
pub fn get_config(key: String) -> Result<Unknown> {
    match CONFIG_STORE.get(&key) {
        Some(value) => config_value_to_js_value(value),
        None => Ok(Unknown::from_undefined()),
    }
}

/// Get all configuration (standalone function)
#[napi]
pub fn get_all_config() -> Result<Object> {
    let config = CONFIG_STORE.get_all();
    let mut result = Object::new();
    
    for (key, value) in config {
        let js_value = config_value_to_js_value(value)?;
        result.set(&key, js_value)?;
    }
    
    Ok(result)
}

/// Initialize configuration with defaults (standalone function)
#[napi]
pub fn init_config() -> Result<()> {
    CONFIG_STORE.initialize();
    Ok(())
}

/// Utility functions

fn object_to_hashmap(obj: Object) -> Result<HashMap<String, Unknown>> {
    let mut map = HashMap::new();
    let keys = obj.get_property_names()?;
    
    for i in 0..keys.len() {
        if let Some(key) = keys.get::<String>(i)? {
            if let Some(value) = obj.get::<Unknown>(&key)? {
                map.insert(key, value);
            }
        }
    }
    
    Ok(map)
}

fn js_value_to_config_value(value: Unknown) -> Result<ConfigValue> {
    if value.is_string() {
        Ok(ConfigValue::String(value.coerce_to_string()?.into_utf8()?.as_str()?.to_string()))
    } else if value.is_number() {
        if let Ok(int_val) = value.coerce_to_number()?.get_int64() {
            Ok(ConfigValue::Integer(int_val))
        } else {
            let float_val = value.coerce_to_number()?.get_double()?;
            Ok(ConfigValue::Float(float_val))
        }
    } else if value.is_boolean() {
        Ok(ConfigValue::Boolean(value.coerce_to_bool()?.get_value()?))
    } else {
        Err(Error::new(Status::InvalidArg, "Unsupported config value type"))
    }
}

fn config_value_to_js_value(value: ConfigValue) -> Result<Unknown> {
    match value {
        ConfigValue::String(s) => Ok(Unknown::from_string(s)?),
        ConfigValue::Integer(i) => Ok(Unknown::from_i64(i)?),
        ConfigValue::Float(f) => Ok(Unknown::from_f64(f)?),
        ConfigValue::Boolean(b) => Ok(Unknown::from_bool(b)?),
    }
}

fn json_value_to_config_value(value: Value) -> Result<ConfigValue> {
    match value {
        Value::String(s) => Ok(ConfigValue::String(s)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(ConfigValue::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(ConfigValue::Float(f))
            } else {
                Err(Error::new(Status::InvalidArg, "Invalid number value"))
            }
        }
        Value::Bool(b) => Ok(ConfigValue::Boolean(b)),
        _ => Err(Error::new(Status::InvalidArg, "Unsupported JSON value type")),
    }
}

fn config_value_to_json_value(value: ConfigValue) -> Value {
    match value {
        ConfigValue::String(s) => Value::String(s),
        ConfigValue::Integer(i) => Value::Number(serde_json::Number::from(i)),
        ConfigValue::Float(f) => Value::Number(serde_json::Number::from_f64(f).unwrap_or_default()),
        ConfigValue::Boolean(b) => Value::Bool(b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_manager_creation() {
        let manager = ConfigManager::new();
        assert!(manager.is_initialized());
    }
    
    #[test]
    fn test_config_value_conversion() {
        // Test string conversion
        let string_val = ConfigValue::String("test".to_string());
        let json_val = config_value_to_json_value(string_val);
        assert_eq!(json_val, Value::String("test".to_string()));
        
        // Test integer conversion
        let int_val = ConfigValue::Integer(42);
        let json_val = config_value_to_json_value(int_val);
        assert_eq!(json_val, Value::Number(serde_json::Number::from(42)));
        
        // Test boolean conversion
        let bool_val = ConfigValue::Boolean(true);
        let json_val = config_value_to_json_value(bool_val);
        assert_eq!(json_val, Value::Bool(true));
    }
    
    #[test]
    fn test_json_to_config_value_conversion() {
        // Test string conversion
        let json_val = Value::String("test".to_string());
        let config_val = json_value_to_config_value(json_val).unwrap();
        assert!(matches!(config_val, ConfigValue::String(s) if s == "test"));
        
        // Test integer conversion
        let json_val = Value::Number(serde_json::Number::from(42));
        let config_val = json_value_to_config_value(json_val).unwrap();
        assert!(matches!(config_val, ConfigValue::Integer(i) if i == 42));
        
        // Test boolean conversion
        let json_val = Value::Bool(true);
        let config_val = json_value_to_config_value(json_val).unwrap();
        assert!(matches!(config_val, ConfigValue::Boolean(b) if b));
    }
}