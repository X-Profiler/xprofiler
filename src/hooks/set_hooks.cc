#include "set_hooks.h"

#include "../configure.h"
#include "fatal_error.h"
#include "out_of_memory.h"

namespace xprofiler {
void SetHooks(const FunctionCallbackInfo<Value> &info) {
  // set fatal error hook
  if (GetEnableFatalErrorHook()) {
    SetFatalErrorHandler();
  }

  // set oom hook
  if (GetEnableOOMErrorHook()) {
    SetOOMErrorHandler();
  }
}
}  // namespace xprofiler