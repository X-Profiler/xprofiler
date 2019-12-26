'use strict';

const fs = require('fs');
const path = require('path');

exports.xprofilerPrefixRegexp =
  /\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\] \[(.+)\] \[(.+)\] \[(\d+)\] \[(\d{1,3}\.\d{1,3}\.\d{1,3}.*)\] (.*)/g;

exports.alinodePrefixRegexp =
  /\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}.\d{6}\] \[(.+)\] \[(.+)\] \[(\d+)\] (.*)/g;

exports.createLogDir = function createLogDir(logdir) {
  const log_dir = path.join(__dirname, logdir);
  if (!fs.existsSync(log_dir)) {
    fs.mkdirSync(log_dir, { recursive: true });
  } else {
    for (const file of fs.readdirSync(log_dir)) {
      fs.unlinkSync(path.join(log_dir, file));
    }
  }
  return log_dir;
};

exports.objKeyEqual = function objKeyEqual(obj1, obj2) {
  const keys1 = Object.keys(obj1);
  const keys2 = Object.keys(obj2);
  return keys1.every(k1 => keys2.includes(k1)) && keys2.every(k2 => keys1.includes(k2));
};

exports.cleanDir = function (dir) {
  for (const file of fs.readdirSync(dir)) {
    fs.unlinkSync(path.join(dir, file));
  }
  fs.rmdirSync(dir);
};

exports.sleep = function (ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
};

exports.getNestingValue = function (origin, key) {
  const keys = key.split('.');
  for (const k of keys) {
    origin = origin[k];
  }
  return origin;
};

exports.arrayEqual = function arrayEqual(arr1, arr2) {
  return arr1.every(a1 => arr2.includes(a1)) && arr2.every(a2 => arr1.includes(a2));
};