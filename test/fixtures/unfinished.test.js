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
      options: { profiling_time: 5 * 60 * 1000 },
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
      options: { profiling_time: 5 * 60 * 1000 },
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
      options: { profiling_time: 5 * 60 * 1000 },
      checkExitInfo: false,
      check(file) {
        const content = fs.readFileSync(file, 'utf8').trim();
        checkProfile(gcprofile, JSON.parse(content));
      }
    },

    // exit 0 with profiling time
    {
      title: '<normal exit> cpu profiling',
      jspath: path.join(__dirname, 'exit.js'),
      tid: 0,
      cmd: 'start_cpu_profiling',
      options: { profiling_time: 5 * 60 * 1000 },
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
      options: { profiling_time: 5 * 60 * 1000 },
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
      options: { profiling_time: 5 * 60 * 1000 },
      checkExitInfo: true,
      check(file) {
        const content = fs.readFileSync(file, 'utf8').trim();
        checkProfile(gcprofile, JSON.parse(content));
      }
    },

    // exit 0 with no profiling time
    {
      title: '<normal exit> cpu profiling',
      jspath: path.join(__dirname, 'exit.js'),
      tid: 0,
      cmd: 'start_cpu_profiling',
      options: undefined,
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
      options: undefined,
      checkExitInfo: true,
      check(file) {
        const content = fs.readFileSync(file, 'utf8').trim();
        checkProfile(cpuprofile, JSON.parse(content));
      }
    },
    {
      title: '<normal exit> gc profiling',
      jspath: path.join(__dirname, 'exit.js'),
      tid: 0,
      cmd: 'start_gc_profiling',
      options: undefined,
      checkExitInfo: true,
      check(file) {
        const content = fs.readFileSync(file, 'utf8').trim();
        checkProfile(cpuprofile, JSON.parse(content));
      }
    },
  ];

  return list;
};