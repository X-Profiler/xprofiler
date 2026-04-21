use napi_derive::napi;
use napi::{Env, JsObject, Result, JsUnknown};
use std::collections::HashMap;
use std::sync::RwLock;
use serde_json::Value;

lazy_static::lazy_static! {
    pub static ref CONFIG_STORE: RwLock<HashMap<String, ConfigItem>> = RwLock::new(HashMap::new());
}

#[derive(Clone)]
pub struct ConfigItem {
    pub format: String,
    pub configurable: bool,
    pub value: Value,
}

#[napi]
pub fn configure(env: Env, configuration: Vec<JsObject>) -> Result<bool> {
    let mut store = CONFIG_STORE.write().unwrap();
    
    for item in configuration {
        let name: String = item.get_named_property("name")?;
        let format: String = item.get_named_property("format")?;
        let configurable: bool = item.get_named_property("configurable")?;
        
        let value_js: Option<JsUnknown> = item.get_named_property("value")?;
        let value = if let Some(v) = value_js {
            if format == "string" {
                if let Ok(s) = v.coerce_to_string() {
                    Value::String(s.into_utf8()?.into_owned()?)
                } else {
                    Value::Null
                }
            } else if format == "number" {
                if let Ok(n) = v.coerce_to_number() {
                    Value::Number(serde_json::Number::from_f64(n.get_double()?).unwrap())
                } else {
                    Value::Null
                }
            } else if format == "boolean" {
                if let Ok(b) = v.coerce_to_bool() {
                    Value::Bool(b.get_value()?)
                } else {
                    Value::Null
                }
            } else {
                Value::Null
            }
        } else {
            Value::Null
        };

        // If the item exists, just update its value, else insert
        store.insert(name, ConfigItem {
            format,
            configurable,
            value,
        });
    }

    Ok(true)
}

#[napi]
pub fn get_config(env: Env) -> Result<JsObject> {
    let store = CONFIG_STORE.read().unwrap();
    let mut obj = env.create_object()?;
    
    for (key, item) in store.iter() {
        match &item.value {
            Value::String(s) => obj.set_named_property(key, env.create_string(s)?)?,
            Value::Number(n) => {
                if let Some(f) = n.as_f64() {
                    obj.set_named_property(key, env.create_double(f)?)?;
                }
            },
            Value::Bool(b) => obj.set_named_property(key, env.get_boolean(*b)?)?,
            _ => {}
        }
    }
    
    Ok(obj)
}

pub fn get_config_string(key: &str) -> String {
    let store = CONFIG_STORE.read().unwrap();
    if let Some(item) = store.get(key) {
        if let Value::String(s) = &item.value {
            return s.clone();
        }
    }
    String::new()
}

pub fn get_config_bool(key: &str) -> bool {
    let store = CONFIG_STORE.read().unwrap();
    if let Some(item) = store.get(key) {
        if let Value::Bool(b) = &item.value {
            return *b;
        }
    }
    false
}

pub fn get_config_number(key: &str) -> u32 {
    let store = CONFIG_STORE.read().unwrap();
    if let Some(item) = store.get(key) {
        if let Value::Number(n) = &item.value {
            if let Some(f) = n.as_u64() {
                return f as u32;
            }
        }
    }
    0
}
