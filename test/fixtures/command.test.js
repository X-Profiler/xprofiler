'use strict';

const os = require('os');
const moment = require('moment');
const pkg = require('../../package.json');

function escape(str) {
  str = JSON.stringify(str);
  return str.slice(1, str.length - 1);
}

let sep = '/';
if (os.platform() === 'win32') {
  sep = '\\';
}

const isArray = value => Array.isArray(value);

const cpuprofile = {
  typeId: /^xprofiler-cpu-profile$/,
  title: /^xprofiler$/,
  head: {
    functionName: /^([\w\s()]+|)$/,
    url: /^([.\w()/\\]+|)$/,
    lineNumber: /^\d+$/,
    columnNumber: /^\d+$/,
    bailoutReason: /^([\w\s]+|)$/,
    id: /^\d+$/,
    scriptId: /^\d+$/,
    hitCount: /^\d+$/,
    children: isArray
  },
  startTime: /^\d+$/,
  endTime: /^\d+$/,
  samples: isArray,
  timestamps: isArray
};

const heapsnapshot = {
  snapshot: {
    meta: {
      node_fields: isArray,
      node_types: isArray,
      edge_fields: isArray,
      edge_types: isArray,
      trace_function_info_fields: isArray,
      trace_node_fields: isArray,
      sample_fields: isArray,
      // location_fields: isArray
    },
    node_count: /^\d+$/,
    edge_count: /^\d+$/,
    trace_function_count: /^\d+$/
  },
  nodes: isArray,
  edges: isArray,
  trace_function_infos: isArray,
  trace_tree: isArray,
  samples: isArray,
  // locations: isArray,
  strings: isArray
};

const heapprofile = {
  head: {
    callFrame: {
      functionName: /^([\w\s()]+|)$/,
      scriptId: /^\d+$/,
      url: /^([.\w()/\\]+|)$/,
      lineNumber: /^\d+$/,
      columnNumber: /^\d+$/
    },
    selfSize: /^\d+$/,
    children: isArray
  }
};

const gcprofile = {
  startTime: /^\d+$/,
  gc: isArray,
  stopTime: /^\d+$/,
};

const diag = {
  dumpTime: /\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}/,
  loadTime: /\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}/
};

module.exports = function (logdir) {
  return [
    {
      cmd: 'check_version',
      xctlRules: [{ key: 'data.version', rule: new RegExp(`^${pkg.version}$`) }],
      xprofctlRules(data) {
        return [new RegExp(`^X-Profiler 插件版本号\\(pid ${data.pid}\\): v${pkg.version}$`)];
      }
    },
    {
      cmd: 'get_config',
      xctlRules: [
        { key: 'data.log_dir', rule: new RegExp(`^${escape(logdir)}$`) },
        { key: 'data.log_interval', rule: /^60$/ },
        { key: 'data.enable_log_uv_handles', rule: { label: 'true', test: value => value === true } },
        { key: 'data.log_format_alinode', rule: { label: 'false', test: value => value === false } },
        { key: 'data.log_level', rule: /^2$/ },
        { key: 'data.log_type', rule: /^1$/ }
      ],
      xprofctlRules(data) {
        return [new RegExp(`^X-Profiler 当前配置\\(pid ${data.pid}\\):\n`
          + '  - enable_log_uv_handles: true\n'
          + `  - log_dir: ${escape(logdir)}\n`
          + '  - log_format_alinode: false\n'
          + '  - log_interval: 60\n'
          + '  - log_level: 2\n'
          + '  - log_type: 1')
        ];
      }
    },
    {
      cmd: 'set_config',
      options: { enable_log_uv_handles: false, log_level: 2, log_type: 1 },
      xctlRules: [
        { key: 'data.enable_log_uv_handles', rule: { label: 'false', test: value => value === false } },
        { key: 'data.log_level', rule: /^2$/ },
        { key: 'data.log_type', rule: /^1$/ }
      ],
      xprofctlRules(data) {
        return [new RegExp(`^X-Profiler 配置\\(pid ${data.pid}\\)成功:\n`
          + '  - enable_log_uv_handles: false\n'
          + '  - log_level: 2\n'
          + '  - log_type: 1')
        ];
      }
    },
    {
      cmd: 'set_config',
      errored: true,
      xctlRules: [],
      xprofctlRules() {
        return [/^set_config 缺少必须参数，执行 xprofctl set_config 查看正确用法$/];
      }
    },
    {
      cmd: 'set_config',
      options: { enable_log_uv_handles: 1 },
      errored: true,
      /* eslint-disable */
      xctlRules: [{ key: 'message', rule: /^<enable_log_uv_handles> type error: \[json.exception.type_error.302\] type must be boolean, but is number$/ }],
      xprofctlRules() { return []; }
    },
    {
      cmd: 'start_cpu_profiling',
      options: { profiling_time: 1000 },
      profileRules: cpuprofile,
      xctlRules(data) {
        return [{
          key: 'data.filepath', rule: new RegExp(escape(data.logdir + sep) +
            `x-cpuprofile-${data.pid}-${moment().format('YYYYMMDD')}-(\\d+).cpuprofile`)
        }];
      },
      xprofctlRules() { return []; }
    },
    {
      cmd: 'stop_cpu_profiling',
      errored: true,
      xctlRules() {
        return [{ key: 'message', rule: /^stop_cpu_profiling dependent action start_cpu_profiling is not running.$/ }];
      },
      xprofctlRules() { return [/^执行命令失败: stop_cpu_profiling dependent action start_cpu_profiling is not running.$/]; }
    },
    {
      cmd: 'heapdump',
      profileRules: heapsnapshot,
      xctlRules(data) {
        return [{
          key: 'data.filepath', rule: new RegExp(escape(data.logdir + sep) +
            `x-heapdump-${data.pid}-${moment().format('YYYYMMDD')}-(\\d+).heapsnapshot`)
        }];
      },
      xprofctlRules() { return []; }
    },
    {
      cmd: 'start_heap_profiling',
      options: { profiling_time: 1000 },
      profileRules: heapprofile,
      xctlRules(data) {
        return [{
          key: 'data.filepath', rule: new RegExp(escape(data.logdir + sep) +
            `x-heapprofile-${data.pid}-${moment().format('YYYYMMDD')}-(\\d+).heapprofile`)
        }];
      },
      xprofctlRules() { return []; }
    },
    {
      cmd: 'stop_heap_profiling',
      errored: true,
      xctlRules() {
        return [{ key: 'message', rule: /^stop_sampling_heap_profiling dependent action start_sampling_heap_profiling is not running.$/ }];
      },
      xprofctlRules() { return [/^执行命令失败: stop_sampling_heap_profiling dependent action start_sampling_heap_profiling is not running.$/]; }
    },
    {
      cmd: 'start_gc_profiling',
      options: { profiling_time: 1000 },
      profileRules: gcprofile,
      xctlRules(data) {
        return [{
          key: 'data.filepath', rule: new RegExp(escape(data.logdir + sep) +
            `x-gcprofile-${data.pid}-${moment().format('YYYYMMDD')}-(\\d+).gcprofile`)
        }];
      },
      xprofctlRules() { return []; }
    },
    {
      cmd: 'stop_gc_profiling',
      errored: true,
      xctlRules() {
        return [{ key: 'message', rule: /^stop_gc_profiling dependent action start_gc_profiling is not running.$/ }];
      },
      xprofctlRules() { return [/^执行命令失败: stop_gc_profiling dependent action start_gc_profiling is not running.$/]; }
    },
    {
      cmd: 'diag_report',
      profileRules: diag,
      xctlRules(data) {
        return [{
          key: 'data.filepath', rule: new RegExp(escape(data.logdir + sep) +
            `x-diagreport-${data.pid}-${moment().format('YYYYMMDD')}-(\\d+).diag`)
        }];
      },
      xprofctlRules() { return []; }
    },
  ];
};