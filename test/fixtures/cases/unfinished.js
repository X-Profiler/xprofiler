'use strict';

const fs = require('fs');
const path = require('path');
const { profileRule: { cpuprofile, heapprofile, gcprofile }, checkProfile } = require('./command');

const exitFatalErrorScriptPath = path.join(__dirname, '../scripts/fatal_error.js');
const exitNormalScriptPath = path.join(__dirname, '../scripts/exit.js');

exports = module.exports = function () {
  const list = [
    // fatal error
    {
      title: '<fatal_error> cpu profiling',
      jspath: exitFatalErrorScriptPath,
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
      title: '<fatal_error> heap profiling',
      jspath: exitFatalErrorScriptPath,
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
      title: '<fatal_error> gc profiling',
      jspath: exitFatalErrorScriptPath,
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
      title: '<at_exit> cpu profiling',
      jspath: exitNormalScriptPath,
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
      title: '<at_exit> heap profiling',
      jspath: exitNormalScriptPath,
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
      title: '<at_exit> gc profiling',
      jspath: exitNormalScriptPath,
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
      title: '<at_exit> cpu profiling',
      jspath: exitNormalScriptPath,
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
      title: '<at_exit> heap profiling',
      jspath: exitNormalScriptPath,
      tid: 0,
      cmd: 'start_heap_profiling',
      options: undefined,
      checkExitInfo: true,
      check(file) {
        const content = fs.readFileSync(file, 'utf8').trim();
        checkProfile(heapprofile, JSON.parse(content));
      }
    },
    {
      title: '<at_exit> gc profiling',
      jspath: exitNormalScriptPath,
      tid: 0,
      cmd: 'start_gc_profiling',
      options: undefined,
      checkExitInfo: true,
      check(file) {
        const content = fs.readFileSync(file, 'utf8').trim();
        checkProfile(gcprofile, JSON.parse(content));
      }
    },
  ];

  return list;
};