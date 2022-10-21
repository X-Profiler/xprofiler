'use strict';

const workerThreads = require('worker_threads');
const os = require('os');
const mm = require('mm');
const moment = require('moment');
const { v4: uuid } = require('uuid');
const utils = require('../utils');

const traceid = uuid();

if (process.env.XPROFILER_UNIT_TEST_TMP_HOMEDIR) {
  mm(os, 'homedir', () => process.env.XPROFILER_UNIT_TEST_TMP_HOMEDIR);
}

const xprofiler = require('../../../');
xprofiler.start({ check_throw: false });

if (workerThreads.isMainThread) {
  console.log(`[${moment().format('YYYY-MM-DD HH:mm:ss')}]`, traceid, 'blocking start.');
  const w = new workerThreads.Worker(__filename, {
    env: process.env,
  });
  w.on('exit', code => {
    console.log(`[${moment().format('YYYY-MM-DD HH:mm:ss')}]`, traceid, 'worker exited', code);
  });
  w.on('message', msg => process.send(msg));
} else {
  workerThreads.parentPort.postMessage({ type: utils.clientConst.xprofilerDone });
  console.log(`[${moment().format('YYYY-MM-DD HH:mm:ss')}]`, traceid, 'blocking start.');

  const start = Date.now();
  while (Date.now() - start < 10000) { /** ignore */ }

  console.log(`[${moment().format('YYYY-MM-DD HH:mm:ss')}]`, traceid, 'blocking done.');
}
