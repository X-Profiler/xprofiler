'use strict';

const os = require('os');
const path = require('path');
const utils = require('./utils');

function getExtra(key, expected, ext) {
  return ext.map(item => {
    if (item.key && item.value && item.expected) {
      return item;
    }
    return { key, value: item, expected };
  });
}

function getTestKey(configKey, defaultValue, envKey, envValue, userValue, envExt = [], userExt = []) {
  const rule = {
    env: { key: envKey, value: envValue },
    user: { key: configKey, value: userValue },
    expected: userValue
  };

  if (envValue === userValue) {
    rule.env.value = defaultValue;
  }

  const booleanEnvValue = ['YES', 'NO'];
  if (booleanEnvValue.includes(envValue) && (envValue === 'YES') === userValue) {
    rule.env.value = envValue === 'YES' ? 'NO' : 'YES';
  }

  const envExtra = getExtra(envKey, defaultValue, envExt);
  const userExtra = getExtra(configKey, defaultValue, userExt);

  return {
    defaultValue, rule,
    env: [{
      key: envKey, value: envValue,
      expected: booleanEnvValue.includes(envValue) ? envValue === 'YES' : envValue
    }].concat(envExtra),
    user: [
      { key: configKey, value: userValue, expected: userValue },
      { key: configKey, value: defaultValue, expected: defaultValue }
    ].concat(userExtra),
  };
}

function getTestKeys(configure) {
  const result = {};
  for (const key of Object.keys(configure)) {
    const data = configure[key];
    result[key] = getTestKey(key, data.defaultValue, data.envKey, data.envValue,
      data.userValue, data.envExt, data.userExt);
  }
  return result;
}

const configure = {
  log_dir: {
    defaultValue: os.tmpdir(),
    envKey: 'XPROFILER_LOG_DIR',
    envValue: utils.createLogDir('logdir_env'),
    userValue: utils.createLogDir('logdir_user'),
    envExt: ['env/not/absolute/path', path.join(__dirname, 'fixtures/no_dir_env')],
    userExt: ['user/not/absolute/path', path.join(__dirname, 'fixtures/no_dir_user')],
  },
  log_interval: {
    defaultValue: 60,
    envKey: 'XPROFILER_LOG_INTERVAL',
    envValue: 30,
    userValue: 66
  },
  enable_log_uv_handles: {
    defaultValue: true,
    envKey: 'XPROFILER_ENABLE_LOG_UV_HANDLES',
    envValue: 'NO',
    userValue: false
  },
  log_format_alinode: {
    defaultValue: false,
    envKey: 'XPROFILER_LOG_FORMAT_ALINODE',
    envValue: 'YES',
    userValue: true
  },
  log_level: {
    defaultValue: 1,
    envKey: 'XPROFILER_LOG_LEVEL',
    envValue: 0,
    userValue: 2
  },
  log_type: {
    defaultValue: 0,
    envKey: 'XPROFILER_LOG_TYPE',
    envValue: 1,
    userValue: 1
  },
  enable_fatal_error_hook: {
    defaultValue: true,
    envKey: 'XPROFILER_ENABLE_FATAL_ERROR_HOOK',
    envValue: 'NO',
    userValue: false
  },
  patch_http: {
    defaultValue: true,
    envKey: 'XPROFILER_PATCH_HTTP',
    envValue: 'NO',
    userValue: false
  },
  patch_http_timeout: {
    defaultValue: 30,
    envKey: 'XPROFILER_PATCH_HTTP_TIMEOUT',
    envValue: 60,
    userValue: 45
  },
  check_throw: {
    defaultValue: true,
    envKey: 'XPROFILER_CHECK_THROW',
    envValue: 'NO',
    userValue: false
  }
};

module.exports = getTestKeys(configure);