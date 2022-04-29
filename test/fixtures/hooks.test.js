'use strict';

const fs = require('fs');
const promisify = require('util').promisify;
const readFile = promisify(fs.readFile);
const path = require('path');
const { profileRule: { diag }, checkProfile } = require('./command.test');

exports = module.exports = [
  {
    title: 'fatal error hook is valid',
    subTitle: 'x-fatal-error.diag is created when fatal error occured.',
    jspath: path.join(__dirname, 'fatal-error.js'),
    regexp: /x-fatal-error-(\d+)-(\d+)-(\d+).diag/,
    async check(file) {
      const content = (await readFile(file, 'utf8')).trim();
      console.log('fatal error report:', content);
      checkProfile(diag, JSON.parse(content));
    },
  },
  {
    title: 'fatal error hook is valid',
    subTitle: 'x-fatal-error.core is created when fatal error occured.',
    jspath: path.join(__dirname, 'fatal-error.js'),
    regexp: /x-fatal-error-(\d+)-(\d+)-(\d+).core/,
    async check(file) {
      const content = (await readFile(file, 'utf8')).trim();
      console.log('fatal error core:', content);
    },
    env: {
      XPROFILER_ENABLE_FATAL_ERROR_REPORT: 'NO',
      XPROFILER_ENABLE_FATAL_ERROR_COREDUMP: 'YES',
    }
  }
];