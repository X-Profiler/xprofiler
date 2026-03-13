# xprofiler 项目代码维基 (Code Wiki)

## 1. 项目概述 (Project Overview)

`xprofiler` 是一个专为 Node.js 设计的高性能 C++ 原生插件（Addon），旨在提供生产环境下的全方位性能监控、实时分析与故障诊断能力。它能够在不显著影响应用性能的前提下，深入 V8 引擎和 Libuv 底层，采集 CPU、内存、GC、句柄等核心指标，并支持动态触发堆快照、CPU Profile 和诊断报告。

### 核心特性
- **低开销监控**: 采用独立的后台线程（LogBypass）进行指标采集和日志落盘，避免阻塞主线程。
- **多维分析**: 支持 CPU Profiling、Heap Dump、GC Profiling、Libuv 句柄分析等。
- **实时交互**: 提供 IPC 通讯机制，允许通过外部命令（`xprofctl`）实时控制采样和导出数据。
- **故障自愈**: 内置 FatalError 钩子，可在进程崩溃前自动生成 Coredump 或诊断报告。
- **灵活配置**: 支持动态调整配置，兼容 Alinode 日志格式。

---

## 2. 整体架构 (Architecture)

项目采用 **JavaScript + C++ Addon** 的混合架构，利用 N-API (通过 NAN 抽象层) 与 V8 引擎交互。

### 2.1 线程模型
`xprofiler` 的运行时模型包含以下关键组件：
1.  **Main Thread (JS Thread)**: 执行用户 JS 代码，加载插件，处理部分同步 API。
2.  **LogBypass Thread**: 一个独立的 Libuv 线程，负责周期性（默认 60s）采集性能数据并写入日志文件，确保监控逻辑不阻塞业务。
3.  **Command Listener Thread**: 启动一个 IPC Server (Unix Domain Socket 或 Named Pipe)，监听外部管理工具（如 `xprofctl`）的指令。

### 2.2 模块交互
- **JS 层 (`lib/`, `xprofiler.js`)**: 负责参数归一化、环境初始化、加载 C++ 模块、以及部分高层逻辑（如 HTTP Patch）。
- **C++ 层 (`src/`)**: 核心实现，直接操作 V8 Inspector、Isolate 和 OS 系统调用。
- **IPC 通讯**: 外部工具通过 Socket 发送 JSON 指令，Listener 线程解析后调度相应模块执行（如开始 CPU 采样）。

---

## 3. 主要模块职责 (Main Modules)

### 3.1 核心入口
- **[xprofiler.js](file:///workspace/xprofiler.js)**: 用户侧入口。负责加载 binary，合并配置，调用 `xprofiler.start()` 启动插件。
- **[src/xprofiler.cc](file:///workspace/src/xprofiler.cc)**: C++ 插件入口。注册模块导出函数（`Configure`, `Start`, `GetConfig` 等），初始化全局状态。

### 3.2 功能模块 (`src/commands/`)
这些模块实现了具体的分析与采样逻辑，通常由 IPC 指令触发：

| 模块 | 源码路径 | 职责 |
| :--- | :--- | :--- |
| **cpuprofiler** | `src/commands/cpuprofiler/` | 封装 V8 `CpuProfiler`，采集 CPU 执行热点，生成 `.cpuprofile` 文件。 |
| **heapdump** | `src/commands/heapdump/` | 封装 V8 `HeapProfiler`，生成堆快照 (`.heapsnapshot`)，用于分析内存泄漏。 |
| **gcprofiler** | `src/commands/gcprofiler/` | 监听 GC 回调，统计 GC 类型、暂停时间及频率。 |
| **heapprofiler** | `src/commands/heapprofiler/` | 实现 Sampling Heap Profiler，低开销地分析内存分配模式。 |
| **report** | `src/commands/report/` | 生成类似于 `node --report` 的详细诊断报告，包含堆栈、环境信息、资源限制等。 |
| **listener** | `src/commands/listener.cc` | IPC 服务端实现，解析并分发指令。 |

### 3.3 监控模块 (`src/logbypass/`)
后台日志采集系统的核心实现：
- **[log.cc](file:///workspace/src/logbypass/log.cc)**: 调度器，定时触发各子模块采集数据。
- **cpu.cc, heap.cc, gc.cc, libuv.cc, http.cc**: 分别负责采集对应的性能指标。
- **日志格式**: 支持标准格式和 Alinode 兼容格式。

### 3.4 钩子与补丁 (`src/hooks/`, `patch/`)
- **[src/hooks/fatal_error.cc](file:///workspace/src/hooks/fatal_error.cc)**: 注册 V8 FatalErrorCallback，在 OOM 或崩溃时保留现场（生成 Report/Coredump）。
- **[lib/patch/http.js](file:///workspace/lib/patch/http.js)**: Monkey Patch Node.js 的 `http` 模块，用于统计 QPS、响应时间等 HTTP 指标。

---

## 4. 关键类与函数 (Key Classes & Functions)

### C++ 层
- **`XProfiler::Initialize`** ([src/xprofiler.cc](file:///workspace/src/xprofiler.cc)): 插件初始化函数，设置 V8 Platform，初始化各个子模块。
- **`Listener::Run`** ([src/commands/listener.cc](file:///workspace/src/commands/listener.cc)): 启动 IPC 监听循环。
- **`LogBypass::Run`** ([src/logbypass/log.cc](file:///workspace/src/logbypass/log.cc)): 启动后台日志线程循环。
- **`Parser::Parse`** ([src/commands/parser.cc](file:///workspace/src/commands/parser.cc)): 解析 IPC 消息并路由到对应的 Handler。

### JavaScript 层
- **`XProfiler.start(options)`** ([xprofiler.js](file:///workspace/xprofiler.js)): 用户调用的启动方法，配置并启动插件。
- **`configure(options)`** ([lib/configure.js](file:///workspace/lib/configure.js)): 处理用户配置与默认值的合并。

---

## 5. 配置与使用 (Configuration & Usage)

### 5.1 安装
```bash
npm install xprofiler
```
*依赖 `@mapbox/node-pre-gyp` 进行预编译二进制下载，如果下载失败会自动尝试本地编译。*

### 5.2 常用配置
配置可在 `start()` 方法中传入，或通过环境变量设置。完整定义见 [configuration.js](file:///workspace/configuration.js)。

| 配置项 (Key) | 环境变量 (Env) | 默认值 | 说明 |
| :--- | :--- | :--- | :--- |
| `log_dir` | `XPROFILER_LOG_DIR` | `/tmp` | 日志及 Profiling 文件输出目录 |
| `log_interval` | `XPROFILER_LOG_INTERVAL` | `60` | 监控日志采集间隔 (秒) |
| `enable_fatal_error_report` | `XPROFILER_ENABLE_FATAL_ERROR_REPORT` | `true` | 崩溃时是否生成诊断报告 |
| `enable_fatal_error_coredump` | `XPROFILER_ENABLE_FATAL_ERROR_COREDUMP` | `false` | 崩溃时是否生成 Coredump (Linux only) |
| `patch_http` | `XPROFILER_PATCH_HTTP` | `true` | 是否采集 HTTP 接口性能数据 |

### 5.3 运行方式
在应用入口最顶部引入并启动：
```javascript
const xprofiler = require('xprofiler');
xprofiler.start({
  log_dir: './logs',
  log_interval: 60
});
```

---

## 6. 指令与控制 (Commands)

可以通过 IPC 通道向运行中的进程发送指令。通常使用配套的 CLI 工具或代码触发。

### 支持的指令 (IPC Commands)
定义于 [src/commands/parser.cc](file:///workspace/src/commands/parser.cc)：
- `check_version`: 返回插件版本。
- `get_config` / `set_config`: 获取或动态修改配置。
- `start_cpu_profiling` / `stop_cpu_profiling`: 控制 CPU 采样。
- `heapdump`: 生成堆快照。
- `diag_report`: 生成诊断报告。
- `start_gc_profiling` / `stop_gc_profiling`: 控制 GC 监控。

---

## 7. 构建与依赖 (Build & Dependencies)

### 依赖关系
- **nan**: C++ Native Abstraction for Node.js，处理不同 Node 版本 API 差异。
- **node-pre-gyp**: 用于分发预编译的二进制包。
- **moment**: 用于 JS 层的时间处理。
- **uuid**: 生成唯一 ID。

### 构建流程
1.  **环境准备**: 需要 Python, Make, GCC/Clang (Linux/macOS) 或 MSVC (Windows)。
2.  **安装依赖**: `npm install`
3.  **本地编译**: `npm run build` (调用 `node-pre-gyp rebuild`)
4.  **测试**: `npm test` (运行 `test/` 目录下的 Mocha 测试用例)

### 目录结构说明
```
/
├── bin/            # CLI 工具入口
├── lib/            # JavaScript 源码 (配置, HTTP Patch 等)
├── src/            # C++ 源码
│   ├── commands/   # 具体的 Profiling/Reporting 命令实现
│   ├── hooks/      # V8 钩子 (FatalError, HeapLimit)
│   ├── logbypass/  # 后台日志采集线程实现
│   ├── platform/   # 平台相关实现 (Linux/Mac/Win)
│   └── xprofiler.cc # 插件入口
├── test/           # 测试用例
├── binding.gyp     # 编译配置文件
└── package.json    # 项目元数据
```
