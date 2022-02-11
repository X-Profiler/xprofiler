#include "set_hooks.h"

#include "configure-inl.h"
#include "fatal_error.h"

namespace xprofiler {
void SetHooks(const FunctionCallbackInfo<Value>& info) {
  // set fatal error hook
  if (GetEnableFatalErrorHook()) {
    SetFatalErrorHandler();
  }
}
}  // namespace xprofiler
