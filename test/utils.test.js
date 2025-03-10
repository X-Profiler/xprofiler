'use strict';

const utils = require('../lib/utils');
const assert = require('assert');
const mm = require('mm');
const os = require('os');

describe('utils.test.js', function () {
  afterEach(mm.restore);

  describe('get prefix dir', () => {
    it('should work with XPROFILER_PREFIX', () => {
      mm(process.env, 'XPROFILER_PREFIX', '/tmp');
      const p = utils.getXprofilerPath();
      if (os.platform() === 'win32') {
        assert.equal(p, '\\tmp\\.xprofiler');
      } else {
        assert.equal(p, '/tmp/.xprofiler');
      }
    });

    it('should work default is home', () => {
      mm(os, 'homedir', () => {
        return '/home/xxx';
      });
      const p = utils.getXprofilerPath();
      if (os.platform() === 'win32') {
        assert.equal(p, '\\home\\xxx\\.xprofiler');
      } else {
        assert.equal(p, '/home/xxx/.xprofiler');
      }
    });
  });
});
