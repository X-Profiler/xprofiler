'use strict';

const os = require('os');
const path = require('path');
const cp = require('child_process');
const promisify = require('util').promisify;
const exec = promisify(cp.exec);
const mm = require('mm');
const expect = require('expect.js');
const pkg = require('../package.json');
const xprofctl = path.join(__dirname, '../bin/xprofctl');
const xctl = require('../lib/xctl');
const utils = require('./fixtures/utils');

const logdir = utils.createLogDir('logdir_command');
const tmphome = utils.createLogDir('tmphome_command');

const testFiles = [
  { jspath: path.join(__dirname, './fixtures/blocking.js'), desc: 'when js main thread blocking' },
  { jspath: path.join(__dirname, './fixtures/non-blocking.js'), desc: 'when js main thread non blocking' }
];

const testConfig = [
  {
    cmd: 'check_version',
    xctlRules: [{ key: 'data', rule: new RegExp(`^${pkg.version}$`) }],
    xprofctlRules: [/X-Profiler 插件版本号: v(\d{1,3}\.\d{1,3}\.\d{1,3})/]
  }
];

for (let i = 0; i < testConfig.length; i++) {
  const { cmd, options = {}, xctlRules, xprofctlRules } = testConfig[i];
  for (let j = 0; j < testFiles.length; j++) {
    const { jspath, desc } = testFiles[j];
    const title = `execute [${cmd}] with options: ${JSON.stringify(options)} ${desc}`;
    describe(title, function () {
      let resByXctl = '';
      let resByXprofctl = '';
      before(async function () {
        mm(os, 'homedir', () => tmphome);
        const p = cp.fork(jspath, {
          env: Object.assign({}, process.env, {
            XPROFILER_LOG_DIR: logdir,
            XPROFILER_UNIT_TEST_TMP_HOMEDIR: tmphome
          })
        });
        await utils.sleep(1000);
        // send cmd with xctl (function)
        resByXctl = await xctl(p.pid, cmd, options);
        // send cmd with xprofctl (cli)
        const extra = options.profilingTime ? ` -t ${options.profilingTime}` : '';
        const nodeExe = os.platform() === 'win32' ? 'node ' : '';
        resByXprofctl = await exec(`${nodeExe}${xprofctl} ${cmd} -p ${p.pid}${extra}`, {
          env: Object.assign({}, process.env, {
            XPROFILER_UNIT_TEST_TMP_HOMEDIR: tmphome
          })
        });
        resByXprofctl = resByXprofctl.stdout.trim();
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
        expect(resByXctl['ok']).to.be.ok();
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
            it(`response.${rule.key}: ${value} should be ${rule.rule}`, function () {
              expect(rule.rule.test(value)).to.be.ok();
            });
          }

          for (const rule of xprofctlRules) {
            const value = resByXprofctl;
            it(`${value} should be ${rule}`, function () {
              expect(rule.test(value)).to.be.ok();
            });
          }
        });
      });
    });
  }
}