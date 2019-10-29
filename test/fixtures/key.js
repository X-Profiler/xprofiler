'use strict';

const os = require('os');
const path = require('path');

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
    user: [{ key: configKey, value: userValue, expected: userValue }].concat(userExtra),
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
    envValue: path.join(__dirname, 'logdir_env'),
    userValue: path.join(__dirname, 'logdir_user'),
    envExt: ['env/not/absolute/path'],
    userExt: ['user/not/absolute/path']
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
  }
};

module.exports = getTestKeys(configure);