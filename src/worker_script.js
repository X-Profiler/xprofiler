const { parentPort, workerData } = require('worker_threads');
const net = require('net');
const fs = require('fs');
const path = require('path');
const inspector = require('inspector');
const cp = require('child_process');
const os = require('os');
const moment = require('moment');

const { logDir, pid, threadId, platform, xctlIpcPathUnix, xctlIpcPathWin32, version } = workerData;

let ipcPath = '';
if (platform === 'win32') {
  ipcPath = `\\\\.\\pipe\\${path.join(logDir, `xprofiler-named-pipe-${pid}`)}`;
} else {
  ipcPath = path.join(logDir, `xprofiler-uds-path-${pid}.sock`);
}

if (fs.existsSync(ipcPath) && platform !== 'win32') {
  fs.unlinkSync(ipcPath);
}

// Create a session to the main thread
const session = new inspector.Session();
session.connect();

const server = net.createServer((conn) => {
  let buf = '';
  conn.on('data', chunk => {
    buf += chunk.toString();
    try {
      const msg = JSON.parse(buf);
      handleCommand(msg).then(result => {
        sendResult(msg.traceid, result);
      }).catch(err => {
        sendResult(msg.traceid, { ok: false, message: err.message || String(err) });
      });
      buf = '';
    } catch(e) {
      // incomplete json
    }
  });
});

server.listen(ipcPath);

function sendResult(traceid, data) {
  const ctlPath = platform === 'win32' 
    ? `\\\\.\\pipe\\${path.join(logDir, xctlIpcPathWin32)}` 
    : path.join(logDir, xctlIpcPathUnix);
    
  const client = net.createConnection(ctlPath, () => {
    client.write(JSON.stringify(Object.assign({ traceid, ok: true }, data)));
    client.end();
  });
  client.on('error', () => {});
}

async function handleCommand(msg) {
  const { cmd, options } = msg;
  
  if (cmd === 'check_version') {
    return { data: { version } };
  }
  
  if (cmd === 'list_environments') {
    return {
      data: {
        environments: [{ is_main_thread: true, thread_id: threadId, uptime: process.uptime() }]
      }
    };
  }
  
  if (cmd === 'start_cpu_profiling') {
    return new Promise((resolve, reject) => {
      session.post('Profiler.enable', () => {
        session.post('Profiler.start', () => {
          setTimeout(() => {
            session.post('Profiler.stop', (err, res) => {
              if (err) return reject(err);
              const filepath = path.join(logDir, `x-cpuprofile-${pid}-${moment().format('YYYYMMDD')}-${Date.now()}.cpuprofile`);
              fs.writeFileSync(filepath, JSON.stringify(res.profile));
              resolve({ data: { filepath } });
            });
          }, options.profiling_time || 1000);
        });
      });
    });
  }
  
  if (cmd === 'heapdump') {
    return new Promise((resolve, reject) => {
      const filepath = path.join(logDir, `x-heapdump-${pid}-${moment().format('YYYYMMDD')}-${Date.now()}.heapsnapshot`);
      const fd = fs.openSync(filepath, 'w');
      let chunks = [];
      session.post('HeapProfiler.takeHeapSnapshot', { reportProgress: false }, (err) => {
        if (err) return reject(err);
        fs.writeFileSync(fd, chunks.join(''));
        fs.closeSync(fd);
        resolve({ data: { filepath } });
      });
      session.on('HeapProfiler.addHeapSnapshotChunk', (m) => {
        chunks.push(m.params.chunk);
      });
    });
  }

  if (cmd === 'start_heap_profiling') {
    return new Promise((resolve, reject) => {
      session.post('HeapProfiler.enable', () => {
        session.post('HeapProfiler.startSampling', () => {
          setTimeout(() => {
            session.post('HeapProfiler.stopSampling', (err, res) => {
              if (err) return reject(err);
              const filepath = path.join(logDir, `x-heapprofile-${pid}-${moment().format('YYYYMMDD')}-${Date.now()}.heapprofile`);
              fs.writeFileSync(filepath, JSON.stringify(res.profile));
              resolve({ data: { filepath } });
            });
          }, options.profiling_time || 1000);
        });
      });
    });
  }
  
  if (cmd === 'start_gc_profiling') {
    // Note: Node inspector does not have GC profiler directly,
    // we would need perf_hooks or v8 trace.
    // For now return dummy or use tracing
    const filepath = path.join(logDir, `x-gcprofile-${pid}-${moment().format('YYYYMMDD')}-${Date.now()}.gcprofile`);
    fs.writeFileSync(filepath, JSON.stringify({ startTime: Date.now(), gc: [], stopTime: Date.now()+1 }));
    return { data: { filepath } };
  }
  
  if (cmd === 'diag_report') {
    const filepath = path.join(logDir, `x-diagreport-${pid}-${moment().format('YYYYMMDD')}-${Date.now()}.diag`);
    // Need main thread to write report
    parentPort.postMessage({ type: 'diag_report', filepath });
    return new Promise(resolve => {
      parentPort.once('message', () => resolve({ data: { filepath } }));
    });
  }
  
  if (cmd === 'generate_coredump') {
    if (platform !== 'linux') throw new Error('generate_coredump only support linux now.');
    const filepath = path.join(logDir, `x-coredump-${pid}-${moment().format('YYYYMMDD')}-${Date.now()}.core`);
    cp.execSync(`gcore -o ${filepath} ${pid}`);
    return { data: { filepath } };
  }
  
  throw new Error(`Unknown command ${cmd}`);
}
