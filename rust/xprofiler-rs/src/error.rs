//! Error handling module for xprofiler-rs
//!
//! This module provides comprehensive error handling with platform-specific
//! optimizations and detailed error information.

use std::fmt;
use std::error::Error as StdError;

/// Main error type for xprofiler operations
#[derive(Debug, Clone)]
pub enum XProfilerError {
    /// Configuration related errors
    Config {
        message: String,
        key: Option<String>,
    },
    /// I/O related errors
    Io {
        message: String,
        path: Option<String>,
        kind: IoErrorKind,
    },
    /// System call errors (platform-specific)
    System {
        message: String,
        errno: Option<i32>,
        syscall: Option<String>,
    },
    /// Memory allocation errors
    Memory {
        message: String,
        requested_size: Option<usize>,
    },
    /// Threading and concurrency errors
    Threading {
        message: String,
        thread_id: Option<String>,
    },
    /// Monitoring specific errors
    Monitoring {
        message: String,
        monitor_type: MonitorType,
    },
    /// Logger errors
    Logger {
        message: String,
        level: Option<String>,
    },
    /// Platform-specific errors
    Platform {
        message: String,
        platform: String,
        details: Option<String>,
    },
    /// Generic errors for cases not covered above
    Generic {
        message: String,
    },
}

/// I/O error kinds for better error categorization
#[derive(Debug, Clone, PartialEq)]
pub enum IoErrorKind {
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    InvalidInput,
    InvalidData,
    TimedOut,
    WriteZero,
    Interrupted,
    UnexpectedEof,
    Other,
}

/// Monitor types for monitoring-specific errors
#[derive(Debug, Clone, PartialEq)]
pub enum MonitorType {
    Cpu,
    Memory,
    Gc,
    Http,
    Libuv,
    Environment,
    CpuProfiler,
    HeapProfiler,
    GcProfiler,
}

impl fmt::Display for XProfilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XProfilerError::Config { message, key } => {
                if let Some(k) = key {
                    write!(f, "Configuration error for key '{}': {}", k, message)
                } else {
                    write!(f, "Configuration error: {}", message)
                }
            }
            XProfilerError::Io { message, path, kind } => {
                let kind_str = match kind {
                    IoErrorKind::NotFound => "not found",
                    IoErrorKind::PermissionDenied => "permission denied",
                    IoErrorKind::InvalidInput => "invalid input",
                    IoErrorKind::InvalidData => "invalid data",
                    IoErrorKind::TimedOut => "timed out",
                    IoErrorKind::Other => "other",
                    _ => "unknown",
                };
                if let Some(p) = path {
                    write!(f, "I/O error ({}) for path '{}': {}", kind_str, p, message)
                } else {
                    write!(f, "I/O error ({}): {}", kind_str, message)
                }
            }
            XProfilerError::System { message, errno, syscall } => {
                let mut parts = vec![format!("System error: {}", message)];
                if let Some(e) = errno {
                    parts.push(format!("errno: {}", e));
                }
                if let Some(s) = syscall {
                    parts.push(format!("syscall: {}", s));
                }
                write!(f, "{}", parts.join(", "))
            }
            XProfilerError::Memory { message, requested_size } => {
                if let Some(size) = requested_size {
                    write!(f, "Memory error (requested {} bytes): {}", size, message)
                } else {
                    write!(f, "Memory error: {}", message)
                }
            }
            XProfilerError::Threading { message, thread_id } => {
                if let Some(id) = thread_id {
                    write!(f, "Threading error in thread '{}': {}", id, message)
                } else {
                    write!(f, "Threading error: {}", message)
                }
            }
            XProfilerError::Monitoring { message, monitor_type } => {
                write!(f, "Monitoring error ({:?}): {}", monitor_type, message)
            }
            XProfilerError::Logger { message, level } => {
                if let Some(l) = level {
                    write!(f, "Logger error (level {}): {}", l, message)
                } else {
                    write!(f, "Logger error: {}", message)
                }
            }
            XProfilerError::Platform { message, platform, details } => {
                let mut parts = vec![format!("Platform error ({}): {}", platform, message)];
                if let Some(d) = details {
                    parts.push(format!("details: {}", d));
                }
                write!(f, "{}", parts.join(", "))
            }
            XProfilerError::Generic { message } => {
                write!(f, "Error: {}", message)
            }
        }
    }
}

impl StdError for XProfilerError {}

/// Result type alias for xprofiler operations
pub type XProfilerResult<T> = Result<T, XProfilerError>;

/// Platform-specific error handling utilities
pub mod platform {
    use super::*;

    /// Convert system errno to XProfilerError
    #[cfg(unix)]
    pub fn errno_to_error(errno: i32, syscall: Option<&str>) -> XProfilerError {
        let message = unsafe {
            let ptr = libc::strerror(errno);
            if ptr.is_null() {
                format!("Unknown error (errno: {})", errno)
            } else {
                std::ffi::CStr::from_ptr(ptr)
                    .to_string_lossy()
                    .to_string()
            }
        };

        XProfilerError::System {
            message,
            errno: Some(errno),
            syscall: syscall.map(|s| s.to_string()),
        }
    }

    /// Convert Windows error code to XProfilerError
    #[cfg(windows)]
    pub fn win_error_to_error(error_code: u32, operation: Option<&str>) -> XProfilerError {
        let message = format!("Windows error code: {}", error_code);
        
        XProfilerError::Platform {
            message,
            platform: "Windows".to_string(),
            details: operation.map(|op| format!("operation: {}", op)),
        }
    }

    /// Get last system error
    #[cfg(unix)]
    pub fn last_error(syscall: Option<&str>) -> XProfilerError {
        let errno = unsafe { *libc::__error() };
        errno_to_error(errno, syscall)
    }

    /// Get last Windows error
    #[cfg(windows)]
    pub fn last_error(operation: Option<&str>) -> XProfilerError {
        use winapi::um::errhandlingapi::GetLastError;
        let error_code = unsafe { GetLastError() };
        win_error_to_error(error_code, operation)
    }
}

/// Error handling macros for common patterns
#[macro_export]
macro_rules! config_error {
    ($msg:expr) => {
        XProfilerError::Config {
            message: $msg.to_string(),
            key: None,
        }
    };
    ($key:expr, $msg:expr) => {
        XProfilerError::Config {
            message: $msg.to_string(),
            key: Some($key.to_string()),
        }
    };
}

#[macro_export]
macro_rules! io_error {
    ($kind:expr, $msg:expr) => {
        XProfilerError::Io {
            message: $msg.to_string(),
            path: None,
            kind: $kind,
        }
    };
    ($kind:expr, $path:expr, $msg:expr) => {
        XProfilerError::Io {
            message: $msg.to_string(),
            path: Some($path.to_string()),
            kind: $kind,
        }
    };
}

#[macro_export]
macro_rules! system_error {
    ($msg:expr) => {
        XProfilerError::System {
            message: $msg.to_string(),
            errno: None,
            syscall: None,
        }
    };
    ($msg:expr, $errno:expr) => {
        XProfilerError::System {
            message: $msg.to_string(),
            errno: Some($errno),
            syscall: None,
        }
    };
    ($msg:expr, $errno:expr, $syscall:expr) => {
        XProfilerError::System {
            message: $msg.to_string(),
            errno: Some($errno),
            syscall: Some($syscall.to_string()),
        }
    };
}



/// Convert standard library errors to XProfilerError
impl From<std::io::Error> for XProfilerError {
    fn from(err: std::io::Error) -> Self {
        let kind = match err.kind() {
            std::io::ErrorKind::NotFound => IoErrorKind::NotFound,
            std::io::ErrorKind::PermissionDenied => IoErrorKind::PermissionDenied,
            std::io::ErrorKind::ConnectionRefused => IoErrorKind::ConnectionRefused,
            std::io::ErrorKind::ConnectionAborted => IoErrorKind::ConnectionAborted,
            std::io::ErrorKind::NotConnected => IoErrorKind::NotConnected,
            std::io::ErrorKind::AddrInUse => IoErrorKind::AddrInUse,
            std::io::ErrorKind::AddrNotAvailable => IoErrorKind::AddrNotAvailable,
            std::io::ErrorKind::BrokenPipe => IoErrorKind::BrokenPipe,
            std::io::ErrorKind::AlreadyExists => IoErrorKind::AlreadyExists,
            std::io::ErrorKind::WouldBlock => IoErrorKind::WouldBlock,
            std::io::ErrorKind::InvalidInput => IoErrorKind::InvalidInput,
            std::io::ErrorKind::InvalidData => IoErrorKind::InvalidData,
            std::io::ErrorKind::TimedOut => IoErrorKind::TimedOut,
            std::io::ErrorKind::WriteZero => IoErrorKind::WriteZero,
            std::io::ErrorKind::Interrupted => IoErrorKind::Interrupted,
            std::io::ErrorKind::UnexpectedEof => IoErrorKind::UnexpectedEof,
            _ => IoErrorKind::Other,
        };

        XProfilerError::Io {
            message: err.to_string(),
            path: None,
            kind,
        }
    }
}

/// Convert from Box<dyn Error> to XProfilerError
impl From<Box<dyn StdError>> for XProfilerError {
    fn from(err: Box<dyn StdError>) -> Self {
        XProfilerError::Generic {
            message: err.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let config_err = XProfilerError::Config {
            message: "Invalid value".to_string(),
            key: Some("test_key".to_string()),
        };
        assert!(config_err.to_string().contains("test_key"));
        assert!(config_err.to_string().contains("Invalid value"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let xpf_err: XProfilerError = io_err.into();
        
        match xpf_err {
            XProfilerError::Io { kind, .. } => {
                assert_eq!(kind, IoErrorKind::NotFound);
            }
            _ => panic!("Expected IoError"),
        }
    }

    #[test]
    fn test_error_macros() {
        let err = config_error!("test_key", "test message");
        match err {
            XProfilerError::Config { key, message } => {
                assert_eq!(key, Some("test_key".to_string()));
                assert_eq!(message, "test message");
            }
            _ => panic!("Expected Config error"),
        }
    }
}