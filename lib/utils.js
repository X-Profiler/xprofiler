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
  return [
    process.pid,
    logdir,
    process.cwd(),
    process.execPath,
    process.argv[1],
    path.join(__dirname, '..')
  ].join(SPLITTER) + '\n';
}
exports.composeLogDirInfo = composeLogDirInfo;

function splitLogDirInfo(line) {
  return line.split(SPLITTER);
}
exports.splitLogDirInfo = splitLogDirInfo;

function setLogDirToFile(logdir) {
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
    fs.writeFileSync(dataFile, composeLogDirInfo(logdir), { flag: 'a' });
  }

  // clean invalid record after randon 1min
  const checkTime = process.env.UNIT_TEST_CHECK_TIME || (50 + Math.random() * 15);
  const timer = setTimeout(() => {
    let processes = fs
      .readFileSync(dataFile, 'utf8')
      .split('\n')
      .filter(p => p && processAlive(p.split(SPLITTER)[0]));
    processes = Array.from(new Set(processes)).join('\n') + '\n';
    fs.writeFileSync(dataFile, processes);
  }, parseInt(checkTime * 1000));
  timer.unref();
}
exports.setLogDirToFile = setLogDirToFile;

function printConfig(config) {
  const keys = Object.keys(config);
  return keys.map(key => `  - ${key}: ${config[key]}`).join('\n');
}
exports.printConfig = printConfig;

const check = {
  number: v => v !== null && !isNaN(v),
  boolean: v => v === true || v === false
};

function getXctOptions(action, args, configList) {
  const options = {};
  for (const config of configList) {
    if (config.actions.includes(action)) {
      for (const [key, rule] of Object.entries(config.opts)) {
        const value = args[key];
        if (typeof check[rule] === 'function' && check[rule](value)) {
          if (key.startsWith('disable_')) {
            options[key.replace('disable_', 'enable_')] = !value;
          } else {
            options[key] = value;
          }
        }
      }
      if (config.opt_required && Object.keys(options).length === 0) {
        return { ok: false, message: `${action} 缺少必须参数，执行 xprofctl ${action} 查看正确用法` };
      }
    }
  }
  return { ok: true, data: options };
}
exports.getXctOptions = getXctOptions;

function sleep(expired) {
  return new Promise(resolve => setTimeout(resolve, expired));
}
exports.sleep = sleep;