'use strict';

const assert = require('assert');
const path = require('path');
const { once, fork, createLogDir, cleanDir } = require('./fixtures/utils');
const xctl = require('../lib/xctl');
const { sleep } = require('../lib/utils');
const mm = require('mm');

let logdir;
let tmphome;

describe('worker_threads', () => {
  let proc;
  beforeEach(() => {
    mm(process.env, 'XPROFILER_UNIT_TEST_TMP_HOMEDIR', tmphome);
  });
  afterEach(() => {
    mm.restore();
    proc && proc.kill();
  });

  before(() => {
    logdir = createLogDir('logdir_worker');
    tmphome = createLogDir('tmphome_worker');
  });
  after(() => {
    cleanDir(logdir);
    cleanDir(tmphome);
  });

  describe('load', () => {
    it('should load xprofiler and exit cleanly', async () => {
      proc = fork(path.join(__dirname, 'fixtures/worker.js'), {
        env: Object.assign({}, process.env, {
          XPROFILER_LOG_DIR: logdir,
          XPROFILER_LOG_LEVEL: 2,
          XPROFILER_LOG_TYPE: 1,
        }),
      });
      const [code, signal] = await once(proc, 'exit');
      assert.strictEqual(code, 0);
      assert.strictEqual(signal, null);
    });
  });

  describe('xctl', () => {
    it('list_environments', async () => {
      proc = fork(path.join(__dirname, 'fixtures/worker_indefinite.js'), {
        env: Object.assign({}, process.env, {
          XPROFILER_LOG_DIR: logdir,
          XPROFILER_LOG_LEVEL: 2,
          XPROFILER_LOG_TYPE: 1,
        }),
      });
      await sleep(2000);
      console.log('perform list_environments');
      const result = await xctl(proc.pid, 0, 'list_environments', {
        logdir,
      });
      console.log('xctl result:', JSON.stringify(result, null, 2));
      assert.strictEqual(typeof result.data, 'object');
      assert(Array.isArray(result.data.environments));
      assert.strictEqual(result.data.environments.length, 2);

      let includesWorker = false;
      for (const env of result.data.environments) {
        if (!env.is_main_thread) {
          includesWorker = true;
        }
        /** uptime(s) */
        assert(env.uptime < 5);
      }
      assert(includesWorker);

      process.kill(proc.pid);

      const [code, signal] = await once(proc, 'exit');
      if (process.platform === 'win32') {
        assert.strictEqual(code, 1);
      } else {
        assert.strictEqual(code, null);
        assert.strictEqual(signal, 'SIGTERM');
      }
    });
  });
});
