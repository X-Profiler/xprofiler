'use strict';

const xprofiler = require('../../');
xprofiler();

// start log bypass
xprofiler.runLogBypass();
xprofiler.runLogBypass();

// start commands listener
xprofiler.runCommandsListener();
xprofiler.runCommandsListener();

setInterval(() => { }, 1000);

setTimeout(() => {
  process.exit(0);
}, 6000);