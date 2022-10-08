'use strict';

const os = require('os');
const fs = require('fs');
const path = require('path');
const cp = require('child_process');
const mm = require('mm');
const expect = require('expect.js');
const utils = require('./fixtures/utils');
const xctl = require('../lib/xctl');

const logdir = utils.createLogDir('logdir_sampling');
const tmphome = utils.createLogDir('tmphome_sampling');

describe(`unfinished sampling before process exit`, function () {
  describe('fatal error / oom', function () {
    let resByXctl = { ok: false };

    before(async function () {
      mm(os, 'homedir', () => tmphome);

      const p = cp.fork(path.join(__dirname, 'fixtures/fatal-error.js'), {
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
      resByXctl = await xctl(pid, 0, 'start_cpu_profiling', { profiling_time: 5 * 60 * 1000 });

      // process exit
      await new Promise(resolve => p.on('close', resolve));
      await utils.sleep(2000);
    });

    after(function () {
      mm.restore();
      utils.cleanDir(logdir);
      utils.cleanDir(tmphome);
    });

    it(`sampling file should be exit`, function () {
      console.log('xtcl:', JSON.stringify(resByXctl));
      expect(resByXctl.ok);
      const filepath = resByXctl.data.filepath;
      expect(filepath);
      console.log(`file ${filepath} exits: ${fs.existsSync(filepath)}.`);
    });
  });
});