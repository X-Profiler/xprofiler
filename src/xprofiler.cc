#include "commands/listener.h"
#include "configure.h"
#include "library/common.h"
#include "logbypass/log.h"
#include "logger.h"
#include "nan.h"

namespace xprofiler {
using Nan::GetFunction;
using Nan::New;
using Nan::Set;
using v8::FunctionTemplate;
using v8::String;

NAN_MODULE_INIT(Initialize) {
  // init global variables
  InitGlobalVariables();

#define V(js_func, native_func)                       \
  Set(target, New<String>(#js_func).ToLocalChecked(), \
      GetFunction(New<FunctionTemplate>(native_func)).ToLocalChecked());
  // config
  V(configure, Configure)
  V(getConfig, GetConfig)

  // js logger
  V(info, JsInfo)
  V(error, JsError)
  V(debug, JsDebug)

  // performance log
  V(runLogBypass, RunLogBypass)

  // commands listener
  V(runCommandsListener, RunCommandsListener)
#undef V
}

NODE_MODULE(xprofiler, Initialize)
}  // namespace xprofiler
