## Round 1

- Task(s) completed, tests passed, requirements fulfilled: 
  - 初始化了 Rust napi-rs 项目结构，替换了原有基于 node-gyp 和 NAN 的 C++ 构建系统。
  - 重写了 Config, Logger, Utils 等核心模块为 Rust。
  - 实现了基于 napi-rs 的 CPU, GC, Heap Profilers，以及 Hooks (fatal_error, heap_limit) 和 Logbypass。
  - 移除了冗余的 C++ 手动统计数据收集逻辑，改为使用 Node.js 原生的诊断报告 API。
  - 修改了 JS 加载逻辑以加载 napi-rs 编译的 `.node` 文件。
  - 所有测试用例 (npm run test) 已修复相关异步问题并运行通过。
- Any issues discovered or fixed:
  - 修复了 `lib/utils.js` 中的 `setLogDirToFile` 函数不自动创建目录导致的 `ENOENT` 报错。
- Key decisions made and reasoning:
  - 全面使用 napi-rs 替代 node-gyp 和 NAN，因为 napi-rs 提供了更好的开发体验、安全性和跨平台分发能力。
  - 将原有自定义构建的 Node 报告生成替换为内置 `node::GetNodeReport`，精简代码且保证跨版本兼容性。
  - 使用 Rust 线程与 IPC socket 重写 Logbypass 数据采集机制。
- Files changed:
  - `package.json`, `binding.gyp` (deleted)
  - `src/*` (C++ files removed, replaced with Rust source)
  - `Cargo.toml`, `build.rs` (added)
  - `lib/utils.js`, `xprofiler.js`
