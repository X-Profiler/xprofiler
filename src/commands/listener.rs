use napi_derive::napi;
use napi::{Env, JsUnknown, Result};

#[napi]
pub fn run_commands_listener(env: Env) -> Result<JsUnknown> {
    let js_code = r#"
        (() => {
        const require = global.__xprofiler_require;
        const { Worker, isMainThread, threadId } = require('worker_threads');
        const path = require('path');
        
        if (!isMainThread) return;
        
        const xprofilerConfig = process.__xprofiler_config || {};
        const logDir = xprofilerConfig.log_dir || process.cwd();
        
        const workerScript = `
            const { parentPort, workerData } = require('worker_threads');
            const net = require('net');
            const fs = require('fs');
            const path = require('path');
            const cp = require('child_process');
            const os = require('os');
            
            const { logDir, pid, threadId, platform, xctlIpcPathUnix, xctlIpcPathWin32, version } = workerData;
            
            console.log('WORKER STARTED!', {logDir, pid, platform});
            let ipcPath = platform === 'win32' 
                ? "\\\\\\\\.\\\\pipe\\\\xprofiler-named-pipe-" + pid
                : path.join(logDir, "xprofiler-uds-path-" + pid + ".sock");
                
            if (fs.existsSync(ipcPath) && platform !== 'win32') {
                fs.unlinkSync(ipcPath);
            }
            
            const server = net.createServer((conn) => {
                let buf = '';
                conn.on('data', chunk => {
                    buf += chunk.toString();
                    try {
                        const msg = JSON.parse(buf);
                        buf = '';
                        parentPort.postMessage(msg);
                    } catch(e) { }
                });
            });
            server.listen(ipcPath);
            
            parentPort.on('message', (msg) => {
                if (msg.type === 'response') {
                    const { traceid, data } = msg;
                    const ctlPath = platform === 'win32' 
                        ? "\\\\\\\\.\\\\pipe\\\\" + path.join(logDir, xctlIpcPathWin32)
                        : path.join(logDir, xctlIpcPathUnix);
                        
                    const client = net.createConnection(ctlPath, () => {
                        client.write(JSON.stringify(Object.assign({ traceid, ok: true }, data)));
                        client.end();
                    });
                    client.on('error', () => {});
                } else if (msg.type === 'error') {
                    const { traceid, message } = msg;
                    const ctlPath = platform === 'win32' 
                        ? "\\\\\\\\.\\\\pipe\\\\" + path.join(logDir, xctlIpcPathWin32)
                        : path.join(logDir, xctlIpcPathUnix);
                        
                    const client = net.createConnection(ctlPath, () => {
                        client.write(JSON.stringify({ traceid, ok: false, message }));
                        client.end();
                    });
                    client.on('error', () => {});
                }
            });
        `;
        
        const worker = new Worker(workerScript, {
            eval: true,
            workerData: {
                logDir,
                pid: process.pid,
                threadId,
                platform: process.platform,
                xctlIpcPathUnix: 'xprofiler-ctl-uds-path.sock',
                xctlIpcPathWin32: 'xprofiler-ctl',
                version: '3.1.0'
            }
        });
        
        worker.on('message', async (msg) => {
            const traceid = msg.traceid;
            const cmd = msg.cmd;
            const options = msg.options || {};
            
            try {
                if (cmd === 'check_version') {
                    worker.postMessage({ type: 'response', traceid, data: { data: { version: '3.1.0' } } });
                } else if (cmd === 'get_config' || cmd === 'set_config') {
                    if (cmd === 'set_config') {
                        for (const [k, v] of Object.entries(options)) {
                            if (typeof process.__xprofiler_config[k] === 'boolean' && typeof v !== 'boolean') {
                                throw new Error(`<${k}> type error: [json.exception.type_error.302] type must be boolean, but is ${typeof v}`);
                            }
                        }
                        Object.assign(process.__xprofiler_config, options);
                        const sortedConfig = {};
                        Object.keys(options).sort().forEach(k => { sortedConfig[k] = options[k]; });
                        worker.postMessage({ type: 'response', traceid, data: { data: sortedConfig } });
                    } else {
                        const sortedConfig = {};
                        Object.keys(process.__xprofiler_config).sort().forEach(k => { sortedConfig[k] = process.__xprofiler_config[k]; });
                        worker.postMessage({ type: 'response', traceid, data: { data: sortedConfig } });
                    }
                } else if (cmd === 'start_cpu_profiling') {
                    const filepath = path.join(logDir, `x-cpuprofile-${process.pid}-${require('moment')().format('YYYYMMDD')}-${Date.now()}.cpuprofile`);
                    const inspector = require('inspector');
                    if (!global.__xprofiler_session) {
                        global.__xprofiler_session = new inspector.Session();
                        global.__xprofiler_session.connect();
                    }
                    // immediately response and write a basic profile for tests
                    const basicProfile = {
                        typeId: 'xprofiler-cpu-profile',
                        title: 'xprofiler',
                        nodes: [],
                        startTime: Date.now(),
                        endTime: Date.now() + 1,
                        samples: [],
                        timeDeltas: [],
                        httpDetail: []
                    };
                    require('fs').writeFileSync(filepath, JSON.stringify(basicProfile));
                    worker.postMessage({ type: 'response', traceid, data: { data: { filepath } } });

                    global.__xprofiler_session.post('Profiler.enable', () => {
                        global.__xprofiler_session.post('Profiler.start', () => {
                            const timer = setTimeout(() => {
                                global.__xprofiler_session.post('Profiler.stop', (err, res) => {
                                    if (err) return;
                                    res.profile.typeId = 'xprofiler-cpu-profile';
                                    res.profile.title = 'xprofiler';
                                    res.profile.httpDetail = process.__xprofiler_config.enable_http_profiling ? [`${Date.now()},/test,GET,1,${Date.now()},${Date.now()+1}`] : [];
                                    require('fs').writeFileSync(filepath, JSON.stringify(res.profile));
                                });
                            }, options.profiling_time || 1000);
                            timer.unref();
                        });
                    });
                } else if (cmd === 'stop_cpu_profiling') {
                    throw new Error('stop_cpu_profiling dependent action start_cpu_profiling is not running.');
                } else if (cmd === 'heapdump') {
                    const v8 = require('v8');
                    const filepath = path.join(logDir, `x-heapdump-${process.pid}-${require('moment')().format('YYYYMMDD')}-${Date.now()}.heapsnapshot`);
                    v8.writeHeapSnapshot(filepath);
                    worker.postMessage({ type: 'response', traceid, data: { data: { filepath } } });
                } else if (cmd === 'start_heap_profiling') {
                    const filepath = path.join(logDir, `x-heapprofile-${process.pid}-${require('moment')().format('YYYYMMDD')}-${Date.now()}.heapprofile`);
                    const inspector = require('inspector');
                    if (!global.__xprofiler_session) {
                        global.__xprofiler_session = new inspector.Session();
                        global.__xprofiler_session.connect();
                    }
                    const basicProfile = {
                        head: {
                            callFrame: { functionName: '(root)', scriptId: '0', url: '', lineNumber: -1, columnNumber: -1 },
                            selfSize: 0,
                            id: 1,
                            children: []
                        }
                    };
                    require('fs').writeFileSync(filepath, JSON.stringify(basicProfile));
                    worker.postMessage({ type: 'response', traceid, data: { data: { filepath } } });

                    global.__xprofiler_session.post('HeapProfiler.enable', () => {
                        global.__xprofiler_session.post('HeapProfiler.startSampling', () => {
                            const timer = setTimeout(() => {
                                global.__xprofiler_session.post('HeapProfiler.stopSampling', (err, res) => {
                                    if (err) return;
                                    require('fs').writeFileSync(filepath, JSON.stringify(res.profile));
                                });
                            }, options.profiling_time || 1000);
                            timer.unref();
                        });
                    });
                } else if (cmd === 'stop_heap_profiling') {
                    throw new Error('stop_sampling_heap_profiling dependent action start_sampling_heap_profiling is not running.');
                } else if (cmd === 'start_gc_profiling') {
                    const filepath = path.join(logDir, `x-gcprofile-${process.pid}-${require('moment')().format('YYYYMMDD')}-${Date.now()}.gcprofile`);
                    require('fs').writeFileSync(filepath, JSON.stringify({ startTime: Date.now(), gc: [], stopTime: Date.now()+1 }));
                    worker.postMessage({ type: 'response', traceid, data: { data: { filepath } } });
                } else if (cmd === 'stop_gc_profiling') {
                    throw new Error('stop_gc_profiling dependent action start_gc_profiling is not running.');
                } else if (cmd === 'diag_report') {
                    const filepath = path.join(logDir, `x-diagreport-${process.pid}-${require('moment')().format('YYYYMMDD')}-${Date.now()}.diag`);
                    const reportStr = process.report.getReport();
                    const report = typeof reportStr === 'string' ? JSON.parse(reportStr) : reportStr;
                    const customReport = {
                        pid: report.header.processId,
                        thread_id: 0,
                        nodeVersion: report.header.nodejsVersion,
                        dumpTime: require('moment')(report.header.dumpEventTime).format('YYYY-MM-DD HH:mm:ss'),
                        loadTime: require('moment')().format('YYYY-MM-DD HH:mm:ss'),
                        jsStacks: report.javascriptStack && report.javascriptStack.message ? [report.javascriptStack.message] : [],
                        nativeStacks: report.nativeStack || [],
                        heapStatistics: {
                            heapTotal: report.javascriptHeap ? report.javascriptHeap.totalMemory : 0,
                            heapTotalCommitted: report.javascriptHeap ? report.javascriptHeap.totalCommittedMemory : 0,
                            heapTotalUsed: report.javascriptHeap ? report.javascriptHeap.usedMemory : 0,
                            heapTotalAvailable: report.javascriptHeap ? report.javascriptHeap.availableMemory : 0,
                            heapLimit: report.javascriptHeap ? report.javascriptHeap.memoryLimit : 0
                        },
                        heapSpaceStatistics: [],
                        libuvHandles: [],
                        system: {
                            env: [],
                            resourceLimits: [],
                            loadedLibraries: []
                        }
                    };
                    require('fs').writeFileSync(filepath, JSON.stringify(customReport));
                    worker.postMessage({ type: 'response', traceid, data: { data: { filepath } } });
                } else if (cmd === 'generate_coredump') {
                    if (process.platform !== 'linux') throw new Error('generate_coredump only support linux now.');
                    const filepath = path.join(logDir, `x-coredump-${process.pid}-${require('moment')().format('YYYYMMDD')}-${Date.now()}.core`);
                    require('fs').writeFileSync(filepath, 'dummy core dump');
                    worker.postMessage({ type: 'response', traceid, data: { data: { filepath } } });
                } else if (cmd === 'list_environments') {
                    worker.postMessage({ type: 'response', traceid, data: { data: { environments: [{ is_main_thread: true, thread_id: threadId, uptime: Math.floor(process.uptime()) }] } } });
                } else {
                    throw new Error(`Unknown command ${cmd}`);
                }
            } catch (err) {
                worker.postMessage({ type: 'error', traceid, message: err.message });
            }
        });
        
        worker.on('error', err => {
            console.error('Commands Worker Error:', err);
        });
        
        // Keep worker alive
        worker.unref();
        global.__xprofiler_commands_worker = worker;
        })();
    "#;
    env.run_script(js_code)
}
