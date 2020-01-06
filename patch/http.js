'use strict';

const shimmer = require('./shimmer');
const http = require('http');
const https = require('https');

function requestListenerWrapper(original, addLiveRequest, addCloseRequest, addSentRequest) {
  return function (req, res) {
    addLiveRequest();
    const start = Date.now();

    res.on('finish', () => addSentRequest(Date.now() - start));

    res.on('close', () => addCloseRequest());

    // call origin function
    const returned = original.apply(this, arguments);
    return returned;
  };
}

function serverWrapper(addLiveRequest, addCloseRequest, addSentRequest, original) {
  return function (opts, requestListener) {
    const args = Array.from(arguments);
    let returned;

    if (typeof opts === 'function') {
      args.splice(0, 1, requestListenerWrapper(opts, addLiveRequest, addCloseRequest, addSentRequest));
      returned = original.apply(this, args);
    }
    if (typeof requestListener === 'function') {
      args.splice(1, 1, requestListenerWrapper(requestListener, addLiveRequest, addCloseRequest, addSentRequest));
      returned = original.apply(this, args);
    }

    return returned;
  };
}

function patchHttp(addLiveRequest, addCloseRequest, addSentRequest) {
  // patch http server
  shimmer.wrap(http, 'createServer', serverWrapper.bind(this, addLiveRequest, addCloseRequest, addSentRequest));
  // patch https server
  shimmer.wrap(https, 'createServer', serverWrapper.bind(this, addLiveRequest, addCloseRequest, addSentRequest));
}

module.exports = { patchHttp };