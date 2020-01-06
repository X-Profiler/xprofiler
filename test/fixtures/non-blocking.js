'use strict';

const mm = require('mm');
const os = require('os');
const http = require('http');
const xprofiler = require('../../');

if (process.env.XPROFILER_UNIT_TEST_TMP_HOMEDIR) {
  mm(os, 'homedir', () => process.env.XPROFILER_UNIT_TEST_TMP_HOMEDIR);
}

xprofiler();

// start log bypass
xprofiler.runLogBypass();
xprofiler.runLogBypass();

// start commands listener
xprofiler.runCommandsListener();
xprofiler.runCommandsListener();

// set v8 hooks
xprofiler.setHooks();
xprofiler.setHooks();

// http server
const server1 = http.createServer(function (req, res) {
  setTimeout(() => res.end('hello world.'), 100);
});
server1.listen(8443, () => console.log('http server listen at 8443...'));
server1.unref();

const server2 = http.createServer({}, function (req, res) { res.end('hello world.'); });
server2.listen(9443, () => console.log('http server listen at 9443...'));
server2.unref();

function sendRequest(abort) {
  const req = http.request('http://localhost:8443');
  req.on('error', err => console.error('non-blocking', err.message));
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
}, 150);
interval.unref();

setTimeout(() => {
  mm.restore();
  clearInterval(interval);
  console.log('will close...');
  setTimeout(() => {
    server1.close();
    server2.close();
    console.log('closed');
  }, 200);
}, 8000);
