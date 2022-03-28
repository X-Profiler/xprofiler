'use strict';

const cp = require('child_process');
const fs = require('fs');
const path = require('path');
const pack = require('../../package.json');

const MAGIC_BLURRY_TAG = pack.blurryTag;

exports.xprofilerPrefixRegexp =
// eslint-disable-next-line max-len
  /\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\] \[(.+)\] \[(.+)\] \[(\d+)\] \[(\d+)\] \[(\d{1,3}\.\d{1,3}\.\d{1,3}[a-zA-Z0-9\-_]*)\] (.*)/g;

exports.alinodePrefixRegexp =
  /\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{6}\] \[(.+)\] \[(.+)\] \[(\d+)\] (.*)/g;

exports.createLogDir = function createLogDir(logdir) {
  const log_dir = path.join(__dirname, logdir);
  if (!fs.existsSync(log_dir)) {
    fs.mkdirSync(log_dir, { recursive: true });
  } else {
    for (const file of fs.readdirSync(log_dir)) {
      fs.unlinkSync(path.join(log_dir, file));
    }
  }
  return log_dir;
};

exports.formatKey = function formatKey(key) {
  if (key.includes(MAGIC_BLURRY_TAG)) {
    return key.slice(0, key.indexOf(MAGIC_BLURRY_TAG));
  }
  return key;
};

exports.objKeyEqual = function objKeyEqual(obj1, obj2) {
  const keys1 = Object.keys(obj1).map(exports.formatKey);
  const keys2 = Object.keys(obj2).map(exports.formatKey);
  return keys1.every(k1 => keys2.includes(k1) || obj1[k1].notRequired) &&
    keys2.every(k2 => keys1.includes(k2) || obj2[k2].notRequired);
};

exports.cleanDir = function (dir) {
  for (const file of fs.readdirSync(dir)) {
    fs.unlinkSync(path.join(dir, file));
  }
  fs.rmdirSync(dir);
};

exports.sleep = function (ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
};

exports.getNestingValue = function (origin, key) {
  const keys = key.split('.');
  for (const k of keys) {
    origin = origin[k];
  }
  return origin;
};

exports.arrayEqual = function (arr1, arr2) {
  return arr1.every(a1 => arr2.includes(a1)) && arr2.every(a2 => arr1.includes(a2));
};

exports.getChildProcessExitInfo = function (proc) {
  return new Promise(resolve => proc.on('close', (code, signal) => resolve({ code, signal })));
};

exports.checkChildProcessExitInfo = function (expect, exitInfo) {
  const { code, signal } = exitInfo;
  // One of the code | signal will always be non-null.
  expect(code === 0).to.be.ok();
  expect(signal === null).to.be.ok();
};

function createDeferred() {
  let resolve, reject;
  const promise = new Promise((res, rej) => {
    resolve = res;
    reject = rej;
  });
  return {
    promise,
    resolve,
    reject,
  };
}
exports.createDeferred = createDeferred;

/** Node.js v8.x compat for events.once */
exports.once = function once(eventemitter, event) {
  const deferred = createDeferred();
  let uninstallListeners;
  const listener = (...args) => {
    deferred.resolve(args);
    uninstallListeners();
  };
  const errorListener = (err) => {
    deferred.reject(err);
    uninstallListeners();
  };
  uninstallListeners = () => {
    eventemitter.removeListener(event, listener);
    eventemitter.removeListener('error', errorListener);
  };
  eventemitter.on(event, listener);
  eventemitter.on('error', errorListener);
  return deferred.promise;
};

exports.fork = function fork(filepath, options = {}) {
  const proc = cp.fork(filepath, Object.assign({
    stdio: ['ignore', 'pipe', 'pipe', 'ipc'],
  }, options));
  proc.stdout.setEncoding('utf8');
  proc.stderr.setEncoding('utf8');

  let stdout = '';
  let stderr = '';
  proc.stdout.on('data', chunk => {
    stdout += chunk;
  });
  proc.stderr.on('data', chunk => {
    stderr += chunk;
  });

  proc.on('exit', (code, signal) => {
    if (code !== 0) {
      console.log('process exited with non-zero code: pid(%d), code(%d), signal(%d)', proc.pid, code, signal);
      console.log('stdout:\n', stdout);
      console.log('');
      console.log('stderr:\n', stderr);
    }
  });
  return proc;
};
