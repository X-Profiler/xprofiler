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
const utils = require('./fixtures/utils');
const { profileRule: { diag, heapsnapshot }, checkProfile } = require('./fixtures/command.test');

const logdir = utils.createLogDir('logdir_hooks');

const cases = [
  {
    title: 'fatal error hook is valid',
    subTitle: 'x-fatal-error.diag is created when fatal error occured.',
    jspath: path.join(__dirname, 'fixtures/fatal-error.js'),
    hookFileRegexp: /x-fatal-error-(\d+)-(\d+)-(\d+).diag/,
    profileRule: diag,
  },
  {
    title: 'oom error hook is valid',
    subTitle: 'x-oom-error.heapsnapshot is created when oom error occured.',
    jspath: path.join(__dirname, 'fixtures/oom-error.js'),
    hookFileRegexp: /x-oom-(\d+)-(\d+)-(\d+).heapsnapshot/,
    profileRule: heapsnapshot,
    env: { XPROFILER_ENABLE_OOM_HOOK: 'YES' }
  }];
const casesLength = cases.length;

for (const cse of cases) {
  describe(cse.title, function () {
    let hookFile = '';
    before(async function () {
      const p = cp.fork(cse.jspath, {
        execArgv: ['--max-old-space-size=64'],
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
        if (cse.hookFileRegexp.test(file)) {
          hookFile = path.join(logdir, file);
          const fileExists = await exists(hookFile);
          console.log('check hook file exists:', hookFile, fileExists);
          if (!fileExists) {
            continue;
          }
          const fileContent = (await readFile(hookFile, 'utf8')).trim();
          console.log('check hook file content:', hookFile, !!fileContent);
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
      describe(`it has expected structure`, function () {
        const content = fs.readFileSync(hookFile, 'utf8').trim();
        console.log('fatal error report:', content);
        checkProfile(cse.profileRule, JSON.parse(content));
      });
    });
  });
}