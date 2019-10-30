#include "nan.h"

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
  // config
  JS_METHOD(configure, Configure);
  JS_METHOD(getConfig, GetConfig);

  // js logger
  JS_METHOD(info, JsInfo);
  JS_METHOD(error, JsError);
  JS_METHOD(debug, JsDebug);

  // performance log
  JS_METHOD(runLogBypass, RunLogBypass);
}

NODE_MODULE(xprofiler, Initialize)
} // namespace xprofiler
