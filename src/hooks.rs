use napi_derive::napi;
use napi::{Env, JsObject, Result, JsUnknown};

#[napi]
pub fn set_hooks(env: Env) -> Result<JsUnknown> {
    let js_code = r#"
        (() => {
        const require = global.__xprofiler_require;
        const fs = require('fs');
        const path = require('path');
        const moment = require('moment');
        const xprofilerConfig = process.__xprofiler_config || {};
        
        if (xprofilerConfig.enable_fatal_error_report) {
            process.report.reportOnFatalError = true;
            process.report.directory = xprofilerConfig.log_dir;
            process.report.filename = `x-fatal-error-${process.pid}-${moment().format('YYYYMMDD')}-${Date.now()}.diag`;
        } else {
            process.report.reportOnFatalError = false;
        }

        if (xprofilerConfig.enable_fatal_error_coredump) {
            // Cannot enable coredump on windows/darwin via JS easily.
            // In Node, process.abort() generates a core dump if ulimit -c is unlimited.
            process.report.reportOnSignal = true; // or just write a file indicating coredump logic
        }

        // Heap limit hook using v8 flags
        if (xprofilerConfig.enable_auto_incr_heap_limit) {
            // Note: v8.setFlagsFromString is available
            const v8 = require('v8');
            v8.setFlagsFromString(`--heapsnapshot-near-heap-limit=${xprofilerConfig.auto_incr_heap_limit_size || 256}`);
        }
        })();
    "#;
    env.run_script(js_code)
}
