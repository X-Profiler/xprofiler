'use strict';

const fs = require('fs');
const path = require('path');
const mm = require('mm');
const expect = require('expect.js');
const moment = require('moment');
const xprofiler = require('../xprofiler');
const pack = require('../package.json');
const utils = require('./fixtures/utils');

// clean logdir
const log_dir = utils.createLogDir('logdir');

const date = moment().format('YYYYMMDD');
// xprofiler file
const xprofilerLogPath = path.join(log_dir, `xprofiler-${date}.log`);
const xprofilerErrorLogPath = path.join(log_dir, `xprofiler-error-${date}.log`);
const xprofilerDebugLogPath = path.join(log_dir, `xprofiler-debug-${date}.log`);
// alinode file
const alinodeLogPath = path.join(log_dir, `node-${date}.log`);
const alinodeErrorLogPath = path.join(log_dir, `node-error-${date}.log`);
const alinodeDebugLogPath = path.join(log_dir, `node-debug-${date}.log`);

const xprofilerPrefixRegexp = utils.xprofilerPrefixRegexp;
const alinodePrefixRegexp = utils.alinodePrefixRegexp;

function parseXprofilerLog(type, content) {
  let matched;
  const parsed = [];
  let regexp = xprofilerPrefixRegexp;
  if (type === 'alinode') {
    regexp = alinodePrefixRegexp;
  }
  while ((matched = regexp.exec(content)) !== null) {
    const obj = {
      log_level: matched[1],
      log_type: matched[2],
      pid: matched[3]
    };
    if (type === 'alinode') {
      obj.detail = matched[4];
    } else {
      obj.tid = matched[4];
      obj.version = matched[5];
      obj.detail = matched[6];
    }
    parsed.push(obj);
  }
  return parsed;
}

const testConfigList = [
  {
    type: 'xprofiler',
    config: { log_dir, log_level: 2 },
    logs: [
      { level: 'info', type: 'log.test', content: 'test info log', path: xprofilerLogPath },
      { level: 'error', type: 'log.test', content: 'test error log', path: xprofilerErrorLogPath },
      { level: 'debug', type: 'log.test', content: 'test debug log', path: xprofilerDebugLogPath }
    ]
  },
  {
    type: 'alinode',
    config: { log_dir, log_level: 2, log_format_alinode: true },
    logs: [
      { level: 'info', type: 'log.test', content: 'test info log', path: alinodeLogPath },
      { level: 'error', type: 'log.test', content: 'test error log', path: alinodeErrorLogPath },
      { level: 'debug', type: 'log.test', content: 'test debug log', path: alinodeDebugLogPath }
    ]
  }
];

for (const testConfig of testConfigList) {
  describe(`${testConfig.type} log`, function () {
    before(function () {
      mm(process.env, 'XPROFILER_UNIT_TEST_SINGLE_MODULE', 'YES');
      xprofiler(testConfig.config);
      for (const log of testConfig.logs) {
        xprofiler[log.level](log.type, log.content);
      }
    });

    after(function () {
      if (testConfigList.indexOf(testConfig) === testConfigList.length - 1) {
        utils.cleanDir(log_dir);
      }
    });

    for (const log of testConfig.logs) {
      it(`<${log.path}> should exists`, function () {
        expect(fs.existsSync(log.path)).to.be.ok();
        const parsed = parseXprofilerLog(testConfig.type, fs.readFileSync(log.path, 'utf8'));
        describe(`${testConfig.type} ${log.level} log parsed`, function () {
          it(`${log.level} log should be parsed ok`, function () {
            expect(parsed.length).to.be.ok();
          });
          for (const d of parsed) {
            if (testConfig.type !== 'alinode') {
              it(`version should be v${pack.version}`, function () {
                expect(d.version).to.be(pack.version);
              });
            }

            it(`log_level should be "${log.level}"`, function () {
              expect(d.log_level).to.be(log.level);
            });

            it(`log_type should be "${log.type}"`, function () {
              expect(d.log_type).to.be(log.type);
            });

            it(`pid should be ${process.pid}`, function () {
              expect(Number(d.pid)).to.be(process.pid);
            });

            it(`content should be "${log.content}"`, function () {
              expect(d.detail).to.be(log.content);
            });
          }
        });
      });
    }
  });
}
