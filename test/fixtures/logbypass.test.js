'use strict';

const path = require('path');
const moment = require('moment');
const utils = require('./utils');
const blocking = path.join(__dirname, 'blocking.js');
const nonBlocking = path.join(__dirname, 'non-blocking.js');

const alinodeCpuRule = /^\d{1,3}.\d{2}/;
const xprofilerCpuRule = /^\d{1,3}.\d{2}/;

function getCpuRules(list, alinode) {
  const rules = {};
  for (const r of list) {
    if (alinode) {
      rules[r] = alinodeCpuRule;
    } else {
      rules[r] = xprofilerCpuRule;
    }
  }
  return rules;
}

const alinodeLogStructure = {
  other: getCpuRules(['now', 'cpu_15', 'cpu_30', 'cpu_60'], true)
};

const xprofilerLogStructure = {
  cpu: getCpuRules(['cpu_now', 'cpu_15', 'cpu_30', 'cpu_60'])
};

function getTestCases(title, logdir, envConfig = {}, structure = {}) {
  const cases = [];
  const date = moment().format('YYYYMMDD');

  // common env
  const commonEnvConfig = {
    XPROFILER_LOG_DIR: logdir,
    XPROFILER_LOG_INTERVAL: 1,
    XPROFILER_UNIT_TEST_SINGLE_MODULE: 'YES',
    TEST_START_XPROFILER_LOG_THREAD: 'YES'
  };

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
    env: Object.assign({ XPROFILER_LOG_FORMAT_ALINODE: 'YES' }, commonEnvConfig, envConfig),
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