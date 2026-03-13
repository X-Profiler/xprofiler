# X-Profiler Code Wiki

## 1. 项目概述 (Project Overview)
X-Profiler 是一个专为 Node.js 设计的高性能分析插件（C++ Addon）。它的核心目标是在不影响业务性能的前提下，提供实时的运行时状态采样、性能诊断和崩溃分析能力。它支持多线程架构，能够独立于主线程进行数据采集和指令处理。

## 2. 整体架构 (Architecture)
X-Profiler 采用多线程和 IPC（进程间通信）机制，确保监控逻辑与业务逻辑解耦：

*   **主线程 (Main Thread)**: 执行 Node.js 业务逻辑。
*   **命令监听线程 (Commands Listener Thread)**: 独立于主线程（基于 libuv 线程池），通过 Unix Domain Socket (Linux/macOS) 或 Named Pipe (Windows) 监听外部指令。
*   **日志采集线程 (Log Bypass Thread)**: 定期采样 CPU、内存、GC、libuv 句柄等数据并写入文件，避免阻塞业务事件循环。
*   **IPC 通信**: 外部控制工具（如 `xprofctl`）通过 JSON 格式的消息与插件通信，触发各种诊断操作。
*   **V8 深度集成**: 利用 V8 Profiler API 进行采样，并使用 `v8::Isolate::RequestInterrupt` 机制安全地在主线程空闲时执行诊断任务。

## 3. 主要模块职责 (Main Modules & Responsibilities)

### 3.1 核心指令处理 (`src/commands`)
*   **`listener.cc`**: 启动 IPC 服务端，管理监听线程。
*   **`parser.cc`**: 解析接收到的 JSON 指令并分发给对应的处理器。
*   **`dump.cc`**: 指令执行的中枢，处理指令间的冲突（如不能同时进行 CPU 和 Heap 采样）及依赖关系。
*   **子模块**:
    *   `cpuprofiler/`: CPU 性能采样 (CPU Profile)。
    *   `heapdump/`: 堆快照生成 (Heap Snapshot)。
    *   `gcprofiler/`: GC 耗时分析。
    *   `report/`: 生成节点报告 (Node Report)。

### 3.2 JS/C++ 桥接 (`src/jsapi`)
*   负责将 C++ 实现的底层功能（如日志记录、配置设置、线程启动）导出为 JavaScript 可调用的方法。
*   主要文件如 `export_configure.cc`, `export_logger.cc` 等对应不同功能的导出。

### 3.3 性能日志采样 (`src/logbypass`)
*   负责定期从系统和 V8 引擎获取性能指标。
*   **关键点**: 直接绕过 Node.js 的 JS 层，从 C++ 层写入日志文件，减少对 JS 主线程的干扰。

### 3.4 关键钩子 (`src/hooks`)
*   **`fatal_error.cc`**: 监控 V8 的 Fatal Error，在进程崩溃前生成报告。
*   **`heap_limit.cc`**: 监控堆内存使用，在 OOM (Out of Memory) 前自动触发快照或报告。

### 3.5 跨平台适配 (`src/platform`)
*   抽象了不同操作系统（Unix/Windows）在 IPC、内存分配优化 (`mallopt`) 和系统指标获取上的差异。

## 4. 关键类与函数 (Key Classes & Functions)

### C++ 层
*   **`xprofiler::ParseCmd` (src/commands/parser.cc)**: 指令解析入口，决定了哪些外部命令可以被执行。
*   **`xprofiler::HandleAction` (src/commands/dump.cc)**: 统一的诊断任务执行函数，负责调用具体的 V8 接口或平台接口。
*   **`xprofiler::EnvironmentData` (src/environment_data.h)**: 在 C++ 层存储每个 Isolate 的上下文信息，包括当前的配置、活跃的 Profiler 句柄等。

### JavaScript 层
*   **`xprofiler.js` -> `start()`**: JS 层入口，负责合并用户配置、环境变量，并初始化 native 层的监听线程和钩子。

## 5. 依赖关系 (Dependencies)

### 原生依赖 (Native)
*   **nan**: 用于处理 Node.js 不同版本间的 API 兼容性。
*   **V8 & libuv**: Node.js 核心引擎和异步 IO 库。
*   **json.hpp**: 内置的高性能 JSON 处理库 (`src/library/json.hpp`)。

### JavaScript 依赖
*   **@mapbox/node-pre-gyp**: 用于分发和安装预编译的二进制文件，无需用户本地编译。
*   **moment**: 用于格式化日志文件名和时间戳。
*   **yargs**: 用于解析控制工具 `xprofctl` 的命令行参数。

## 6. 项目运行方式 (Usage & Running)

### 6.1 项目引入
在应用入口文件的第一行引入并启动：

```javascript
const xprofiler = require('xprofiler');
xprofiler.start({
  log_dir: '/path/to/logs', // 日志输出目录
  log_interval: 60          // 采样间隔（秒）
});
```

### 6.2 配置选项 (Configuration)
可以在 `start()` 方法中传入配置对象，或通过环境变量设置：

| 配置项 (Key) | 环境变量 (Env) | 类型 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `log_dir` | `XPROFILER_LOG_DIR` | string | `os.tmpdir()` | 日志输出目录 |
| `log_interval` | `XPROFILER_LOG_INTERVAL` | number | `60` | 采样间隔 (秒) |
| `log_level` | `XPROFILER_LOG_LEVEL` | number | `1` | 日志级别 (0: info, 1: error, 2: debug) |
| `log_type` | `XPROFILER_LOG_TYPE` | number | `0` | 日志输出位置 (0: file, 1: console) |
| `enable_log_uv_handles` | `XPROFILER_ENABLE_LOG_UV_HANDLES` | boolean | `true` | 是否开启 libuv 句柄详情采集 |
| `enable_fatal_error_report` | `XPROFILER_ENABLE_FATAL_ERROR_REPORT` | boolean | `true` | FatalError 时自动生成 Report |
| `enable_fatal_error_coredump` | `XPROFILER_ENABLE_FATAL_ERROR_COREDUMP` | boolean | `false` | FatalError 时自动生成 Coredump |
| `enable_http_profiling` | `XPROFILER_ENABLE_HTTP_PROFILING` | boolean | `false` | CPU 采样期间开启 HTTP Profiling |
| `patch_http` | `XPROFILER_PATCH_HTTP` | boolean | `true` | 是否 Patch HTTP 模块以采集数据 |

### 6.3 手动控制 (xprofctl)
全局安装插件后，可使用命令行工具触发实时分析：

```bash
# 对指定 PID 的进程生成 Heap Snapshot
xprofctl heapdump -p <PID>

# 对指定 PID 的进程进行 30 秒 CPU 采样
xprofctl start_cpu_profiling -p <PID> -t 30
```

### 6.4 自动触发
通过配置 `enable_fatal_error_report: true`，当进程因不可恢复错误崩溃时，会自动在 `log_dir` 生成诊断报告。

### 6.5 编译与测试
*   **编译**: `npm install` (会自动调用 `node-pre-gyp` 或 `node-gyp rebuild`)
*   **测试**: `npm test` (运行 `test/` 目录下的测试用例)
