# XProfiler Rust 重构工作计划

## 1. 项目概述

XProfiler 是一个 Node.js 运行时性能监控工具，当前使用 C++ 和 NAN (Native Abstractions for Node.js) 实现 NAPI 绑定。本文档制定了将 C++ 代码重构为 Rust 并使用 napi-rs 替换 NAN 的详细计划。

## 2. 当前项目架构分析

### 2.1 技术栈现状
- **语言**: C++ (核心逻辑) + JavaScript (接口层)
- **NAPI 绑定**: NAN (Native Abstractions for Node.js)
- **构建系统**: node-gyp + binding.gyp
- **平台支持**: Linux, macOS, Windows
- **依赖管理**: @mapbox/node-pre-gyp

### 2.2 核心模块分析

#### 2.2.1 主要功能模块
1. **环境数据管理** (`environment_data.cc/h`)
   - 线程环境数据存储
   - 配置信息管理
   - 进程数据收集

2. **性能监控** (`logbypass/`)
   - CPU 使用率监控
   - 内存堆监控
   - GC 性能监控
   - libuv 事件循环监控
   - HTTP 请求监控

3. **命令处理** (`commands/`)
   - CPU Profiler
   - Heap Dump
   - Heap Profiler
   - GC Profiler
   - Core Dumper
   - 报告生成

4. **平台适配** (`platform/`)
   - Unix/Linux 实现
   - Windows 实现
   - macOS 实现

5. **JavaScript API 导出** (`jsapi/`)
   - 配置管理接口
   - 日志接口
   - 环境数据接口
   - HTTP 监控接口
   - 钩子函数接口

#### 2.2.2 文件结构统计
- 总计约 50+ C++ 源文件
- 核心逻辑约 15,000+ 行代码
- 平台特定代码约 30%
- NAPI 绑定代码约 20%

### 2.3 当前架构优势
- 成熟稳定的 C++ 实现
- 完整的跨平台支持
- 丰富的性能监控功能
- 良好的 Node.js 生态集成

### 2.4 当前架构痛点
- C++ 内存管理复杂
- NAN API 学习曲线陡峭
- 构建配置复杂 (binding.gyp)
- 调试和维护困难
- 新功能开发效率低

## 3. Rust 重构目标和范围

### 3.1 重构目标
1. **提升开发效率**: 利用 Rust 的内存安全和现代语言特性
2. **简化构建流程**: 使用 napi-rs 简化 NAPI 绑定
3. **增强代码质量**: 利用 Rust 的类型系统和错误处理
4. **保持功能完整**: 100% 兼容现有 JavaScript API
5. **优化性能**: 利用 Rust 的零成本抽象

### 3.2 重构范围

#### 3.2.1 第一阶段 (核心基础)
- 环境数据管理模块
- 配置管理系统
- 日志系统
- 基础工具函数

#### 3.2.2 第二阶段 (监控核心)
- CPU 监控
- 内存监控
- GC 监控
- HTTP 监控

#### 3.2.3 第三阶段 (高级功能)
- Profiler 功能
- Dump 功能
- 报告生成
- 命令处理

#### 3.2.4 第四阶段 (平台优化)
- 平台特定优化
- 性能调优
- 测试完善

### 3.3 不在重构范围
- JavaScript 层代码 (保持不变)
- 测试用例逻辑 (仅更新构建配置)
- 文档和配置文件

## 4. 技术栈迁移策略

### 4.1 napi-rs 技术栈

#### 4.1.1 核心依赖
```toml
[dependencies]
napi = "2.16"
napi-derive = "2.16"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
libc = "0.2"
once_cell = "1.19"
log = "0.4"
env_logger = "0.10"

[build-dependencies]
napi-build = "2.16"
```

#### 4.1.2 平台特定依赖
```toml
[target.'cfg(unix)'.dependencies]
nix = "0.27"

[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3", features = ["full"] }
```

### 4.2 项目结构设计

```
xprofiler-rs/
├── Cargo.toml
├── build.rs
├── src/
│   ├── lib.rs                 # 主入口
│   ├── config/                # 配置管理
│   │   ├── mod.rs
│   │   └── store.rs
│   ├── environment/           # 环境数据
│   │   ├── mod.rs
│   │   ├── data.rs
│   │   └── registry.rs
│   ├── monitoring/            # 监控模块
│   │   ├── mod.rs
│   │   ├── cpu.rs
│   │   ├── memory.rs
│   │   ├── gc.rs
│   │   └── http.rs
│   ├── profiler/              # 性能分析
│   │   ├── mod.rs
│   │   ├── cpu_profiler.rs
│   │   ├── heap_profiler.rs
│   │   └── gc_profiler.rs
│   ├── platform/              # 平台适配
│   │   ├── mod.rs
│   │   ├── unix.rs
│   │   └── windows.rs
│   ├── logger/                # 日志系统
│   │   ├── mod.rs
│   │   └── writer.rs
│   ├── utils/                 # 工具函数
│   │   ├── mod.rs
│   │   └── common.rs
│   └── bindings/              # NAPI 绑定
│       ├── mod.rs
│       ├── config.rs
│       ├── environment.rs
│       ├── logger.rs
│       ├── monitoring.rs
│       └── profiler.rs
├── tests/
└── benches/
```

### 4.3 API 设计原则

#### 4.3.1 保持 JavaScript API 兼容
```rust
// 配置管理
#[napi]
pub fn configure(config: Array) -> Result<bool> { ... }

#[napi]
pub fn get_config() -> Result<Object> { ... }

// 日志接口
#[napi]
pub fn info(component: String, content: String) -> Result<()> { ... }

#[napi]
pub fn error(component: String, content: String) -> Result<()> { ... }

// 监控接口
#[napi]
pub fn run_log_bypass() -> Result<()> { ... }

#[napi]
pub fn run_commands_listener() -> Result<()> { ... }
```

#### 4.3.2 内部模块设计
```rust
// 配置存储
pub struct ConfigStore {
    configs: HashMap<String, ConfigValue>,
    descriptions: HashMap<String, ConfigDescription>,
}

// 环境数据
pub struct EnvironmentData {
    thread_id: u32,
    is_main_thread: bool,
    node_version: String,
}

// 监控数据
pub struct MonitoringData {
    cpu_usage: f64,
    memory_usage: u64,
    gc_stats: GcStats,
    http_stats: HttpStats,
}
```

## 5. 分阶段实施计划

### 5.1 第一阶段：基础设施 (2-3 周)

#### 5.1.1 项目初始化
- [x] 使用 napi-rs CLI 创建项目结构
- [x] 配置 Cargo.toml 和构建脚本
- [ ] 设置 CI/CD 流水线
- [x] 建立测试框架

#### 5.1.2 核心模块实现
- [x] 配置管理系统 (`config/`)
- [x] 环境数据管理 (`environment/`)
- [x] 日志系统 (`logger/`)
- [x] 基础工具函数 (`utils/`)

#### 5.1.3 NAPI 绑定
- [x] 配置相关 API
- [x] 环境数据 API
- [x] 日志 API

#### 5.1.4 测试验证
- [x] 单元测试
- [x] 集成测试
- [ ] 与原有 JavaScript 代码集成测试

### 5.2 第二阶段：监控核心 (3-4 周)

#### 5.2.1 监控模块实现
- [x] CPU 监控 (`monitoring/cpu.rs`)
- [x] 内存监控 (`monitoring/memory.rs`)
- [x] GC 监控 (`monitoring/gc.rs`)
- [ ] HTTP 监控 (`monitoring/http.rs`)
- [ ] libuv 监控 (`monitoring/libuv.rs`)

#### 5.2.2 平台适配
- [x] Unix/Linux 平台实现
- [x] macOS 平台实现
- [x] Windows 平台实现

#### 5.2.3 NAPI 绑定
- [x] 监控相关 API
- [x] 平台特定 API

#### 5.2.4 性能测试
- [x] 基准测试
- [ ] 内存使用测试
- [ ] 多线程安全测试

### 5.3 第三阶段：高级功能 (4-5 周)

#### 5.3.1 Profiler 实现
- [ ] CPU Profiler (`profiler/cpu_profiler.rs`)
- [ ] Heap Profiler (`profiler/heap_profiler.rs`)
- [ ] GC Profiler (`profiler/gc_profiler.rs`)
- [ ] Sampling Heap Profiler

#### 5.3.2 Dump 功能
- [ ] Heap Dump
- [ ] Core Dump (平台特定)

#### 5.3.3 报告生成
- [ ] Node Report
- [ ] JavaScript Stack
- [ ] Native Stack
- [ ] 系统统计

#### 5.3.4 命令处理
- [ ] 命令监听器
- [ ] 命令解析器
- [ ] 命令执行器

### 5.4 第四阶段：优化和完善 (2-3 周)

#### 5.4.1 性能优化
- [x] 内存使用优化
- [x] CPU 使用优化 (SIMD优化)
- [ ] 异步处理优化
- [ ] 缓存策略优化

#### 5.4.2 稳定性提升
- [x] 错误处理完善
- [ ] 异常恢复机制
- [ ] 资源清理优化
- [ ] 内存泄漏检测

#### 5.4.3 测试完善
- [ ] 压力测试
- [ ] 长时间运行测试
- [ ] 边界条件测试
- [ ] 兼容性测试

#### 5.4.4 文档和发布
- [ ] API 文档更新
- [ ] 迁移指南
- [ ] 性能对比报告
- [ ] 发布准备

## 6. 风险评估和缓解措施

### 6.1 技术风险

#### 6.1.1 高风险
**风险**: napi-rs 功能覆盖不完整
- **影响**: 某些 NAN API 无法直接迁移
- **缓解**: 提前调研 napi-rs 能力边界，必要时贡献上游
- **应急**: 保留部分 C++ 代码，混合使用

**风险**: 平台兼容性问题
- **影响**: 某些平台功能无法实现
- **缓解**: 分平台逐步验证，建立完整测试矩阵
- **应急**: 平台特定功能降级或禁用

#### 6.1.2 中风险
**风险**: 性能回归
- **影响**: Rust 版本性能不如 C++ 版本
- **缓解**: 建立性能基准测试，持续监控
- **应急**: 性能关键路径优化，必要时回退

**风险**: 内存使用增加
- **影响**: Rust 版本内存占用更高
- **缓解**: 内存使用分析，优化数据结构
- **应急**: 内存池化，延迟加载

#### 6.1.3 低风险
**风险**: 构建配置复杂
- **影响**: 构建流程变更影响 CI/CD
- **缓解**: 渐进式迁移，保持向后兼容
- **应急**: 双构建系统并行

### 6.2 项目风险

#### 6.2.1 时间风险
**风险**: 开发周期超预期
- **影响**: 延迟发布计划
- **缓解**: 分阶段交付，优先核心功能
- **应急**: 缩减功能范围，延后非关键特性

#### 6.2.2 资源风险
**风险**: 开发人员 Rust 经验不足
- **影响**: 开发效率低，代码质量差
- **缓解**: 提前培训，代码审查，最佳实践
- **应急**: 外部专家支持，结对编程

### 6.3 业务风险

#### 6.3.1 兼容性风险
**风险**: API 行为变更
- **影响**: 现有用户代码无法正常工作
- **缓解**: 严格的兼容性测试，API 契约测试
- **应急**: 兼容性层，渐进式迁移

#### 6.3.2 稳定性风险
**风险**: 新版本稳定性不如旧版本
- **影响**: 用户体验下降，bug 增多
- **缓解**: 充分测试，灰度发布
- **应急**: 快速回滚机制，热修复

## 7. 成功标准

### 7.1 功能标准
- [ ] 100% JavaScript API 兼容
- [ ] 所有现有测试用例通过
- [ ] 支持所有目标平台
- [ ] 功能完整性验证

### 7.2 性能标准
- [ ] CPU 使用率不超过原版本 110%
- [ ] 内存使用不超过原版本 120%
- [ ] 启动时间不超过原版本 105%
- [ ] 监控数据精度保持一致

### 7.3 质量标准
- [ ] 代码覆盖率 > 85%
- [ ] 无内存泄漏
- [ ] 无数据竞争
- [ ] 通过所有安全扫描

### 7.4 维护标准
- [ ] 完整的 API 文档
- [ ] 详细的迁移指南
- [ ] 性能对比报告
- [ ] 故障排查手册

## 8. 总结

本重构计划将 XProfiler 从 C++ + NAN 迁移到 Rust + napi-rs，预计总工期 12-15 周。通过分阶段实施、风险控制和质量保证，确保重构成功并提升项目的长期可维护性。

重构完成后，XProfiler 将具备：
- 更安全的内存管理
- 更简洁的代码结构
- 更高效的开发流程
- 更好的错误处理
- 更强的类型安全

这将为 XProfiler 的未来发展奠定坚实的技术基础。