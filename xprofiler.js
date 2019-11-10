'use strict';

const os = require('os');
const fs = require('fs');
const path = require('path');
const xprofiler = require('bindings')('xprofiler');
const utils = require('./lib/utils');
const clean = require('./lib/clean');
const SPLITTER = utils.SPLITTER;

let configured = false;
let bypassLogThreadStarted = false;
let commandsListenerThreadStarted = false;

const defaultConfig = {
  log_dir: os.tmpdir(),
  log_interval: 60, // seconds
  enable_log_uv_handles: true,
  log_format_alinode: false,
  log_level: 1,
  log_type: 0
};

function checkNecessary() {
  if (!configured) {
    throw new Error('must run "require(\'xprofiler\')()" to set xprofiler config first!');
  }
}

function checkLogDirAccessiable(logdir) {
  const exists = fs.existsSync(logdir);
  let accessiable;
  try {
    fs.accessSync(logdir, fs.constants.R_OK | fs.constants.W_OK);
    accessiable = true;
  } catch (err) {
    accessiable = false;
  }
  return exists && accessiable;
}

function composeLogDirInfo(logdir) {
  return [process.pid, logdir].join(SPLITTER) + '\n';
}

function setLogDirToFile(logdir) {
  clean(logdir);
  const dataFile = path.join(os.homedir(), '.xprofiler');
  if (fs.existsSync(dataFile)) {
    const processes = fs
      .readFileSync(dataFile, 'utf8')
      .split('\n')
      .filter(p => utils.processAlive(p.split(SPLITTER)[0]))
      .join('\n') + composeLogDirInfo(logdir);
    fs.writeFileSync(dataFile, processes);
  } else {
    fs.writeFileSync(dataFile, composeLogDirInfo(logdir));
  }
}

const simpleCheck = {
  string: value => typeof value === 'string',
  path: value => path.isAbsolute(value),
  number: value => value !== null && !isNaN(value),
  boolean: value => ['YES', 'NO', true, false].includes(value)
};

const formatValue = {
  string: value => String(value),
  number: value => Number(value),
  boolean: value => ['YES', 'NO'].includes(value) ? value === 'YES' : value
};

function checkRule(rules, value, { config, key, format }) {
  if (rules.every(rule => simpleCheck[rule] && simpleCheck[rule](value))) {
    config[key] = typeof format === 'function' && format(value);
  }
}

function getFinalUserConfigure(envConfig, userConfig) {
  // check user configured log_dir is accessiable
  const finalConfigure = Object.assign({}, defaultConfig, envConfig, userConfig);
  const logDirIllegal =
    typeof finalConfigure.log_dir === 'string' && !checkLogDirAccessiable(finalConfigure.log_dir);
  let logDirMessage = '';
  if (logDirIllegal) {
    // todo: need check default log_dir is accessiable
    // if (!checkLogDirAccessiable(defaultConfig.log_dir)) {
    //   throw new Error(`can't access default log dir: ${defaultConfig.log_dir}`);
    // }
    const extra = `env: ${envConfig.log_dir}, user: ${userConfig.log_dir}`;
    logDirMessage =
      `${finalConfigure.log_dir} will be ignored (${extra}), use default log_dir: ${defaultConfig.log_dir}`;
    // output error log
    console.error('[config_int]', logDirMessage);
    finalConfigure.log_dir = defaultConfig.log_dir;
  }
  setLogDirToFile(finalConfigure.log_dir);
  return finalConfigure;
}

function getConfigure(configList, user) {
  const envConfig = {};
  const userConfig = {};
  for (const config of configList) {
    const rules = config.rules;
    const key = config.name;
    const format = formatValue[config.format];
    const envValue = process.env[config.env];
    checkRule(rules, envValue, { config: envConfig, key, format });
    const userValue = user[config.name];
    checkRule(rules, userValue, { config: userConfig, key, format });
  }
  return getFinalUserConfigure(envConfig, userConfig);
}

function runLogBypass() {
  if (bypassLogThreadStarted) {
    return;
  }
  bypassLogThreadStarted = true;
  checkNecessary();
  return xprofiler.runLogBypass();
};

function runCommandsListener() {
  if (commandsListenerThreadStarted) {
    return;
  }
  commandsListenerThreadStarted = true;
  checkNecessary();
  return xprofiler.runCommandsListener();
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

  const finalUserConfigure = getConfigure(configList, config);

  // set config
  const finalConfig = Object.assign({}, defaultConfig, finalUserConfigure);
  xprofiler.configure(finalConfig);
  configured = true;

  if (process.env.XPROFILER_UNIT_TEST_SINGLE_MODULE !== 'YES') {
    // start performance log thread
    runLogBypass();
    // start commands listener thread
    runCommandsListener();
  }
};

exports.getXprofilerConfig = function () {
  checkNecessary();
  return xprofiler.getConfig();
};

exports.info = function (...args) {
  checkNecessary();
  return xprofiler.info(...args);
};

exports.error = function (...args) {
  checkNecessary();
  return xprofiler.error(...args);
};

exports.debug = function (...args) {
  checkNecessary();
  return xprofiler.debug(...args);
};

exports.runLogBypass = runLogBypass;

exports.runCommandsListener = runCommandsListener;
