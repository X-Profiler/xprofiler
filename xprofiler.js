'use strict';

const path = require('path');
const utils = require('./lib/utils');
const clean = require('./lib/clean');
const { patch } = require('./patch');
const configure = require('./lib/configure');
const moment = require('moment');
const pkg = require('./package.json');
const workerThreads = require('./lib/worker_threads');

// xprofiler.node
const binary = require('@xprofiler/node-pre-gyp');
const bindingPath = binary.find(path.resolve(path.join(__dirname, './package.json')));
const xprofiler = require(bindingPath);
xprofiler.setup({
  isMainThread: workerThreads.isMainThread,
  threadId: workerThreads.threadId,
  nodeVersion: process.version
});

const runOnceStatus = {
  bypassLogThreadStarted: false,
  commandsListenerThreadStarted: false,
  hooksSetted: false,
  malloptInited: false,
};

let configured = false;

function checkNecessary() {
  if (!configured) {
    throw new Error('must run "require(\'xprofiler\')()" to set xprofiler config first!');
  }
}

/* istanbul ignore next */
function checkSocketPath(finalConfig) {
  const passed = xprofiler.checkSocketPath(true);
  if (!passed) {
    const message = 'socket path is too long, complete log of this error can be found in:\n'
      + `  ${path.join(finalConfig.log_dir, `xprofiler-error-${moment().format('YYYYMMDD')}.log`)}\n`;
    if (finalConfig.check_throw) {
      throw new Error(message);
    }
    console.error(`\n[${moment().format('YYYY-MM-DD HH:mm:ss')}] [error] [xprofiler-ipc] [${pkg.version}] ${message}`);
    return;
  }

  return passed;
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

  const singleModuleMode = process.env.XPROFILER_UNIT_TEST_SINGLE_MODULE === 'YES';

  if (workerThreads.isMainThread) {
    // check socket path
    checkSocketPath(finalConfig);

    // clean & set logdir info to file
    const logdir = finalConfig.log_dir;
    clean(logdir);
    utils.setLogDirToFile(logdir);
    if (!singleModuleMode) {
      // start commands listener thread if needed
      exports.runCommandsListener();
    }
  }

  if (!singleModuleMode) {
    // start performance log thread if needed
    exports.runLogBypass();
    // set hooks
    exports.setHooks();
  }

  // patch modules
  patch(finalConfig, {
    // http status
    setHttpConfig: xprofiler.setHttpConfig,
    addLiveRequest: xprofiler.addLiveRequest,
    addCloseRequest: xprofiler.addCloseRequest,
    addSentRequest: xprofiler.addSentRequest,
    addRequestTimeout: xprofiler.addRequestTimeout,
    addHttpStatusCode: xprofiler.addHttpStatusCode,
    addHttpProfilingDetail: xprofiler.addHttpProfilingDetail,
  });
}

exports = module.exports = start;

exports.start = start;

exports.setConfig = function (config) {
  // set config
  const { flattern } = configure(config);
  configured = true;
  if (workerThreads.isMainThread) {
    xprofiler.configure(flattern);
  }

  return exports.getXprofilerConfig();
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

exports.initMallopt = runOnce.bind(null, 'malloptInited', xprofiler.initMallopt);
