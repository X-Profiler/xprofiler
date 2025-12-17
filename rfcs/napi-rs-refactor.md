# RFC: Migrating xprofiler from C++/NAN to Rust/napi-rs

## Summary

This document outlines a phased migration plan to refactor xprofiler from a C++ Node.js native addon (using NAN) to a Rust-based addon using napi-rs. The migration will be incremental, maintaining backward compatibility throughout the process.

## Motivation

### Current Challenges with C++/NAN

1. **Memory Safety**: Manual memory management in C++ is error-prone and difficult to audit
2. **Cross-Platform Complexity**: Platform-specific code (Windows/Linux/macOS) requires separate implementations with subtle differences
3. **NAN Deprecation**: Some NAN APIs are deprecated in newer Node.js versions
4. **Maintenance Burden**: C++ requires careful handling of V8 API changes between Node.js versions
5. **Build Complexity**: Managing C++ toolchains across platforms is challenging

### Benefits of Rust/napi-rs

1. **Memory Safety**: Rust's ownership model prevents memory leaks and data races at compile time
2. **Modern Tooling**: Cargo provides excellent dependency management and cross-compilation
3. **napi-rs Stability**: napi-rs targets Node-API (stable ABI) rather than V8 directly
4. **Cross-Platform**: Single codebase with conditional compilation for platform-specific code
5. **Performance**: Rust matches C++ performance with better safety guarantees
6. **Community**: Growing ecosystem of Rust crates for system programming

## Current Architecture Overview

### Multi-Threading Model

```
┌─────────────────────────────────────────────────────────────────┐
│                         Main Thread                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │ V8 Isolate  │  │ uv_loop_t   │  │ EnvironmentData         │  │
│  │ (JS exec)   │  │ (node loop) │  │ (per-thread profiling)  │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
          │                                       │
          ▼                                       ▼
┌─────────────────────┐               ┌─────────────────────┐
│  LogByPass Thread   │               │  Commands Listener  │
│  (Statistics)       │               │  Thread (IPC)       │
└─────────────────────┘               └─────────────────────┘
```

### Component Inventory

| Component | Files | Complexity | Migration Priority |
|-----------|-------|------------|-------------------|
| Configuration | `configure.h`, `export_configure.cc` | Low | Phase 1 |
| Logger | `logger.h/cc`, `export_logger.cc` | Low | Phase 1 |
| Utilities | `library/*.cc` | Low | Phase 1 |
| Platform Utils | `platform/*/utils.cc`, `cpu.cc` | Medium | Phase 2 |
| IPC System | `platform/*/ipc.cc`, `commands/listener.cc` | Medium | Phase 2 |
| Command Parser | `commands/parser.cc`, `send.cc` | Medium | Phase 3 |
| Simple Commands | `commands/simple/*.cc` | Low | Phase 3 |
| LogByPass Thread | `logbypass/*.cc` | High | Phase 4 |
| Environment Data | `environment_data.cc`, `environment_registry.cc` | High | Phase 4 |
| CPU Profiler | `commands/cpuprofiler/*.cc` | High | Phase 5 |
| Heap Profiler | `commands/heapdump/*.cc`, `heapprofiler/*.cc` | High | Phase 5 |
| GC Profiler | `commands/gcprofiler/*.cc` | Medium | Phase 5 |
| Node Report | `commands/report/*.cc` | High | Phase 6 |
| V8 Hooks | `hooks/*.cc` | High | Phase 6 |
| Coredump | `platform/*/core/*.cc` | Very High | Phase 7 (Optional) |

## Migration Phases

---

## Phase 0: Project Setup and Infrastructure

**Goal**: Set up the Rust project structure alongside existing C++ code, enabling hybrid builds.

### 0.1 Initialize Rust Project

```bash
# Create Rust workspace in project root
cargo init --lib crates/xprofiler-rs
```

**Directory Structure**:
```
xprofiler/
├── crates/
│   └── xprofiler-rs/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           └── ...
├── src/                    # Existing C++ code
├── binding.gyp            # Existing (will be deprecated)
└── package.json
```

### 0.2 Configure Cargo.toml

```toml
[package]
name = "xprofiler-rs"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
napi = { version = "2", features = ["full"] }
napi-derive = "2"
tokio = { version = "1", features = ["rt-multi-thread", "sync", "time", "net"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
parking_lot = "0.12"
once_cell = "1"
tracing = "0.1"
thiserror = "1"

[target.'cfg(unix)'.dependencies]
libc = "0.2"
nix = { version = "0.27", features = ["socket", "uio", "process", "signal"] }

[target.'cfg(windows)'.dependencies]
windows = { version = "0.48", features = [
    "Win32_Foundation",
    "Win32_System_Pipes",
    "Win32_System_Threading",
    "Win32_System_Performance",
    "Win32_Security",
]}

[build-dependencies]
napi-build = "2"

[profile.release]
lto = true
strip = "symbols"
```

### 0.3 Setup Build Integration

**package.json changes**:
```json
{
  "scripts": {
    "build": "npm run build:rs",
    "build:rs": "napi build --platform --release ./build/binding/Release",
    "build:cpp": "node-pre-gyp rebuild",
    "prepublishOnly": "napi prepublish -t npm"
  },
  "napi": {
    "name": "xprofiler",
    "triples": {
      "defaults": true,
      "additional": [
        "x86_64-unknown-linux-musl",
        "aarch64-unknown-linux-gnu",
        "aarch64-apple-darwin",
        "aarch64-unknown-linux-musl"
      ]
    }
  }
}
```

### 0.4 Create Feature Flag System

Implement a feature flag in JavaScript to switch between C++ and Rust implementations during migration:

```javascript
// xprofiler.js
const useRust = process.env.XPROFILER_USE_RUST === 'true';
const binding = useRust
  ? require('./xprofiler-rs.node')
  : require('./build/binding/...'); // existing path
```

### 0.5 Deliverables

- [ ] Rust workspace initialized with napi-rs dependencies
- [ ] Build scripts configured for both C++ and Rust
- [ ] CI/CD updated to build and test both implementations
- [ ] Feature flag system for gradual rollout
- [ ] Documentation for local development setup

---

## Phase 1: Core Utilities and Configuration

**Goal**: Migrate foundational components that have no V8 dependencies.

### 1.1 Configuration System

**Files to migrate**:
- `src/configure.h`, `src/configure-inl.h`
- `src/jsapi/export_configure.cc`

**Rust Implementation** (`crates/xprofiler-rs/src/config.rs`):

```rust
use napi::bindgen_prelude::*;
use napi_derive::napi;
use parking_lot::RwLock;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object)]
pub struct XprofilerConfig {
    pub log_dir: String,
    pub log_interval: u32,
    pub log_level: u32,
    pub log_type: u32,
    pub log_format_alinode: bool,
    pub patch_http: bool,
    pub patch_http_timeout: u32,
    pub check_throw: bool,
    pub enable_fatal_error_hook: bool,
    pub enable_fatal_error_report: bool,
    pub enable_fatal_error_coredump: bool,
    pub enable_http_profiling: bool,
    pub enable_auto_incr_heap_limit: bool,
    pub auto_incr_heap_limit_size: u32,
    pub enable_avoid_rss_leak: bool,
    pub enable_log_uv_handles: bool,
    pub mmap_threshold: i32,
}

impl Default for XprofilerConfig {
    fn default() -> Self {
        Self {
            log_dir: std::env::temp_dir().to_string_lossy().to_string(),
            log_interval: 60,
            log_level: 0,
            log_type: 0,
            log_format_alinode: false,
            patch_http: true,
            patch_http_timeout: 30,
            check_throw: true,
            enable_fatal_error_hook: true,
            enable_fatal_error_report: true,
            enable_fatal_error_coredump: false,
            enable_http_profiling: false,
            enable_auto_incr_heap_limit: false,
            auto_incr_heap_limit_size: 256,
            enable_avoid_rss_leak: false,
            enable_log_uv_handles: true,
            mmap_threshold: 128,
        }
    }
}

static CONFIG: Lazy<RwLock<XprofilerConfig>> = Lazy::new(|| {
    RwLock::new(XprofilerConfig::default())
});

#[napi]
pub fn configure(config: XprofilerConfig) -> Result<()> {
    let mut guard = CONFIG.write();
    *guard = config;
    Ok(())
}

#[napi]
pub fn get_config() -> XprofilerConfig {
    CONFIG.read().clone()
}
```

### 1.2 Logger System

**Files to migrate**:
- `src/logger.h`, `src/logger.cc`
- `src/jsapi/export_logger.cc`

**Rust Implementation** (`crates/xprofiler-rs/src/logger.rs`):

```rust
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use parking_lot::Mutex;
use chrono::{Local, Datelike};

static LOG_FILE: Lazy<Mutex<Option<LogFile>>> = Lazy::new(|| Mutex::new(None));

struct LogFile {
    file: File,
    date: u32, // YYYYMMDD format
    path: PathBuf,
}

#[napi]
pub fn info(content: String) -> Result<()> {
    write_log("info", &content)
}

#[napi]
pub fn error(content: String) -> Result<()> {
    write_log("error", &content)
}

#[napi]
pub fn debug(content: String) -> Result<()> {
    let config = crate::config::get_config();
    if config.log_level >= 2 {
        write_log("debug", &content)?;
    }
    Ok(())
}

fn write_log(level: &str, content: &str) -> Result<()> {
    let config = crate::config::get_config();
    let now = Local::now();
    let today = (now.year() as u32) * 10000 + (now.month() as u32) * 100 + now.day() as u32;

    let mut guard = LOG_FILE.lock();

    // Rotate log file if date changed
    if guard.as_ref().map_or(true, |f| f.date != today) {
        let log_path = PathBuf::from(&config.log_dir)
            .join(format!("xprofiler-{}.log", now.format("%Y%m%d")));

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .map_err(|e| Error::from_reason(format!("Failed to open log file: {}", e)))?;

        *guard = Some(LogFile {
            file,
            date: today,
            path: log_path,
        });
    }

    if let Some(log_file) = guard.as_mut() {
        let timestamp = now.format("%Y-%m-%d %H:%M:%S");
        let pid = std::process::id();
        let line = format!("[{}] [{}] [{}] {}\n", timestamp, pid, level, content);
        log_file.file.write_all(line.as_bytes())
            .map_err(|e| Error::from_reason(format!("Failed to write log: {}", e)))?;
    }

    Ok(())
}
```

### 1.3 Common Utilities

**Files to migrate**:
- `src/library/common.h`, `src/library/common.cc`
- `src/library/utils.h`, `src/library/utils.cc`
- `src/library/error.h`, `src/library/error.cc`

**Rust Implementation** (`crates/xprofiler-rs/src/utils.rs`):

```rust
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use std::sync::atomic::{AtomicU64, Ordering};
use once_cell::sync::Lazy;

static START_TIME: Lazy<Instant> = Lazy::new(Instant::now);
static DIAG_FILE_ID: AtomicU64 = AtomicU64::new(0);

pub fn get_uptime() -> u64 {
    START_TIME.elapsed().as_secs()
}

pub fn get_next_diag_file_id() -> u64 {
    DIAG_FILE_ID.fetch_add(1, Ordering::SeqCst)
}

pub fn get_pid() -> u32 {
    std::process::id()
}

#[cfg(unix)]
pub fn get_path_separator() -> &'static str {
    "/"
}

#[cfg(windows)]
pub fn get_path_separator() -> &'static str {
    "\\"
}

pub fn format_timestamp() -> String {
    chrono::Local::now().format("%Y%m%d").to_string()
}
```

### 1.4 JSON Writer

**Files to migrate**:
- `src/library/writer.h`, `src/library/writer.cc`

**Rust Implementation**: Use `serde_json` directly - no custom writer needed.

### 1.5 Deliverables

- [ ] Configuration system fully migrated to Rust
- [ ] Logger system migrated with file rotation
- [ ] Common utilities migrated
- [ ] Unit tests for all Phase 1 components
- [ ] Integration tests verifying JS API compatibility

---

## Phase 2: Platform Abstraction Layer

**Goal**: Create a unified platform abstraction for cross-platform functionality.

### 2.1 Platform Trait Definition

```rust
// crates/xprofiler-rs/src/platform/mod.rs

pub mod cpu;
pub mod ipc;
pub mod utils;

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub use unix::*;
#[cfg(windows)]
pub use windows::*;
```

### 2.2 CPU Usage

**Files to migrate**:
- `src/platform/unix/cpu.cc` (getrusage)
- `src/platform/win/cpu_win.cc` (Performance Counters)

**Rust Implementation** (`crates/xprofiler-rs/src/platform/cpu.rs`):

```rust
use std::time::Instant;

#[derive(Debug, Clone, Default)]
pub struct CpuUsage {
    pub user_time: f64,
    pub system_time: f64,
    pub cpu_now: f64,
    pub cpu_15: f64,
    pub cpu_30: f64,
    pub cpu_60: f64,
}

#[cfg(unix)]
pub fn get_cpu_usage() -> CpuUsage {
    use libc::{rusage, getrusage, RUSAGE_SELF};
    use std::mem::MaybeUninit;

    let mut usage = MaybeUninit::<rusage>::uninit();
    unsafe {
        getrusage(RUSAGE_SELF, usage.as_mut_ptr());
        let usage = usage.assume_init();

        CpuUsage {
            user_time: usage.ru_utime.tv_sec as f64 + usage.ru_utime.tv_usec as f64 / 1_000_000.0,
            system_time: usage.ru_stime.tv_sec as f64 + usage.ru_stime.tv_usec as f64 / 1_000_000.0,
            ..Default::default()
        }
    }
}

#[cfg(windows)]
pub fn get_cpu_usage() -> CpuUsage {
    use windows::Win32::System::Threading::{GetCurrentProcess, GetProcessTimes};
    use windows::Win32::Foundation::FILETIME;

    // Windows implementation using GetProcessTimes
    // ...
    CpuUsage::default()
}
```

### 2.3 IPC System

**Files to migrate**:
- `src/platform/unix/ipc.cc` (Unix domain sockets)
- `src/platform/win/ipc_win.cc` (Named pipes)

**Rust Implementation** (`crates/xprofiler-rs/src/platform/ipc.rs`):

```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::path::PathBuf;

pub trait IpcServer: Send + Sync {
    fn start(&self, handler: Box<dyn Fn(String) -> String + Send + Sync>) -> Result<(), IpcError>;
    fn stop(&self) -> Result<(), IpcError>;
}

pub trait IpcClient: Send + Sync {
    fn send(&self, message: &str) -> Result<String, IpcError>;
}

#[cfg(unix)]
mod unix_ipc {
    use super::*;
    use tokio::net::{UnixListener, UnixStream};

    pub struct UnixSocketServer {
        path: PathBuf,
    }

    impl UnixSocketServer {
        pub fn new(log_dir: &str, pid: u32) -> Self {
            let path = PathBuf::from(log_dir)
                .join(format!("xprofiler-uds-path-{}.sock", pid));
            Self { path }
        }
    }

    impl IpcServer for UnixSocketServer {
        fn start(&self, handler: Box<dyn Fn(String) -> String + Send + Sync>) -> Result<(), IpcError> {
            // Implementation using UnixListener
            Ok(())
        }

        fn stop(&self) -> Result<(), IpcError> {
            std::fs::remove_file(&self.path).ok();
            Ok(())
        }
    }
}

#[cfg(windows)]
mod windows_ipc {
    use super::*;
    use windows::Win32::System::Pipes::*;

    pub struct NamedPipeServer {
        name: String,
    }

    impl NamedPipeServer {
        pub fn new(pid: u32) -> Self {
            Self {
                name: format!(r"\\.\pipe\xprofiler-named-pipe-{}", pid),
            }
        }
    }

    impl IpcServer for NamedPipeServer {
        fn start(&self, handler: Box<dyn Fn(String) -> String + Send + Sync>) -> Result<(), IpcError> {
            // Implementation using Windows named pipes
            Ok(())
        }

        fn stop(&self) -> Result<(), IpcError> {
            Ok(())
        }
    }
}
```

### 2.4 Socket Path Validation

**Files to migrate**:
- `src/jsapi/export_utils.h`

```rust
#[napi]
pub fn check_socket_path(log_dir: String) -> Result<bool> {
    #[cfg(unix)]
    {
        // Unix socket path max length is typically 108 bytes
        let path = format!("{}/xprofiler-uds-path-{}.sock", log_dir, std::process::id());
        Ok(path.len() < 108)
    }

    #[cfg(windows)]
    {
        // Windows named pipes have different constraints
        Ok(true)
    }
}
```

### 2.5 Deliverables

- [ ] Platform abstraction traits defined
- [ ] CPU usage collection for all platforms
- [ ] IPC server/client for Unix and Windows
- [ ] Socket path validation
- [ ] Unit tests with platform-specific test cases
- [ ] Integration tests for IPC communication

---

## Phase 3: Command System

**Goal**: Migrate the command parsing and dispatch system.

### 3.1 Command Parser

**Files to migrate**:
- `src/commands/parser.cc`
- `src/commands/send.cc`

**Rust Implementation** (`crates/xprofiler-rs/src/commands/mod.rs`):

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Command {
    pub traceid: String,
    pub cmd: String,
    pub thread_id: Option<i64>,
    pub options: Option<CommandOptions>,
}

#[derive(Debug, Deserialize)]
pub struct CommandOptions {
    pub profiling_time: Option<u64>,
    pub filepath: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CommandResponse {
    pub ok: bool,
    pub traceid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

pub fn parse_command(input: &str) -> Result<Command, serde_json::Error> {
    serde_json::from_str(input)
}

pub fn dispatch_command(cmd: Command) -> CommandResponse {
    match cmd.cmd.as_str() {
        "check_version" => handle_version(cmd),
        "list_environments" => handle_list_environments(cmd),
        "get_config" => handle_get_config(cmd),
        "set_config" => handle_set_config(cmd),
        "start_cpu_profiling" => handle_start_cpu_profiling(cmd),
        "stop_cpu_profiling" => handle_stop_cpu_profiling(cmd),
        "heapdump" => handle_heapdump(cmd),
        "start_heap_profiling" => handle_start_heap_profiling(cmd),
        "stop_heap_profiling" => handle_stop_heap_profiling(cmd),
        "start_gc_profiling" => handle_start_gc_profiling(cmd),
        "stop_gc_profiling" => handle_stop_gc_profiling(cmd),
        "diag_report" => handle_diag_report(cmd),
        _ => CommandResponse {
            ok: false,
            traceid: cmd.traceid,
            data: None,
            message: Some(format!("Unknown command: {}", cmd.cmd)),
        },
    }
}
```

### 3.2 Simple Commands

**Files to migrate**:
- `src/commands/simple/version.cc`
- `src/commands/simple/registry.cc`
- `src/commands/simple/config.cc`

```rust
// crates/xprofiler-rs/src/commands/simple.rs

pub fn handle_version(cmd: Command) -> CommandResponse {
    CommandResponse {
        ok: true,
        traceid: cmd.traceid,
        data: Some(serde_json::json!({
            "version": env!("CARGO_PKG_VERSION"),
        })),
        message: None,
    }
}

pub fn handle_get_config(cmd: Command) -> CommandResponse {
    let config = crate::config::get_config();
    CommandResponse {
        ok: true,
        traceid: cmd.traceid,
        data: Some(serde_json::to_value(config).unwrap()),
        message: None,
    }
}
```

### 3.3 Command Listener Thread

**Files to migrate**:
- `src/commands/listener.cc`

```rust
// crates/xprofiler-rs/src/commands/listener.rs

use std::sync::Arc;
use tokio::sync::mpsc;
use parking_lot::Mutex;

pub struct CommandsListener {
    shutdown_tx: Option<mpsc::Sender<()>>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl CommandsListener {
    pub fn new() -> Self {
        Self {
            shutdown_tx: None,
            handle: None,
        }
    }

    pub fn start(&mut self, log_dir: &str) -> Result<(), String> {
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);
        self.shutdown_tx = Some(shutdown_tx);

        let log_dir = log_dir.to_string();

        let handle = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            rt.block_on(async {
                let server = crate::platform::ipc::create_server(&log_dir);

                // Run server until shutdown signal
                tokio::select! {
                    _ = server.run() => {}
                    _ = shutdown_rx.recv() => {}
                }
            });
        });

        self.handle = Some(handle);
        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.blocking_send(());
        }
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

#[napi]
pub fn run_commands_listener() -> Result<()> {
    // Start the commands listener in a separate thread
    Ok(())
}
```

### 3.4 Deliverables

- [ ] Command parsing with serde
- [ ] Command dispatch system
- [ ] Simple commands (version, config, registry)
- [ ] Command listener thread with IPC
- [ ] Response formatting
- [ ] Integration tests for command flow

---

## Phase 4: Statistics Collection (LogByPass)

**Goal**: Migrate the statistics collection thread and metrics gathering.

### 4.1 Statistics Data Structures

**Files to migrate**:
- `src/environment_data.h` (statistics structs)

```rust
// crates/xprofiler-rs/src/stats/mod.rs

use std::sync::atomic::{AtomicU64, AtomicI64, Ordering};
use parking_lot::RwLock;

#[derive(Debug, Default)]
pub struct GcStatistics {
    pub total_gc_times: AtomicU64,
    pub total_gc_duration: AtomicU64,
    pub total_scavange_duration: AtomicU64,
    pub total_marksweep_duration: AtomicU64,
    pub total_incremental_marking_duration: AtomicU64,
    pub gc_time_during_last_record: AtomicU64,
    pub scavange_duration_last_record: AtomicU64,
    pub marksweep_duration_last_record: AtomicU64,
    pub incremental_marking_duration_last_record: AtomicU64,
}

#[derive(Debug, Default)]
pub struct HttpStatistics {
    pub live_http_request: AtomicI64,
    pub http_response_close: AtomicU64,
    pub http_response_sent: AtomicU64,
    pub http_request_timeout: AtomicU64,
    pub http_rt: AtomicU64,
    pub http_response_status: RwLock<std::collections::HashMap<u16, u64>>,
}

#[derive(Debug, Default)]
pub struct MemoryStatistics {
    pub rss: AtomicU64,
    pub heap_used: AtomicU64,
    pub heap_available: AtomicU64,
    pub heap_total: AtomicU64,
    pub heap_limit: AtomicU64,
    pub heap_executeable: AtomicU64,
    pub total_physical_size: AtomicU64,
    pub malloced_memory: AtomicU64,
    pub amount_of_external_allocated_memory: AtomicU64,
}

#[derive(Debug, Default)]
pub struct UvHandleStatistics {
    pub active_handles: AtomicU64,
    pub active_file_handles: AtomicU64,
    pub active_tcp_handles: AtomicU64,
    pub active_udp_handles: AtomicU64,
    pub active_timer_handles: AtomicU64,
}
```

### 4.2 Environment Data

**Files to migrate**:
- `src/environment_data.cc`
- `src/environment_registry.cc`

```rust
// crates/xprofiler-rs/src/env/mod.rs

use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use napi::Env;

pub struct EnvironmentData {
    pub thread_id: u64,
    pub is_main_thread: bool,
    pub gc_stats: Arc<GcStatistics>,
    pub http_stats: Arc<HttpStatistics>,
    pub memory_stats: Arc<MemoryStatistics>,
    pub uv_stats: Arc<UvHandleStatistics>,
}

pub struct EnvironmentRegistry {
    environments: RwLock<HashMap<u64, Arc<EnvironmentData>>>,
}

impl EnvironmentRegistry {
    pub fn global() -> &'static Self {
        static REGISTRY: once_cell::sync::Lazy<EnvironmentRegistry> =
            once_cell::sync::Lazy::new(EnvironmentRegistry::new);
        &REGISTRY
    }

    pub fn new() -> Self {
        Self {
            environments: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, thread_id: u64, data: EnvironmentData) {
        self.environments.write().insert(thread_id, Arc::new(data));
    }

    pub fn unregister(&self, thread_id: u64) {
        self.environments.write().remove(&thread_id);
    }

    pub fn get(&self, thread_id: u64) -> Option<Arc<EnvironmentData>> {
        self.environments.read().get(&thread_id).cloned()
    }

    pub fn get_main_thread(&self) -> Option<Arc<EnvironmentData>> {
        self.environments.read()
            .values()
            .find(|e| e.is_main_thread)
            .cloned()
    }

    pub fn for_each<F>(&self, f: F)
    where F: Fn(&EnvironmentData) {
        for env in self.environments.read().values() {
            f(env);
        }
    }
}
```

### 4.3 LogByPass Thread

**Files to migrate**:
- `src/logbypass/log.h`, `src/logbypass/log.cc`
- `src/logbypass/cpu.cc`
- `src/logbypass/heap.cc`
- `src/logbypass/gc.cc`
- `src/logbypass/libuv.cc`
- `src/logbypass/http.cc`

```rust
// crates/xprofiler-rs/src/logbypass/mod.rs

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use parking_lot::Mutex;

pub struct LogByPass {
    shutdown_tx: Option<mpsc::Sender<()>>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl LogByPass {
    pub fn new() -> Self {
        Self {
            shutdown_tx: None,
            handle: None,
        }
    }

    pub fn start(&mut self, log_interval: u64) -> Result<(), String> {
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);
        self.shutdown_tx = Some(shutdown_tx);

        let handle = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            rt.block_on(async {
                let mut cpu_interval = tokio::time::interval(Duration::from_secs(1));
                let mut log_interval = tokio::time::interval(Duration::from_secs(log_interval));

                loop {
                    tokio::select! {
                        _ = cpu_interval.tick() => {
                            collect_cpu_stats();
                        }
                        _ = log_interval.tick() => {
                            collect_and_write_stats();
                        }
                        _ = shutdown_rx.recv() => {
                            break;
                        }
                    }
                }
            });
        });

        self.handle = Some(handle);
        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.blocking_send(());
        }
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

fn collect_cpu_stats() {
    let usage = crate::platform::cpu::get_cpu_usage();
    // Store in circular buffer for different time windows
}

fn collect_and_write_stats() {
    let config = crate::config::get_config();
    let registry = crate::env::EnvironmentRegistry::global();

    // Collect stats from all environments
    registry.for_each(|env| {
        // Write stats to log file
    });
}

#[napi]
pub fn run_log_bypass() -> Result<()> {
    // Start the log bypass thread
    Ok(())
}
```

### 4.4 Deliverables

- [ ] Statistics data structures
- [ ] Environment data and registry
- [ ] LogByPass thread with timers
- [ ] CPU statistics collection
- [ ] Memory statistics collection (via Node-API)
- [ ] GC statistics collection (via Node-API callbacks)
- [ ] HTTP statistics collection
- [ ] Log file writing with proper formatting
- [ ] Integration tests for statistics collection

---

## Phase 5: Profilers

**Goal**: Migrate CPU, heap, and GC profilers.

### 5.1 CPU Profiler

**Files to migrate**:
- `src/commands/cpuprofiler/cpu_profiler.cc`
- `src/commands/cpuprofiler/cpu_profile.cc`
- `src/commands/cpuprofiler/cpu_profile_node.cc`

**Note**: V8 CPU profiler access requires special handling in napi-rs. We'll need to use `napi::sys` for low-level V8 access or consider using Node.js inspector protocol.

```rust
// crates/xprofiler-rs/src/profilers/cpu.rs

use napi::bindgen_prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

static CPU_PROFILING: AtomicBool = AtomicBool::new(false);

pub struct CpuProfiler {
    // Will need V8 profiler handle
}

impl CpuProfiler {
    pub fn start() -> Result<()> {
        if CPU_PROFILING.swap(true, Ordering::SeqCst) {
            return Err(Error::from_reason("CPU profiling already in progress"));
        }

        // Use V8 CpuProfiler API
        // This requires accessing V8 directly through napi::sys
        Ok(())
    }

    pub fn stop(filepath: &str) -> Result<String> {
        if !CPU_PROFILING.swap(false, Ordering::SeqCst) {
            return Err(Error::from_reason("CPU profiling not in progress"));
        }

        // Stop profiler and serialize to .cpuprofile format
        Ok(filepath.to_string())
    }
}
```

**Alternative approach using Node.js Inspector**:

```rust
// Use inspector module via JavaScript interop
#[napi]
pub fn start_cpu_profiling_via_inspector(env: Env) -> Result<()> {
    // Call into JS to use inspector.Session
    Ok(())
}
```

### 5.2 Heap Profiler / Heap Dump

**Files to migrate**:
- `src/commands/heapdump/heap_profiler.cc`
- `src/commands/heapdump/heap_snapshot.cc`
- `src/commands/heapprofiler/sampling_heap_profiler.cc`

```rust
// crates/xprofiler-rs/src/profilers/heap.rs

use napi::bindgen_prelude::*;

pub struct HeapProfiler;

impl HeapProfiler {
    pub fn take_snapshot(filepath: &str) -> Result<String> {
        // Use V8 HeapProfiler::TakeHeapSnapshot
        // Serialize to .heapsnapshot format
        Ok(filepath.to_string())
    }

    pub fn start_sampling() -> Result<()> {
        // Use V8 HeapProfiler::StartSamplingHeapProfiler
        Ok(())
    }

    pub fn stop_sampling(filepath: &str) -> Result<String> {
        // Use V8 HeapProfiler::StopSamplingHeapProfiler
        Ok(filepath.to_string())
    }
}
```

### 5.3 GC Profiler

**Files to migrate**:
- `src/commands/gcprofiler/gc_profiler.cc`

```rust
// crates/xprofiler-rs/src/profilers/gc.rs

use napi::bindgen_prelude::*;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

static GC_LOG: Lazy<Mutex<Option<GcLog>>> = Lazy::new(|| Mutex::new(None));

struct GcLog {
    file: File,
    start_time: std::time::Instant,
}

pub fn start_gc_profiling(filepath: &str) -> Result<()> {
    let file = File::create(filepath)
        .map_err(|e| Error::from_reason(format!("Failed to create GC log: {}", e)))?;

    let mut guard = GC_LOG.lock().unwrap();
    *guard = Some(GcLog {
        file,
        start_time: std::time::Instant::now(),
    });

    // Enable detailed GC callbacks
    Ok(())
}

pub fn stop_gc_profiling() -> Result<String> {
    let mut guard = GC_LOG.lock().unwrap();
    if let Some(mut log) = guard.take() {
        log.file.flush().ok();
        // Return filepath
    }
    Ok(String::new())
}

pub fn on_gc_callback(gc_type: &str, flags: u32, duration_ms: f64) {
    if let Some(ref mut log) = *GC_LOG.lock().unwrap() {
        let elapsed = log.start_time.elapsed().as_secs_f64();
        let entry = serde_json::json!({
            "time": elapsed,
            "type": gc_type,
            "flags": flags,
            "duration_ms": duration_ms,
        });
        writeln!(log.file, "{}", entry).ok();
    }
}
```

### 5.4 Profiling Watchdog

**Files to migrate**:
- `src/commands/dump.cc` (ProfilingWatchdog class)

```rust
// crates/xprofiler-rs/src/profilers/watchdog.rs

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::oneshot;

pub struct ProfilingWatchdog {
    cancel_tx: Option<oneshot::Sender<()>>,
}

impl ProfilingWatchdog {
    pub fn new() -> Self {
        Self { cancel_tx: None }
    }

    pub fn start<F>(&mut self, duration: Duration, on_timeout: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let (cancel_tx, cancel_rx) = oneshot::channel();
        self.cancel_tx = Some(cancel_tx);

        tokio::spawn(async move {
            tokio::select! {
                _ = tokio::time::sleep(duration) => {
                    on_timeout();
                }
                _ = cancel_rx => {
                    // Cancelled
                }
            }
        });
    }

    pub fn cancel(&mut self) {
        if let Some(tx) = self.cancel_tx.take() {
            let _ = tx.send(());
        }
    }
}
```

### 5.5 Deliverables

- [ ] CPU profiler with .cpuprofile output
- [ ] Heap snapshot with .heapsnapshot output
- [ ] Sampling heap profiler
- [ ] GC profiler with JSON output
- [ ] Profiling watchdog for timed profiling
- [ ] Thread-safe profiling state management
- [ ] Integration tests for all profilers

---

## Phase 6: V8 Hooks and Diagnostic Report

**Goal**: Migrate V8 hooks and diagnostic report generation.

### 6.1 Fatal Error Hook

**Files to migrate**:
- `src/hooks/fatal_error.cc`

```rust
// crates/xprofiler-rs/src/hooks/fatal_error.rs

use napi::bindgen_prelude::*;

pub fn set_fatal_error_handler(env: Env) -> Result<()> {
    // Use napi to set OOM error handler
    // Note: This requires special V8 access
    Ok(())
}

fn on_fatal_error(location: &str, message: &str) {
    let config = crate::config::get_config();

    if config.enable_fatal_error_report {
        // Generate diagnostic report
        let _ = crate::report::generate_report();
    }

    if config.enable_fatal_error_coredump {
        // Generate coredump
        #[cfg(target_os = "linux")]
        crate::platform::core::write_core();
    }
}
```

### 6.2 Heap Limit Hook

**Files to migrate**:
- `src/hooks/heap_limit.cc`

```rust
// crates/xprofiler-rs/src/hooks/heap_limit.rs

use napi::bindgen_prelude::*;

pub fn auto_increase_heap_limit(env: Env) -> Result<()> {
    // Use napi to add near-heap-limit callback
    // This requires special V8 access
    Ok(())
}

fn on_near_heap_limit(current_limit: usize, _initial_limit: usize) -> usize {
    let config = crate::config::get_config();
    let increase = config.auto_incr_heap_limit_size as usize * 1024 * 1024;

    tracing::warn!(
        "Near heap limit, increasing from {} to {} MB",
        current_limit / 1024 / 1024,
        (current_limit + increase) / 1024 / 1024
    );

    current_limit + increase
}
```

### 6.3 Diagnostic Report

**Files to migrate**:
- `src/commands/report/node_report.cc`
- `src/commands/report/javascript_stack.cc`
- `src/commands/report/native_stack.cc`
- `src/commands/report/heap_statistics.cc`
- `src/commands/report/uv_statistics.cc`
- `src/commands/report/system_statistics.cc`

```rust
// crates/xprofiler-rs/src/report/mod.rs

use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
pub struct DiagnosticReport {
    pub header: ReportHeader,
    pub javascript_stack: JavaScriptStack,
    pub native_stack: NativeStack,
    pub heap_statistics: HeapStatistics,
    pub uv_statistics: UvStatistics,
    pub system_info: SystemInfo,
    pub environment_variables: std::collections::HashMap<String, String>,
}

#[derive(Serialize)]
pub struct ReportHeader {
    pub event: String,
    pub trigger: String,
    pub timestamp: String,
    pub process_id: u32,
    pub command_line: Vec<String>,
    pub node_version: String,
    pub os_name: String,
    pub os_version: String,
    pub machine: String,
}

pub fn generate_report() -> Result<String, String> {
    let config = crate::config::get_config();
    let filepath = format!(
        "{}/xprofiler-diag-{}-{}.json",
        config.log_dir,
        std::process::id(),
        chrono::Local::now().format("%Y%m%d-%H%M%S")
    );

    let report = DiagnosticReport {
        header: collect_header(),
        javascript_stack: collect_js_stack(),
        native_stack: collect_native_stack(),
        heap_statistics: collect_heap_stats(),
        uv_statistics: collect_uv_stats(),
        system_info: collect_system_info(),
        environment_variables: std::env::vars().collect(),
    };

    let mut file = File::create(&filepath)
        .map_err(|e| format!("Failed to create report: {}", e))?;

    serde_json::to_writer_pretty(&mut file, &report)
        .map_err(|e| format!("Failed to write report: {}", e))?;

    Ok(filepath)
}

fn collect_native_stack() -> NativeStack {
    #[cfg(unix)]
    {
        // Use backtrace crate
        use backtrace::Backtrace;
        let bt = Backtrace::new();
        // Convert to report format
    }

    #[cfg(windows)]
    {
        // Use Windows APIs
    }

    NativeStack::default()
}
```

### 6.4 Deliverables

- [ ] Fatal error handler
- [ ] Near-heap-limit callback
- [ ] Complete diagnostic report generation
- [ ] JavaScript stack collection
- [ ] Native stack collection (platform-specific)
- [ ] System information collection
- [ ] Integration tests for hooks and reports

---

## Phase 7: Coredump Support (Optional)

**Goal**: Migrate Linux coredump support. This is optional and can be deferred.

**Files to migrate**:
- `src/platform/unix/core/linux/coredumper.cc`
- `src/platform/unix/core/linux/elfcore.cc`
- `src/platform/unix/core/linux/linuxthreads.cc`
- `src/platform/unix/core/linux/thread_lister.cc`

**Note**: This is highly platform-specific and complex. Consider:
1. Using an existing Rust crate if available
2. Keeping as FFI to existing C code
3. Deferring to a later phase

```rust
// crates/xprofiler-rs/src/platform/core.rs

#[cfg(target_os = "linux")]
pub fn write_core() -> Result<(), String> {
    // Complex implementation involving:
    // - Thread suspension
    // - ELF core file format
    // - Memory mapping
    // Consider FFI to existing C implementation
    Ok(())
}

#[cfg(not(target_os = "linux"))]
pub fn write_core() -> Result<(), String> {
    Err("Coredump not supported on this platform".to_string())
}
```

### 7.1 Deliverables (If Implemented)

- [ ] ELF core file generation
- [ ] Thread listing and suspension
- [ ] Memory mapping serialization
- [ ] Tests on Linux

---

## Phase 8: Final Integration and Cleanup

**Goal**: Complete migration, remove C++ code, and optimize.

### 8.1 JavaScript API Compatibility

Ensure all JavaScript APIs remain compatible:

```rust
// crates/xprofiler-rs/src/lib.rs

use napi::bindgen_prelude::*;
use napi_derive::napi;

mod config;
mod logger;
mod utils;
mod platform;
mod env;
mod logbypass;
mod commands;
mod profilers;
mod hooks;
mod report;

#[napi]
pub fn setup(thread_id: i64, is_main_thread: bool, node_version: String) -> Result<()> {
    env::setup_environment(thread_id as u64, is_main_thread, node_version)
}

#[napi]
pub fn start(config_obj: config::XprofilerConfig) -> Result<()> {
    config::configure(config_obj)?;

    let config = config::get_config();

    // Start commands listener (main thread only)
    if env::is_main_thread() {
        commands::start_listener(&config.log_dir)?;
    }

    // Start log bypass
    logbypass::start(config.log_interval as u64)?;

    // Set hooks if enabled
    if config.enable_fatal_error_hook {
        hooks::set_fatal_error_handler()?;
    }

    if config.enable_auto_incr_heap_limit {
        hooks::auto_increase_heap_limit()?;
    }

    Ok(())
}

// HTTP tracking APIs
#[napi]
pub fn add_live_request(request_id: String) -> Result<()> {
    env::get_current()?.http_stats.add_live_request(request_id)
}

#[napi]
pub fn add_close_request(request_id: String, rt: f64) -> Result<()> {
    env::get_current()?.http_stats.add_close_request(request_id, rt)
}

#[napi]
pub fn add_sent_request(request_id: String, rt: f64) -> Result<()> {
    env::get_current()?.http_stats.add_sent_request(request_id, rt)
}

#[napi]
pub fn add_request_timeout(request_id: String) -> Result<()> {
    env::get_current()?.http_stats.add_request_timeout(request_id)
}

#[napi]
pub fn add_http_status_code(status_code: u32) -> Result<()> {
    env::get_current()?.http_stats.add_status_code(status_code as u16)
}
```

### 8.2 Remove C++ Code

1. Remove `src/` directory (C++ sources)
2. Remove `binding.gyp`
3. Remove NAN dependency from `package.json`
4. Update build scripts to only use napi-rs

### 8.3 Performance Optimization

1. Profile the Rust implementation
2. Optimize hot paths (logging, statistics collection)
3. Minimize allocations in critical paths
4. Use `#[inline]` for small functions

### 8.4 Documentation Update

1. Update README.md with new build instructions
2. Update CLAUDE.md with Rust architecture
3. Add Rust code documentation
4. Update TypeScript definitions if needed

### 8.5 Deliverables

- [ ] All JavaScript APIs working with Rust backend
- [ ] C++ code removed
- [ ] Performance benchmarks passing
- [ ] All tests passing
- [ ] Documentation updated
- [ ] CI/CD updated for Rust-only builds

---

## Migration Timeline Estimation

| Phase | Components | Estimated Effort |
|-------|------------|------------------|
| 0 | Project Setup | Small |
| 1 | Core Utilities | Small |
| 2 | Platform Abstraction | Medium |
| 3 | Command System | Medium |
| 4 | Statistics Collection | Large |
| 5 | Profilers | Large |
| 6 | V8 Hooks & Reports | Large |
| 7 | Coredump (Optional) | Very Large |
| 8 | Final Integration | Medium |

## Risk Assessment

### High Risk Items

1. **V8 Profiler Access**: napi-rs doesn't directly expose V8 profiler APIs
   - Mitigation: Use Node.js inspector protocol or napi::sys for raw V8 access

2. **Thread Safety**: Rust's ownership model may conflict with current shared state patterns
   - Mitigation: Use Arc<Mutex<>> or lock-free data structures

3. **Platform-Specific Code**: Windows named pipes and Linux coredump are complex
   - Mitigation: Consider FFI to existing C code for complex platform code

### Medium Risk Items

1. **GC Callbacks**: Need to ensure GC callback registration works correctly
   - Mitigation: Test extensively with memory-intensive workloads

2. **Performance Regression**: Rust implementation might have different characteristics
   - Mitigation: Benchmark early and often

### Low Risk Items

1. **Configuration System**: Straightforward port
2. **Logger**: Simple file I/O operations
3. **Command Parser**: serde_json makes this easier

## Testing Strategy

### Unit Tests

Each module should have comprehensive unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = XprofilerConfig::default();
        assert_eq!(config.log_interval, 60);
    }
}
```

### Integration Tests

Test the full flow from JavaScript:

```javascript
// test/rust-migration.test.js
const xprofiler = require('../');

describe('Rust Migration', () => {
  it('should start and stop profiling', async () => {
    xprofiler.start({ log_dir: os.tmpdir() });
    // ... test commands
  });
});
```

### Benchmark Tests

Compare C++ and Rust implementations:

```javascript
const Benchmark = require('benchmark');

new Benchmark.Suite()
  .add('C++ configure', () => cppBinding.configure(config))
  .add('Rust configure', () => rustBinding.configure(config))
  .run();
```

## Rollback Plan

If issues are discovered after deployment:

1. Feature flag `XPROFILER_USE_RUST=false` falls back to C++
2. Keep C++ code in `legacy/` branch for emergency patches
3. Maintain both npm packages during transition period

## Conclusion

This migration will modernize xprofiler's native layer while maintaining full backward compatibility. The phased approach allows for incremental progress and validation at each step. Key benefits include improved memory safety, better cross-platform support, and easier long-term maintenance.

## Appendix A: napi-rs Quick Reference

### Basic Function Export

```rust
use napi_derive::napi;

#[napi]
pub fn my_function(arg: String) -> napi::Result<String> {
    Ok(format!("Hello, {}!", arg))
}
```

### Object Export

```rust
#[napi(object)]
pub struct MyObject {
    pub name: String,
    pub value: i32,
}
```

### Async Function

```rust
#[napi]
pub async fn async_function() -> napi::Result<String> {
    tokio::time::sleep(Duration::from_secs(1)).await;
    Ok("Done".to_string())
}
```

### ThreadsafeFunction

```rust
use napi::threadsafe_function::{ThreadsafeFunction, ThreadSafeCallContext};

#[napi]
pub fn with_callback(callback: ThreadsafeFunction<String>) {
    std::thread::spawn(move || {
        callback.call(Ok("Hello from thread".to_string()), ThreadsafeFunctionCallMode::Blocking);
    });
}
```

## Appendix B: Key Dependencies

| Crate | Purpose |
|-------|---------|
| napi | Node-API bindings |
| napi-derive | Procedural macros |
| tokio | Async runtime |
| serde | Serialization |
| parking_lot | Fast synchronization primitives |
| once_cell | Lazy static initialization |
| chrono | Date/time handling |
| tracing | Structured logging |
| libc | Unix system calls |
| windows | Windows API bindings |
| backtrace | Stack trace capture |
