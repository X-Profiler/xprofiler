'use strict';

const os = require('os');

exports.os7u = [
  'node-v12.22.12',
  'node-v13.14.0',
  'node-v14.21.3',
  'node-v15.14.0',
  'node-v16.20.2',
  'node-v17.9.1',
];

if (os.platform() === 'darwin' && os.arch() === 'arm64') {
  exports.os7u = exports.os7u.slice(4);
}

exports.os8u = [
  'node-v18.17.1',
  'node-v19.9.0',
  'node-v20.6.0',
];
