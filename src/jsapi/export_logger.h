#pragma once

#include "logger.h"
#include "nan.h"

namespace xprofiler {
void JsInfo(const Nan::FunctionCallbackInfo<v8::Value>& info);
void JsError(const Nan::FunctionCallbackInfo<v8::Value>& info);
void JsDebug(const Nan::FunctionCallbackInfo<v8::Value>& info);
}  // namespace xprofiler
