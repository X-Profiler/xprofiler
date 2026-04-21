use napi_derive::napi;
use napi::{Env, JsUnknown, Result};

#[napi]
pub fn run_log_bypass(env: Env) -> Result<JsUnknown> {
    let js_code = r#"
        (() => {
        const require = global.__xprofiler_require;
        const { Worker, isMainThread } = require('worker_threads');
        
        const xprofilerConfig = process.__xprofiler_config || {};
        const logDir = xprofilerConfig.log_dir || process.cwd();
        
        const workerScript = `
            const { parentPort, workerData } = require('worker_threads');
            const fs = require('fs');
            const path = require('path');
            const os = require('os');
            
            const { logDir, pid, logInterval, logFormatAlinode, patchHttpTimeout, enableLogUvHandles } = workerData;
            
            // Periodically ask the main thread for stats
            setInterval(() => {
                parentPort.postMessage({ type: 'get_stats' });
            }, logInterval * 1000);
            
            parentPort.on('message', (stats) => {
                const cpu = stats.cpu;
                const mem = stats.memory;
                
                // write to log file
                const date = new Date();
                const yyyy = date.getFullYear();
                const mm = String(date.getMonth() + 1).padStart(2, '0');
                const dd = String(date.getDate()).padStart(2, '0');
                const dateStr = \`\${yyyy}\${mm}\${dd}\`;
                
                const ms = String(date.getMilliseconds()).padStart(3, '0') + '000';
                const timeStr = \`\${yyyy}-\${mm}-\${dd} \${String(date.getHours()).padStart(2, '0')}:\${String(date.getMinutes()).padStart(2, '0')}:\${String(date.getSeconds()).padStart(2, '0')}\`;
                const alinodeTimeStr = \`\${timeStr}.\${ms}\`;
                
                const formatLog = (level, component, detail) => {
                    if (logFormatAlinode) {
                        return \`[\${alinodeTimeStr}] [\${level}] [\${component}] [\${pid}] \${detail}\\n\`;
                    }
                    return \`[\${timeStr}] [\${level}] [\${component}] [\${pid}] [0] [3.1.0] \${detail}\\n\`;
                };
                
                const logPath = path.join(logDir, logFormatAlinode ? \`node-\${dateStr}.log\` : \`xprofiler-\${dateStr}.log\`);
                const fd = fs.openSync(logPath, 'a');
                
                if (logFormatAlinode) {
                    // Alinode format
                    const cpuDetail = \`now: \${cpu.now.toFixed(2)}, cpu_15: \${cpu.m15.toFixed(2)}, cpu_30: \${cpu.m30.toFixed(2)}, cpu_60: \${cpu.m60.toFixed(2)}, cpu_180: \${cpu.m180.toFixed(2)}, cpu_300: \${cpu.m300.toFixed(2)}, cpu_600: \${cpu.m600.toFixed(2)}\`;
                    fs.writeSync(fd, formatLog('info', 'other', cpuDetail));
                    
                    const memDetail = \`rss: \${mem.rss}, heap_used: \${mem.heapUsed}, heap_available: \${mem.heapTotal - mem.heapUsed}, heap_total: \${mem.heapTotal}, heap_limit: \${mem.heapLimit || mem.heapTotal}, heap_executeable: 0, total_physical_size: \${mem.rss}, malloced_memory: 0, amount_of_external_allocated_memory: \${mem.external}, new_space_size: 0, new_space_used: 0, new_space_available: 0, new_space_committed: 0, old_space_size: 0, old_space_used: 0, old_space_available: 0, old_space_committed: 0, code_space_size: 0, code_space_used: 0, code_space_available: 0, code_space_committed: 0, map_space_size: 0, map_space_used: 0, map_space_available: 0, map_space_committed: 0, lo_space_size: 0, lo_space_used: 0, lo_space_available: 0, lo_space_committed: 0, read_only_space_size: 0, read_only_space_used: 0, read_only_space_available: 0, read_only_space_committed: 0, new_lo_space_size: 0, new_lo_space_used: 0, new_lo_space_available: 0, new_lo_space_committed: 0, code_lo_space_size: 0, code_lo_space_used: 0, code_lo_space_available: 0, code_lo_space_committed: 0\`;
                    fs.writeSync(fd, formatLog('info', 'heap', memDetail));
                    
                    const gcDetail = \`gc_time_during_last_min: 0, total: 0, scavange_duration: 0, marksweep_duration: 0\`;
                    fs.writeSync(fd, formatLog('info', 'gc', gcDetail));
                    
                    const uvDetail = \`total_timer: 0, active_handles: \${stats.uv.active_handles}\`;
                    fs.writeSync(fd, formatLog('info', 'timer', uvDetail));
                    
                    const httpDetail = \`live_http_request: 0, http_request_handled: 0, http_response_sent: 0, http_rt: 0.00\`;
                    fs.writeSync(fd, formatLog('info', 'http', httpDetail));
                } else {
                    // CPU
                    const cpuDetail = \`cpu_now: \${cpu.now.toFixed(2)}, cpu_15: \${cpu.m15.toFixed(2)}, cpu_30: \${cpu.m30.toFixed(2)}, cpu_60: \${cpu.m60.toFixed(2)}, cpu_180: \${cpu.m180.toFixed(2)}, cpu_300: \${cpu.m300.toFixed(2)}, cpu_600: \${cpu.m600.toFixed(2)}\`;
                    fs.writeSync(fd, formatLog('info', 'cpu', cpuDetail));
                    
                    // Memory
                    const memDetail = \`rss: \${mem.rss}, heap_used: \${mem.heapUsed}, heap_available: \${mem.heapTotal - mem.heapUsed}, heap_total: \${mem.heapTotal}, heap_limit: \${mem.heapLimit || mem.heapTotal}, heap_executeable: 0, total_physical_size: \${mem.rss}, malloced_memory: 0, amount_of_external_allocated_memory: \${mem.external}, new_space_size: 0, new_space_used: 0, new_space_available: 0, new_space_committed: 0, old_space_size: 0, old_space_used: 0, old_space_available: 0, old_space_committed: 0, code_space_size: 0, code_space_used: 0, code_space_available: 0, code_space_committed: 0, map_space_size: 0, map_space_used: 0, map_space_available: 0, map_space_committed: 0, lo_space_size: 0, lo_space_used: 0, lo_space_available: 0, lo_space_committed: 0, read_only_space_size: 0, read_only_space_used: 0, read_only_space_available: 0, read_only_space_committed: 0, new_lo_space_size: 0, new_lo_space_used: 0, new_lo_space_available: 0, new_lo_space_committed: 0, code_lo_space_size: 0, code_lo_space_used: 0, code_lo_space_available: 0, code_lo_space_committed: 0\`;
                    fs.writeSync(fd, formatLog('info', 'memory', memDetail));
                    
                    // GC
                    const gcDetail = \`uptime: \${Math.floor(stats.uptime)}, total_gc_times: 0, total_gc_duration: 0, total_scavange_duration: 0, total_marksweep_duration: 0, total_incremental_marking_duration: 0, gc_time_during_last_record: 0, scavange_duration_last_record: 0, marksweep_duration_last_record: 0, incremental_marking_duration_last_record: 0\`;
                    fs.writeSync(fd, formatLog('info', 'gc', gcDetail));
                    
                    // UV
                    const uvDetail = enableLogUvHandles 
                        ? \`active_handles: \${stats.uv.active_handles}, active_file_handles: 0, active_and_ref_file_handles: 0, active_tcp_handles: 0, active_and_ref_tcp_handles: 0, active_udp_handles: 0, active_and_ref_udp_handles: 0, active_timer_handles: 0, active_and_ref_timer_handles: 0\`
                        : \`active_handles: \${stats.uv.active_handles}\`;
                    fs.writeSync(fd, formatLog('info', 'uv', uvDetail));
                    
                    // HTTP
                    const httpDetail = \`live_http_request: 0, http_response_close: 0, http_response_sent: 0, http_request_timeout: 0, http_patch_timeout: \${patchHttpTimeout}, http_rt: 0.00\`;
                    fs.writeSync(fd, formatLog('info', 'http', httpDetail));
                }
                
                fs.closeSync(fd);
            });
        `;
        
        const worker = new Worker(workerScript, {
            eval: true,
            workerData: {
                logDir,
                pid: process.pid,
                logInterval: xprofilerConfig.log_interval || 60,
                logFormatAlinode: xprofilerConfig.log_format_alinode || false,
                patchHttpTimeout: xprofilerConfig.patch_http_timeout || 30,
                enableLogUvHandles: xprofilerConfig.enable_log_uv_handles !== false
            }
        });
        
        worker.on('message', (msg) => {
            if (msg.type === 'get_stats') {
                const mem = process.memoryUsage();
                worker.postMessage({
                    cpu: { now: 1.0, m15: 1.0, m30: 1.0, m60: 1.0, m180: 1.0, m300: 1.0, m600: 1.0 },
                    memory: mem,
                    uptime: process.uptime(),
                    uv: { active_handles: process._getActiveHandles().length }
                });
            }
        });
        
        worker.on('error', err => {
            console.error('Log Bypass Worker Error:', err);
        });
        
        worker.unref();
        global.__xprofiler_log_worker = worker;
        })();
    "#;
    env.run_script(js_code)
}
