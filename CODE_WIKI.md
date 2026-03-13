# Code Wiki: xprofiler

## 1. 项目概述

**xprofiler** 是一个用于 Node.js 的高性能原生性能分析与诊断插件。它结合了 C++ Addon 的底层能力与 JavaScript 的灵活性，能够在生产环境中低开销地采集系统监控指标、生成性能快照（CPU Profile, Heap Snapshot）以及处理运行时错误。

### 核心功能
*   **性能监控**：采集 CPU 使用率、内存趋势、GC 统计、Libuv 句柄数、HTTP吞吐量等。
*   **诊断工具**：生成 CPU Profile、Heap Snapshot、GC Trace，支持生成 Coredump 文件。
*   **自动保护**：支持堆内存上限监控与自动快照，支持 Fatal Error 时的自动报告。
*   **低开销**：核心采集逻辑在独立的 C++ 线程中运行，最小化对 Node.js 主线程（Event Loop）的影响。

### 技术栈
*   **语言**：C++ (核心逻辑), JavaScript (接口与工具)
*   **构建系统**：node-gyp, CMake (部分平台)
*   **依赖库**：Nan (Native Abstractions for Node.js), v8 Profiler API

---

## 2. 整体架构

xprofiler 采用典型的 **C++ Addon + JavaScript Wrapper** 架构。

*   **C++ Layer (`src/`)**：直接与 V8 引擎和操作系统交互。
    *   **Log Bypass**：独立线程，定时采集各项性能指标并写入日志。
    *   **Commands Listener**：监听 IPC 通信（Unix Socket / Named Pipe），接收外部控制指令。
    *   **Profiler Implementation**：封装 V8 的 CPU/Heap Profiler 接口。
    *   **Platform Abstraction**：屏蔽 Linux/macOS/Windows 的系统调用差异。

*   **JavaScript Layer (`lib/`, `xprofiler.js`)**：
    *   **API Surface**：暴露给用户的配置与控制接口。
    *   **Patching**：通过 `shimmer` 对 Node.js 核心模块（如 `http`）进行非侵入式插桩。
    *   **Configuration**：管理用户配置与默认值合并。

---

## 3. 主要模块职责

### 3.1 C++ 核心模块 (`src/`)

| 目录/文件 | 职责说明 |
| :--- | :--- |
| **`commands/`** | **诊断命令实现**。包含各类 Profiler 的具体逻辑。 |
| &emsp;`cpuprofiler/` | 对接 `v8::CpuProfiler`，生成 CPU 采样数据。 |
| &emsp;`heapdump/` | 对接 `v8::HeapProfiler`，生成全量堆快照。 |
| &emsp;`gcprofiler/` | 注册 GC 回调，采集详细的 GC 耗时与内存变化。 |
| &emsp;`coredumper/` | 实现进程级 Core Dump 生成（主要支持 Linux）。 |
| &emsp;`report/` | 生成类似 Node.js 官方的诊断报告。 |
| **`hooks/`** | **系统钩子**。监听关键生命周期事件。 |
| &emsp;`fatal_error.cc` | 挂载 V8 Fatal Error 回调，在崩溃前抢救现场信息。 |
| &emsp;`heap_limit.cc` | 监控堆内存使用量，超限时触发快照或报警。 |
| **`logbypass/`** | **日志采集线程**。在独立线程中采集 CPU、Memory、Libuv 等指标，避免阻塞主线程。 |
| **`jsapi/`** | **JS 桥接层**。定义导出给 JS 的 Native 方法，如 `StartCpuProfiling` 等。 |
| **`platform/`** | **跨平台兼容**。封装 Linux/Windows/macOS 的底层差异（如获取 CPU 占用率、IPC 通信）。 |

### 3.2 JavaScript 模块

| 目录/文件 | 职责说明 |
| :--- | :--- |
| **`xprofiler.js`** | **入口文件**。负责初始化 Addon、启动后台线程、加载 Patch。 |
| **`lib/`** | **辅助库**。 |
| &emsp;`configure.js` | 处理配置项，支持从环境变量覆盖配置。 |
| &emsp;`xctl.js` | 实现控制客户端，通过 IPC 向运行中的 xprofiler 发送指令。 |
| &emsp;`worker_threads.js` | 兼容 Node.js Worker Threads 环境。 |
| **`patch/`** | **自动插桩**。对 `http` 等模块进行 monkey-patch，自动采集请求耗时与状态码。 |

---

## 4. 关键类与函数说明

### 4.1 C++ 类 (Native)

#### `CpuProfiler` (`src/commands/cpuprofiler/cpu_profiler.h`)
*   **功能**：管理 CPU 采样会话。
*   **主要方法**：
    *   `StartProfiling(isolate, title)`: 开始采样。
    *   `StopProfiling(isolate, title)`: 停止采样并返回 Profile 结构树。

#### `HeapProfiler` (`src/commands/heapdump/heap_profiler.h`)
*   **功能**：管理堆内存快照。
*   **主要方法**：
    *   `TakeSnapshot(isolate)`: 暂停执行并生成当前堆内存的快照。

#### `XProfiler` (概念上的单例)
*   虽然代码中通过 `xprofiler.cc` 的 `Initialize` 函数初始化，但它充当了整个 Addon 的控制器，负责启动 `LogBypassThread` 和 `CommandsListenerThread`。

### 4.2 JavaScript 函数

#### `xprofiler.start(options)`
*   **位置**：[xprofiler.js](file:///workspace/xprofiler.js)
*   **功能**：启动监控。
*   **参数**：`options` 对象，包含 `log_dir`, `log_interval` 等配置。
*   **逻辑**：
    1.  调用 `configure.js` 处理配置。
    2.  调用 Native 方法启动日志线程。
    3.  应用 `patch/` 下的自动插桩。

#### `xctl.run(command)`
*   **位置**：[lib/xctl.js](file:///workspace/lib/xctl.js)
*   **功能**：向指定 PID 的 Node.js 进程发送诊断命令。
*   **支持命令**：`check_version`, `start_cpu_profiling`, `stop_cpu_profiling`, `heapdump`, `gc_tracing` 等。

---

## 5. 依赖关系与构建

### 主要依赖
*   **nan**: C++ Addon 开发的基础抽象层，确保跨 Node.js 版本兼容。
*   **moment**: JavaScript 侧的日期处理。
*   **uuid**: 生成唯一的 Profile ID 或 Request ID。
*   **@mapbox/node-pre-gyp**: 用于分发预编译的二进制包，减少用户安装时的编译时间。

### 构建系统
项目使用 `node-gyp` 进行构建。关键配置文件为 [binding.gyp](file:///workspace/binding.gyp)，定义了编译目标、源文件列表、包含路径和链接库。

### 运行方式

1.  **安装依赖**：
    ```bash
    npm install
    ```

2.  **编译项目** (开发模式)：
    ```bash
    npm run build
    ```
    或者完全重新编译：
    ```bash
    node-gyp rebuild
    ```

3.  **运行测试**：
    ```bash
    npm test
    ```
    测试框架使用 `mocha`，测试用例位于 `test/` 目录下。

4.  **集成到应用**：
    ```javascript
    const xprofiler = require('xprofiler');
    xprofiler.start({
        log_dir: '/tmp/xprofiler_logs',
        log_interval: 60
    });
    ```
