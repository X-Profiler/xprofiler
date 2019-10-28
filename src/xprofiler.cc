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

NAN_MODULE_INIT(Initialize) {
  // config
  Set(target, New<String>("configure").ToLocalChecked(),
      GetFunction(New<FunctionTemplate>(Configure)).ToLocalChecked());
  Set(target, New<String>("getConfig").ToLocalChecked(),
      GetFunction(New<FunctionTemplate>(GetConfig)).ToLocalChecked());

  // js logger
  Set(target, New<String>("info").ToLocalChecked(),
      GetFunction(New<FunctionTemplate>(JsInfo)).ToLocalChecked());
  Set(target, New<String>("error").ToLocalChecked(),
      GetFunction(New<FunctionTemplate>(JsError)).ToLocalChecked());
  Set(target, New<String>("debug").ToLocalChecked(),
      GetFunction(New<FunctionTemplate>(JsDebug)).ToLocalChecked());

  // performance log
  Set(target, New<String>("runLogBypass").ToLocalChecked(),
      GetFunction(New<FunctionTemplate>(RunLogBypass)).ToLocalChecked());
}

NODE_MODULE(xprofiler, Initialize)
} // namespace xprofiler
