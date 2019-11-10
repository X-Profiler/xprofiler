'use strict';

const os = require('os');
const fs = require('fs');
const path = require('path');
const utils = require('./utils');

module.exports = function clean(logdir) {
  /* istanbul ignore if */
  if (os.platform() === 'win32') {
    return;
  }

  // shoule clean files
  let needClean = [];
  const files = fs.readdirSync(logdir);

  // not alive socks
  const needCleanSocks = files.filter(file => {
    const matched = file.match(/^(xprofiler-uds-path)-(\d+)\.sock$/);
    if (matched) {
      const pid = parseInt(matched[2]);
      if (pid === null || isNaN(pid) || !utils.processAlive(pid)) {
        return true;
      }
    }
    return false;
  });

  // clean these files
  needClean = needClean.concat(needCleanSocks);
  needClean.forEach(filename => fs.unlinkSync(path.join(logdir, filename)));
};
