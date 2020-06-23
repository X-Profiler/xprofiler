'use strict';

const os = require('os');
const mm = require('mm');
const expect = require('expect.js');
const xprofiler = require('../xprofiler');
const utils = require('./fixtures/utils');

if (os.platform() === 'win32') {
  describe(`platform ${os.platform()}: check socket path illegal`, function () {

  });
} else {
  describe(`platform ${os.platform()}: check socket path illegal`, function () {
    const tmpLogdir = utils.createLogDir(`logdir_long_${new Array(100).fill('*').join('')}`);
    const coreMessage = 'socket path is too long, complete log of this error can be found in';
    let consoleError;
    before(function () {
      mm(process.env, 'XPROFILER_UNIT_TEST_SINGLE_MODULE', 'YES');
      mm(process.env, 'XPROFILER_LOG_DIR', tmpLogdir);
      mm(console, 'error', function (message) {
        consoleError = message;
      });
    });

    after(function () {
      mm.restore();
      utils.cleanDir(tmpLogdir);
    });

    it(`should throw socket check error: ${coreMessage}`, function () {
      let error;
      try {
        xprofiler.start();
      } catch (err) {
        error = err.message;
      }
      expect(error).to.be.ok();
      expect(consoleError).not.to.be.ok();
      expect(error.includes(coreMessage)).to.be.ok();
    });

    it(`should console check error: ${coreMessage}`, function () {
      let error;
      try {
        xprofiler.start({
          check_throw: false
        });
      } catch (err) {
        error = err.message;
      }
      expect(consoleError).to.be.ok();
      expect(error).not.to.be.ok();
      expect(consoleError.includes(coreMessage)).to.be.ok();
    });
  });
}