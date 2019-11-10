'use strict';

const os = require('os');
const fs = require('fs');
const net = require('net');
const path = require('path');
const pkg = require('../package.json');
const utils = require('../lib/utils');

const TIMEOUT_ERROR = Symbol('XPROFILER_CTL_TIMEOUT');

function getXprofilerLogDir(pid) {
  const xprofilerPath = path.join(os.homedir(), '.xprofiler');
  if (!fs.existsSync(xprofilerPath)) {
    throw new Error(`xprofiler 日志目录 ${xprofilerPath} 不存在!`);
  }
  let logDir = '';
  const processes = fs.readFileSync(xprofilerPath, 'utf8').split('\n');
  for (const line of processes) {
    const tmp = utils.splitLogDirInfo(line);
    const processid = tmp[0];
    if (Number(processid) === Number(pid)) {
      logDir = tmp[1];
      break;
    }
  }

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
  if (platform === 'win32') {
    ipcPath = `\\\\.\\pipe\\${path.join(logDir, `xprofiler-named-pipe-${pid}`)}`;
  }

  // unix domain socket
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
  if (platform === 'win32') {
    serverCtlPath = `\\\\.\\pipe\\${path.join(logDir, pkg.xctlIpcPath.win32)}`;
  }

  if (platform === 'darwin' || platform === 'linux') {
    serverCtlPath = path.join(logDir, pkg.xctlIpcPath.unix);
  }

  // clean path
  if (fs.existsSync(serverCtlPath)) {
    fs.unlinkSync(serverCtlPath);
  }

  return serverCtlPath;
}

function createMessageConnection(pid, xprofilerPath, message) {
  message = JSON.stringify(message);

  if (!fs.existsSync(xprofilerPath)) {
    throw new Error(`进程 ${pid} 不存在或者没有启动 xprofiler 信令线程!`);
  }

  return new Promise((resolve, reject) => {
    const client = net.createConnection(xprofilerPath, () => {
      client.write(message, err => err ? reject(err) : resolve());
    });
    client.on('error', error => {
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
      conn.on('error', err => reject(`接收响应失败: ${err}`));
      conn.on('close', () => server.close());
    });
    server.listen(serverCtlPath);
    server.on('error', err => {
      reject(`创建接收响应服务失败: ${err}`);
      server.close();
    });
  });
}


let timer;
function timeout(time) {
  return new Promise(resolve => timer = setTimeout(() => resolve(TIMEOUT_ERROR), time));
}

async function sendCommands(pid, command, options = {}) {
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
    result = { ok: false, message: err };
  }

  // release resource
  server && server.close();
  timer && clearTimeout(timer);

  return result;
};
