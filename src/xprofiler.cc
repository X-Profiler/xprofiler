#include "jsapi/include/export_configure.h"
#include "jsapi/include/export_environment.h"
#include "jsapi/include/export_hooks.h"
#include "jsapi/include/export_http.h"
#include "jsapi/include/export_logger.h"
#include "jsapi/include/export_thread_listener.h"
#include "jsapi/include/export_thread_logbypass.h"
#include "library/common.h"
#include "nan.h"
#include "platform/platform.h"
#include "process_data.h"

namespace xprofiler {
using Nan::GetFunction;
using Nan::New;
using Nan::Set;
using v8::FunctionTemplate;
using v8::Isolate;
using v8::String;

NODE_C_CTOR(Main) {
  // init global variables
  InitOnceLoadTime();
}

#define CREATE_JS_BINDING(js_func, native_func)       \
  Set(target, New<String>(#js_func).ToLocalChecked(), \
      GetFunction(New<FunctionTemplate>(native_func)).ToLocalChecked())

NAN_MODULE_INIT(Initialize) {
  Isolate* isolate = target->GetIsolate();
  EnvironmentData::Create(isolate);

  // environment
  CREATE_JS_BINDING(setup, JsSetupEnvironmentData);

  // config
  CREATE_JS_BINDING(configure, Configure);
  CREATE_JS_BINDING(getConfig, GetConfig);

  // logger
  CREATE_JS_BINDING(info, JsInfo);
  CREATE_JS_BINDING(error, JsError);
  CREATE_JS_BINDING(debug, JsDebug);

  // set hooks
  CREATE_JS_BINDING(setHooks, SetHooks);

  // http status
  CREATE_JS_BINDING(addLiveRequest, AddLiveRequest);
  CREATE_JS_BINDING(addCloseRequest, AddCloseRequest);
  CREATE_JS_BINDING(addSentRequest, AddSentRequest);
  CREATE_JS_BINDING(addRequestTimeout, AddRequestTimeout);
  CREATE_JS_BINDING(addHttpStatusCode, AddHttpStatusCode);

  // performance log
  CREATE_JS_BINDING(runLogBypass, RunLogBypass);

  // commands listener
  CREATE_JS_BINDING(checkSocketPath, CheckSocketPath);
  CREATE_JS_BINDING(runCommandsListener, RunCommandsListener);
}

NODE_MODULE_CONTEXT_AWARE(xprofiler, Initialize)
}  // namespace xprofiler
