'use strict';

const { patchHttp, subscribeHttpServerRequestStart } = require('./http');

function patch(config, methods) {
  if (config.patch_http) {
    const options = { ...methods, patch_http_timeout: config.patch_http_timeout };
    if (config.patch_http_with_diagnostics_channel) {
      subscribeHttpServerRequestStart(options);
    } else {
      patchHttp(options);
    }
  }
}

module.exports = { patch };
