use napi_derive::napi;
use napi::{Env, JsObject, Result};
use crate::config::{CONFIG_STORE};

#[napi]
pub fn get_xprofiler_config(env: Env) -> Result<JsObject> {
    crate::config::get_config(env)
}

#[napi]
pub fn set_xprofiler_config(env: Env, options: JsObject) -> Result<JsObject> {
    // A simplified placeholder implementation since the real implementation
    // would be part of the command parser which is complex.
    // For now, we just return the current config to satisfy the interface.
    crate::config::get_config(env)
}
