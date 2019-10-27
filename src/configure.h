#ifndef _SRC_CONFIGURE_H
#define _SRC_CONFIGURE_H

#include "nan.h"

namespace xprofiler {
using Nan::FunctionCallbackInfo;
using v8::Value;

// javascript-accessible
void Configure(const FunctionCallbackInfo<Value> &info);
void GetConfig(const FunctionCallbackInfo<Value> &info);
} // namespace xprofiler

#endif