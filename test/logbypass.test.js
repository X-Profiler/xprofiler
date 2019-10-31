'use strict';

const fs = require('fs');
const cp = require('child_process');
const expect = require('expect.js');
const pack = require('../package.json');
const utils = require('./fixtures/utils');
const getTestCases = require('./fixtures/logbypass.test');

const logdir = utils.createLogDir('logbypass');

const cases = getTestCases('performance log correctly', logdir);

function parseLog(logType, content, patt, alinode) {
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
      before(async function () {
        const p = cp.fork(target.file, { env: testCase.env });
        pid = p.pid;
        await new Promise(resolve => p.on('close', resolve));
        logContent = fs.readFileSync(testCase.logfile, 'utf8');
      });

      after(function () {
        fs.unlinkSync(testCase.logfile);
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

          it(`type [${type}] should as expected`, function () {
            const detail = parsed.detail;
            const struct = testCase.struct[type];
            expect(utils.objKeyEqual(detail, struct)).to.be.ok();
            describe(`${testCase.title} ${target.title}: ${type}`, function () {
              for (const key of Object.keys(detail)) {
                it(`${key}: ${detail[key]} shoule be ${struct[key]}`, function () {
                  expect(struct[key].test(detail[key])).to.be.ok();
                });
              }
            });
          });
        });
      }
    });
  }
}