'use strict';

const os = require('os');
const cp = require('child_process');
const path = require('path');
const expect = require('expect.js');
const moment = require('moment');
const utils = require('./fixtures/utils');

const logdir = utils.createLogDir('logdir_mallopt');
const rssPath = path.join(__dirname, 'fixtures/scripts/rss.js');

(os.platform() === 'linux' ? describe : describe.skip)('avoid rss leak by mallopt', function () {
  let rssMap;
  let exitInfo = { code: null, signal: null };
  before(async function () {
    const p = cp.fork(rssPath, {
      env: Object.assign({
        XPROFILER_LOG_DIR: logdir,
        XPROFILER_LOG_LEVEL: 2,
        XPROFILER_LOG_TYPE: 1,
        XPROFILER_ENABLE_AVOID_RSS_LEAK: 'YES',
      })
    });
    p.on('message', msg => rssMap = msg);
    exitInfo = await utils.getChildProcessExitInfo(p);
  });

  after(function () {
    utils.cleanDir(logdir);
  });

  it('child process should be exited with code 0', function () {
    console.log(`[${moment().format('YYYY-MM-DD HH:mm:ss')}]`, `exit info: ${JSON.stringify(exitInfo)}`);
    utils.checkChildProcessExitInfo(expect, exitInfo);
  });

  it('should avoid rss leak', function () {
    console.log(`process rss map: ${JSON.stringify(rssMap)}`);
    const threshold = 200;
    const list = Object.keys(rssMap).map(key => ({
      key,
      value: rssMap[key]
    }));

    for (const pair of list) {
      describe(pair.key, function () {
        for (const item of pair.value) {
          it(`should ${item} < ${threshold} (MiB)`, function () {
            expect(Number(item) < threshold).to.be.ok();
          });
        }
      });
    }
  });
});