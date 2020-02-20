'use strict';

const http = require('http');
const EventEmitter = require('events').EventEmitter;
const mm = require('mm');
const expect = require('expect.js');
const { patchHttp } = require('../../patch/http');

const status = {};

describe(`patch http.createServer(cb)`, function () {
  const requestTimes = 5;
  let triggerTimes = 0;

  let liveRequest = 0;
  let closeRequest = 0;
  let sentRequest = 0;

  function addLiveRequest() {
    liveRequest++;
  }

  function addCloseRequest() {
    closeRequest++;
  }

  function addSentRequest() {
    sentRequest++;
  }

  function addHttpStatusCode(code) {
    if (status[code]) {
      status[code]++;
    } else {
      status[code] = 1;
    }
  }

  function mockCreateServer(opts, requestHandle) {
    return new Promise(resolve => {
      let times = 0;
      const interval = setInterval(() => {
        if (times < requestTimes) {
          const request = new EventEmitter();
          const response = new EventEmitter();
          if (typeof opts === 'function') {
            opts(request, response);
          } else if (typeof requestHandle === 'function') {
            requestHandle(request, response);
          }
          times++;
        } else {
          clearInterval(interval);
          resolve();
        }
      }, 100);
    });
  }

  before(async function () {
    mm(http, 'createServer', mockCreateServer);
    patchHttp({ addLiveRequest, addCloseRequest, addSentRequest, addHttpStatusCode });
    await http.createServer(function (request, response) {
      triggerTimes++;
      response.statusCode = 200;
      response.emit('finish');
      response.emit('close');
    });

    await http.createServer({}, function (request, response) {
      triggerTimes++;
      response.statusCode = 200;
      response.emit('finish');
      response.emit('close');
    });

    await http.createServer({}, {}, function (request, response) {
      triggerTimes++;
      response.statusCode = 200;
      response.emit('finish');
      response.emit('close');
    });
  });

  after(function () {
    mm.restore();
  });

  it('patch should be ok', function () {
    expect(http.createServer).not.to.be(mockCreateServer);
  });

  it(`request handler should trigger ${requestTimes} * 2 times`, function () {
    expect(triggerTimes).to.be(requestTimes * 2);
  });

  it(`live request shoule be ${requestTimes} * 2`, function () {
    expect(liveRequest).to.be(requestTimes * 2);
  });

  it(`close request shoule be ${requestTimes} * 2`, function () {
    expect(closeRequest).to.be(requestTimes * 2);
  });

  it(`sent request shoule be ${requestTimes} * 2`, function () {
    expect(sentRequest).to.be(requestTimes * 2);
  });

  it(`count of http status code 200 should be ${requestTimes} * 2`, function () {
    expect(status[200]).to.be(requestTimes * 2);
  });
});