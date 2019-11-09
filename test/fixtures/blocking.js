'use strict';

const xprofiler = require('../../');

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