'use strict';

const os = require('os');
const path = require('path');
const cp = require('child_process');
const promisify = require('util').promisify;
const exec = promisify(cp.exec);
const mm = require('mm');
const expect = require('expect.js');
const xprofctl = path.join(__dirname, '../bin/xprofctl');
const xctl = require('../lib/xctl');
const utils = require('./fixtures/utils');

const logdir = utils.createLogDir('logdir_command');
const tmphome = utils.createLogDir('tmphome_command');
const testConfig = require('./fixtures/command.test')(logdir);
const testFiles = [
  { jspath: path.join(__dirname, './fixtures/blocking.js'), desc: 'when js main thread blocking' },
  { jspath: path.join(__dirname, './fixtures/non-blocking.js'), desc: 'when js main thread non blocking' }
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

for (let i = 0; i < testConfig.length; i++) {
  const { cmd, options = {}, errored = false, xctlRules, xprofctlRules } = testConfig[i];
  for (let j = 0; j < testFiles.length; j++) {
    const { jspath, desc } = testFiles[j];
    const title = `execute [${cmd}] with options: ${JSON.stringify(options)} ${desc}`;
    describe(title, function () {
      let resByXctl = '';
      let resByXprofctl = '';
      let pid = 0;
      before(async function () {
        mm(os, 'homedir', () => tmphome);
        const p = cp.fork(jspath, {
          env: Object.assign({}, process.env, {
            XPROFILER_LOG_DIR: logdir,
            XPROFILER_UNIT_TEST_TMP_HOMEDIR: tmphome
          })
        });
        pid = p.pid;
        await utils.sleep(1000);
        // send cmd with xctl (function)
        resByXctl = await xctl(pid, cmd, options);
        // send cmd with xprofctl (cli)
        const extra = convertOptions(options);
        const nodeExe = os.platform() === 'win32' ? 'node ' : '';
        resByXprofctl = await exec(`${nodeExe}${xprofctl} ${cmd} -p ${pid}${extra}`, {
          env: Object.assign({}, process.env, {
            XPROFILER_UNIT_TEST_TMP_HOMEDIR: tmphome
          })
        });
        if (errored) {
          resByXprofctl = resByXprofctl.stderr.trim();
        } else {
          resByXprofctl = resByXprofctl.stdout.trim();
        }
        await new Promise(resolve => p.on('close', resolve));
      });

      after(function () {
        mm.restore();
        if (i === testConfig.length - 1 && j === testFiles.length - 1) {
          utils.cleanDir(logdir);
          utils.cleanDir(tmphome);
        }
      });

      it(`response with xctl should be ok`, function () {
        console.log('xtcl:', JSON.stringify(resByXctl));
        expect(resByXctl).to.be.ok();
        expect(typeof resByXctl === 'object').to.be.ok();
        if (errored) {
          expect(resByXctl['ok']).not.to.be.ok();
        } else {
          expect(resByXctl['ok']).to.be.ok();
        }
      });

      it(`response with xprofctl should be ok`, function () {
        console.log('xprofctl:', resByXprofctl);
        expect(resByXprofctl).to.be.ok();
        expect(typeof resByXprofctl === 'string').to.be.ok();
      });

      it(`response value should be ok`, function () {
        describe(title, function () {
          for (const rule of xctlRules) {
            const value = utils.getNestingValue(resByXctl, rule.key);
            it(`response.${rule.key}: ${value} should be ${rule.rule.label || rule.rule}`, function () {
              expect(rule.rule.test(value)).to.be.ok();
            });
          }

          const data = { pid };
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