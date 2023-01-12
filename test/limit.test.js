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
const { parseLog } = require('./fixtures/cases/logbypass');
const limit = require('./fixtures/cases/limit');
const { increasedHeapLogStructure: struct } = limit;
const cases = limit();

const currentPlatform = os.platform();

const logdir = utils.createLogDir('logdir_limit');

const casesLength = cases.length;

for (const cse of cases) {
  const ospt = cse.platform || currentPlatform;
  describe(`[${ospt}] ${cse.title}`, function () {
    const initialHeapLimit = 128;
    const autoIncreaseHeapLimitSize = 128;
    const MB = 1024 * 1024;
    const component = 'heap_limit';
    let parsed;

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
          XPROFILER_FATAL_ERROR_INTERVAL: 250,
        }, cse.env),
        stdio: [0, 'pipe', 'pipe', 'ipc'],
      });
      subprocess.stdout.on('data', chunk => stdout += chunk.toString());
      await utils.sleep(10000);
      subprocess.kill();
      console.log('========= stdout =========\n\n', stdout, '\n========= end =========');
      parsed = parseLog(component, stdout, utils.xprofilerPrefixRegexp, false);
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
        const increaseLog = `current_heap_limit: ${(initialHeapLimit + (i - 1) * autoIncreaseHeapLimitSize) * MB}, `
          + `initial_heap_limit: ${initialHeapLimit * MB}, `
          + `auto_incr_heap_limit_size: ${autoIncreaseHeapLimitSize}, `
          + `increased_heap: ${initialHeapLimit * MB + i * autoIncreaseHeapLimitSize * MB}`;
        console.log('increaseLog:', increaseLog);
        expect(stdout).to.contain(increaseLog);
      });
    }

    (cse.skip ? it.skip : it)(`${cse.subTitle} should has comonent: ${component}`, function () {
      expect(parsed.prefix.component).to.be(component);
    });

    (cse.skip ? it.skip : it)(`component [${component}] should as expected`, function () {
      const detail = parsed.detail;
      describe(`${cse.subTitle} content should be ok`, function () {
        for (const key of Object.keys(detail)) {
          const key2 = utils.formatKey(key);
          const regexp = key2 !== key ? struct[key2].regexp : struct[key2];
          it(`${key}: ${detail[key]} shoule be ${regexp}`, function () {
            if (regexp instanceof RegExp) {
              expect(regexp.test(detail[key])).to.be.ok();
            }
            if (typeof regexp === 'function') {
              expect(regexp(detail[key])).to.be.ok();
            }
          });
        }
      });
    });
  });
}