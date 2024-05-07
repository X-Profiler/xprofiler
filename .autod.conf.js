'use strict';

module.exports = {
  write: true,
  prefix: '^',
  test: [
    'test',
  ],
  dep: [
    'moment',
    'nan',
    '@mapbox/node-pre-gyp',
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
    'mm',
    'mocha',
    'nyc',
  ],
  exclude: [
    './build',
    './scripts',
    './test/fixtures',
    './demo.js',
    './nyc.config.js',
  ],
  semver: []
};
