'use strict';

const os = require('os');
const fs = require('fs');
const path = require('path');
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
    const content = fs.readFileSync(xprofilerPath, 'utf8');
    const alivePids = content.split('\n').map(line => line.split(SPLITTER)[0]);
    expect(alivePids.length).to.be(1);
    expect(Number(alivePids[0])).to.be(Number(process.pid));
  });
});