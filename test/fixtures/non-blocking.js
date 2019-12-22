'use strict';

const mm = require('mm');
const os = require('os');
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

setTimeout(() => {
  mm.restore();
}, 8000);