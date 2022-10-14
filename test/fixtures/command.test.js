'use strict';

const os = require('os');
const cp = require('child_process');
const moment = require('moment');
const expect = require('expect.js');
const { filterTestCaseByPlatform } = require('./utils');
const pkg = require('../../package.json');

const currentPlatform = os.platform();

const REGEXP_NUMBER = /^\d+(\.\d+)?$/;

function escape(str) {
  str = JSON.stringify(str);
  return str.slice(1, str.length - 1);
}

let sep = '/';
if (currentPlatform === 'win32') {
  sep = '\\';
}

function checkCoreDump(filepath, log) {
  if (currentPlatform === 'linux') {
    const stdout = cp.execSync(`readelf -a ${filepath}`);
    log && console.log(`${log}: ${stdout}`);
    it(`should generate elf coredump file on linux`, function () {
      expect(stdout.includes('ELF Header')).to.be.ok();
    });
  }
}

function checkProfile(rules, obj, rawKey) {
  if (rules instanceof RegExp) {
    rules = { [rawKey]: rules };
    obj = { [rawKey]: obj };
  }

  for (const [key, rule] of Object.entries(rules)) {
    const value = obj[key];
    if (rule instanceof RegExp) {
      it(`${key}: ${value} shoule be ${rule}`, function () {
        expect(rule.test(value)).to.be.ok();
      });
    } else if (Array.isArray(rule)) {
      let checkTimes = 0;
      for (const v of value) {
        checkProfile(rule[0], v, `${key}[${checkTimes++}]`);
        if (checkTimes > 5) {
          break;
        }
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
    id: REGEXP_NUMBER,
    hitCount: REGEXP_NUMBER,
    callFrame: {
      functionName: /^([$.\w\s()-_]+|)$/,
      scriptId: REGEXP_NUMBER,
      bailoutReason: /^([\w\s]+|)$/,
      url: /^([@.\w()/\\:_-\s]+|)$/,
      lineNumber: REGEXP_NUMBER,
      columnNumber: REGEXP_NUMBER,
    },
    children: [REGEXP_NUMBER]
  }],
  startTime: REGEXP_NUMBER,
  endTime: REGEXP_NUMBER,
  samples: [REGEXP_NUMBER],
  timeDeltas: [REGEXP_NUMBER]
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
    node_count: REGEXP_NUMBER,
    edge_count: REGEXP_NUMBER,
    trace_function_count: REGEXP_NUMBER
  },
  nodes: [REGEXP_NUMBER],
  edges: [REGEXP_NUMBER],
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
      scriptId: REGEXP_NUMBER,
      url: /^([@.\w()/\\:_-\s]+|)$/,
      lineNumber: REGEXP_NUMBER,
      columnNumber: REGEXP_NUMBER
    },
    selfSize: REGEXP_NUMBER,
    children: isArray
  }
};

const gcprofile = {
  startTime: REGEXP_NUMBER,
  gc: [{
    totalSpentfromStart: REGEXP_NUMBER,
    totalTimesfromStart: REGEXP_NUMBER,
    timeFromStart: REGEXP_NUMBER,
    start: REGEXP_NUMBER,
    type: /^(scavenge|marksweep|marking|weakcallbacks)$/,
    before: [{
      name: /^(.*)_space$/,
      space_size: REGEXP_NUMBER,
      space_used_size: REGEXP_NUMBER,
      space_available_size: REGEXP_NUMBER,
      physical_space_size: REGEXP_NUMBER,
    }],
    end: REGEXP_NUMBER,
    after: [{
      name: /^(.*)_space$/,
      space_size: REGEXP_NUMBER,
      space_used_size: REGEXP_NUMBER,
      space_available_size: REGEXP_NUMBER,
      physical_space_size: REGEXP_NUMBER,
    }],
  }],
  stopTime: REGEXP_NUMBER,
};

const diag = {
  pid: REGEXP_NUMBER,
  thread_id: REGEXP_NUMBER,
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
    heapTotal: REGEXP_NUMBER,
    heapTotalCommitted: REGEXP_NUMBER,
    heapTotalUsed: REGEXP_NUMBER,
    heapTotalAvailable: REGEXP_NUMBER,
    heapLimit: REGEXP_NUMBER
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
  const list = [
    {
      cmd: 'check_version',
      xctlRules: [{ key: 'data.version', rule: new RegExp(`^${pkg.version}$`) }],
      xprofctlRules(data) {
        return [new RegExp(`^X-Profiler 插件版本号\\(pid ${data.pid}\\): v${pkg.version}$`)];
      }
    },
    {
      cmd: 'list_environments',
      xctlRules: [
        {
          key: 'data.environments.0.is_main_thread',
          rule: { label: 'boolean', test: value => typeof value === 'boolean' },
        },
        {
          key: 'data.environments.0.thread_id',
          rule: { label: 'number', test: value => typeof value === 'number' },
        },
        {
          key: 'data.environments.0.uptime',
          rule: { label: 'number', test: value => typeof value === 'number' },
        },
      ],
      xprofctlRules(data) {
        return [new RegExp(`^X-Profiler 环境列表\\(pid ${data.pid}\\):\n`
          + '(?:  - 线程\\(tid \\d+\\): (?:主|Worker)线程已启动 \\d+ 秒\n?)+')];
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
        { key: 'data.patch_http', rule: { label: 'true', test: value => value === true } },
        { key: 'data.patch_http_timeout', rule: /^30$/ },
        { key: 'data.check_throw', rule: { label: 'false', test: value => value === false } },
        { key: 'data.enable_fatal_error_hook', rule: { label: 'true', test: value => value === true } },
        { key: 'data.enable_fatal_error_report', rule: { label: 'true', test: value => value === true } },
        { key: 'data.enable_fatal_error_coredump', rule: { label: 'false', test: value => value === false } },
      ],
      xprofctlRules(data) {
        return [new RegExp(`^X-Profiler 当前配置\\(pid ${data.pid}\\):\n`
          + '  - check_throw: false\n'
          + '  - enable_fatal_error_coredump: false\n'
          + '  - enable_fatal_error_hook: true\n'
          + '  - enable_fatal_error_report: true\n'
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
      options: { enable_log_uv_handles: false, log_level: 2, log_type: 1 },
      xctlRules: [
        { key: 'data.enable_log_uv_handles', rule: { label: 'false', test: value => value === false } },
        { key: 'data.log_level', rule: /^2$/ },
        { key: 'data.log_type', rule: /^1$/ },
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
    {
      platform: 'linux',
      cmd: 'generate_coredump',
      profileRules: checkCoreDump,
      xctlRules(data) {
        return [{
          key: 'data.filepath', rule: new RegExp(escape(data.logdir + sep) +
            `x-coredump-${data.pid}-${moment().format('YYYYMMDD')}-(\\d+).core`)
        }];
      },
      xprofctlRules() { return []; }
    },
    {
      platform: 'win32',
      cmd: 'generate_coredump',
      errored: true,
      xctlRules: [],
      xprofctlRules() { return [/执行命令失败: generate_coredump only support linux now./]; }
    },
    {
      platform: 'darwin',
      cmd: 'generate_coredump',
      errored: true,
      xctlRules: [],
      xprofctlRules() { return [/执行命令失败: generate_coredump only support linux now./]; }
    },
  ];

  return filterTestCaseByPlatform(list);
};

exports.profileRule = { diag, cpuprofile, heapprofile, gcprofile };

exports.checkProfile = checkProfile;

exports.checkCoreDump = checkCoreDump;