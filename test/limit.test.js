'use strict';

const os = require('os');
const fs = require('fs');
const cp = require('child_process');
const path = require('path');
const expect = require('expect.js');
const promisify = require('util').promisify;
const readdir = promisify(fs.readdir);
const unlink = promisify(fs.unlink);
const utils = require('./fixtures/utils');
const cases = require('./fixtures/cases/limit')();

const currentPlatform = os.platform();

const logdir = utils.createLogDir('logdir_limit');

const casesLength = cases.length;

for (const cse of cases) {
  const ospt = cse.platform || currentPlatform;
  describe(`[${ospt}] ${cse.title}`, function () {
    const initialHeapLimit = 128;
    const autoIncreaseHeapLimitSize = 128;
    const MB = 1024 * 1024;

    let stdout = '';
    let subprocess = null;
    before(async function () {
      subprocess = cp.fork(cse.jspath, {
        execArgv: [`--max-old-space-size=${initialHeapLimit}`],
        env: Object.assign({}, process.env, {
          XPROFILER_LOG_DIR: logdir,
          XPROFILER_LOG_LEVEL: 2,
          XPROFILER_LOG_TYPE: 1,
          XPROFILER_ENABLE_AUTO_INCR_HEAP_LIMIT: 'YES',
          XPROFILER_AUTO_INCR_HEAP_LIMIT_SIZE: autoIncreaseHeapLimitSize,
          XPROFILER_FATAL_ERROR_INTERVAL: 500,
        }, cse.env),
        stdio: [0, 'pipe', 'pipe', 'ipc'],
      });
      subprocess.stdout.on('data', chunk => stdout += chunk.toString());
      await utils.sleep(5000);
      subprocess.kill();
      console.log('========= stdout =========\n\n', stdout, '\n========= end =========');
    });
    after(async function () {
      const files = await readdir(logdir);
      for (const file of files) {
        await unlink(path.join(logdir, file));
      }
      if (cse === cases[casesLength - 1]) {
        utils.cleanDir(logdir);
      }

      subprocess.kill();
    });

    for (let i = 1; i < 3; i++) {
      (cse.skip ? it.skip : it)(`${cse.subTitle} with ${i} times heap increase factor`, function () {
        const increaseLog = `current_heap_limit is ${(initialHeapLimit + (i - 1) * autoIncreaseHeapLimitSize) * MB}, `
          + `initial_heap_limit is ${initialHeapLimit * MB}, `
          + `auto_incr_heap_limit_size is ${autoIncreaseHeapLimitSize}, `
          + `increased_heap is ${initialHeapLimit * MB + i * autoIncreaseHeapLimitSize * MB}`;
        console.log('increaseLog:', increaseLog);
        expect(stdout).to.contain(increaseLog);
      });
    }
  });
}