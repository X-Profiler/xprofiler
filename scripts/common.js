'use strict';

const build = require('./build');

const nodeVersions = [
  'node-v8.17.0',
  'node-v9.11.2',
  'node-v10.22.0',
  'node-v11.15.0',
  'node-v12.18.3',
  'node-v13.14.0',
  'node-v14.6.0',
  'node-v15.1.0',
];

build(nodeVersions);