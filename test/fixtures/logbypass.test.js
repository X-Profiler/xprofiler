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
  const spaces = ['new', 'old', 'code', 'map', 'lo'];
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
const alinodeLogStructure = {
  other: getCpuRules(['now', 'cpu_15', 'cpu_30', 'cpu_60'], true),
  heap: getMemoryRules(memoryKeys, true)
};

const xprofilerLogStructure = {
  cpu: getCpuRules(['cpu_now', 'cpu_15', 'cpu_30', 'cpu_60']),
  memory: getMemoryRules(memoryKeys)
};

function getTestCases(title, logdir, envConfig = {}, structure = {}) {
  const cases = [];
  const date = moment().format('YYYYMMDD');

  // common env
  const commonEnvConfig = Object.assign({}, process.env, {
    XPROFILER_LOG_DIR: logdir,
    XPROFILER_LOG_INTERVAL: 1,
    XPROFILER_UNIT_TEST_SINGLE_MODULE: 'YES',
    TEST_START_XPROFILER_LOG_THREAD: 'YES'
  });

  // common case config
  const subtitle = 'when js worker thread';
  const commonCaseConfig = {
    targets: [
      { title: `${subtitle} blocking`, file: blocking },
      { title: `${subtitle} non-blocking`, file: nonBlocking }
    ],
    env: Object.assign({}, commonEnvConfig, envConfig),
    execTime: 3500
  };

  // add alinode performance log test
  cases.push(Object.assign({}, commonCaseConfig, {
    title: `alinode ${title}`,
    env: Object.assign({}, commonEnvConfig, { XPROFILER_LOG_FORMAT_ALINODE: 'YES' }, envConfig),
    struct: Object.assign({}, alinodeLogStructure, structure),
    logfile: path.join(logdir, `node-${date}.log`),
    logparse: utils.alinodePrefixRegexp,
    alinode: true
  }));

  // add xprofiler performance log test
  cases.push(Object.assign({}, commonCaseConfig, {
    title: `xprofiler ${title}`,
    struct: Object.assign({}, xprofilerLogStructure, structure),
    logfile: path.join(logdir, `xprofiler-${date}.log`),
    logparse: utils.xprofilerPrefixRegexp,
    alinode: false
  }));

  return cases;
}

module.exports = getTestCases;