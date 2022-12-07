'use strict';

const fs = require('fs');
const path = require('path');
const configuration = require('../configuration')();

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
  const defaultConfig = configuration
    .reduce((defaultConfig, next) => Object.assign(defaultConfig, { [next.name]: next.value }), {});

  // check user configured log_dir is accessiable
  const finalConfig = Object.assign({}, defaultConfig, envConfig, userConfig);

  // check logdir accessible
  if (!checkLogDirAccessiable(finalConfig.log_dir)) {
    const extra = `env: ${envConfig.log_dir}, user: ${userConfig.log_dir}`;
    const logDirMessage =
      `${finalConfig.log_dir} will be ignored (${extra}), use default log_dir: ${defaultConfig.log_dir}`;
    console.error('[config_int]', logDirMessage);
    finalConfig.log_dir = defaultConfig.log_dir;
  }

  // replace value
  const flattern = configuration.map(config => Object.assign({}, config, { value: finalConfig[config.name] }));
  return { flattern, origin: finalConfig };
}

const simpleCheck = {
  string: value => typeof value === 'string',
  path: value => value && path.isAbsolute(value),
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

module.exports = function (user) {
  const envConfig = {};
  const userConfig = {};
  for (const config of configuration) {
    const rules = Array.isArray(config.rules) ? config.rules.concat(config.format) : [config.format];
    const key = config.name;
    const format = formatValue[config.format];
    const envValue = process.env[config.env];
    checkRule(rules, envValue, { config: envConfig, key, format });
    const userValue = user[config.name];
    checkRule(rules, userValue, { config: userConfig, key, format });
  }
  return getFinalUserConfigure(envConfig, userConfig);
};