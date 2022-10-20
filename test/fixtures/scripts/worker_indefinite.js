'use strict';

const workerThreads = require('worker_threads');
const os = require('os');
const mm = require('mm');

if (process.env.XPROFILER_UNIT_TEST_TMP_HOMEDIR) {
  mm(os, 'homedir', () => process.env.XPROFILER_UNIT_TEST_TMP_HOMEDIR);
}

const xprofiler = require('../../../');
xprofiler.start();

if (workerThreads.isMainThread) {
  const w = new workerThreads.Worker(__filename, {
    env: process.env,
  });
  w.on('exit', code => {
    console.log('worker exited', code);
  });
} else {
  setInterval(() => { }, 1000);
}
