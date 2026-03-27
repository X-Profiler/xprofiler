# X-Profiler Code Wiki

## 1. 项目概览 (Project Overview)

`xprofiler` 是一个用于 Node.js 的性能监控与诊断 C++ 插件（addon）。它能够周期性地输出运行时的性能日志，并且允许开发者通过外部命令对运行中的 Node.js 进程进行实时的状态采样（如 CPU 采样、堆快照、GC 采样等）。
该插件基于 V8 和 libuv 提供的底层能力，支持 Node.js 的主线程以及 Worker 线程环境，是构建企业级 Node.js 监控平台（如 Easy-Monitor v3.0）的核心组件。

**核心能力：**
- **性能日志记录 (Log Bypass)**：后台定时收集 CPU、内存（Heap）、GC 状态、libuv 句柄数量、HTTP 响应等指标，并写入日志文件（支持兼容 AliNode 格式）。
- **诊断与采样 (Profiling & Dumping)**：支持 CPU Profiling、Heapdump、Sampling Heap Profiling、GC Profiling 以及 Node.js Diagnostic Report。
- **异常捕获与保护 (Hooks)**：拦截 V8 Fatal Error 并自动生成诊断报告或 Coredump；支持在进程接近 OOM (Out Of Memory) 时自动增加堆内存上限。
- **CLI 命令行交互**：内置 `xprofctl` 命令行工具，通过 IPC 与宿主进程通信触发上述操作。

---

## 2. 项目整体架构 (Overall Architecture)

`xprofiler` 采用了 **JS 层 + Native C++ 层 + 独立后台线程** 的架构模式。

1. **JS 层 (JavaScript API & CLI)**
   - 负责提供对外的配置初始化入口 `xprofiler.start()`。
   - 使用 Node.js 的 `diagnostics_channel` 模块对 HTTP 请求进行安全拦截，收集请求数据并传递给 C++ 层。
   - 提供 `xprofctl` CLI 工具，通过进程间通信 (IPC) 向 Native 层发送控制指令。

2. **Native C++ 层 (Core Addon)**
   - 基于 `nan` (Native Abstractions for Node.js) 编写，桥接 JS 和 C++。
   - **EnvironmentRegistry & EnvironmentData**：负责管理和隔离 Node.js 主线程与各个 Worker 线程的上下文状态。
   - **V8 交互**：通过 `v8::Isolate` 获取 V8 引擎内部数据，或使用 `RequestInterrupt` 安全地中断 JS 执行来完成 Profiling/Dumping 操作。

3. **独立后台线程 (Background Threads)**
   为避免阻塞 Node.js 的主事件循环，插件在 C++ 层启动了独立的 libuv 线程：
   - **LogBypass Thread**：包含两个定时器，分别负责每秒采集一次 CPU 数据，和每隔特定时间（默认 60s）收集并写入所有环境的性能日志。
   - **CommandsListener Thread**：运行 IPC Server（Unix Domain Socket 或 Windows Named Pipe），监听并解析 `xprofctl` 发送的指令。

---

## 3. 主要模块职责 (Main Module Responsibilities)

### 3.1 C++ 源码模块 (`src/`)

- **`src/xprofiler.cc`**: C++ 插件的入口文件，负责通过 `nan` 宏暴露 C++ 方法给 JS 层调用。
- **`src/logbypass/` (日志旁路模块)**:
  - 核心模块，运行在独立的后台线程中。负责周期性收集 CPU (`cpu.cc`)、GC 状态 (`gc.cc`)、堆内存 (`heap.cc`)、HTTP 指标 (`http.cc`) 和 Libuv 句柄 (`libuv.cc`)，最终由 `log.cc` 写入日志文件。
- **`src/commands/` (命令控制模块)**:
  - `listener.cc`: 启动并维护 IPC 服务端线程。
  - `parser.cc`: 接收并解析来自客户端的 JSON 格式命令。
  - `dump.cc`: 统筹各种诊断命令（DumpAction）的执行。当需要进行耗时操作时，会通过 V8 的 `RequestInterrupt` 将任务抛给目标 JS 线程执行，确保线程安全。
  - 子目录（`cpuprofiler/`, `gcprofiler/`, `heapdump/`, `report/`, `coredumper/` 等）：对应具体底层 V8 接口的调用与文件生成逻辑。
- **`src/hooks/` (V8 钩子模块)**:
  - `fatal_error.cc`: 设置 V8 `FatalErrorHandler` 和 `OOMErrorHandler`，在进程崩溃前保存诊断报告或 Coredump。
  - `heap_limit.cc`: 使用 `AddNearHeapLimitCallback` 实现 OOM 发生前自动扩容堆内存上限的功能。
- **`src/platform/` (跨平台模块)**:
  - 封装 Unix 和 Windows 下的系统级调用差异（如 CPU 统计、IPC 管道创建、Coredump 生成、进程信息获取）。

### 3.2 JS 源码模块

- **`xprofiler.js`**: 模块入口，加载编译后的 `.node` 文件并初始化各个子系统。
- **`patch/` (模块 Patch)**:
  - `http.js`: 利用 Node 内部的 `diagnostics_channel` (`http.server.request.start`) 监听请求生命周期，无侵入地获取 HTTP QPS、耗时、状态码等数据并送给 C++ 层统计。
- **`lib/` (工具与通信)**:
  - `xctl.js`: 实现客户端与服务端（目标 Node 进程）基于 Socket 的 IPC 通信机制。
- **`bin/xprofctl`**: 命令行客户端，通过 `yargs` 解析终端参数，并向目标进程发送控制命令。

---

## 4. 关键类与函数说明 (Key Classes & Functions)

### 4.1 JS 侧关键方法
- **`xprofiler.start(config)`**: 
  - 插件初始化方法。该方法会根据传入参数和环境变量生成最终配置，初始化 C++ 层的日志线程 (`runLogBypass`)、命令监听线程 (`runCommandsListener`)，设置异常钩子 (`setHooks`)，并启动 HTTP 请求的拦截补丁 (`patch()`)。
- **`subscribeHttpServerRequestStart(options)`** ([http.js](file:///workspace/patch/http.js)):
  - HTTP 拦截核心。使用 `diagnostics_channel` 监听 HTTP 请求，并在请求开始、结束、超时时调用 C++ 层暴露的方法（如 `addLiveRequest`, `addSentRequest`）进行计数。

### 4.2 C++ 侧关键类与函数
- **`class EnvironmentData`** ([environment_data.cc](file:///workspace/src/environment_data.cc)):
  - 用于绑定每个 `v8::Isolate` (即对应一个 Node.js 线程环境)。由于 Node.js 支持 Worker Threads，因此插件必须支持多线程上下文。此类用于存放该线程的独立状态，如 Profiling 状态和文件路径等。
- **`class LogByPass`** ([log.cc](file:///workspace/src/logbypass/log.cc)):
  - 日志旁路线程的核心类。其内部维护了两个 `uv_timer_t` 句柄：`OnCpuInterval` 用于高频抓取 CPU 状态；`OnLogInterval` 负责定时通知各个 JS 线程汇集统计数据并落盘。
- **`void ParseCmd(char* command)`** ([parser.cc](file:///workspace/src/commands/parser.cc)):
  - 命令分发中心。将收到的字符串解析为 JSON，提取出具体的 `cmd`，分发给相应的处理函数（如 `StartCpuProfiling`、`Heapdump` 等）。
- **`void HandleAction(...)`** ([dump.cc](file:///workspace/src/commands/dump.cc)):
  - 在目标 JS 线程中执行具体的诊断动作（DumpAction）。此函数必须在 V8 Isolate 被锁定的情况下执行。为了处理诸如 CPU 采样等需要延迟的操作，使用了 `ProfilingWatchdog` 等机制进行异步守护。
- **`size_t NearHeapLimitCallback(...)`** ([heap_limit.cc](file:///workspace/src/hooks/heap_limit.cc)):
  - 当 V8 堆内存接近上限时的回调函数。根据配置中的 `auto_incr_heap_limit_size`，动态返回一个更大的数值，从而避免程序直接 OOM 崩溃。

---

## 5. 依赖关系 (Dependencies)

### 5.1 运行时依赖 (Dependencies)
- **`@mapbox/node-pre-gyp`**: 用于在用户安装插件时，优先从 GitHub Releases (或指定的国内镜像源) 下载预编译的 C++ 二进制文件，避免本地编译环境缺失导致安装失败。
- **`nan`**: Node.js C++ 插件的兼容性抽象层，屏蔽了不同 Node.js / V8 版本之间的 API 差异。
- **`yargs`**: 强大的 Node.js 命令行参数解析工具，用于构建 `xprofctl`。
- **`moment`**: 时间格式化处理库。
- **`uuid`**: 唯一标识生成工具。

### 5.2 C++ 依赖
- **`nlohmann/json`** (`src/library/json.hpp`): C++ 端的轻量级 JSON 序列化/反序列化库，用于 IPC 通信过程中的数据解析。

### 5.3 开发与测试依赖 (DevDependencies)
- **`mocha`**, **`expect.js`**: 单元测试框架。
- **`nyc`**: 代码覆盖率统计工具。
- **`clang-format`**: C++ 代码格式化工具。

---

## 6. 项目运行方式 (Usage & Execution)

### 6.1 开发与编译
项目基于 `node-gyp` 和 `node-pre-gyp` 构建：
```bash
# 安装依赖并触发本地编译 (如果未找到预编译版本)
npm install

# 强制本地重新编译
npm run build

# 运行单元测试
npm run test

# 检查代码覆盖率
npm run cov
```

### 6.2 嵌入 Node.js 项目中
在 Node.js 应用的入口文件顶部引入并启动，即可开启性能日志输出与后台诊断监听：

```javascript
const xprofiler = require('xprofiler');
xprofiler.start({
  log_dir: '/path/to/log',         // 性能分析日志输出目录
  log_interval: 60,                // 采样间隔（秒）
  enable_fatal_error_hook: true,   // 开启 OOM/Fatal Error 捕获
  log_format_alinode: false        // 是否以 AliNode 兼容格式输出
  // 更多配置参考 README...
});
```

### 6.3 使用命令行工具 (CLI)
通过全局安装或在项目 `node_modules/.bin/` 中调用 `xprofctl`：

```bash
# 获取进程支持的 xprofiler 版本
xprofctl check_version -p <pid>

# 生成 CPU 采样 (默认持续时间后停止，需手动/定时停止)
xprofctl start_cpu_profiling -p <pid> -t 5000

# 生成 Heap 快照
xprofctl heapdump -p <pid>

# 生成诊断报告 (Diagnostic Report)
xprofctl diag_report -p <pid>
```
生成的分析文件通常保存在配置指定的 `log_dir` 目录中（默认在系统的 `os.tmpdir()` 目录下）。
