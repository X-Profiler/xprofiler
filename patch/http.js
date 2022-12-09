'use strict';

const shimmer = require('./shimmer');
const http = require('http');
const https = require('https');

const httpConfig = {
  http_detail_profiling: false,
  start_time: 0,
};

function getRequestDetail(req, res, start, sent) {
  const offset = Number((Date.now() - httpConfig.start_time).toFixed(2));
  const rt = Date.now() - start;
  const url = req.url.slice(0, 1024);
  return `${offset},${url},${req.method},${sent},${res.statusCode},${rt}`;
}

function requestListenerWrapper(original, methods) {
  const { setHttpConfig, addLiveRequest, addCloseRequest, addSentRequest,
    addRequestTimeout, addHttpStatusCode, addHttpProfilingDetail, patch_http_timeout } = methods;

  setHttpConfig(httpConfig);

  return function (req, res) {
    addLiveRequest();

    const start = Date.now();

    const timer = setTimeout(() => {
      addRequestTimeout();
      if (httpConfig.http_detail_profiling) {
        const detail = getRequestDetail(req, res, start, 0);
        addHttpProfilingDetail(detail);
      }
    }, patch_http_timeout * 1000);
    timer.unref();

    res.on('finish', () => {
      addHttpStatusCode(res.statusCode);
      addSentRequest(Date.now() - start);
      clearTimeout(timer);
      if (httpConfig.http_detail_profiling) {
        const detail = getRequestDetail(req, res, start, 1);
        addHttpProfilingDetail(detail);
      }
    });

    res.on('close', () => {
      addCloseRequest();
      clearTimeout(timer);
    });

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