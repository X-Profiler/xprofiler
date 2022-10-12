'use strict';

const os = require('os');
const fs = require('fs');
const path = require('path');
const cp = require('child_process');
const promisify = require('util').promisify;
const exec = promisify(cp.exec);
const mm = require('mm');
const expect = require('expect.js');
const moment = require('moment');
const xprofctl = path.join(__dirname, '../bin/xprofctl');
const xctl = require('../lib/xctl');
const utils = require('./fixtures/utils');

const currentPlatform = os.platform();
const commandTestFixture = require('./fixtures/command.test');
const { checkProfile } = commandTestFixture;
const logdir = utils.createLogDir('logdir_command');
const tmphome = utils.createLogDir('tmphome_command');
const testConfig = commandTestFixture(logdir);
const testFiles = [
  {
    jspath: path.join(__dirname, './fixtures/blocking.js'),
    desc: 'when js main thread blocking'
  },
  {
    jspath: path.join(__dirname, './fixtures/non-blocking.js'),
    desc: 'when js main thread non blocking'
  },
  {
    jspath: path.join(__dirname, './fixtures/worker_blocking.js'),
    desc: 'when js worker thread blocking',
    threadId: 1,
  },
];

function convertOptions(options) {
  let extra = '';
  for (const [key, value] of Object.entries(options)) {
    if (key.startsWith('enable_')) {
      extra += ` --${key.replace('enable_', 'disable_')}`;
    } else {
      extra += ` --${key} ${value}`;
    }
  }
  return extra;
}

describe('commands', () => {
  for (let i = 0; i < testConfig.length; i++) {
    const { cmd, options = {}, profileRules, errored = false, xctlRules, xprofctlRules, platform } = testConfig[i];
    for (let j = 0; j < testFiles.length; j++) {
      const { jspath, desc, threadId = 0 } = testFiles[j];
      const ospt = platform || currentPlatform;
      const title =
        `[${ospt}] execute [${cmd}] on thread(${threadId}) with options: ${JSON.stringify(options)} ${desc}`;
      describe(title, function () {
        const commandExpiredTime = 5000;
        let resByXctl = '';
        let resByXprofctl = '';
        let pid = 0;
        let exitInfo = { code: null, signal: null };
        before(async function () {
          mm(os, 'homedir', () => tmphome);
          mm(process.env, 'UNIT_TEST_COMMAND_EXPIRED_TIME', commandExpiredTime);
          console.log(`[${moment().format('YYYY-MM-DD HH:mm:ss')}]`, 'start fork.');
          const p = cp.fork(jspath, {
            env: Object.assign({}, process.env, {
              XPROFILER_LOG_DIR: logdir,
              XPROFILER_UNIT_TEST_TMP_HOMEDIR: tmphome,
              XPROFILER_LOG_LEVEL: 2,
              XPROFILER_LOG_TYPE: 1
            })
          });
          pid = p.pid;

          // wait for xprofiler to start
          await new Promise(resolve => p.on('message', msg =>
            msg.type === utils.clientConst.xprofilerDone && resolve()));

          // send cmd with xctl (function)
          console.log(`[${moment().format('YYYY-MM-DD HH:mm:ss')}]`, 'send xctl cmd.');
          resByXctl = await xctl(pid, threadId, cmd, options);
          await utils.sleep(500);

          // send cmd with xprofctl (cli)
          const extra = convertOptions(options);
          const nodeExe = currentPlatform === 'win32' ? 'node ' : '';
          const xprofctlCmd = `${nodeExe}${xprofctl} ${cmd} -p ${pid} -w ${threadId}${extra}`;
          console.log(`[${moment().format('YYYY-MM-DD HH:mm:ss')}]`, `send xprofctl cmd: ${xprofctlCmd}.`);
          resByXprofctl = await exec(xprofctlCmd, {
            env: Object.assign({}, process.env, {
              XPROFILER_UNIT_TEST_TMP_HOMEDIR: tmphome,
              UNIT_TEST_COMMAND_EXPIRED_TIME: commandExpiredTime,
            })
          });
          resByXprofctl = resByXprofctl.stderr.trim() + resByXprofctl.stdout.trim();

          // exit info
          console.log(`[${moment().format('YYYY-MM-DD HH:mm:ss')}]`, 'wait for child process done.');
          exitInfo = await utils.getChildProcessExitInfo(p);
        });

        after(function () {
          mm.restore();
          if (i === testConfig.length - 1 && j === testFiles.length - 1) {
            utils.cleanDir(logdir);
            utils.cleanDir(tmphome);
          }
        });

        it(`child process should be exited with code 0`, function () {
          console.log(`[${moment().format('YYYY-MM-DD HH:mm:ss')}]`, `exit info: ${JSON.stringify(exitInfo)}`);
          utils.checkChildProcessExitInfo(expect, exitInfo);
        });

        it(`response with xctl should be ok`, function () {
          console.log('xtcl:', JSON.stringify(resByXctl));
          expect(resByXctl).to.be.ok();
          expect(typeof resByXctl === 'object').to.be.ok();
          expect(resByXctl['traceid']).to.be.ok();
          if (errored) {
            expect(resByXctl['ok']).not.to.be.ok();
          } else {
            expect(resByXctl['ok']).to.be.ok();
          }
        });

        it(`response with xprofctl should be ok`, function () {
          console.log('xprofctl:', resByXprofctl);
          // expect(resByXprofctl).to.be.ok();
          expect(typeof resByXprofctl === 'string').to.be.ok();
        });

        it(`response value should be ok`, function () {
          describe(title, function () {
            const data = { pid, threadId, logdir };
            const rules = typeof xctlRules === 'function' ? xctlRules(data) : xctlRules;
            for (const rule of rules) {
              const value = utils.getNestingValue(resByXctl, rule.key);
              it(`response.${rule.key}: ${value} should be ${rule.rule.label || rule.rule}`, function () {
                expect(rule.rule.test(value)).to.be.ok();
              });

              // check dump file
              if (profileRules && typeof profileRules === 'object') {
                const profile = JSON.parse(fs.readFileSync(resByXctl.data.filepath, 'utf8'));
                checkProfile(profileRules, profile);
              }

              if (typeof profileRules === 'string') {
                const profile = fs.readFileSync(resByXctl.data.filepath, 'utf8').trim();
                it(`profile content should be ${profileRules}`, function () {
                  expect(profileRules).to.be(profile);
                });
              }

              if (typeof profileRules === 'function') {
                profileRules(resByXctl.data.filepath);
              }
            }

            for (const rule of xprofctlRules(data)) {
              const value = resByXprofctl;
              it(`${JSON.stringify(value)} should be ${rule}`, function () {
                expect(rule.test(value)).to.be.ok();
              });
            }
          });
        });
      });
    }
  }
});
