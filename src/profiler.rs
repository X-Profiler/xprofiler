use napi_derive::napi;
use napi::{Env, JsObject, Result, JsFunction, JsUnknown};

#[napi]
pub fn start_cpu_profiling(env: Env, options: JsObject) -> Result<JsUnknown> {
    // We will evaluate JS code to use inspector
    let js_code = r#"
        const inspector = require('inspector');
        if (!global.__xprofiler_session) {
            global.__xprofiler_session = new inspector.Session();
            global.__xprofiler_session.connect();
        }
        global.__xprofiler_session.post('Profiler.enable');
        global.__xprofiler_session.post('Profiler.start');
    "#;
    env.run_script(js_code)
}

#[napi]
pub fn stop_cpu_profiling(env: Env, options: JsObject) -> Result<JsUnknown> {
    let js_code = r#"
        // ...
    "#;
    env.run_script(js_code)
}
