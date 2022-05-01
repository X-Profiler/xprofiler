'use strict';

const fs = require('fs');

const path = require('path');
const expect = require('expect.js');
const { profileRule: { diag }, checkProfile, checkCoreDump } = require('./command.test');
const { filterTestCaseByPlatform } = require('./utils');

exports = module.exports = function () {
  const list = [
    {
      title: 'fatal error hook is valid',
      subTitle: 'x-fatal-error.diag is created when fatal error occured.',
      jspath: path.join(__dirname, 'fatal-error.js'),
      regexp: /x-fatal-error-(\d+)-(\d+)-(\d+).diag/,
      check(file) {
        const content = fs.readFileSync(file, 'utf8').trim();
        console.log('fatal error report:', content);
        checkProfile(diag, JSON.parse(content));
      },
    },
    {
      platform: 'linux',
      title: 'fatal error hook is valid',
      subTitle: 'x-fatal-error.core is created when fatal error occured.',
      jspath: path.join(__dirname, 'fatal-error.js'),
      regexp: /x-fatal-error-(\d+)-(\d+)-(\d+).core/,
      check(file) {
        checkCoreDump(file, 'fatal error core elf information');
      },
      env: {
        XPROFILER_ENABLE_FATAL_ERROR_REPORT: 'NO',
        XPROFILER_ENABLE_FATAL_ERROR_COREDUMP: 'YES',
      }
    },
    {
      platform: 'win32',
      title: 'fatal error hook is valid',
      subTitle: 'x-fatal-error.core is created when fatal error occured.',
      jspath: path.join(__dirname, 'fatal-error.js'),
      regexp: /x-fatal-error-(\d+)-(\d+)-(\d+).core/,
      check(file) {
        const content = fs.readFileSync(file, 'utf8').trim();
        console.log('fatal error core:', content);
        expect(content).to.be('Generator core file is not supported on windows now.');
      },
      env: {
        XPROFILER_ENABLE_FATAL_ERROR_REPORT: 'NO',
        XPROFILER_ENABLE_FATAL_ERROR_COREDUMP: 'YES',
      }
    },
    {
      platform: 'darwin',
      title: 'fatal error hook is valid',
      subTitle: 'x-fatal-error.core is created when fatal error occured.',
      jspath: path.join(__dirname, 'fatal-error.js'),
      regexp: /x-fatal-error-(\d+)-(\d+)-(\d+).core/,
      check(file) {
        const content = fs.readFileSync(file, 'utf8').trim();
        console.log('fatal error core:', content);
        expect(content).to.be('Generator core file is not supported on darwin now.');
      },
      env: {
        XPROFILER_ENABLE_FATAL_ERROR_REPORT: 'NO',
        XPROFILER_ENABLE_FATAL_ERROR_COREDUMP: 'YES',
      }
    }
  ];

  return filterTestCaseByPlatform(list);
};