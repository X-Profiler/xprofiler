'use strict';

const os = require('os');
const path = require('path');
const xprofiler = require('bindings')('xprofiler');

let configured = false;

const defaultConfig = {
  log_dir: os.tmpdir(),
  log_interval: 60, // seconds
  enable_log_uv_handles: true,
  log_format_alinode: false,
  log_level: 1
};

function check() {
  if (!configured) {
    throw new Error('must run "require(\'xprofiler\')()" to set xprofiler config first!');
  }
}

exports = module.exports = (config = {}) => {
  configured = true;

  const envConfig = {};
  const userConfig = {};
  // log dir
  const logDirEnv = process.env.XPROFILER_LOG_DIR;
  if (typeof logDirEnv === 'string' && path.isAbsolute(logDirEnv)) {
    envConfig.log_dir = path.resolve(logDirEnv);
  }
  const logDirUser = config.log_dir;
  if (typeof logDirUser === 'string' && path.isAbsolute(logDirUser)) {
    userConfig.log_dir = path.resolve(config.log_dir);
  }

  // log interval
  const logIntervalEnv = process.env.XPROFILER_LOG_INTERVAL;
  if (logIntervalEnv !== null && !isNaN(logIntervalEnv)) {
    envConfig.log_interval = Number(logIntervalEnv);
  }
  const logIntervalUser = config.log_interval;
  if (logIntervalUser !== null && !isNaN(logIntervalUser)) {
    userConfig.log_interval = Number(logIntervalUser);
  }

  // enable collecting uv handles
  const enableLogUvHandlesEnv = process.env.XPROFILER_ENABLE_LOG_UV_HANDLES;
  if (typeof enableLogUvHandlesEnv === 'string') {
    envConfig.enable_log_uv_handles = enableLogUvHandlesEnv === 'YES';
  }
  if (config.enable_log_uv_handles === false || config.enable_log_uv_handles === true) {
    userConfig.enable_log_uv_handles = config.enable_log_uv_handles;
  }

  // log format alinode
  const logFormatAlinodeEnv = process.env.XPROFILER_LOG_FORMAT_ALINODE;
  if (typeof logFormatAlinodeEnv === 'string') {
    envConfig.log_format_alinode = logFormatAlinodeEnv === 'YES';
  }
  if (config.log_format_alinode === false || config.log_format_alinode === true) {
    userConfig.log_format_alinode = config.log_format_alinode;
  }

  // log level
  const logLevelEnv = process.env.XPROFILER_LOG_LEVEL;
  if (logLevelEnv !== null && !isNaN(logLevelEnv)) {
    envConfig.log_level = Number(logLevelEnv);
  }
  const logLevelUser = config.log_level;
  if (logLevelUser !== null && !isNaN(logLevelUser)) {
    userConfig.log_level = Number(logLevelUser);
  }

  // set config
  xprofiler.configure(Object.assign({}, defaultConfig, envConfig, userConfig));
};

exports.getXprofilerConfig = function () {
  check();
  return xprofiler.getConfig();
};

exports.info = function (...args) {
  check();
  return xprofiler.info(...args);
};

exports.error = function (...args) {
  check();
  return xprofiler.error(...args);
};

exports.debug = function (...args) {
  check();
  return xprofiler.debug(...args);
};
