'use strict';

const os = require('os');
const fs = require('fs');
const net = require('net');
const path = require('path');
const pkg = require('../package.json');
const utils = require('../lib/utils');

const TIMEOUT_ERROR = Symbol('XPROFILER_CTL_TIMEOUT');

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
  message = JSON.stringify(message);

  /* istanbul ignore if */
  if (!fs.existsSync(xprofilerPath)) {
    throw new Error(`进程 ${pid} 不存在或者没有启动 xprofiler 信令线程!`);
  }

  /* istanbul ignore next */
  if (os.platform() === 'win32') {
    await new Promise(resolve => setTimeout(resolve, 100));
  }

  return new Promise((resolve, reject) => {
    const client = net.createConnection(xprofilerPath, () => {
      client.write(message, err => err ? /* istanbul ignore next */reject(err) : resolve());
    });
    client.on('error', /* istanbul ignore next */error => {
      reject(`发送命令 ${message} 失败: ${error}`);
    });
    client.on('end', () => {
      client.destroy();
    });
  });
}


let server;
function createResultServer(serverCtlPath) {
  return new Promise((resolve, reject) => {
    server = net.createServer(conn => {
      conn.on('data', data => resolve(data.toString()));
      conn.on('error', /* istanbul ignore next */err => reject(`接收响应失败: ${err}`));
      conn.on('close', () => server.close());
    });
    server.listen(serverCtlPath);
    server.on('error', /* istanbul ignore next */err => {
      reject(`创建接收响应服务失败: ${err}`);
      server.close();
    });
  });
}


let timer;
/* istanbul ignore next */
function timeout(time) {
  return new Promise(resolve => timer = setTimeout(() => resolve(TIMEOUT_ERROR), time));
}

async function sendCommands(pid, command, /* istanbul ignore next */options = {}) {
  /* istanbul ignore if */
  if (!command) {
    throw new Error('命令参数不能缺失！');
  }

  const resultTasks = [];
  // wait for result
  const serverCtlPath = composeCtlServerPath(pid);
  resultTasks.push(createResultServer(serverCtlPath));
  // expired
  const expired = 1000;
  resultTasks.push(timeout(expired));

  // send message & check result
  const ipcPath = composeXprofilerPath(pid);
  let result = await Promise.all([
    createMessageConnection(pid, ipcPath, { cmd: command, options }),
    Promise.race(resultTasks)
  ]);

  result = result[1];

  /* istanbul ignore if */
  if (result === TIMEOUT_ERROR) {
    throw new Error(`命令 ${command} 超时(${expired / 1000}s)!`);
  }

  return JSON.parse(result);
}

module.exports = async function (...args) {
  let result;
  try {
    result = await sendCommands(...args);
  } catch (err) {
    /* istanbul ignore next */
    result = { ok: false, message: err };
  }

  // release resource
  server && server.close();
  timer && clearTimeout(timer);

  return result;
};
