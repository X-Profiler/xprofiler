'use strict';

const defaultExclude = require('@istanbuljs/schema/default-exclude');
const os = require('os');

let platformExclude = [
  os.platform() === 'win32' ? 'lib/clean.js' : ''
];

module.exports = {
  exclude: platformExclude.concat(defaultExclude)
};