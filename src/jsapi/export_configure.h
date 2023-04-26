#pragma once

#include "nan.h"

namespace xprofiler {
void Configure(const Nan::FunctionCallbackInfo<v8::Value>& info);
void GetConfig(const Nan::FunctionCallbackInfo<v8::Value>& info);
}  // namespace xprofiler
