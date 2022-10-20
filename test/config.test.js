'use strict';

const fs = require('fs');
const mm = require('mm');
const expect = require('expect.js');
const xprofiler = require('../xprofiler');
const testXprofilerConfigKeys = require('./fixtures/cases/config');
const utils = require('./fixtures/utils');
const testKeys = Object.keys(testXprofilerConfigKeys);

function cleanDir(key, dir) {
  if ((key === 'log_dir' || key === 'XPROFILER_LOG_DIR') && fs.existsSync(dir)) {
    utils.cleanDir(dir);
  }
}


describe('xprofiler config', function () {
  const message = 'must run "require(\'xprofiler\')()" to set xprofiler config first!';
  let error;

  before(function () {
    mm(process.env, 'XPROFILER_UNIT_TEST_SINGLE_MODULE', 'YES');
  });

  after(function () {
    mm.restore();
  });

  it(`should throw error if not init config: ${message}`, function () {
    try {
      xprofiler.getXprofilerConfig();
    } catch (err) {
      expect(err.message).to.be(message);
      error = err.message;
    }
    expect(error).to.be.ok();
  });

  it('should be ok after init config', function () {
    let defaultConfig;
    let error;
    try {
      xprofiler();
      defaultConfig = xprofiler.getXprofilerConfig();
    } catch (err) {
      error = err;
    }
    expect(error).not.to.be.ok();

    describe('xprofiler config keys', function () {
      const configKeys = Object.keys(defaultConfig);
      it(`should have these keys: [${configKeys.join(', ')}]`, function () {
        expect(utils.arrayEqual(testKeys, configKeys)).to.be.ok();
      });
    });

    for (const testKey of testKeys) {
      describe(`xprofiler config.${testKey}`, function () {
        const defaultValue = defaultConfig[testKey];
        const envTestList = testXprofilerConfigKeys[testKey].env;
        const userTestList = testXprofilerConfigKeys[testKey].user;
        const assignRule = testXprofilerConfigKeys[testKey].rule;

        // test default value
        it(`default value should be ${defaultValue}`, function () {
          expect(testXprofilerConfigKeys[testKey].defaultValue).to.be(defaultValue);
        });

        // test env config
        for (const envTest of envTestList) {
          describe(`set env ${envTest.key}=${envTest.value}`, function () {
            let config;
            before(function () {
              mm(process.env, envTest.key, envTest.value);
              mm(process.env, 'XPROFILER_UNIT_TEST_SINGLE_MODULE', 'YES');
              xprofiler();
              config = xprofiler.getXprofilerConfig();
            });
            after(function () {
              mm.restore();
            });

            it(`config.${testKey} should be ${envTest.expected}`, function () {
              expect(config[testKey]).to.be(envTest.expected);
            });
          });
        }

        // test user config
        for (const userTest of userTestList) {
          const testValue = userTest.value;
          const userConfigValue = typeof testValue === 'string' ? `"${testValue}"` : testValue;
          describe(`set user config { ${userTest.key}: ${userConfigValue} }`,
            function () {
              before(function () {
                mm(process.env, 'XPROFILER_UNIT_TEST_SINGLE_MODULE', 'YES');
                xprofiler({ [userTest.key]: userTest.value });
              });

              after(function () {
                mm.restore();
              });

              it(`config.${testKey} should be ${userTest.expected}`, function () {
                const config = xprofiler.getXprofilerConfig();
                expect(config[testKey]).to.be(userTest.expected);
              });
            });
        }

        // test config assign rule
        const envDescription = `${assignRule.env.key}=${assignRule.env.value}`;
        const userAssignRuleValue = assignRule.user.value;
        const userConfigDescription = `{ ${assignRule.user.key}: ` +
          (typeof userAssignRuleValue === 'string' ? `"${userAssignRuleValue}"` : userAssignRuleValue) + ' }';
        describe(`both set env ${envDescription} and user config ${userConfigDescription}`, function () {
          let config;
          before(function () {
            mm(process.env, assignRule.env.key, assignRule.env.value);
            mm(process.env, 'XPROFILER_UNIT_TEST_SINGLE_MODULE', 'YES');
            xprofiler({ [assignRule.user.key]: assignRule.user.value });
            config = xprofiler.getXprofilerConfig();
          });
          after(function () {
            mm.restore();
            cleanDir(assignRule.env.key, assignRule.env.value);
            cleanDir(assignRule.user.key, assignRule.user.value);
          });

          it(`config.${testKey} shoule be ${assignRule.expected}`, function () {
            expect(config[testKey]).to.be(assignRule.expected);
          });
        });
      });
    }
  });
});