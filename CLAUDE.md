# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

X-Profiler is a Node.js native addon for runtime profiling and performance monitoring. It outputs performance logs at regular intervals and enables real-time runtime state sampling via external commands (xprofctl CLI).

**Supported Platforms:** Windows, Linux (x64/arm64), macOS (x64/arm64)
**Node.js Versions:** v18.x, v20.x, v22.x, v24.x (LTS only)

## Build Commands

```bash
# Build the native addon (requires C++ toolchain)
npm run build

# Build with pre-built binary download (fallback to build from source)
npm install

# Run all tests
npm run test

# Run a single test file
npm run test-single test/config.test.js

# Generate coverage report
npm run cov

# Run coverage for a single test
npm run cov-single test/config.test.js

# Full CI suite (lint + build + coverage)
npm run ci

# Format C++ code (requires clang-format)
npm run format

# Lint JavaScript code
npm run lint
```

## Build System & C++ Compilation

**CRITICAL:** Node.js v24+ requires C++20. The binding.gyp must use:
- Linux: `-std=c++20` in `cflags`
- macOS: `-std=c++20` in `xcode_settings.OTHER_CFLAGS`

The project uses:
- **node-gyp** for native compilation
- **@mapbox/node-pre-gyp** for binary distribution via GitHub Releases
- **NAN (Native Abstractions for Node.js)** for V8 API compatibility

Build output: `build/binding/Release/node-v{abi}-{platform}-{arch}/xprofiler.node`

## Architecture

### Multi-Threading Model

The addon runs **three concurrent threads**:

1. **Main Thread** - JavaScript execution, configuration, API calls
2. **LogBypass Thread** - Periodic statistics collection (separate libuv loop)
3. **CommandsListener Thread** - IPC socket server accepting xprofctl commands

### Code Organization

**JavaScript Layer** (`/lib/`, `/patch/`, root):
- `xprofiler.js` - Main entry point and public API
- `lib/configure.js` - Configuration merging (env vars → defaults → user config)
- `patch/http.js` - HTTP module patching via diagnostics_channel (Node.js v18+)

**Native C++ Layer** (`/src/`):

**Core:**
- `xprofiler.cc` - Module initialization, NAN bindings export
- `environment_data.h/cc` - Per-isolate profiling state
- `environment_registry.h/cc` - Multi-isolate management

**JavaScript Bindings** (`/src/jsapi/`):
- Bridge between JS and C++ (export_configure, export_logger, export_http, etc.)

**Statistics Collection** (`/src/logbypass/`):
- Runs in separate thread, samples at `log_interval` (default 60s)
- Collects: CPU, heap, GC, libuv handles, HTTP metrics

**Remote Commands** (`/src/commands/`):
- IPC listener accepts commands from `xprofctl` CLI
- Profiling: CPU profiling, heap profiling, heap snapshots, GC profiling
- Diagnostics: Reports (JS/native stacks, heap stats, UV stats, system info)
- Simple: version, config get/set

**Platform-Specific** (`/src/platform/`):
- `unix/` - Linux/macOS (Unix sockets, /proc/stat, getrusage)
- `win/` - Windows (named pipes, Performance Counters, DbgHelp)

**V8 Hooks** (`/src/hooks/`):
- `fatal_error.cc` - FatalError handler (can trigger diagnostics/coredump)
- `heap_limit.cc` - Auto-increment heap limit on OOM

## Configuration System

**3-Level Priority:**
```
User Config (xprofiler.start({...}))  [highest]
    ↓
Environment Variables (XPROFILER_*)
    ↓
Default Config (configuration.js)     [lowest]
```

**Key Configurations:**
- `log_dir` - Output directory (default: `os.tmpdir()`)
- `log_interval` - Sampling interval in seconds (default: 60)
- `log_format_alinode` - Alinode format compatibility (default: false)
- `patch_http` - Enable HTTP patching (default: true)
- `enable_fatal_error_hook` - FatalError handling (default: true)
- `enable_auto_incr_heap_limit` - Auto-grow heap on OOM (default: false)

See `configuration.js` for full list and README.md for environment variable names.

## Testing

**Framework:** Mocha + expect.js
**Coverage:** nyc (Istanbul)

**Test Structure:**
- `/test/*.test.js` - Unit tests for core features
- `/test/patch/*.test.js` - Module patching tests
- `/test/fixtures/` - Test helpers and utilities

**Important:** Tests use `mm` library for mocking. Always call `mm.restore()` in afterEach hooks.

## Common Development Tasks

### Adding a New Configuration Option

1. Add default value to `configuration.js`
2. Add environment variable parsing to `lib/configure.js`
3. Update native config struct in `src/configure.h`
4. Add JS binding in `src/jsapi/export_configure.cc`
5. Add test to `test/config.test.js`

### Adding a New xprofctl Command

1. Define command in `src/commands/parser.cc`
2. Implement handler in appropriate `src/commands/*/` subdirectory
3. Add command output logic in `src/commands/dump.cc`
4. Update `bin/xprofctl` CLI with new command/args
5. Add test to `test/commands.test.js`

### Modifying Statistics Collection

1. Update data structures in `src/library/common.h`
2. Modify collection logic in `src/logbypass/*.cc`
3. Update output formatting in logger
4. Add test to `test/logbypass.test.js`

## Platform-Specific Considerations

**Linux:**
- CPU stats via `/proc/stat` parsing
- Coredump support via `src/platform/unix/core/linux/`
- glibc mallopt configuration

**macOS:**
- CPU stats via `getrusage()`
- Stack traces via macOS-specific APIs
- No coredump support

**Windows:**
- Named pipes for IPC (not Unix sockets)
- Windows Performance Counters for CPU
- DbgHelp.dll for stack traces
- Different path handling (`\\.\pipe\xprofiler-ctl`)

## IPC Protocol

**Socket Path:**
- Unix: `<log_dir>/xprofiler-ctl-uds-path.sock`
- Windows: `\\.\pipe\xprofiler-ctl`

**Command Format:** JSON messages between xprofctl and CommandsListener thread

**Output Location:** Profiling artifacts saved to `log_dir` with timestamps

## CI/CD

**GitHub Actions:** `.github/workflows/nodejs.yml`
- Matrix: 3 OS × 4 Node.js versions
- Steps: Build → Test → Coverage → Upload to codecov
- Runs on every commit

**Binary Distribution:**
- Pre-built binaries released to GitHub Releases
- node-pre-gyp downloads matching binary during npm install
- Fallback to source compilation if binary unavailable

## Important Notes

- **Thread Safety:** Use `xpf_mutex.h` for synchronization between threads
- **V8 Isolation:** Use `RequestInterrupt()` for safe cross-thread V8 access
- **NAN Deprecations:** Some NAN APIs are deprecated in Node.js v24 (warnings are non-critical)
- **Log Format:** Two formats supported (xprofiler native vs Alinode compatibility)
- **Process Tracking:** Uses `~/.xprofiler` file to track running processes with xprofiler enabled
