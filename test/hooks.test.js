'use strict';

const fs = require('fs');
const cp = require('child_process');
const path = require('path');
const expect = require('expect.js');
const promisify = require('util').promisify;
const readdir = promisify(fs.readdir);
const unlink = promisify(fs.unlink);
const utils = require('./fixtures/utils');
const { profileRule: { diag }, checkProfile } = require('./fixtures/command.test');

const logdir = utils.createLogDir('logdir_hooks');

const cases = [{
  title: 'fatal error hook is valid',
  subTitle: 'x-fatal-error.diag is created when fatal error occured.',
  jspath: path.join(__dirname, 'fixtures/fatal-error.js')
}];
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
        })
      });
      await new Promise(resolve => p.on('close', resolve));
      const files = await readdir(logdir);
      for (const file of files) {
        if (/x-fatal-error-(\d+)-(\d+)-(\d+).diag/.test(file)) {
          hookFile = path.join(logdir, file);
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
      describe(`it has expected structure`, function () {
        const content = fs.readFileSync(hookFile, 'utf8').trim();
        console.log(content);
        checkProfile(diag, JSON.parse(content));
      });
    });
  });
}