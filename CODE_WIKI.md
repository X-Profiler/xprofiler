# X-Profiler Code Wiki

## 1. 项目整体架构 (Overall Architecture)
X-Profiler 是一个用于 Node.js 的性能监控与诊断运行时插件（Easy-Monitor v3.0 核心组件）。它通过 C++ Addon 与 JavaScript 层的配合，在不阻塞主线程的情况下，收集 Node.js 进程的底层性能数据（如 CPU、堆内存、GC、libuv 句柄、HTTP 请求等），并支持通过 IPC（进程间通信）下发指令进行实时状态采样（如导出 CPU Profile、Heapdump、诊断报告等）。

项目架构主要分为三层：
- **JavaScript 暴露层与工具层**：负责配置管理、入口封装、CLI 命令行工具（`xprofctl`）以及对原生模块（如 `http`）的 Monkey Patch。
- **C++ Addon 接口层 (`jsapi`)**：提供 JS 与 C++ 核心逻辑交互的桥梁，利用 `nan` 绑定 V8 引擎暴露配置和日志接口。
- **C++ 核心诊断层 (`src`)**：包括多线程的后台日志收集（`logbypass`）、独立的 IPC 命令监听线程（`commands`）、V8 引擎钩子（`hooks`）以及跨平台适配实现（`platform`）。

## 2. 主要模块职责 (Main Module Responsibilities)

### 2.1 C++ 核心模块 (`src/`)
- **commands (`src/commands`)**: 诊断命令中心。在独立线程中启动 IPC 服务器（Domain Socket / Named Pipe），接收来自 CLI 工具的指令（如生成 CPU 性能分析、Heapdump、诊断报告），解析并执行诊断操作后返回结果。
- **hooks (`src/hooks`)**: V8 引擎与 Node.js 生命周期钩子模块。拦截 OOM (Out-Of-Memory) 等 Fatal Error 导出崩溃报告或 Coredump；拦截内存上限触发自动扩容堆内存等。
- **jsapi (`src/jsapi`)**: JS 绑定层。负责导出 C++ 内部函数（如日志记录、初始化 Hooks、设置配置）给 Node.js/JavaScript 层调用。
- **logbypass (`src/logbypass`)**: 旁路日志收集模块。在后台独立线程（基于 libuv 的 event loop）中定期收集 CPU、GC、Heap、HTTP 及 Libuv 的性能指标并异步落盘，完全不阻塞 JS 主线程。
- **platform (`src/platform`)**: 跨平台系统级适配模块。屏蔽 Unix/Linux、macOS 和 Windows 之间的系统差异，提供获取 CPU 利用率、创建 IPC Server/Client、生成 Coredump 等统一接口。
- **library (`src/library`)**: 基础类库。包含跨模块使用的 JSON 格式化、错误处理 (`XpfError`)、日志写入、时间与字符串工具函数。

### 2.2 JavaScript 工具模块 (`lib/`, `patch/`, `bin/`)
- **lib/ (`lib/`)**: Node.js 进程控制与 IPC 通信管理。其中 `xctl.js` 负责与 C++ 层建立 IPC 连接发送诊断指令；`configure.js` 负责处理和校验用户传入的参数；`utils.js` 包含通用工具类；`clean.js` 用于清理进程遗留的僵尸 Socket。
- **patch/ (`patch/`)**: 埋点插桩模块。通过 `shimmer.js` 提供安全的 Monkey Patch 能力，重点拦截并包装了 Node.js 原生的 `http` 模块（借助 `diagnostics_channel`），以采集请求耗时、状态码、并发数等指标。
- **bin/ (`bin/`)**: 提供 `xprofctl` 命令行可执行文件，基于 `yargs` 构建，供用户在终端触发各类性能诊断命令。

## 3. 关键类与函数说明 (Key Classes and Functions)

### 3.1 C++ 端关键类/函数
- **`LogByPass` 类** ([log.h](file:///workspace/src/logbypass/log.h)): 继承自 `XpfThread`，实现后台旁路日志线程，定时拉取和记录各类统计信息（如 `GcStatistics`, `MemoryStatistics`）。
- **`StartCommandsListener()` 函数** ([listener.h](file:///workspace/src/commands/listener.h)): 初始化并启动 IPC 服务器，监听来自外部的诊断指令。
- **`DumpData` 结构体与相关回调** ([dump.h](file:///workspace/src/commands/dump.h)): 定义了各种命令操作及其回调函数（如 `StartCpuProfiling()`, `Heapdump()`, `GetNodeReport()`）。
- **`SetFatalErrorHandler()` 函数** ([fatal_error.h](file:///workspace/src/hooks/fatal_error.h)): 挂载 V8 崩溃钩子，抓取 Native 堆栈并生成分析报告。
- **`AutoIncreaseHeapLimit()` 函数** ([heap_limit.h](file:///workspace/src/hooks/heap_limit.h)): 挂载 V8 近乎 OOM 时的钩子，通过动态调整限制来避免进程崩溃。

### 3.2 JavaScript 端关键类/函数
- **`xprofiler.start(config)`** ([xprofiler.js](file:///workspace/xprofiler.js)): 插件主入口函数，用于应用启动时初始化 C++ 核心，启动后台旁路日志线程和命令监听线程，并进行 HTTP Patch。
- **`sendCommands(pid, thread_id, command, options)`** ([xctl.js](file:///workspace/lib/xctl.js)): 建立与目标进程通信的 UNIX Domain Socket / Named Pipe，发送诊断指令并接收异步响应结果。
- **`patch()`** ([index.js](file:///workspace/patch/index.js)): 判断配置并对指定 Node.js 原生模块进行安全替换（如记录 `http.server.request.start` 事件）。

## 4. 依赖关系 (Dependencies)

### 4.1 生产依赖 (Dependencies)
- **`@mapbox/node-pre-gyp`**: 用于在安装时拉取预编译的 C++ 插件二进制文件（`.node`），避免开发者本地必须安装编译工具链。
- **`nan`**: (Native Abstractions for Node.js) 用于编写跨版本兼容的 C++ 插件，抹平了 V8 引擎版本迭代带来的 API 差异。
- **`moment`**: 日期和时间格式化库，用于日志输出与文件名的标准化。
- **`uuid`**: 生成唯一的 traceid，在 IPC 异步通信时用于匹配请求与响应。
- **`yargs`**: 构建强大且灵活的 CLI 命令行工具 `xprofctl`，处理参数解析。

### 4.2 开发依赖 (Dev Dependencies)
- **测试框架**: `mocha`, `expect.js`, `nyc` 等，用于编写单元测试与生成代码覆盖率报告。
- **代码规范**: `clang-format` (C++ 代码格式化), `eslint` (JS 代码检查)。

## 5. 项目运行方式 (Project Running Instructions)

### 5.1 环境要求
- 操作系统: 支持 Windows, Linux (x64/arm64), macOS (x64/arm64)
- Node.js 版本: >= v18.19.0 (当前分支要求，更低版本可使用 xprofiler 1.x/2.x 分支)
- 编译工具链: 如果需要本地编译 C++ Addon，需要安装 Python, node-gyp, 及对应的 C++ 编译器（如 GCC/Clang/MSVC）。

### 5.2 安装与嵌入项目
在项目目录下直接安装：
```bash
npm install xprofiler --save
```
在 Node.js 应用入口文件（如 `app.js` 或 `index.js`）顶部引入并启动：
```javascript
const xprofiler = require('xprofiler');
xprofiler.start({
  log_dir: '/path/to/logdir', // 可选：指定日志输出目录
  log_level: 1 // 可选：设置日志级别
});
```

### 5.3 命令行状态采样 (`xprofctl`)
若全局安装了 xprofiler (`npm i xprofiler -g`)，则可以使用 `xprofctl` 命令行工具，向集成了 xprofiler 的 Node.js 进程发送实时诊断命令。

常用命令示例：
```bash
# 获取 xprofiler 版本号
xprofctl check_version -p <pid>

# 触发目标进程进行 CPU 采样
xprofctl start_cpu_profiling -p <pid>
xprofctl stop_cpu_profiling -p <pid>

# 触发目标进程生成 Heapdump (内存快照)
xprofctl heapdump -p <pid>

# 生成进程的诊断报告
xprofctl diag_report -p <pid>
```

### 5.4 本地开发与测试
1. 安装依赖并编译 C++ 插件:
```bash
npm install
npm run build
```
2. 运行代码检查 (Lint / Format):
```bash
npm run lint
```
3. 运行单元测试:
```bash
npm run test
```
4. 运行单元测试并生成覆盖率报告:
```bash
npm run cov
```