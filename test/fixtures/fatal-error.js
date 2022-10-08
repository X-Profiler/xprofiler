'use strict';

const os = require('os');
const mm = require('mm');
const xprofiler = require('../../');
const utils = require('./utils');

if (process.env.XPROFILER_UNIT_TEST_TMP_HOMEDIR) {
  mm(os, 'homedir', () => process.env.XPROFILER_UNIT_TEST_TMP_HOMEDIR);
}

xprofiler();

process.send({ type: utils.clientConst.xprofilerDone });

const array = [];

setInterval(() => {
  array.push(new Array(10e6).fill('*'));
  console.log('now rss:', process.memoryUsage().rss / 1024 / 1024 + ' Mb');
}, Number(process.env.XPROFILER_FATAL_ERROR_INTERVAL) || 1);