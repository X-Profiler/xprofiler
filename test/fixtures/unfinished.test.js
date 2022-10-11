'use strict';

const fs = require('fs');
const path = require('path');
const { profileRule: { cpuprofile, heapprofile, gcprofile }, checkProfile } = require('./command.test');

exports = module.exports = function () {
  const list = [
    // fataerror
    {
      title: '<fatalerror / oom> cpu profiling',
      jspath: path.join(__dirname, 'fatal-error.js'),
      tid: 0,
      cmd: 'start_cpu_profiling',
      checkExitInfo: false,
      check(file) {
        const content = fs.readFileSync(file, 'utf8').trim();
        checkProfile(cpuprofile, JSON.parse(content));
      }
    },
    {
      title: '<fatalerror / oom> heap profiling',
      jspath: path.join(__dirname, 'fatal-error.js'),
      tid: 0,
      cmd: 'start_heap_profiling',
      checkExitInfo: false,
      check(file) {
        const content = fs.readFileSync(file, 'utf8').trim();
        checkProfile(heapprofile, JSON.parse(content));
      }
    },
    {
      title: '<fatalerror / oom> gc profiling',
      jspath: path.join(__dirname, 'fatal-error.js'),
      tid: 0,
      cmd: 'start_gc_profiling',
      checkExitInfo: false,
      check(file) {
        const content = fs.readFileSync(file, 'utf8').trim();
        checkProfile(gcprofile, JSON.parse(content));
      }
    },

    // exit 0
    {
      title: '<normal exit> cpu profiling',
      jspath: path.join(__dirname, 'exit.js'),
      tid: 0,
      cmd: 'start_cpu_profiling',
      checkExitInfo: true,
      check(file) {
        const content = fs.readFileSync(file, 'utf8').trim();
        checkProfile(cpuprofile, JSON.parse(content));
      }
    },
    {
      title: '<normal exit> heap profiling',
      jspath: path.join(__dirname, 'exit.js'),
      tid: 0,
      cmd: 'start_heap_profiling',
      checkExitInfo: true,
      check(file) {
        const content = fs.readFileSync(file, 'utf8').trim();
        checkProfile(heapprofile, JSON.parse(content));
      }
    },
    {
      title: '<normal exit> gc profiling',
      jspath: path.join(__dirname, 'exit.js'),
      tid: 0,
      cmd: 'start_gc_profiling',
      checkExitInfo: true,
      check(file) {
        const content = fs.readFileSync(file, 'utf8').trim();
        checkProfile(gcprofile, JSON.parse(content));
      }
    },
  ];

  return list;
};