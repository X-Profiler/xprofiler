'use strict';

const expect = require('expect.js');
const { wrap, unwrap } = require('../../patch/shimmer');

describe('wrap / unwrap module', function () {
  it('wrap module failed with wrong params', function () {
    expect(wrap()).to.be(undefined);

    const mod = { test: 'value' };
    expect(wrap(mod, 'not_exists')).to.be(undefined);
    expect(wrap(mod, 'test')).to.be(undefined);
    expect(wrap(mod, 'test', () => { })).to.be(undefined);

    const mod2 = { test() { } };
    expect(wrap(mod2, 'test', 'function')).to.be(undefined);
  });

  it('wrap module should be ok', function () {
    function testfn() { }
    const mod3 = { test: testfn };
    function wrapper(origin, name) {
      return function wrapped() {
        expect(origin).to.be(testfn);
        expect(name).to.be('test');
      };
    }
    expect(typeof wrap(mod3, 'test', wrapper)).to.be.ok();
    mod3.test();
  });

  it('unwrap module failed with wrong params', function () {
    function testfn() { }
    const mod4 = { test: testfn, test1: () => { } };
    function wrapper() {
      return function wrapped() { };
    }
    wrap(mod4, 'test', wrapper);

    const __unwrap = mod4.test.__unwrap;
    const testfn2 = () => { };
    testfn2.__unwrap = __unwrap;
    mod4.test = testfn2;
    unwrap(mod4, 'test');
    expect(mod4.test).not.to.be(testfn);
    expect(mod4.test).to.be(testfn2);
  });

  it('unwrap module should be ok', function () {
    function testfn() { }
    const mod5 = { test: testfn, test1: () => { } };
    function wrapper() {
      return function wrapped() { };
    }
    wrap(mod5, 'test', wrapper);
    expect(mod5.test).not.to.be(testfn);

    expect(unwrap(mod5)).to.be(undefined);
    expect(unwrap(mod5, 'not_exists')).to.be(undefined);
    expect(unwrap(mod5, 'test1')).to.be(undefined);

    unwrap(mod5, 'test');
    expect(mod5.test).to.be(testfn);
  });
});