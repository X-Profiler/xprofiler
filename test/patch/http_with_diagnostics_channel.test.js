'use strict';

const http = require('http');
const expect = require('expect.js');
const { subscribeHttpServerRequestStart, unsubscribeHttpServerRequestStart } = require('../../patch/http');

const status = {};

describe(`patch with diagnostics_channel`, function () {
  const requestTimes = 5;
  let triggerTimes = 0;

  let httpConfig = {};
  let liveRequest = 0;
  let closeRequest = 0;
  let sentRequest = 0;

  function setHttpConfig(config) {
    httpConfig = config;
  }

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

  before(async function () {
    subscribeHttpServerRequestStart({
      setHttpConfig,
      addLiveRequest,
      addCloseRequest,
      addSentRequest,
      addHttpStatusCode,
    });
    const server = http.createServer(function (request, response) {
      triggerTimes++;
      response.statusCode = 200;
      response.end(`hello world ${request.url}`);
    });
    server.listen(0);
    const port = server.address().port;
    const url = `http://localhost:${port}`;
    for (let i = 0; i < requestTimes; i++) {
      await new Promise((resolve) => {
        http.get(url, resolve);
      });
    }
    server.close();
  });

  after(function () {
    unsubscribeHttpServerRequestStart();
  });

  it('http config.http_detail_profiling should be false', function () {
    expect(httpConfig.http_detail_profiling).to.be(false);
  });

  it('http config.start_time should be 0', function () {
    expect(httpConfig.start_time).to.be(0);
  });

  it(`request handler should trigger ${requestTimes} times`, function () {
    expect(triggerTimes).to.be(requestTimes);
  });

  it(`live request should be ${requestTimes}`, function () {
    expect(liveRequest).to.be(requestTimes);
  });

  it(`close request should be ${requestTimes}`, function () {
    expect(closeRequest).to.be(requestTimes);
  });

  it(`sent request should be ${requestTimes}`, function () {
    expect(sentRequest).to.be(requestTimes);
  });

  it(`count of http status code 200 should be ${requestTimes}`, function () {
    expect(status[200]).to.be(requestTimes);
  });
});
