'use strict';

const { patchHttp } = require('./http');

function patch(config, methods) {
  if (config.patch_http) {
    patchHttp({ ...methods, patch_http_timeout: config.patch_http_timeout });
  }
}

module.exports = { patch };