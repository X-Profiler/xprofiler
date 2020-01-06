'use strict';

const { patchHttp } = require('./http');

function patch(config, methods) {
  if (config.patch_http) {
    const { addLiveRequest, addCloseRequest, addSentRequest } = methods;
    patchHttp(addLiveRequest, addCloseRequest, addSentRequest);
  }
}

module.exports = { patch };