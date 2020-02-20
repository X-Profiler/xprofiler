'use strict';

const shimmer = require('./shimmer');
const http = require('http');
const https = require('https');

function requestListenerWrapper(original, methods) {
  return function (req, res) {
    const { addLiveRequest, addCloseRequest, addSentRequest, addHttpStatusCode } = methods;

    addLiveRequest();

    const start = Date.now();

    res.on('finish', () => {
      addHttpStatusCode(res.statusCode);
      addSentRequest(Date.now() - start);
    });

    res.on('close', () => addCloseRequest());

    // call origin function
    const returned = original.apply(this, arguments);
    return returned;
  };
}

function serverWrapper(methods, original) {
  return function (opts, requestListener) {
    const args = Array.from(arguments);
    let returned;

    if (typeof opts === 'function') {
      args.splice(0, 1, requestListenerWrapper(opts, methods));
    } else if (typeof requestListener === 'function') {
      args.splice(1, 1, requestListenerWrapper(requestListener, methods));
    }

    returned = original.apply(this, args);

    return returned;
  };
}

function patchHttp(methods) {
  // patch http server
  shimmer.wrap(http, 'createServer', serverWrapper.bind(this, methods));
  // patch https server
  shimmer.wrap(https, 'createServer', serverWrapper.bind(this, methods));
}

module.exports = { patchHttp };