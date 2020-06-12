'use strict';

const build = require('./build');

const nodeVersions = [
  'node-v12.18.0',
  'node-v13.14.0',
  'node-v14.4.0',
];

build(nodeVersions);