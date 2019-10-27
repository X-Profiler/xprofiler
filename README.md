# X-Profiler

[![npm](https://img.shields.io/npm/v/xprofiler)](https://www.npmjs.com/package/xprofiler)
[![Codecov branch](https://img.shields.io/codecov/c/github/hyj1991/xprofiler/master)](https://codecov.io/gh/hyj1991/xprofiler)
[![Linux/osX Build Status](https://travis-ci.org/hyj1991/xprofiler.svg?branch=master)](https://travis-ci.org/hyj1991/xprofiler)
[![Windows Build status](https://ci.appveyor.com/api/projects/status/f2wsq37va9d35vw4?svg=true)](https://ci.appveyor.com/project/hyj1991/xprofiler)
[![npm](https://img.shields.io/npm/dm/xprofiler)](https://www.npmjs.com/package/xprofiler)
[![NPM](https://img.shields.io/npm/l/xprofiler)](LICENSE)

Easy-Monitor v3.0 Node.js Runtime 插件，输出性能日志，并且可以进行实时的运行时状态采样。

## 兼容性

xprofiler 插件预计支持三大主流操作系统：

- Windows
- Linux
- macosX

支持的 node.js runtime 版本：

- v8.x
- v10.x
- v12.x

更低的版本因为已经不在官方 LTS 计划中，故正常情况下不再支持。

## 插件架构

// TODO

## 实现原理简述

// TODO

## 性能日志格式

// TODO，考虑兼容 alinode runtime 的日志格式，具体方式待定

## 运行时状态采样

// TODO

## 稳定性

xprofiler 每个 commit 都会通过 travis-ci/appveyor 进行在 windows/linux/macosX 下的编译以及单元测试，如果您在使用中依旧出现意外的 crash，可以反馈到 [Issue 列表](https://github.com/hyj1991/xprofiler/issues) 进行跟踪处理。

## 测试与代码覆盖率

在项目根目录下执行如下命令即可获取单元测试与代码覆盖率状况：

```bash
npm run cov
```

master 分支的代码覆盖率情况可以参见 `codecov badge`。

## 如何参与贡献

// TODO

## License

[BSD-2-Clause](LICENSE)