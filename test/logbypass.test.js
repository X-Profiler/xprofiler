'use strict';

const fs = require('fs');
const cp = require('child_process');
const expect = require('expect.js');
const moment = require('moment');
const pack = require('../package.json');
const utils = require('./fixtures/utils');
const getTestCases = require('./fixtures/logbypass.test');

const logdirBlocking = utils.createLogDir('log_bypass_blocking');
const logdirNonBlocking = utils.createLogDir('log_bypass_non_blocking');

// common cases
let cases = getTestCases('performance log correctly', logdirBlocking, logdirNonBlocking);

// libuv cases
const logdirBlockingForUv = utils.createLogDir('log_bypass_blocking_uv');
const logdirNonBlockingForUv = utils.createLogDir('log_bypass_non_blocking_uv');
const casesForLibuv = getTestCases('performance log correctly with XPROFILER_ENABLE_LOG_UV_HANDLES=NO',
  logdirBlockingForUv, logdirNonBlockingForUv, { XPROFILER_ENABLE_LOG_UV_HANDLES: 'NO' },
  { uv: getTestCases.getUvRules(['active_handles']) });

// http cases
const logdirBlockingForHttp = utils.createLogDir('log_bypass_blocking_http');
const logdirNonBlockingForHttp = utils.createLogDir('log_bypass_non_blocking_http');
const casesForHttp = getTestCases('performance log correctly  XPROFILER_PATCH_HTTP=NO',
  logdirBlockingForHttp, logdirNonBlockingForHttp, { XPROFILER_PATCH_HTTP: 'NO', XPROFILER_PATCH_HTTP_TIMEOUT: 30 },
  {
    http: {
      live_http_request: /^0$/,
      http_response_close: /^0$/,
      http_response_sent: /^0$/,
      http_request_timeout: /^0$/,
      http_patch_timeout: /^30$/,
      http_rt: /^0.00$/,
      res: { notRequired: true, regexp: /^\d+$/ }
    }
  });

// compose cases
cases = cases.concat(casesForLibuv).concat(casesForHttp);

function parseLog(logType, content, patt, alinode) {
  console.log(`parse log ${logType}: ${JSON.stringify(content)}`);
  const reg = /([^\s]*): (\d+(\.\d{0,2})?)/g;
  let matched;
  const res = { prefix: {}, detail: {} };
  while ((matched = patt.exec(content)) !== null) {
    if (!matched || matched[2] !== logType) {
      continue;
    }

    // set prefix;
    res.prefix.level = matched[1];
    res.prefix.type = matched[2];
    res.prefix.pid = Number(matched[3]);
    let detail;
    if (alinode) {
      detail = matched[4];
    } else {
      res.prefix.version = matched[4];
      detail = matched[5];
    }

    // set detail
    let pair;
    while ((pair = reg.exec(detail)) !== null) {
      res.detail[pair[1]] = pair[2];
    }
  }
  return res;
}

for (const testCase of cases) {
  for (const target of testCase.targets) {
    describe(`${testCase.title} ${target.title}`, function () {
      /*eslint no-loop-func: "off" */
      let logContent;
      let pid;
      let exitInfo = { code: null, signal: null };
      before(async function () {
        const p = cp.fork(target.file, { env: Object.assign({ XPROFILER_LOG_TYPE: 1 }, testCase.env, target.env) });
        pid = p.pid;
        exitInfo = await utils.getChildProcessExitInfo(p);
        logContent = fs.readFileSync(target.logfile, 'utf8');
      });

      after(function () {
        fs.unlinkSync(target.logfile);
        if (cases.indexOf(testCase) === cases.length - 1) {
          if (testCase.targets.indexOf(target) === testCase.targets.length - 1) {
            utils.cleanDir(logdirBlocking);
            utils.cleanDir(logdirNonBlocking);
            utils.cleanDir(logdirBlockingForUv);
            utils.cleanDir(logdirNonBlockingForUv);
            utils.cleanDir(logdirBlockingForHttp);
            utils.cleanDir(logdirNonBlockingForHttp);
          }
        }
      });

      it(`child process should be exited with code 0`, function () {
        console.log(`[${moment().format('YYYY-MM-DD HH:mm:ss')}]`, `exit info: ${JSON.stringify(exitInfo)}`);
        utils.checkChildProcessExitInfo(expect, exitInfo);
      });

      const types = Object.keys(testCase.struct);
      for (const type of types) {
        describe(`parse log type [${type}] with content`, function () {
          let parsed;
          before(function () {
            parsed = parseLog(type, logContent, testCase.logparse, testCase.alinode);
          });

          it(`log prefix shoule be ok`, function () {
            const prefix = parsed.prefix;
            expect(/^info$|^error$|^debug$/.test(prefix.level)).to.be.ok();
            expect(new RegExp(`^${type}$`).test(prefix.type)).to.be.ok();
            expect(prefix.pid).to.be(pid);
            if (!testCase.alinode) {
              expect(prefix.version).to.be(pack.version);
            }
          });

          const struct = testCase.struct[type];
          it(`type [${type}] should have keys ${Object.keys(struct)}`, function () {
            const detail = parsed.detail;
            expect(utils.objKeyEqual(detail, struct)).to.be.ok();
          });

          it(`type [${type}] should as expected`, function () {
            const detail = parsed.detail;
            describe(`${testCase.title} ${target.title}: ${type}`, function () {
              for (const key of Object.keys(detail)) {
                const key2 = utils.formatKey(key);
                const regexp = key2 !== key ? struct[key2].regexp : struct[key2];
                it(`${key}: ${detail[key]} shoule be ${regexp}`, function () {
                  expect(regexp.test(detail[key])).to.be.ok();
                });
              }
            });
          });
        });
      }
    });
  }
}