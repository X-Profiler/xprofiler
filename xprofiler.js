'use strict';

const xprofiler = require('bindings')('xprofiler');
const utils = require('./lib/utils');
const clean = require('./lib/clean');
const configure = require('./lib/configure');

const runOnceStatus = {
  bypassLogThreadStarted: false,
  commandsListenerThreadStarted: false
};
let configured = false;

function checkNecessary() {
  if (!configured) {
    throw new Error('must run "require(\'xprofiler\')()" to set xprofiler config first!');
  }
}

function runOnce(onceKey, onceFunc) {
  checkNecessary();
  if (runOnceStatus[onceKey]) {
    return;
  }
  runOnceStatus[onceKey] = true;
  onceFunc();
}

exports = module.exports = (config = {}) => {
  const configList = [
    {
      name: 'log_dir',
      env: 'XPROFILER_LOG_DIR',
      rules: ['string', 'path'],
      format: 'string'
    },
    {
      name: 'log_interval',
      env: 'XPROFILER_LOG_INTERVAL',
      rules: ['number'],
      format: 'number'
    },
    {
      name: 'enable_log_uv_handles',
      env: 'XPROFILER_ENABLE_LOG_UV_HANDLES',
      rules: ['boolean'],
      format: 'boolean'
    },
    {
      name: 'log_format_alinode',
      env: 'XPROFILER_LOG_FORMAT_ALINODE',
      rules: ['boolean'],
      format: 'boolean'
    },
    {
      name: 'log_level',
      env: 'XPROFILER_LOG_LEVEL',
      rules: ['number'],
      format: 'number'
    },
    {
      name: 'log_type',
      env: 'XPROFILER_LOG_TYPE',
      rules: ['number'],
      format: 'number'
    }
  ];

  // set config
  const finalConfig = configure(configList, config);
  configured = true;
  xprofiler.configure(finalConfig);

  // clean & set logdir info to file
  const logdir = finalConfig.log_dir;
  clean(logdir);
  utils.setLogDirToFile(logdir);

  if (process.env.XPROFILER_UNIT_TEST_SINGLE_MODULE !== 'YES') {
    // start performance log thread
    exports.runLogBypass();
    // start commands listener thread
    exports.runCommandsListener();
  }
};

exports.getXprofilerConfig = function () {
  checkNecessary();
  return xprofiler.getConfig();
};

['info', 'error', 'debug'].forEach(level => exports[level] = function (...args) {
  checkNecessary();
  xprofiler[level](...args);
});

exports.runLogBypass = runOnce.bind(null, 'bypassLogThreadStarted', xprofiler.runLogBypass);

exports.runCommandsListener = runOnce.bind(null, 'commandsListenerThreadStarted', xprofiler.runCommandsListener);
