'use strict';

module.exports = {
  write: true,
  prefix: '^',
  test: [
    'test',
  ],
  dep: [
    'bindings',
    'nan',
    'node-gyp',
    'uuid',
    'yargs',
  ],
  devdep: [
    '@istanbuljs/schema',
    'autod',
    'clang-format',
    'codecov',
    'eslint',
    'expect.js',
    'formstream',
    'mm',
    'mocha',
    'moment',
    'nyc',
    'tunnel-agent',
    'urllib',
  ],
  exclude: [
    './build',
    './scripts',
    './test/fixtures',
    './demo.js',
    './nyc.config.js',
  ],
  semver: [
    'eslint@6',
    'node-gyp@6',
  ]
};
