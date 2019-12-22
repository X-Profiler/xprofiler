'use strict';

const os = require('os');
const mm = require('mm');
const moment = require('moment');
const uuid = require('uuid/v4');
const traceid = uuid();
console.log(`[${moment().format('YYYY-MM-DD HH:mm:ss')}]`, traceid, 'blocking start.');

const xprofiler = require('../../');

if (process.env.XPROFILER_UNIT_TEST_TMP_HOMEDIR) {
  mm(os, 'homedir', () => process.env.XPROFILER_UNIT_TEST_TMP_HOMEDIR);
}

process.env.XPROFILER_UNIT_TEST_SINGLE_MODULE = 'YES';

xprofiler();
console.log(`[${moment().format('YYYY-MM-DD HH:mm:ss')}]`, traceid, 'blocking xprofiler() done.');

// start log bypass
xprofiler.runLogBypass();
xprofiler.runLogBypass();
console.log(`[${moment().format('YYYY-MM-DD HH:mm:ss')}]`, traceid, 'blocking xprofiler.runLogBypass() done.');

// start commands listener
xprofiler.runCommandsListener();
xprofiler.runCommandsListener();
console.log(`[${moment().format('YYYY-MM-DD HH:mm:ss')}]`, traceid, 'blocking xprofiler.runCommandsListener() done.');

// set v8 hooks
xprofiler.setHooks();
xprofiler.setHooks();
console.log(`[${moment().format('YYYY-MM-DD HH:mm:ss')}]`, traceid, 'blocking xprofiler.setHooks() done.');

/*eslint no-empty: "off"*/
const start = Date.now();
while (Date.now() - start < 8000) {

}

mm.restore();
console.log(`[${moment().format('YYYY-MM-DD HH:mm:ss')}]`, traceid, 'blocking done.');
process.exit(0);