'use strict';

const build = require('./build');

const nodeVersions = [
  'node-v8.17.0',
  'node-v9.11.2',
  'node-v10.24.1',
  'node-v11.15.0',
  'node-v12.22.7',
  'node-v13.14.0',
  'node-v14.18.1',
  'node-v15.14.0',
  'node-v16.13.0',
  'node-v17.0.1',
];

build(nodeVersions);