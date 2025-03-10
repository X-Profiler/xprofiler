'use strict';

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

let _onHttpServerRequestStart;
function subscribeHttpServerRequestStart(options) {
  const { setHttpConfig, addLiveRequest, addCloseRequest, addSentRequest,
    addRequestTimeout, addHttpStatusCode, addHttpProfilingDetail, patch_http_timeout } = options;

  setHttpConfig(httpConfig);

  function onHttpServerRequestStart({ request, response }) {
    addLiveRequest();

    const start = Date.now();

    const timer = setTimeout(() => {
      addRequestTimeout();
      if (httpConfig.http_detail_profiling) {
        const detail = getRequestDetail(request, response, start, 0);
        addHttpProfilingDetail(detail);
      }
    }, patch_http_timeout * 1000);
    timer.unref();

    response.on('finish', () => {
      addHttpStatusCode(response.statusCode);
      addSentRequest(Date.now() - start);
      clearTimeout(timer);
      if (httpConfig.http_detail_profiling) {
        const detail = getRequestDetail(request, response, start, 1);
        addHttpProfilingDetail(detail);
      }
    });

    response.on('close', () => {
      addCloseRequest();
      clearTimeout(timer);
    });
  }

  // https://nodejs.org/docs/latest/api/diagnostics_channel.html#http
  // use diagnostics_channel
  const diagnosticsChannel = require('diagnostics_channel');
  diagnosticsChannel.subscribe('http.server.request.start', onHttpServerRequestStart);
  _onHttpServerRequestStart = onHttpServerRequestStart;
}

function unsubscribeHttpServerRequestStart() {
  if (!_onHttpServerRequestStart) {
    return;
  }

  const diagnosticsChannel = require('diagnostics_channel');
  diagnosticsChannel.unsubscribe('http.server.request.start', _onHttpServerRequestStart);
}

module.exports = { subscribeHttpServerRequestStart, unsubscribeHttpServerRequestStart };
