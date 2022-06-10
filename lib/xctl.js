'use strict';

const os = require('os');
const fs = require('fs');
const net = require('net');
const path = require('path');
const { v4: uuid } = require('uuid');
const pkg = require('../package.json');
const utils = require('../lib/utils');
const { StringDecoder } = require('string_decoder');
const decoder = new StringDecoder('utf8');

const TIMEOUT_ERROR = Symbol('XPROFILER_CTL_TIMEOUT');

const timer = {};
let server;
let client;

function getXprofilerLogDir(pid) {
  let xprofilerPath;
  if (process.env.XPROFILER_UNIT_TEST_TMP_HOMEDIR) {
    xprofilerPath = path.join(process.env.XPROFILER_UNIT_TEST_TMP_HOMEDIR, '.xprofiler');
  } else {
    xprofilerPath = path.join(os.homedir(), '.xprofiler');
  }

  /* istanbul ignore if */
  if (!fs.existsSync(xprofilerPath)) {
    throw new Error(`xprofiler 日志目录 ${xprofilerPath} 不存在!`);
  }

  let logDir = '';
  const processes = fs.readFileSync(xprofilerPath, 'utf8').split('\n');
  for (const line of processes) {
    const tmp = utils.splitLogDirInfo(line);
    const processid = tmp[0];
    /* istanbul ignore else */
    if (Number(processid) === Number(pid)) {
      logDir = tmp[1];
      break;
    }
  }

  /* istanbul ignore if */
  if (!logDir) {
    throw new Error(`进程 ${pid} 查询不到 xprofiler 日志目录信息`);
  }

  return logDir;
}

function composeXprofilerPath(pid) {
  const platform = os.platform();
  const logDir = getXprofilerLogDir(pid);
  let ipcPath = '';

  // windows named pipe
  /* istanbul ignore next */
  if (platform === 'win32') {
    ipcPath = `\\\\.\\pipe\\${path.join(logDir, `xprofiler-named-pipe-${pid}`)}`;
  }

  // unix domain socket
  /* istanbul ignore next */
  if (platform === 'darwin' || platform === 'linux') {
    ipcPath = path.join(logDir, `xprofiler-uds-path-${pid}.sock`);
  }

  return ipcPath;
}

function composeCtlServerPath(pid) {
  const platform = os.platform();
  const logDir = getXprofilerLogDir(pid);
  let serverCtlPath = '';

  // windows named pipe
  /* istanbul ignore next */
  if (platform === 'win32') {
    serverCtlPath = `\\\\.\\pipe\\${path.join(logDir, pkg.xctlIpcPath.win32)}`;
  }

  // unix domain socket
  /* istanbul ignore next */
  if (platform === 'darwin' || platform === 'linux') {
    serverCtlPath = path.join(logDir, pkg.xctlIpcPath.unix);
  }

  // clean path
  /* istanbul ignore if */
  if (fs.existsSync(serverCtlPath)) {
    fs.unlinkSync(serverCtlPath);
  }

  return serverCtlPath;
}

async function createMessageConnection(pid, xprofilerPath, message) {
  // ensure the ctl server established
  await utils.sleep(100);

  message = JSON.stringify(message);

  /* istanbul ignore next */
  if (os.platform() !== 'win32' && !fs.existsSync(xprofilerPath)) {
    throw new Error(`进程 ${pid} 不存在或者没有启动 xprofiler 信令线程!`);
  }

  /* istanbul ignore next */
  if (os.platform() === 'win32') {
    await utils.sleep(200);
  }

  return new Promise((resolve, reject) => {
    client = net.createConnection(xprofilerPath, () => {
      client.write(message, err => err ? /* istanbul ignore next */reject(err) : resolve('success'));
    });
    client.on('error', /* istanbul ignore next */error => {
      reject(`发送命令 ${message} 失败: ${error}`);
    });
    client.unref();
  });
}

function createResultServer(serverCtlPath, traceid) {
  return new Promise((resolve, reject) => {
    server = net.createServer(conn => {
      conn.on('data', chunk => {
        let data = decoder.end(chunk);
        try {
          data = JSON.parse(data);
          /* istanbul ignore next */
          if (data.traceid === traceid) {
            resolve(data);
          }
        } catch (err) {
          /* istanbul ignore next */
          reject(`json parse [${data}] failed.`);
        }
      });
      conn.on('error', /* istanbul ignore next */err => reject(`接收响应失败: ${err}`));
      conn.unref();
    });
    server.listen(serverCtlPath);
    server.on('error', /* istanbul ignore next */err => reject(`创建接收响应服务失败: ${err}`));
    server.unref();
  });
}

// release resource
function teardown(key) {
  server && server.close();
  server = null;

  client && client.destroy();
  client = null;

  /* istanbul ignore next */
  if (timer[key]) {
    timer[key] && clearTimeout(timer[key]);
    timer[key] = null;
    return;
  }

  for (const t of Object.entries(timer)) {
    t[1] && clearTimeout(t[1]);
    t[1] = null;
  }
}

/* istanbul ignore next */
function timeout(time, key) {
  return new Promise(resolve => timer[key] = setTimeout(() => {
    resolve(TIMEOUT_ERROR);
    teardown(key);
  }, time));
}

async function sendCommands(pid, thread_id, command, /* istanbul ignore next */options = {}) {
  /* istanbul ignore if */
  if (!command) {
    throw new Error('命令参数不能缺失！');
  }
  // create traceid
  const traceid = uuid();

  // expired
  const expired = process.env.UNIT_TEST_COMMAND_EXPIRED_TIME || 1500;

  const resultTasks = [];
  // wait for result
  const serverCtlPath = composeCtlServerPath(pid);
  resultTasks.push(createResultServer(serverCtlPath, traceid));
  resultTasks.push(timeout(expired, 'result'));

  const sendTasks = [];
  // send message
  const ipcPath = composeXprofilerPath(pid);
  sendTasks.push(createMessageConnection(pid, ipcPath, { traceid, cmd: command, thread_id, options }));
  sendTasks.push(timeout(expired, 'send'));

  // send message & check result
  let result = await Promise.all([
    Promise.race(sendTasks),
    Promise.race(resultTasks)
  ]);

  /* istanbul ignore if */
  if (result[0] === TIMEOUT_ERROR) {
    throw new Error(`命令 ${command} 请求超时(${expired / 1000}s)!`);
  }

  /* istanbul ignore if */
  if (result[1] === TIMEOUT_ERROR) {
    throw new Error(`命令 ${command} 响应超时(${expired / 1000}s)!`);
  }

  result = result[1];

  return result;
}

module.exports = async function (...args) {
  let result;
  try {
    result = await sendCommands(...args);
  } catch (err) /* istanbul ignore next */ {
    /* eslint-disable */
    err = err instanceof Error ? err : new Error(err);
    result = { ok: false, message: err.message, stack: err.stack };
  }

  teardown();

  return result;
};
