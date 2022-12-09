'use strict';

const os = require('os');
const fs = require('fs');
const path = require('path');

const SPLITTER = '\u0000';

exports.processAlive = function (pid) {
  try {
    return process.kill(pid, 0);
  } catch (ex) {
    return false;
  }
};

exports.composeLogDirInfo = function (logdir) {
  return [
    process.pid,
    logdir,
    process.cwd(),
    process.execPath,
    process.argv[1],
    path.join(__dirname, '..')
  ].join(SPLITTER) + '\n';
};

exports.splitLogDirInfo = function (line) {
  return line.split(SPLITTER);
};

exports.setLogDirToFile = function (logdir) {
  const dataFile = path.join(os.homedir(), '.xprofiler');

  // check pid exists
  let shouldWrite = true;
  if (fs.existsSync(dataFile)) {
    const content = fs.readFileSync(dataFile, 'utf8');
    if (content.trim().split('\n').some(proc => proc
      && Number(proc.split('\u0000')[0]) === process.pid)) {
      shouldWrite = false;
    }
  }

  // write process info
  if (shouldWrite) {
    fs.writeFileSync(dataFile, exports.composeLogDirInfo(logdir), { flag: 'a' });
  }

  // clean invalid record after randon 1min
  const checkTime = process.env.UNIT_TEST_CHECK_TIME || (50 + Math.random() * 15);
  const timer = setTimeout(() => {
    let processes = fs
      .readFileSync(dataFile, 'utf8')
      .split('\n')
      .filter(p => p && exports.processAlive(p.split(SPLITTER)[0]));
    processes = Array.from(new Set(processes)).join('\n') + '\n';
    fs.writeFileSync(dataFile, processes);
  }, parseInt(checkTime * 1000));
  timer.unref();
};

exports.printConfig = function (config) {
  const keys = Object.keys(config);
  return keys.map(key => `  - ${key}: ${config[key]}`).join('\n');
};

exports.cleanArgs = function (args) {
  delete args['_'];
  delete args['pid'];
  delete args['p'];
  delete args['w'];
  delete args['worker_thread_id'];
  delete args['$0'];
};

exports.formatArgs = function (args) {
  for (const key of Object.keys(args)) {
    if (!key.startsWith('disable_')) {
      continue;
    }

    args[key.replace('disable_', 'enable_')] = false;
    delete args[key];
  }
};

exports.pair = function (name, format) {
  const is = name.startsWith('enable_') && format === 'boolean';
  const pair = is ? [name, name.replace('enable_', 'disable_')] : [name];
  return { is, pair };
};

exports.sleep = function (expired) {
  return new Promise(resolve => setTimeout(resolve, expired));
};
