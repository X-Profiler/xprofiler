'use strict';

const { patchHttp } = require('./http');

function patch(config, methods) {
  if (config.patch_http) {
    patchHttp(methods);
  }
}

module.exports = { patch };