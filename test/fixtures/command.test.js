'use strict';

const pkg = require('../../package.json');

function escape(str) {
  str = JSON.stringify(str);
  return str.slice(1, str.length - 1);
}

module.exports = function (logdir) {
  return [
    {
      cmd: 'check_version',
      xctlRules: [{ key: 'data.version', rule: new RegExp(`^${pkg.version}$`) }],
      xprofctlRules(data) {
        return [new RegExp(`^X-Profiler 插件版本号\\(pid ${data.pid}\\): v${pkg.version}$`)];
      }
    },
    {
      cmd: 'get_config',
      xctlRules: [
        { key: 'data.log_dir', rule: new RegExp(`^${escape(logdir)}$`) },
        { key: 'data.log_interval', rule: /^60$/ },
        { key: 'data.enable_log_uv_handles', rule: { label: 'true', test: value => value === true } },
        { key: 'data.log_format_alinode', rule: { label: 'false', test: value => value === false } },
        { key: 'data.log_level', rule: /^1$/ },
        { key: 'data.log_type', rule: /^0$/ }
      ],
      xprofctlRules(data) {
        return [new RegExp(`^X-Profiler 当前配置\\(pid ${data.pid}\\):\n`
          + '  - enable_log_uv_handles: true\n'
          + `  - log_dir: ${escape(logdir)}\n`
          + '  - log_format_alinode: false\n'
          + '  - log_interval: 60\n'
          + '  - log_level: 1\n'
          + '  - log_type: 0')
        ];
      }
    },
    {
      cmd: 'set_config',
      options: { enable_log_uv_handles: false, log_level: 0, log_type: 1 },
      xctlRules: [
        { key: 'data.enable_log_uv_handles', rule: { label: 'false', test: value => value === false } },
        { key: 'data.log_level', rule: /^0$/ },
        { key: 'data.log_type', rule: /^1$/ }
      ],
      xprofctlRules(data) {
        return [new RegExp(`^X-Profiler 配置\\(pid ${data.pid}\\)成功:\n`
          + '  - enable_log_uv_handles: false\n'
          + '  - log_level: 0\n'
          + '  - log_type: 1')
        ];
      }
    },
    {
      cmd: 'set_config',
      errored: true,
      xctlRules: [],
      xprofctlRules() {
        return [/^set_config 参数不正确，执行 xprofctl set_config 查看正确用法$/];
      }
    },
    {
      cmd: 'set_config',
      options: { enable_log_uv_handles: 1 },
      errored: true,
      /* eslint-disable */
      xctlRules: [{ key: 'message', rule: /<enable_log_uv_handles> type error: \[json.exception.type_error.302\] type must be boolean, but is number/ }],
      xprofctlRules() { return []; }
    }
  ];
};