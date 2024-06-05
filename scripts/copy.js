'use strict';

const path = require('path');
const fs = require('fs');
const { promisify } = require('util');
const exists = promisify(fs.exists);
const mkdir = promisify(fs.mkdir);
const copyFile = promisify(fs.copyFile);
const versioning = require('@mapbox/node-pre-gyp/lib/util/versioning.js');
const { staged_tarball: packagePath } = versioning.evaluate(require('../package.json'));

async function copy() {
  const release = path.join(__dirname, '../release');
  if (!await exists(release)) {
    await mkdir(release);
  }

  if (!await exists(packagePath)) {
    return;
  }

  const filename = path.basename(packagePath);
  const target = path.join(release, filename);
  await copyFile(packagePath, target);
}

copy();
