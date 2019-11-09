'use strict';

exports.SPLITTER = '\u0000';

exports.processAlive = function processAlive(pid) {
  try {
    return process.kill(pid, 0);
  } catch (ex) {
    return false;
  }
}