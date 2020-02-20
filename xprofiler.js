'use strict';

const xprofiler = require('bindings')('xprofiler');
const utils = require('./lib/utils');
const clean = require('./lib/clean');
const { patch } = require('./patch');
const configure = require('./lib/configure');
const configList = require('./xprofiler.json');

const runOnceStatus = {
  bypassLogThreadStarted: false,
  commandsListenerThreadStarted: false,
  hooksSetted: false
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

function start(config = {}) {
  // set config by user and env
  const finalConfig = exports.setConfig(config);

  // clean & set logdir info to file
  const logdir = finalConfig.log_dir;
  clean(logdir);
  utils.setLogDirToFile(logdir);

  if (process.env.XPROFILER_UNIT_TEST_SINGLE_MODULE !== 'YES') {
    // start performance log thread
    exports.runLogBypass();
    // start commands listener thread
    exports.runCommandsListener();
    // set hooks
    exports.setHooks();
  }

  // patch modules
  patch(finalConfig, {
    // http status
    addLiveRequest: xprofiler.addLiveRequest,
    addCloseRequest: xprofiler.addCloseRequest,
    addSentRequest: xprofiler.addSentRequest,
    addRequestTimeout: xprofiler.addRequestTimeout,
    addHttpStatusCode: xprofiler.addHttpStatusCode
  });
}

exports = module.exports = start;

exports.start = start;

exports.setConfig = function (config) {
  // set config
  const finalConfig = configure(configList, config);
  configured = true;
  xprofiler.configure(finalConfig);

  return finalConfig;
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

exports.setHooks = runOnce.bind(null, 'hooksSetted', xprofiler.setHooks);
