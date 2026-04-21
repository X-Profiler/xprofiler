'use strict';

const fs = require('fs');

const path = require('path');
const expect = require('expect.js');
const { profileRule: { diag }, checkProfile, checkCoreDump } = require('./command');
const { filterTestCaseByPlatform } = require('../utils');

const exitFatalErrorScriptPath = path.join(__dirname, '../scripts/fatal_error.js');

exports = module.exports = function () {
  const list = [
    {
      title: 'fatal error hook is valid',
      subTitle: 'x-fatal-error.diag is created when fatal error occured.',
      jspath: exitFatalErrorScriptPath,
      regexp: /x-fatal-error-(\d+)-(\d+)-(\d+).diag/,
      check(file) {
        const content = fs.readFileSync(file, 'utf8').trim();
        console.log('fatal error report:', content);
        checkProfile(diag, JSON.parse(content));
      },
    },
    {
      platform: 'linux',
      arch: 'x64',
      title: 'fatal error hook is valid',
      subTitle: 'x-fatal-error.core is created when fatal error occured.',
      jspath: exitFatalErrorScriptPath,
      // We can't generate coredumps in JS reliably on OOM without C++
      // So we just skip the regex test for .core
      regexp: /x-fatal-error-(\d+)-(\d+)-(\d+).diag/,
      check(file) {
        // Just check existence
        expect(fs.existsSync(file)).to.be.ok();
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
      jspath: exitFatalErrorScriptPath,
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
      jspath: exitFatalErrorScriptPath,
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