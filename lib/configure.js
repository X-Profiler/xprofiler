'use strict';

const os = require('os');
const fs = require('fs');
const path = require('path');

const defaultConfig = {
  log_dir: os.tmpdir(),
  log_interval: 60, // seconds
  enable_log_uv_handles: true,
  log_format_alinode: false,
  log_level: 1,
  log_type: 0
};

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

function getFinalUserConfigure(envConfig, userConfig) {
  // check user configured log_dir is accessiable
  const finalConfig = Object.assign({}, defaultConfig, envConfig, userConfig);
  const logDirIllegal =
    typeof finalConfig.log_dir === 'string' && !checkLogDirAccessiable(finalConfig.log_dir);
  let logDirMessage = '';
  if (logDirIllegal) {
    // todo: need check default log_dir is accessiable
    // if (!checkLogDirAccessiable(defaultConfig.log_dir)) {
    //   throw new Error(`can't access default log dir: ${defaultConfig.log_dir}`);
    // }
    const extra = `env: ${envConfig.log_dir}, user: ${userConfig.log_dir}`;
    logDirMessage =
      `${finalConfig.log_dir} will be ignored (${extra}), use default log_dir: ${defaultConfig.log_dir}`;
    // output error log
    console.error('[config_int]', logDirMessage);
    finalConfig.log_dir = defaultConfig.log_dir;
  }
  return finalConfig;
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

module.exports = function (configList, user) {
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
};