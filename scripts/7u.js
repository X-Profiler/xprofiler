'use strict';

const build = require('./build');

const nodeVersions = [
  'node-v12.22.11',
  'node-v13.14.0',
  'node-v14.19.1',
  'node-v15.14.0',
  'node-v16.14.2',
  'node-v17.7.2',
];

build(nodeVersions);