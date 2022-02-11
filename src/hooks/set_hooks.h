#ifndef XPROFILER_SRC_HOOKS_SET_HOOKS_H
#define XPROFILER_SRC_HOOKS_SET_HOOKS_H

#include "nan.h"

namespace xprofiler {
void SetHooks(const Nan::FunctionCallbackInfo<v8::Value>& info);
}  // namespace xprofiler

#endif /* XPROFILER_SRC_HOOKS_SET_HOOKS_H */
