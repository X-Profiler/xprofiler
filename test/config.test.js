'use strict';

const os = require('os');
const path = require('path');
const mm = require('mm');
const expect = require('expect.js');
const xprofiler = require('../');

describe('xprofiler config', function () {
  const message = 'must run "require(\'xprofiler\')()" to set xprofiler config first!';
  let error;
  it(`should throw error not init config: ${message}`, function () {
    try {
      xprofiler.getXprofilerConfig();
    } catch (err) {
      expect(err.message).to.be(message);
      error = err.message;
    }
    expect(error).to.be.ok();
  });

  it('should be ok after init config', function () {
    describe('xprofiler default config', function () {
      xprofiler();
      const defaultConfig = xprofiler.getXprofilerConfig();
      it(`log_dir should be ${os.tmpdir()}`, function () {
        expect(defaultConfig.log_dir).to.be(os.tmpdir());
      });

      it('log_interval should be 60', function () {
        expect(defaultConfig.log_interval).to.be(60);
      });

      it('enable_log_uv_handles should be true', function () {
        expect(defaultConfig.enable_log_uv_handles).to.be(true);
      });
    });

    describe('xprofiler config with env', function () {
      const logDirEnv = path.join(__dirname, 'logdir_env');
      const logIntervalEnv = 30;
      const enableLogUvHandlesEnv = false;
      let config;

      before(function () {
        mm(process.env, 'XPROFILER_LOG_DIR', logDirEnv);
        mm(process.env, 'XPROFILER_LOG_INTERVAL', logIntervalEnv);
        mm(process.env, 'XPROFILER_ENABLE_LOG_UV_HANDLES', enableLogUvHandlesEnv);
        xprofiler();
        config = xprofiler.getXprofilerConfig();
      });

      after(function () {
        mm.restore();
      });

      it('config should be object', function () {
        expect(config).to.be.ok();
        expect(typeof config).to.be('object');
      });

      it(`log_dir should be ${logDirEnv}`, function () {
        expect(config.log_dir).to.be(logDirEnv);
      });

      it(`log_interval should be ${logIntervalEnv}`, function () {
        expect(config.log_interval).to.be(logIntervalEnv);
      });

      it(`enable_log_uv_handles should be ${enableLogUvHandlesEnv}`, function () {
        expect(config.enable_log_uv_handles).to.be(enableLogUvHandlesEnv);
      });
    });

    describe('xprofiler config with not absolute log dir by env', function () {
      let config;

      before(function () {
        mm(process.env, 'XPROFILER_LOG_DIR', 'env/not/absolute/path');
        xprofiler();
        config = xprofiler.getXprofilerConfig();
      });

      after(function () {
        mm.restore();
      });

      it(`log_dir should be ${os.tmpdir()}`, function () {
        expect(config.log_dir).to.be(os.tmpdir());
        mm.restore();
        xprofiler({
          log_dir: 'user/not/absolute/path'
        });
        config = xprofiler.getXprofilerConfig();
        expect(config.log_dir).to.be(os.tmpdir());
      });
    });

    describe('xprofiler config with user', function () {
      const logDirUser = path.join(__dirname, 'logdir_user');
      const logIntervalUser = 66;
      const enableLogUvHandlesUser = false;
      xprofiler({
        log_dir: logDirUser,
        log_interval: logIntervalUser,
        enable_log_uv_handles: enableLogUvHandlesUser
      });
      const config = xprofiler.getXprofilerConfig();

      it('config should be object', function () {
        expect(config).to.be.ok();
        expect(typeof config).to.be('object');
      });

      it(`log_dir should be ${logDirUser}`, function () {
        expect(config.log_dir).to.be(logDirUser);
      });

      it(`log_interval should be ${logIntervalUser}`, function () {
        expect(config.log_interval).to.be(logIntervalUser);
      });

      it(`enable_log_uv_handles should be ${enableLogUvHandlesUser}`, function () {
        expect(config.enable_log_uv_handles).to.be(enableLogUvHandlesUser);
      });
    });

    describe('xprofiler config with both env and user', function () {
      const logDirEnv = path.join(__dirname, 'logdir_env');
      const logIntervalEnv = 30;
      const enableLogUvHandlesEnv = true;

      const logDirUser = path.join(__dirname, 'logdir_user');
      const logIntervalUser = 66;
      const enableLogUvHandlesUser = false;

      let config;

      before(function () {
        mm(process.env, 'XPROFILER_LOG_DIR', logDirEnv);
        mm(process.env, 'XPROFILER_LOG_INTERVAL', logIntervalEnv);
        mm(process.env, 'XPROFILER_ENABLE_LOG_UV_HANDLES', enableLogUvHandlesEnv);
        xprofiler({
          log_dir: logDirUser,
          log_interval: logIntervalUser,
          enable_log_uv_handles: enableLogUvHandlesUser
        });
        config = xprofiler.getXprofilerConfig();
      });

      after(function () {
        mm.restore();
      });

      it('config should be object', function () {
        expect(config).to.be.ok();
        expect(typeof config).to.be('object');
      });

      it(`log_dir should be ${logDirUser} setting by user`, function () {
        expect(config.log_dir).to.be(logDirUser);
      });

      it(`log_interval should be ${logIntervalUser} setting by user`, function () {
        expect(config.log_interval).to.be(logIntervalUser);
      });

      it(`enable_log_uv_handles should be ${enableLogUvHandlesUser} setting by user`, function () {
        expect(config.enable_log_uv_handles).to.be(enableLogUvHandlesUser);
      });
    });
  });
});