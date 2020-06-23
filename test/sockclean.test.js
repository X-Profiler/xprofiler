'use strict';

const os = require('os');
const fs = require('fs');
const path = require('path');
const mm = require('mm');
const expect = require('expect.js');
const xprofiler = require('../xprofiler');
const utils = require('./fixtures/utils');

if (os.platform() === 'win32') {
  describe(`platform ${os.platform()}: clean not alive domain sock file`, function () {
    it(`alive sock should not be clean`, function () {
      expect(true).to.be.ok();
    });

    it(`not alive sock should be clean`, function () {
      expect(true).to.be.ok();
    });
  });
} else {
  describe(`platform ${os.platform()}: clean not alive domain sock file`, function () {
    const tmpLogdir = utils.createLogDir('logdir_clean');
    const aliveSock = path.join(tmpLogdir, `xprofiler-uds-path-${process.pid}.sock`);
    const notAliveSock = path.join(tmpLogdir, `xprofiler-uds-path-99999999.sock`);
    before(function () {
      mm(process.env, 'XPROFILER_UNIT_TEST_SINGLE_MODULE', 'YES');
      mm(process.env, 'XPROFILER_LOG_DIR', tmpLogdir);
      fs.writeFileSync(aliveSock, 'alive');
      fs.writeFileSync(notAliveSock, 'not alive');
      xprofiler();
    });

    after(function () {
      mm.restore();
      utils.cleanDir(tmpLogdir);
    });

    it(`alive sock should not be clean: ${aliveSock}`, function () {
      expect(fs.existsSync(aliveSock)).to.be.ok();
    });

    it(`not alive sock should be clean: ${notAliveSock}`, function () {
      expect(fs.existsSync(notAliveSock)).not.to.be.ok();
    });
  });
}