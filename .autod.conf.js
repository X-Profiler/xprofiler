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
    '@xprofiler/node-pre-gyp',
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
    'nyc',
    'tunnel-agent',
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
