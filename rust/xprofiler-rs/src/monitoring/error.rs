//! Error handling for monitoring modules
//!
//! This module provides comprehensive error types and handling for all monitoring operations.

use std::error::Error as StdError;
use std::fmt;

/// Main error type for monitoring operations
#[derive(Debug)]
pub enum MonitoringError {
  /// System call failed
  SystemCall {
    operation: String,
    source: Box<dyn StdError + Send + Sync>,
  },
  /// Lock acquisition failed
  LockFailed { resource: String, details: String },
  /// Platform not supported
  UnsupportedPlatform { operation: String, platform: String },
  /// Invalid configuration
  InvalidConfig {
    parameter: String,
    value: String,
    reason: String,
  },
  /// Resource not available
  ResourceUnavailable { resource: String, reason: String },
  /// Data parsing failed
  ParseError {
    data_type: String,
    input: String,
    source: Box<dyn StdError + Send + Sync>,
  },
  /// Monitoring not initialized
  NotInitialized { module: String },
  /// Operation timeout
  Timeout { operation: String, duration_ms: u64 },
  /// Memory allocation failed
  OutOfMemory { requested_bytes: usize },
  /// IO operation failed
  IoError {
    operation: String,
    source: std::io::Error,
  },
}

impl fmt::Display for MonitoringError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      MonitoringError::SystemCall { operation, source } => {
        write!(f, "System call '{}' failed: {}", operation, source)
      }
      MonitoringError::LockFailed { resource, details } => {
        write!(f, "Failed to acquire lock on '{}': {}", resource, details)
      }
      MonitoringError::UnsupportedPlatform {
        operation,
        platform,
      } => {
        write!(
          f,
          "Operation '{}' not supported on platform '{}'",
          operation, platform
        )
      }
      MonitoringError::InvalidConfig {
        parameter,
        value,
        reason,
      } => {
        write!(
          f,
          "Invalid configuration for '{}' = '{}': {}",
          parameter, value, reason
        )
      }
      MonitoringError::ResourceUnavailable { resource, reason } => {
        write!(f, "Resource '{}' unavailable: {}", resource, reason)
      }
      MonitoringError::ParseError {
        data_type,
        input,
        source,
      } => {
        write!(
          f,
          "Failed to parse {} from '{}': {}",
          data_type, input, source
        )
      }
      MonitoringError::NotInitialized { module } => {
        write!(f, "Monitoring module '{}' not initialized", module)
      }
      MonitoringError::Timeout {
        operation,
        duration_ms,
      } => {
        write!(
          f,
          "Operation '{}' timed out after {}ms",
          operation, duration_ms
        )
      }
      MonitoringError::OutOfMemory { requested_bytes } => {
        write!(
          f,
          "Out of memory: failed to allocate {} bytes",
          requested_bytes
        )
      }
      MonitoringError::IoError { operation, source } => {
        write!(f, "IO error during '{}': {}", operation, source)
      }
    }
  }
}

impl StdError for MonitoringError {
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    match self {
      MonitoringError::SystemCall { source, .. } => Some(source.as_ref()),
      MonitoringError::ParseError { source, .. } => Some(source.as_ref()),
      MonitoringError::IoError { source, .. } => Some(source),
      _ => None,
    }
  }
}

/// Result type for monitoring operations
pub type MonitoringResult<T> = Result<T, MonitoringError>;

/// Helper trait for converting errors to MonitoringError
pub trait IntoMonitoringError<T> {
  fn with_context(self, operation: &str) -> MonitoringResult<T>;
  fn with_lock_context(self, resource: &str) -> MonitoringResult<T>;
  fn with_parse_context(self, data_type: &str, input: &str) -> MonitoringResult<T>;
}

impl<T, E> IntoMonitoringError<T> for Result<T, E>
where
  E: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static,
{
  fn with_context(self, operation: &str) -> MonitoringResult<T> {
    self.map_err(|e| MonitoringError::SystemCall {
      operation: operation.to_string(),
      source: Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        e.to_string(),
      )),
    })
  }

  fn with_lock_context(self, resource: &str) -> MonitoringResult<T> {
    self.map_err(|e| MonitoringError::LockFailed {
      resource: resource.to_string(),
      details: e.to_string(),
    })
  }

  fn with_parse_context(self, data_type: &str, input: &str) -> MonitoringResult<T> {
    self.map_err(|e| MonitoringError::ParseError {
      data_type: data_type.to_string(),
      input: input.to_string(),
      source: Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        e.to_string(),
      )),
    })
  }
}

/// Convert IO errors to MonitoringError
impl From<std::io::Error> for MonitoringError {
  fn from(err: std::io::Error) -> Self {
    MonitoringError::IoError {
      operation: "unknown".to_string(),
      source: err,
    }
  }
}

/// Convert string errors to MonitoringError
impl From<&str> for MonitoringError {
  fn from(err: &str) -> Self {
    MonitoringError::SystemCall {
      operation: "unknown".to_string(),
      source: Box::new(std::io::Error::new(std::io::ErrorKind::Other, err)),
    }
  }
}

/// Convert String errors to MonitoringError
impl From<String> for MonitoringError {
  fn from(err: String) -> Self {
    MonitoringError::SystemCall {
      operation: "unknown".to_string(),
      source: Box::new(std::io::Error::new(std::io::ErrorKind::Other, err)),
    }
  }
}

impl From<std::time::SystemTimeError> for MonitoringError {
  fn from(error: std::time::SystemTimeError) -> Self {
    MonitoringError::SystemCall {
      operation: "system_time".to_string(),
      source: Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        error.to_string(),
      )),
    }
  }
}

/// Convenience macros for error creation
#[macro_export]
macro_rules! monitoring_error {
  (system_call, $op:expr, $err:expr) => {
    MonitoringError::SystemCall {
      operation: $op.to_string(),
      source: Box::new($err),
    }
  };
  (lock_failed, $resource:expr, $details:expr) => {
    MonitoringError::LockFailed {
      resource: $resource.to_string(),
      details: $details.to_string(),
    }
  };
  (unsupported_platform, $op:expr, $platform:expr) => {
    MonitoringError::UnsupportedPlatform {
      operation: $op.to_string(),
      platform: $platform.to_string(),
    }
  };
  (not_initialized, $module:expr) => {
    MonitoringError::NotInitialized {
      module: $module.to_string(),
    }
  };
  (timeout, $op:expr, $duration:expr) => {
    MonitoringError::Timeout {
      operation: $op.to_string(),
      duration_ms: $duration,
    }
  };
}

/// Retry mechanism for operations that might fail temporarily
pub struct RetryConfig {
  pub max_attempts: usize,
  pub initial_delay_ms: u64,
  pub max_delay_ms: u64,
  pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
  fn default() -> Self {
    Self {
      max_attempts: 3,
      initial_delay_ms: 100,
      max_delay_ms: 5000,
      backoff_multiplier: 2.0,
    }
  }
}

/// Retry a fallible operation with exponential backoff
pub async fn retry_with_backoff<T, F, Fut>(
  mut operation: F,
  config: RetryConfig,
) -> MonitoringResult<T>
where
  F: FnMut() -> Fut,
  Fut: std::future::Future<Output = MonitoringResult<T>>,
{
  let mut delay = config.initial_delay_ms;
  let mut last_error = None;

  for attempt in 1..=config.max_attempts {
    match operation().await {
      Ok(result) => return Ok(result),
      Err(err) => {
        last_error = Some(err);

        if attempt < config.max_attempts {
          tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
          delay = std::cmp::min(
            (delay as f64 * config.backoff_multiplier) as u64,
            config.max_delay_ms,
          );
        }
      }
    }
  }

  Err(last_error.unwrap_or_else(|| MonitoringError::SystemCall {
    operation: "retry_operation".to_string(),
    source: Box::new(std::io::Error::new(
      std::io::ErrorKind::Other,
      "All retry attempts failed",
    )),
  }))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_error_display() {
    let err = MonitoringError::SystemCall {
      operation: "test_op".to_string(),
      source: Box::new(std::io::Error::new(std::io::ErrorKind::Other, "test error")),
    };
    assert!(err.to_string().contains("System call 'test_op' failed"));
  }

  #[test]
  fn test_error_conversion() {
    let io_err = std::io::Error::new(std::io::ErrorKind::Other, "test");
    let monitoring_err: MonitoringError = io_err.into();
    assert!(matches!(monitoring_err, MonitoringError::IoError { .. }));
  }

  #[tokio::test]
  async fn test_retry_success() {
    let mut attempts = 0;
    let result = retry_with_backoff(
      || {
        attempts += 1;
        async move {
          if attempts < 3 {
            Err(MonitoringError::SystemCall {
              operation: "test".to_string(),
              source: Box::new(std::io::Error::new(std::io::ErrorKind::Other, "temp error")),
            })
          } else {
            Ok(42)
          }
        }
      },
      RetryConfig::default(),
    )
    .await;

    assert_eq!(result.unwrap(), 42);
    assert_eq!(attempts, 3);
  }
}
