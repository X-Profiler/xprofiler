'use strict';

const os = require('os');

function config(name, env, format, value, rules = []) {
  return { name, env, format, value, rules, };
}

function xprofctl(configurable, describe, choices) {
  return { configurable, describe, choices, };
}

module.exports = () => {
  return [
    {
      ...xprofctl(false),
      ...config('log_dir', 'XPROFILER_LOG_DIR', 'string', os.tmpdir(), ['path']),
    },

    {
      ...xprofctl(false),
      ...config('log_interval', 'XPROFILER_LOG_INTERVAL', 'number', 60), // seconds
    },

    {
      ...xprofctl(true, '日志级别: info, error, debug', [0, 1, 2],),
      ...config('log_level', 'XPROFILER_LOG_LEVEL', 'number', 1), // 0: info, 1: error, 2: debug
    },

    {
      ...xprofctl(true, '日志输出位置: 文件, 控制台', [0, 1]),
      ...config('log_type', 'XPROFILER_LOG_TYPE', 'number', 0), // 0: file, 1: console
    },

    {
      ...xprofctl(false),
      ...config('log_format_alinode', 'XPROFILER_LOG_FORMAT_ALINODE', 'boolean', false),
    },

    {
      ...xprofctl(false),
      ...config('patch_http', 'XPROFILER_PATCH_HTTP', 'boolean', true),
    },

    {
      ...xprofctl(false),
      ...config('patch_http_timeout', 'XPROFILER_PATCH_HTTP_TIMEOUT', 'number', 30), // seconds
    },

    {
      ...xprofctl(false),
      ...config('check_throw', 'XPROFILER_CHECK_THROW', 'boolean', true),
    },

    {
      ...xprofctl(false),
      ...config('auto_incr_heap_limit_size', 'XPROFILER_AUTO_INCR_HEAP_LIMIT_SIZE', 'number', 256), // MB
    },

    {
      ...xprofctl(true, enable => `${enable ? '开启' : '关闭'} libuv 句柄详情采集`),
      ...config('enable_log_uv_handles', 'XPROFILER_ENABLE_LOG_UV_HANDLES', 'boolean', true),
    },

    {
      ...xprofctl(false),
      ...config('enable_fatal_error_hook', 'XPROFILER_ENABLE_FATAL_ERROR_HOOK', 'boolean', true),
    },

    {
      ...xprofctl(true, enable => `${enable ? '开启' : '关闭'} FatalError 时自动 Report`),
      ...config('enable_fatal_error_report', 'XPROFILER_ENABLE_FATAL_ERROR_REPORT', 'boolean', true),
    },

    {
      ...xprofctl(true, enable => `${enable ? '开启' : '关闭'} FatalError 时自动 Coredump`),
      ...config('enable_fatal_error_coredump', 'XPROFILER_ENABLE_FATAL_ERROR_COREDUMP', 'boolean', false),
    },

    {
      ...xprofctl(true, enable => `在 CPU 采样期间${enable ? '开启' : '关闭'} HTTP Profiling`),
      ...config('enable_http_profiling', 'XPROFILER_ENABLE_HTTP_PROFILING', 'boolean', false),
    },

    {
      ...xprofctl(true, enable => `${enable ? '启用' : '禁用'} Node.js 自动增加堆上限`),
      ...config('enable_auto_incr_heap_limit', 'XPROFILER_ENABLE_AUTO_INCR_HEAP_LIMIT', 'boolean', false),
    },
  ];
};