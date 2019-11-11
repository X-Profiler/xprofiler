'use strict';

const os = require('os');
const fs = require('fs');
const path = require('path');

const SPLITTER = '\u0000';

function processAlive(pid) {
  try {
    return process.kill(pid, 0);
  } catch (ex) {
    return false;
  }
}
exports.processAlive = processAlive;

function composeLogDirInfo(logdir) {
  return [process.pid, logdir].join(SPLITTER) + '\n';
}
exports.composeLogDirInfo = composeLogDirInfo;

function splitLogDirInfo(line) {
  return line.split(SPLITTER);
}
exports.splitLogDirInfo = splitLogDirInfo;

function setLogDirToFile(logdir) {
  const dataFile = path.join(os.homedir(), '.xprofiler');
  if (fs.existsSync(dataFile)) {
    const processes = fs
      .readFileSync(dataFile, 'utf8')
      .split('\n')
      .filter(p => processAlive(p.split(SPLITTER)[0]))
      .join('\n') + composeLogDirInfo(logdir);
    fs.writeFileSync(dataFile, processes);
  } else {
    fs.writeFileSync(dataFile, composeLogDirInfo(logdir));
  }
}
exports.setLogDirToFile = setLogDirToFile;

function printConfig(config) {
  const keys = Object.keys(config);
  return keys.map(key => `  - ${key}: ${config[key]}`).join('\n');
}
exports.printConfig = printConfig;