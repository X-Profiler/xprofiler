'use strict';

const fs = require('fs');
const path = require('path');
const expect = require('expect.js');
const moment = require('moment');
const xprofiler = require('../xprofiler');
const pack = require('../package.json');

// clean logdir
const log_dir = path.join(__dirname, 'logdir');
if (!fs.existsSync(log_dir)) {
  fs.mkdirSync(log_dir, { recursive: true });
} else {
  for (const file of fs.readdirSync(log_dir)) {
    fs.unlinkSync(path.join(log_dir, file));
  }
}

const date = moment().format('YYYYMMDD');
// xprofiler file
const xprofilerLogPath = path.join(log_dir, `xprofiler-${date}.log`);
const xprofilerErrorLogPath = path.join(log_dir, `xprofiler-error-${date}.log`);
const xprofilerDebugLogPath = path.join(log_dir, `xprofiler-debug-${date}.log`);
// alinode file
const alinodeLogPath = path.join(log_dir, `node-${date}.log`);
const alinodeErrorLogPath = path.join(log_dir, `node-error-${date}.log`);
const alinodeDebugLogPath = path.join(log_dir, `node-debug-${date}.log`);

const xprofilerPrefixRegexp =
  /\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\] \[(\d{1,3}\.\d{1,3}\.\d{1,3})\] \[(.+)\] \[(.+)\] \[(\d+)\] (.*)/g;
const alinodePrefixRegexp =
  /\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{0,6}\] \[(.+)\] \[(.+)\] \[(\d+)\] (.*)/g;

function parseXprofilerLog(type, content) {
  let matched;
  const parsed = [];
  let regexp = xprofilerPrefixRegexp;
  if (type === 'alinode') {
    regexp = alinodePrefixRegexp;
  }
  while ((matched = regexp.exec(content)) !== null) {
    if (type === 'alinode') {
      parsed.push({
        log_level: matched[1],
        log_type: matched[2],
        pid: matched[3],
        detail: matched[4]
      });
    } else {
      parsed.push({
        version: matched[1],
        log_level: matched[2],
        log_type: matched[3],
        pid: matched[4],
        detail: matched[5]
      });
    }
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
      xprofiler(testConfig.config);
      for (const log of testConfig.logs) {
        xprofiler[log.level](log.type, log.content);
      }
    });

    after(function () {
      for (const log of testConfig.logs) {
        fs.unlinkSync(log.path);
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