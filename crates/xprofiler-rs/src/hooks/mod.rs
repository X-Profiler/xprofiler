//! V8 Hooks module
//!
//! This module provides hooks for V8 runtime events.
//!
//! Note: napi-rs doesn't expose direct V8 isolate access, so these hooks
//! have limited functionality compared to the C++ implementation.
//! The configuration options are still respected for compatibility.

use crate::config;
use crate::logger;
use napi::bindgen_prelude::*;

/// Set V8 hooks based on configuration
///
/// This function checks the configuration and logs what hooks would be set.
/// Due to napi-rs limitations, actual V8 hook registration is not possible.
///
/// Configuration options checked:
/// - `enable_fatal_error_hook`: Whether to handle fatal errors
/// - `enable_auto_incr_heap_limit`: Whether to auto-increase heap limit on OOM
pub fn set_hooks_impl() -> Result<()> {
    let cfg = config::get_config();

    // Log fatal error hook status
    if cfg.enable_fatal_error_hook {
        logger::debug("Fatal error hook enabled (via Node.js process handlers)")?;

        // We can't set V8's SetFatalErrorHandler directly from napi-rs,
        // but Node.js already handles fatal errors through process events.
        // The configuration is acknowledged for compatibility.

        if cfg.enable_fatal_error_report {
            logger::debug("Fatal error report generation enabled")?;
        }

        if cfg.enable_fatal_error_coredump {
            logger::debug("Fatal error coredump generation enabled (Linux only)")?;
        }
    }

    // Log heap limit hook status
    if cfg.enable_auto_incr_heap_limit {
        logger::debug(&format!(
            "Auto-increase heap limit enabled (size: {} MB)",
            cfg.auto_incr_heap_limit_size
        ))?;

        // Note: V8's AddNearHeapLimitCallback is not accessible via napi-rs.
        // Users should use --max-old-space-size or NODE_OPTIONS instead.
        logger::info(
            "Note: Auto heap limit increase requires C++ binding. \
             Use --max-old-space-size for heap management.",
        )?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_hooks_impl() {
        // Just verify it doesn't panic
        let result = set_hooks_impl();
        assert!(result.is_ok());
    }
}
