//! Constants used throughout xprofiler-rs

/// Version of xprofiler-rs
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default log interval in seconds
pub const DEFAULT_LOG_INTERVAL: u32 = 60;

/// Default log level (1 = error)
pub const DEFAULT_LOG_LEVEL: u32 = 1;

/// Default log type (0 = file)
pub const DEFAULT_LOG_TYPE: u32 = 0;

/// Default HTTP patch timeout in seconds
pub const DEFAULT_PATCH_HTTP_TIMEOUT: u32 = 30;

/// Default auto increment heap limit size in MB
pub const DEFAULT_AUTO_INCR_HEAP_LIMIT_SIZE: u32 = 256;

/// Default mmap threshold in KB
pub const DEFAULT_MMAP_THRESHOLD: i32 = 128;

/// Log level: info
pub const LOG_LEVEL_INFO: u32 = 0;

/// Log level: error
pub const LOG_LEVEL_ERROR: u32 = 1;

/// Log level: debug
pub const LOG_LEVEL_DEBUG: u32 = 2;

/// Log type: file
pub const LOG_TYPE_FILE: u32 = 0;

/// Log type: console
pub const LOG_TYPE_CONSOLE: u32 = 1;

/// IPC socket filename for Unix
#[cfg(unix)]
pub const IPC_SOCKET_FILENAME: &str = "xprofiler-ctl-uds-path.sock";

/// IPC pipe name for Windows
#[cfg(windows)]
pub const IPC_PIPE_NAME: &str = "xprofiler-ctl";

/// Maximum Unix socket path length
/// macOS: 104 bytes, Linux: 108 bytes
#[cfg(target_os = "macos")]
pub const MAX_UNIX_SOCKET_PATH_LEN: usize = 104;

#[cfg(target_os = "linux")]
pub const MAX_UNIX_SOCKET_PATH_LEN: usize = 108;

#[cfg(all(unix, not(target_os = "macos"), not(target_os = "linux")))]
pub const MAX_UNIX_SOCKET_PATH_LEN: usize = 104;

/// Blurry tag used in log output
pub const BLURRY_TAG: &str = "__";
