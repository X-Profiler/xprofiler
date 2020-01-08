'use strict';

const os = require('os');
const fs = require('fs');
const path = require('path');
const cp = require('child_process');
const mm = require('mm');
const expect = require('expect.js');
const xprofiler = require('../xprofiler');
const utils = require('./fixtures/utils');
const SPLITTER = '\u0000';

const logdir = utils.createLogDir('logdir_start');
const tmphome = utils.createLogDir('tmphome_start');

describe(`xprofiler starting`, function () {
  const invaidPid = 25416;
  const xprofilerPath = path.join(tmphome, '.xprofiler');
  const checkTime = 2;
  before(async function () {
    mm(os, 'homedir', () => tmphome);
    mm(process.env, 'XPROFILER_LOG_DIR', logdir);
    mm(process.env, 'UNIT_TEST_CHECK_TIME', checkTime);
    fs.writeFileSync(xprofilerPath, [invaidPid, logdir].join(SPLITTER) + '\n');
    xprofiler({ log_dir: logdir });
    await utils.sleep((checkTime + 3) * 1000);
  });

  after(function () {
    mm.restore();
    utils.cleanDir(logdir);
    utils.cleanDir(tmphome);
  });

  it(`invalid process ${invaidPid} not exists in ~/.xprofier`, function () {
    const content = fs.readFileSync(xprofilerPath, 'utf8').trim();
    const aliveProcessInfo = content.split('\n').map(line => line.split(SPLITTER));
    expect(aliveProcessInfo.length).to.be(1);
    const aliveProcess = aliveProcessInfo[0];

    describe('.xprofiler info', function () {
      it(`.xprofiler data length should be 6`, function () {
        expect(aliveProcess.length).to.be(6);
      });

      it(`.xprofiler pid: ${Number(aliveProcess[0])} should be ${Number(process.pid)}`, function () {
        expect(Number(aliveProcess[0])).to.be(Number(process.pid));
      });

      it(`.xprofiler logdir: ${aliveProcess[1]} should be ${logdir}`, function () {
        expect(aliveProcess[1]).to.be(logdir);
      });

      it(`.xprofiler cwd: ${aliveProcess[2]} should be ${/^([.\w()/\\:-]+|)$/}`, function () {
        expect(/^([.\w()/\\:-]+|)$/.test(aliveProcess[2])).to.be.ok();
      });

      it(`.xprofiler executable: ${aliveProcess[3]} should be node-${process.version}`, function () {
        let nodeExecutable = aliveProcess[3];
        if (os.platform() === 'win32') {
          nodeExecutable = JSON.stringify(nodeExecutable);
        }
        const version = cp.execSync(`${nodeExecutable} -v`).toString().trim();
        expect(version).to.be(process.version);
      });

      it(`.xprofiler file: ${aliveProcess[4]} should be ${/^([.\w()/\\:-]+|)$/}`, function () {
        expect(/^([.\w()/\\:-]+|)$/.test(aliveProcess[4])).to.be.ok();
      });

      it(`.xprofiler module path: ${aliveProcess[5]} should be ${path.join(__dirname, '..')}`, function () {
        expect(aliveProcess[5]).to.be(path.join(__dirname, '..'));
      });
    });
  });
});