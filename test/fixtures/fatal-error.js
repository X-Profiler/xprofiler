'use strict';

const xprofiler = require('../../');
xprofiler();

const array = [];
setInterval(() => {
  array.push(new Array(10e6).fill('*'));
  console.log('now rss:', process.memoryUsage().rss / 1024 / 1024 + ' Mb');
}, 1);