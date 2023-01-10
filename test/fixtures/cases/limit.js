'use strict';

const path = require('path');
const { filterTestCaseByPlatform } = require('../utils');

const exitFatalErrorScriptPath = path.join(__dirname, '../scripts/fatal_error.js');

exports = module.exports = function () {
  const list = [
    {
      title: 'limit hook is valid',
      subTitle: 'auto increase heap limit is ok.',
      jspath: exitFatalErrorScriptPath,
    }
  ];

  return filterTestCaseByPlatform(list);
};