'use strict';

const build = require('./build');

const nodeVersions = [
  'node-v8.17.0',
  'node-v9.11.2',
  'node-v10.19.0',
  'node-v11.15.0',
  'node-v12.18.0',
  'node-v13.14.0',
  'node-v14.4.0'
];

build(nodeVersions);