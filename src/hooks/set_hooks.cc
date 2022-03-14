#include "set_hooks.h"

#include "configure-inl.h"
#include "fatal_error.h"

namespace xprofiler {
using Nan::FunctionCallbackInfo;
using v8::Value;

void SetHooks(const FunctionCallbackInfo<Value>& info) {
  // set fatal error hook
  if (GetEnableFatalErrorHook()) {
    SetFatalErrorHandler(info.GetIsolate());
  }
}
}  // namespace xprofiler
