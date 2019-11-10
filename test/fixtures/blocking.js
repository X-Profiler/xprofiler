'use strict';


const mm = require('mm');
const os = require('os');
const xprofiler = require('../../');

if (process.env.XPROFILER_UNIT_TEST_TMP_HOMEDIR) {
  mm(os, 'homedir', () => process.env.XPROFILER_UNIT_TEST_TMP_HOMEDIR);
}

process.env.XPROFILER_UNIT_TEST_SINGLE_MODULE = 'YES';

xprofiler();

// start log bypass
xprofiler.runLogBypass();
xprofiler.runLogBypass();

// start commands listener
xprofiler.runCommandsListener();
xprofiler.runCommandsListener();

/*eslint no-empty: "off"*/
const start = Date.now();
while (Date.now() - start < 6000) {

}

mm.restore();