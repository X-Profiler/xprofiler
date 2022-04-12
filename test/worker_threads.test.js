'use strict';

const assert = require('assert');
const path = require('path');
const { once, fork } = require('./fixtures/utils');
const { canIUseWorker } = require('../lib/worker_threads');
const xctl = require('../lib/xctl');
const { sleep } = require('../lib/utils');
const utils = require('./fixtures/utils');
const mm = require('mm');

const logdir = utils.createLogDir('logdir_worker');
const tmphome = utils.createLogDir('tmphome_worker');

(canIUseWorker ? describe : describe.skip)('worker_threads', () => {
  afterEach(() => {
    mm.restore();
  });
  before(() => {
    mm(process.env, 'XPROFILER_UNIT_TEST_TMP_HOMEDIR', tmphome);
  });

  describe('load', () => {
    it('should load xprofiler and exit cleanly', async () => {
      const proc = fork(path.join(__dirname, 'fixtures/worker.js'), {
        env: Object.assign({}, process.env, {
          XPROFILER_LOG_DIR: logdir,
        }),
      });
      const [code, signal] = await once(proc, 'exit');
      assert.strictEqual(code, 0);
      assert.strictEqual(signal, null);
    });
  });

  describe('xcntl', () => {
    it('list_environments', async () => {
      const proc = fork(path.join(__dirname, 'fixtures/worker_indefinite.js'), {
        env: Object.assign({}, process.env, {
          XPROFILER_LOG_DIR: logdir,
        }),
      });
      await sleep(1000);
      const result = await xctl(proc.pid, 'list_environments', {
        logdir,
      });
      assert(Array.isArray(result.data.environments));
      assert.strictEqual(result.data.environments.length, 2);

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
