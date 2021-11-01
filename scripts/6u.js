'use strict';

const build = require('./build');

const nodeVersions = [
  'node-v8.17.0',
  'node-v9.11.2',
  'node-v10.24.1',
  'node-v11.15.0',
];

build(nodeVersions);
