//! Command system for xprofiler-rs
//!
//! This module handles commands from xprofctl CLI via IPC.

pub mod listener;
pub mod parser;

use serde::{Deserialize, Serialize};

/// Command request from xprofctl
#[derive(Debug, Deserialize)]
pub struct CommandRequest {
    pub traceid: String,
    pub cmd: String,
    #[serde(default)]
    pub thread_id: Option<i64>,
    #[serde(default)]
    pub options: Option<CommandOptions>,
}

/// Command options
#[derive(Debug, Deserialize, Default)]
pub struct CommandOptions {
    #[serde(default)]
    pub profiling_time: Option<u64>,
    #[serde(default)]
    pub filepath: Option<String>,
}

/// Command response
#[derive(Debug, Serialize)]
pub struct CommandResponse {
    pub ok: bool,
    pub traceid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl CommandResponse {
    pub fn success(traceid: &str, data: Option<serde_json::Value>) -> Self {
        Self {
            ok: true,
            traceid: traceid.to_string(),
            data,
            message: None,
        }
    }

    pub fn error(traceid: &str, message: &str) -> Self {
        Self {
            ok: false,
            traceid: traceid.to_string(),
            data: None,
            message: Some(message.to_string()),
        }
    }
}
