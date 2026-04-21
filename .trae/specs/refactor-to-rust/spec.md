# X-Profiler Rust 重构 Spec

## Why
当前 xprofiler 使用 C++ (Node-API / NAN) 编写，虽然性能优秀但 C++ 存在内存安全隐患、跨平台编译复杂以及维护成本高的问题。通过使用 Rust (如 napi-rs) 进行重构，可以利用 Rust 的内存安全特性、优秀的生态系统（Cargo）以及极大简化的跨平台构建流程，提高代码的健壮性和开发体验，同时保持甚至超越原有的性能表现。

## What Changes
- 引入 napi-rs 构建系统，替代原来的 node-pre-gyp 和 C++ 构建体系。
- **BREAKING**: 将 `src/` 目录下的所有 C++ 代码使用 Rust 重写，主要模块包括：
  - commands (cpuprofiler, gcprofiler, heapprofiler, coredumper)
  - hooks (fatal_error, heap_limit)
  - logbypass
  - utils & config
- 调整 `package.json` 中的构建和打包脚本以适配 napi-rs。
- 保证 JS 层（`lib/`, `patch/` 等）接口不变，仅在加载原生模块的地方做修改。

## Impact
- Affected specs: 编译流程、发布流程、日志采集核心逻辑、系统性能分析数据获取。
- Affected code:
  - `src/*` (全部移除，替换为 `src/` 下的 Rust 代码)
  - `binding.gyp` (移除)
  - `package.json` (更新构建脚本及依赖)
  - `xprofiler.js` (修改 native module 加载路径)

## ADDED Requirements
### Requirement: 基于 Rust 的安全构建和发布
系统 SHALL 提供跨平台的预编译 binary，使用 Github Actions 和 napi-rs 生成，以保证用户在安装时直接下载预编译包而无需本地编译环境。

#### Scenario: 用户安装依赖
- **WHEN** 用户执行 `npm install xprofiler`
- **THEN** 优先下载平台对应的预编译 Rust napi addon；如果没有预编译版本，则进行本地回退编译。

## MODIFIED Requirements
### Requirement: 原生日志与分析接口
重构后的 Rust 扩展必须与原 C++ 扩展提供完全一致的 API（函数签名与返回值），包括但不限于 `start`, `configure`, `dump_cpu_profiler` 等。

## REMOVED Requirements
### Requirement: node-pre-gyp 支持
**Reason**: node-pre-gyp 维护成本高，且 napi-rs 自带了非常完善的交叉编译和产物分发方案。
**Migration**: 移除 `binding.gyp`，改为在 `package.json` 中配置 `@napi-rs/cli`。