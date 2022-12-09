'use strict';

const mm = require('mm');
const os = require('os');
const http = require('http');
const utils = require('../utils');
const xprofiler = require('../../../');

if (process.env.XPROFILER_UNIT_TEST_TMP_HOMEDIR) {
  mm(os, 'homedir', () => process.env.XPROFILER_UNIT_TEST_TMP_HOMEDIR);
}

xprofiler({ check_throw: false });

// start log bypass
xprofiler.runLogBypass();
xprofiler.runLogBypass();

// start commands listener
xprofiler.runCommandsListener();
xprofiler.runCommandsListener();

// set v8 hooks
xprofiler.setHooks();
xprofiler.setHooks();

process.send({ type: utils.clientConst.xprofilerDone });

// http server
const server = http.createServer(function (req, res) {
  setTimeout(() => res.end('hello world.'), (0.5 + Math.random()) * 1000);
});
server.listen(8445, () => console.log('http server listen at 8445...'));
server.unref();

let logHttpRequestErrorOnce = false;

function sendRequest(abort) {
  const req = http.request('http://localhost:8445');
  req.on('error', err => !logHttpRequestErrorOnce && (logHttpRequestErrorOnce = true)
    && console.error('normal process', err.message));
  req.end();

  if (abort) {
    setTimeout(() => {
      req.abort();
    }, 50);
  }
}

let times = 1;
const interval = setInterval(() => {
  if (times % 2 === 0) {
    sendRequest(true);
  } else {
    sendRequest();
  }
  times++;
}, 10);
interval.unref();

setTimeout(() => {
  mm.restore();
  clearInterval(interval);
  console.log('will close...');
  setTimeout(() => {
    server.close();
    console.log('closed');
  }, 200);
}, 8000);
