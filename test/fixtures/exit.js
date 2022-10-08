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

setTimeout(() => {
  // empty exit
}, 2000);