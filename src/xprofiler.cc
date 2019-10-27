#include "configure.h"
#include "nan.h"

namespace xprofiler {
using Nan::GetFunction;
using Nan::New;
using Nan::Set;
using v8::FunctionTemplate;
using v8::String;

NAN_MODULE_INIT(Initialize) {
  Set(target, New<String>("configure").ToLocalChecked(),
      GetFunction(New<FunctionTemplate>(Configure)).ToLocalChecked());
  Set(target, New<String>("getConfig").ToLocalChecked(),
      GetFunction(New<FunctionTemplate>(GetConfig)).ToLocalChecked());
}

NODE_MODULE(xprofiler, Initialize)
} // namespace xprofiler
