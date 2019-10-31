'use strict';

const xprofiler = require('../../');
xprofiler();

if (process.env.TEST_START_XPROFILER_LOG_THREAD === 'YES') {
  xprofiler.runLogBypass();
  xprofiler.runLogBypass();
}

setInterval(() => { }, 1000);

setTimeout(() => {
  process.exit(0);
}, 5000);