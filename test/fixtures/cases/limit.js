'use strict';

const os = require('os');
const path = require('path');
const { filterTestCaseByPlatform } = require('../utils');

const exitFatalErrorScriptPath = path.join(__dirname, '../scripts/fatal_error.js');

const increasedHeapLogStructure = {
  current_heap_limit: /^\d+$/,
  initial_heap_limit: /^\d+$/,
  auto_incr_heap_limit_size: /^\d+$/,
  increased_heap: /^\d+$/,
};

exports = module.exports = function () {
  const list = [
    {
      title: 'limit hook is valid',
      subTitle: 'auto increase heap limit',
      jspath: exitFatalErrorScriptPath,
      skip: os.platform() === 'win32'
    }
  ];

  return filterTestCaseByPlatform(list);
};

exports.increasedHeapLogStructure = increasedHeapLogStructure;