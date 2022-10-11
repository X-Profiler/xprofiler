'use strict';

const os = require('os');
const fs = require('fs');
const cp = require('child_process');
const mm = require('mm');
const expect = require('expect.js');
const utils = require('./fixtures/utils');
const xctl = require('../lib/xctl');
const cases = require('./fixtures/unfinished.test')();

const casesLength = cases.length;

const logdir = utils.createLogDir('logdir_sampling');
const tmphome = utils.createLogDir('tmphome_sampling');


describe('unfinished sampling before process exit', function () {
  for (const cse of cases) {
    describe(cse.title, function () {
      let resByXctl = { ok: false };

      before(async function () {
        mm(os, 'homedir', () => tmphome);
        const p = cp.fork(cse.jspath, {
          execArgv: ['--max-old-space-size=128'],
          env: Object.assign({}, process.env, {
            XPROFILER_LOG_DIR: logdir,
            XPROFILER_UNIT_TEST_TMP_HOMEDIR: tmphome,
            XPROFILER_LOG_LEVEL: 2,
            XPROFILER_LOG_TYPE: 1,
            XPROFILER_FATAL_ERROR_INTERVAL: 100,
          })
        });

        // wait for xprofiler to start
        await new Promise(resolve => p.on('message', msg =>
          msg.type === utils.clientConst.xprofilerDone && resolve()));

        // send cmd
        const pid = p.pid;
        resByXctl = await xctl(pid, cse.tid, cse.cmd, { profiling_time: 5 * 60 * 1000 });

        // process exit
        await new Promise(resolve => p.on('close', resolve));
        await utils.sleep(2000);
      });

      after(function () {
        if (cse === cases[casesLength - 1]) {
          mm.restore();
          utils.cleanDir(logdir);
          utils.cleanDir(tmphome);
        }
      });

      it(`sampling file should be exists when process exit`, function () {
        console.log('xtcl:', JSON.stringify(resByXctl));
        expect(resByXctl.ok);
        const filepath = resByXctl.data.filepath;
        expect(filepath);
        expect(fs.existsSync(filepath));
      });

      it('value should be ok', async function () {
        describe(`it has expected structure`, function () {
          if (typeof cse.check !== 'function') {
            return;
          }
          cse.check(resByXctl.data.filepath);
        });
      });
    });
  }
});