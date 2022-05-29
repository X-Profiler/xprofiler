'use strict';

const os = require('os');
const fs = require('fs');
const cp = require('child_process');
const path = require('path');
const { promisify } = require('util');
const readdir = promisify(fs.readdir);
const exists = promisify(fs.exists);
const unlink = promisify(fs.unlink);
const mkdir = promisify(fs.mkdir);

const isWindows = os.platform() === 'win32';

function debug(message) {
  console.log(`\n\x1b[32;1m${message}\x1b[0m\n`);
}

// function error(message) {
//   console.log(`\n\x1b[31m${message}\x1b[0m\n`);
// }

function execCmd(cmd) {
  debug(cmd);
  cp.execSync(cmd, {
    env: process.env,
    cwd: path.join(__dirname, '../'),
    stdio: 'inherit',
    shell: isWindows ? undefined : '/bin/bash',
  });
}

async function cleanReleaseDir() {
  const release = path.join(__dirname, '../release');
  if (!await exists(release)) {
    await mkdir(release);
  }
  const files = await readdir(release);
  for (const file of files) {
    const filePath = path.join(release, file);
    if (!await exists(filePath)) {
      continue;
    }
    await unlink(filePath);
  }
}

module.exports = async versions => {
  await cleanReleaseDir();

  for (const version of versions) {
    debug(`>>>>>>>> start build with ${version}`);
    const tnvmPath = path.join(os.homedir(), '.tnvm/tnvm.sh');
    let npmBin = 'npm';
    let change = `source ${tnvmPath} && tnvm use ${version}`;
    const nvmNodeVersion = /^node-v(.*)$/.exec(version)[1];
    if (isWindows) {
      npmBin = path.join(os.tmpdir(), '../../', `Roaming\\nvm\\v${nvmNodeVersion}\\npm.cmd`);
      change = `nvm use ${nvmNodeVersion}`;
    }

    const install = 'npm install --no-audit';
    const build = `${npmBin} run dep`;
    const pack = 'npx node-pre-gyp package && npx node-pre-gyp testpackage';
    const copy = `${npmBin} run copy`;
    execCmd(`${change} && ${build} && ${install} && ${pack} && ${copy}`);
    debug(`<<<<<<<< build with ${version} done.`);
  }

  debug('all build tasks done.');
};