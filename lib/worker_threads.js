'use strict';

const workerThreads = require('worker_threads');

const isMainThread = workerThreads.isMainThread;
const threadId = workerThreads.threadId;

module.exports = {
  isMainThread,
  threadId,
};
