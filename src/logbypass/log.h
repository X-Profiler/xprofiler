#ifndef _SRC_LOGBYPASS_LOG_H
#define _SRC_LOGBYPASS_LOG_H

#include "nan.h"

namespace xprofiler {
using Nan::FunctionCallbackInfo;
using v8::Value;

// javascript-accessible
void RunLogBypass(const FunctionCallbackInfo<Value> &info);
}  // namespace xprofiler

#endif