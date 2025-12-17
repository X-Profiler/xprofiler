//! Error types for xprofiler-rs

use thiserror::Error;

/// Error types for xprofiler operations
#[derive(Error, Debug)]
pub enum XprofilerError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Logger error: {0}")]
    LoggerError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Environment not initialized")]
    EnvironmentNotInitialized,

    #[error("Thread not found: {0}")]
    ThreadNotFound(i64),

    #[error("IPC error: {0}")]
    IpcError(String),

    #[error("Profiler error: {0}")]
    ProfilerError(String),

    #[error("Command error: {0}")]
    CommandError(String),

    #[error("Platform error: {0}")]
    PlatformError(String),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

impl From<XprofilerError> for napi::Error {
    fn from(err: XprofilerError) -> Self {
        napi::Error::from_reason(err.to_string())
    }
}

/// Result type alias for xprofiler operations
pub type XprofilerResult<T> = std::result::Result<T, XprofilerError>;
