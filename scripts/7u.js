'use strict';

const build = require('./build');

const nodeVersions = [
  'node-v12.22.12',
  'node-v13.14.0',
  'node-v14.19.2',
  'node-v15.14.0',
  'node-v16.15.0',
  'node-v17.9.0',
];

build(nodeVersions);