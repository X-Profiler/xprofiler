'use strict';

const path = require('path');
const moment = require('moment');
const utils = require('./utils');
const blocking = path.join(__dirname, 'blocking.js');
const nonBlocking = path.join(__dirname, 'non-blocking.js');

function setRules(list, alinode, { alinodeRule, xprofilerRule }) {
  const rules = {};
  for (const r of list) {
    if (alinode) {
      rules[r] = alinodeRule;
    } else {
      rules[r] = xprofilerRule;
    }
  }
  return rules;
}

// cpu rule
const alinodeCpuRule = /^\d{1,3}.\d{2}$/;
const xprofilerCpuRule = /^\d{1,3}.\d{2}$/;
function getCpuRules(list, alinode) {
  return setRules(list, alinode, { alinodeRule: alinodeCpuRule, xprofilerRule: xprofilerCpuRule });
}

// memory rule
const alindoeMemoryRule = /^\d+$/;
const xprofilerMemoryRule = /^\d+$/;
function getMemoryRules(list, alinode) {
  return setRules(list, alinode, { alinodeRule: alindoeMemoryRule, xprofilerRule: xprofilerMemoryRule });
}
function setSpaceKeys(list) {
  const spaces = ['new', 'old', 'code', 'map', 'lo', 'read_only', 'new_lo', 'code_lo'];
  for (const key of spaces) {
    list.push(`${key}_space_size`);
    list.push(`${key}_space_used`);
    list.push(`${key}_space_available`);
    list.push(`${key}_space_committed`);
  }
}
const memoryKeys = ['rss', 'heap_used', 'heap_available', 'heap_total', 'heap_limit',
  'heap_executeable', 'total_physical_size', 'malloced_memory', 'amount_of_external_allocated_memory'];
setSpaceKeys(memoryKeys);

// gc rulr
const alindoeGcRule = /^\d+$/;
const xprofilerGcRule = /^\d+$/;
function getGcRules(list, alinode) {
  return setRules(list, alinode, { alinodeRule: alindoeGcRule, xprofilerRule: xprofilerGcRule });
}

// libuv handles
const alindoeUvHandleRule = /^\d+$/;
const xprofilerUvHandleRule = /^\d+$/;
function getUvRules(list, alinode) {
  return setRules(list, alinode, { alinodeRule: alindoeUvHandleRule, xprofilerRule: xprofilerUvHandleRule });
}

// alinode log structure
const alinodeLogStructure = {
  other: getCpuRules(['now', 'cpu_15', 'cpu_30', 'cpu_60'], true),
  heap: getMemoryRules(memoryKeys, true),
  gc: getGcRules(['gc_time_during_last_min', 'total', 'scavange_duration', 'marksweep_duration'], true),
  timer: getUvRules(['total_timer', 'active_handles'], true),
  http: {
    live_http_request: /^\d+$/,
    http_request_handled: /^\d+$/,
    http_response_sent: /^\d+$/,
    http_rt: /^\d+.\d{2}$/
  }
};

// xprofiler log structure
const xprofilerLogStructure = {
  cpu: getCpuRules(['cpu_now', 'cpu_15', 'cpu_30', 'cpu_60']),
  memory: getMemoryRules(memoryKeys),
  gc: getGcRules(['uptime', 'total_gc_times', 'total_gc_duration', 'total_scavange_duration',
    'total_marksweep_duration', 'total_incremental_marking_duration', 'gc_time_during_last_record',
    'scavange_duration_last_record', 'marksweep_duration_last_record', 'incremental_marking_duration_last_record']),
  uv: getUvRules(['active_handles', 'active_file_handles', 'active_and_ref_file_handles', 'active_tcp_handles',
    'active_and_ref_tcp_handles', 'active_udp_handles', 'active_and_ref_udp_handles',
    'active_timer_handles', 'active_and_ref_timer_handles']),
  http: {
    live_http_request: /^\d+$/,
    http_response_close: /^\d+$/,
    http_response_sent: /^\d+$/,
    http_request_timeout: /^\d+$/,
    http_patch_timeout: /^1$/,
    http_rt: /^\d+.\d{2}$/,
    res: { notRequired: true, regexp: /^\d+$/ }
  }
};

function getTestCases(title, logdirBlocking, logdirNonBlocking, envConfig, structure, alinode) {
  const cases = [];
  const date = moment().format('YYYYMMDD');

  // common env
  const commonEnvConfig = Object.assign({}, process.env, {
    XPROFILER_LOG_INTERVAL: 1,
    XPROFILER_PATCH_HTTP_TIMEOUT: 1,
  });

  // common case config
  const subtitle = 'when js worker thread';
  const blockingTarget = {
    title: `${subtitle} blocking`,
    file: blocking,
    env: { XPROFILER_LOG_DIR: logdirBlocking }
  };
  const nonBlockingTarget = {
    title: `${subtitle} non-blocking`,
    file: nonBlocking,
    env: { XPROFILER_LOG_DIR: logdirNonBlocking }
  };
  const commonCaseConfig = {
    env: Object.assign({}, commonEnvConfig, envConfig),
    execTime: 3500
  };

  // alinode common config
  const alinodeCommonConfig = {
    title: `alinode ${title}`,
    env: Object.assign({}, commonEnvConfig, { XPROFILER_LOG_FORMAT_ALINODE: 'YES' }, envConfig),
    targets: [
      Object.assign({}, blockingTarget, { logfile: path.join(logdirBlocking, `node-${date}.log`) }),
      Object.assign({}, nonBlockingTarget, { logfile: path.join(logdirNonBlocking, `node-${date}.log`) }),
    ],
    logparse: utils.alinodePrefixRegexp,
    alinode: true
  };

  // xprofiler common config
  const xprofilerCommonConfig = {
    title: `xprofiler ${title}`,
    targets: [
      Object.assign({}, blockingTarget, { logfile: path.join(logdirBlocking, `xprofiler-${date}.log`) }),
      Object.assign({}, nonBlockingTarget, { logfile: path.join(logdirNonBlocking, `xprofiler-${date}.log`) }),
    ],
    logparse: utils.xprofilerPrefixRegexp,
    alinode: false
  };

  if (envConfig && structure) {
    if (alinode) {
      cases.push(Object.assign({},
        commonCaseConfig,
        alinodeCommonConfig,
        { struct: structure }));
    } else {
      cases.push(Object.assign({},
        commonCaseConfig,
        xprofilerCommonConfig,
        { struct: structure }));
    }
  } else {
    // add alinode performance log test
    cases.push(Object.assign({},
      commonCaseConfig,
      alinodeCommonConfig,
      { struct: Object.assign({}, alinodeLogStructure, structure) }));

    // add xprofiler performance log test
    cases.push(Object.assign({},
      commonCaseConfig,
      xprofilerCommonConfig,
      { struct: Object.assign({}, xprofilerLogStructure, structure) }));
  }

  return cases;
}

exports = module.exports = getTestCases;

exports.getUvRules = getUvRules;