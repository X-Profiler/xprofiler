'use strict';

const build = require('./build');

const nodeVersions = [
  'node-v12.18.3',
  'node-v13.14.0',
  'node-v14.6.0',
];

build(nodeVersions);