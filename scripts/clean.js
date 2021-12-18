'use strict';

const fs = require('fs');
const path = require('path');

const release = path.join(__dirname, '../release');
if (!fs.existsSync(release)) {
  fs.mkdirSync(release);
}
const files = fs.readdirSync(release);
for (const file of files) {
  const filePath = path.join(release, file);
  if (!fs.existsSync(filePath)) {
    continue;
  }
  fs.unlinkSync(filePath);
}
