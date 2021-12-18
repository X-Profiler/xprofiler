'use strict';

const os = require('os');
const moment = require('moment');
const expect = require('expect.js');
const pkg = require('../../package.json');

function escape(str) {
  str = JSON.stringify(str);
  return str.slice(1, str.length - 1);
}

let sep = '/';
if (os.platform() === 'win32') {
  sep = '\\';
}

function checkProfile(rules, obj) {
  for (const [key, rule] of Object.entries(rules)) {
    const value = obj[key];
    if (rule instanceof RegExp) {
      it(`${key}: ${value} shoule be ${rule}`, function () {
        expect(rule.test(value)).to.be.ok();
      });
    } else if (Array.isArray(rule)) {
      for (const v of value) {
        checkProfile(rule[0], v);
      }
    } else if (typeof rule === 'function') {
      let label = value;
      if (Array.isArray(value)) {
        if (value.length < 10) {
          label = `${JSON.stringify(value)} length: ${value.length}`;
        } else {
          label = `Array [${value[0]}, ${value[1]}, ${value[2]}, ...] length: ${value.length}`;
        }
      }
      it(`${key}: ${label} shoule be ${rule}`, function () {
        expect(rule(value)).to.be.ok();
      });
    } else if (typeof rule === 'object') {
      checkProfile(rule, value);
    }
  }
}

const isArray = value => Array.isArray(value);

const cpuprofile = {
  typeId: /^xprofiler-cpu-profile$/,
  title: /^xprofiler$/,
  nodes: [{
    id: /^\d+$/,
    hitCount: /^\d+$/,
    callFrame: {
      functionName: /^([$.\w\s()-_]+|)$/,
      scriptId: /^\d+$/,
      bailoutReason: /^([\w\s]+|)$/,
      url: /^([@.\w()/\\:_-\s]+|)$/,
      lineNumber: /^\d+$/,
      columnNumber: /^\d+$/,
    },
    children: isArray
  }],
  startTime: /^\d+$/,
  endTime: /^\d+$/,
  samples: isArray,
  timeDeltas: isArray
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
      functionName: /^([$.\w\s()-_]+|)$/,
      scriptId: /^\d+$/,
      url: /^([@.\w()/\\:_-\s]+|)$/,
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
  pid: /^\d+$/,
  location: /^([\w\s()-:]+|)$/,
  message: /^([\w\s()-:]+|)$/,
  nodeVersion: new RegExp(`^${process.version}$`),
  osVersion: /^([\w\s()-_/.:~#]+|)$/,
  dumpTime: /\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}/,
  loadTime: /\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}/,
  vmState: /^[A-Za-z]+$/,
  jsStacks: isArray,
  nativeStacks: isArray,
  heapStatistics: {
    heapTotal: /^\d+$/,
    heapTotalCommitted: /^\d+$/,
    heapTotalUsed: /^\d+$/,
    heapTotalAvailable: /^\d+$/,
    heapLimit: /^\d+$/
  },
  heapSpaceStatistics: isArray,
  libuvHandles: isArray,
  system: {
    env: isArray,
    resourceLimits: isArray,
    loadedLibraries: isArray
  }
};

exports = module.exports = function (logdir) {
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
        { key: 'data.log_type', rule: /^1$/ },
        { key: 'data.enable_fatal_error_hook', rule: { label: 'true', test: value => value === true } },
        { key: 'data.patch_http', rule: { label: 'true', test: value => value === true } },
        { key: 'data.patch_http_timeout', rule: /^30$/ },
        { key: 'data.check_throw', rule: { label: 'false', test: value => value === false } },
      ],
      xprofctlRules(data) {
        return [new RegExp(`^X-Profiler 当前配置\\(pid ${data.pid}\\):\n`
          + '  - check_throw: false\n'
          + '  - enable_fatal_error_hook: true\n'
          + '  - enable_log_uv_handles: true\n'
          + `  - log_dir: ${escape(logdir)}\n`
          + '  - log_format_alinode: false\n'
          + '  - log_interval: 60\n'
          + '  - log_level: 2\n'
          + '  - log_type: 1\n'
          + '  - patch_http: true\n'
          + '  - patch_http_timeout: 30')
        ];
      }
    },
    {
      cmd: 'set_config',
      options: { enable_log_uv_handles: false, log_level: 2, log_type: 1, enable_fatal_error_hook: false },
      xctlRules: [
        { key: 'data.enable_log_uv_handles', rule: { label: 'false', test: value => value === false } },
        { key: 'data.log_level', rule: /^2$/ },
        { key: 'data.log_type', rule: /^1$/ },
        { key: 'data.enable_fatal_error_hook', rule: { label: 'false', test: value => value === false } },
      ],
      xprofctlRules(data) {
        return [new RegExp(`^X-Profiler 配置\\(pid ${data.pid}\\)成功:\n`
          + '  - enable_fatal_error_hook: false\n'
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
      xctlRules: [{ key: 'message', rule: /^<enable_log_uv_handles> type error: \[json.exception.type_error.302\] type must be boolean, but is number$/ }], // eslint-disable-line
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
        return [{ key: 'message', rule: /^stop_sampling_heap_profiling dependent action start_sampling_heap_profiling is not running.$/ }]; // eslint-disable-line
      },
      xprofctlRules() { return [/^执行命令失败: stop_sampling_heap_profiling dependent action start_sampling_heap_profiling is not running.$/]; } // eslint-disable-line
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

exports.profileRule = { diag };

exports.checkProfile = checkProfile;