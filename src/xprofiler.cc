#include "nan.h"

#include "commands/listener.h"
#include "common.h"
#include "configure.h"
#include "logbypass/log.h"
#include "logger.h"

namespace xprofiler {
using Nan::GetFunction;
using Nan::New;
using Nan::Set;
using v8::FunctionTemplate;
using v8::String;

#define JS_METHOD(js_func, native_func)                                        \
  Set(target, New<String>(#js_func).ToLocalChecked(),                          \
      GetFunction(New<FunctionTemplate>(native_func)).ToLocalChecked());

NAN_MODULE_INIT(Initialize) {
  // init global variables
  InitGlobalVariables();

  // config
  JS_METHOD(configure, Configure);
  JS_METHOD(getConfig, GetConfig);

  // js logger
  JS_METHOD(info, JsInfo);
  JS_METHOD(error, JsError);
  JS_METHOD(debug, JsDebug);

  // performance log
  JS_METHOD(runLogBypass, RunLogBypass);

  // commands listener
  JS_METHOD(runCommandsListener, RunCommandsListener);
}

NODE_MODULE(xprofiler, Initialize)
} // namespace xprofiler
