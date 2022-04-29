'use strict';

const fs = require('fs');
const cp = require('child_process');
const path = require('path');
const expect = require('expect.js');
const promisify = require('util').promisify;
const readdir = promisify(fs.readdir);
const unlink = promisify(fs.unlink);
const exists = promisify(fs.exists);
const readFile = promisify(fs.readFile);
const stat = promisify(fs.stat);
const utils = require('./fixtures/utils');
const cases = require('./fixtures/hooks.test');

const logdir = utils.createLogDir('logdir_hooks');

const casesLength = cases.length;

for (const cse of cases) {
  describe(cse.title, function () {
    let hookFile = '';
    before(async function () {
      const p = cp.fork(cse.jspath, {
        execArgv: ['--max-old-space-size=128'],
        env: Object.assign({}, process.env, {
          XPROFILER_LOG_DIR: logdir,
          XPROFILER_LOG_LEVEL: 2,
          XPROFILER_LOG_TYPE: 1
        }, cse.env)
      });
      await new Promise(resolve => p.on('close', resolve));
      await utils.sleep(2000);
      const files = await readdir(logdir);
      for (const file of files) {
        if (cse.regexp.test(file)) {
          hookFile = path.join(logdir, file);
          const fileExists = await exists(hookFile);
          console.log('check hook file exists:', fileExists);
          if (!fileExists) {
            continue;
          }
          const { size } = await stat(hookFile);
          console.log('check hook file size:', size);
          if (!size > 0) {
            continue;
          }
          const fileContent = (await readFile(hookFile, 'utf8')).trim();
          console.log('check hook file content:', !!fileContent);
          if (!fileContent) {
            continue;
          }
          console.log('final hook file:', hookFile);
          break;
        }
      }
    });
    after(async function () {
      const files = await readdir(logdir);
      for (const file of files) {
        await unlink(path.join(logdir, file));
      }
      if (cse === cases[casesLength - 1]) {
        utils.cleanDir(logdir);
      }
    });

    it(cse.subTitle, function () {
      expect(hookFile).to.be.ok();
    });

    it('value should be ok', function () {
      describe(`it has expected structure`, async function () {
        if (typeof cse.check !== 'function') {
          return;
        }
        await cse.check(hookFile);
      });
    });
  });
}