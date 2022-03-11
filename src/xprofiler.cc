#include "commands/listener.h"
#include "configure.h"
#include "environment_data.h"
#include "hooks/set_hooks.h"
#include "library/common.h"
#include "logbypass/http.h"
#include "logbypass/log.h"
#include "logger.h"
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

namespace per_process {
ProcessData process_data;
}

NODE_C_CTOR(Main) {
  // init global variables
  InitOnceLoadTime();
  InitOnceLogger();
}

#define CREATE_JS_BINDING(js_func, native_func)       \
  Set(target, New<String>(#js_func).ToLocalChecked(), \
      GetFunction(New<FunctionTemplate>(native_func)).ToLocalChecked())

NAN_MODULE_INIT(Initialize) {
  Isolate* isolate = target->GetIsolate();
  EnvironmentData::Create(isolate);

  // config
  CREATE_JS_BINDING(configure, Configure);
  CREATE_JS_BINDING(getConfig, GetConfig);

  // js logger
  CREATE_JS_BINDING(info, JsInfo);
  CREATE_JS_BINDING(error, JsError);
  CREATE_JS_BINDING(debug, JsDebug);

  // performance log
  CREATE_JS_BINDING(runLogBypass, RunLogBypass);

  // commands listener
  CREATE_JS_BINDING(checkSocketPath, CheckSocketPath);
  CREATE_JS_BINDING(runCommandsListener, RunCommandsListener);

  // set hooks
  CREATE_JS_BINDING(setHooks, SetHooks);

  // http status
  CREATE_JS_BINDING(addLiveRequest, AddLiveRequest);
  CREATE_JS_BINDING(addCloseRequest, AddCloseRequest);
  CREATE_JS_BINDING(addSentRequest, AddSentRequest);
  CREATE_JS_BINDING(addRequestTimeout, AddRequestTimeout);
  CREATE_JS_BINDING(addHttpStatusCode, AddHttpStatusCode);
}

// TODO(legendecas): declare context aware when ready.
NODE_MODULE(xprofiler, Initialize)
}  // namespace xprofiler
