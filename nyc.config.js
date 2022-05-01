'use strict';

const { testWorkerThreads } = require('./test/fixtures/utils');
const defaultExclude = require('@istanbuljs/schema/default-exclude');
const os = require('os');

const ingores = [];
if (os.platform() === 'win32') {
  ingores.push('lib/clean.js');
}

if (!testWorkerThreads()) {
  ingores.push('lib/worker_threads.js');
}

module.exports = {
  exclude: ingores.concat(defaultExclude)
};