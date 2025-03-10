'use strict';

const { subscribeHttpServerRequestStart } = require('./http');

function patch(config, methods) {
  if (config.patch_http) {
    const options = { ...methods, patch_http_timeout: config.patch_http_timeout };
    subscribeHttpServerRequestStart(options);
  }
}

module.exports = { patch };
