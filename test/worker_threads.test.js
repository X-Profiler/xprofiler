'use strict';

const assert = require('assert');
const path = require('path');
const { once, fork } = require('./fixtures/utils');

describe('worker_threads', () => {
  describe('load', () => {
    it('should load xprofiler and exit cleanly', async () => {
      const proc = fork(path.join(__dirname, 'fixtures/worker.js'));
      const [code, signal] = await once(proc, 'exit');
      assert.strictEqual(code, 0);
      assert.strictEqual(signal, null);
    });
  });
});
