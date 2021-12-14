'use strict';

const build = require('./build');

const nodeVersions = [
  'node-v12.22.7',
  'node-v13.14.0',
  'node-v14.18.2',
  'node-v15.14.0',
  'node-v16.13.1',
  'node-v17.2.0',
];

build(nodeVersions);