'use strict';

const xprofiler = require('../../');
xprofiler();

if (process.env.TEST_START_XPROFILER_LOG_THREAD === 'YES') {
  xprofiler.runLogBypass();
  xprofiler.runLogBypass();
}

/*eslint no-empty: "off"*/
const start = Date.now();
while (Date.now() - start < 3500) {

}