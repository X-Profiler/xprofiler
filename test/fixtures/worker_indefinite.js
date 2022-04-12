'use strict';

if (Number.parseInt(process.versions.node.split('.')[0], 10) <= 10) {
  process.exit(0);
}

const workerThreads = require('worker_threads');
const os = require('os');
const mm = require('mm');

if (process.env.XPROFILER_UNIT_TEST_TMP_HOMEDIR) {
  mm(os, 'homedir', () => process.env.XPROFILER_UNIT_TEST_TMP_HOMEDIR);
}

const xprofiler = require('../../xprofiler');
xprofiler.start();

if (workerThreads.isMainThread) {
  const w = new workerThreads.Worker(__filename, {
    env: process.env,
  });
  w.on('exit', code => {
    console.log('worker exited', code);
  });
} else {
  setInterval(() => {}, 1000);
}
