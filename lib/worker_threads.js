'use strict';

let isMainThread = true;
let threadId = 0;
let canIUseWorker = false;
try {
  const workerThreads = require('worker_threads');
  isMainThread = workerThreads.isMainThread;
  threadId = workerThreads.threadId;
  canIUseWorker = true;
  /** Node.js v8.x compat: remove the unused catch-binding */
} catch (_) { /** ignore */ }

module.exports = {
  isMainThread,
  threadId,
  canIUseWorker,
};
