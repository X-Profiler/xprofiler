#include "export_hooks.h"

#include "configure-inl.h"
#include "hooks/fatal_error.h"
#include "hooks/heap_limit.h"

namespace xprofiler {
using Nan::FunctionCallbackInfo;
using v8::Value;

void SetHooks(const FunctionCallbackInfo<Value>& info) {
  // set fatal error hook
  if (GetConfig<bool>("enable_fatal_error_hook")) {
    SetFatalErrorHandler(info.GetIsolate());
  }

  // set auto increas heap limit hook
  if (GetConfig<bool>("enable_auto_incr_heap_limit")) {
    AutoIncreaseHeapLimit(info.GetIsolate());
  }
}
}  // namespace xprofiler
