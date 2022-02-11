#ifndef _SRC_HOOKS_SET_HOOKS_H
#define _SRC_HOOKS_SET_HOOKS_H

#include "nan.h"

namespace xprofiler {
using Nan::FunctionCallbackInfo;
using v8::Value;
void SetHooks(const FunctionCallbackInfo<Value>& info);
}  // namespace xprofiler

#endif