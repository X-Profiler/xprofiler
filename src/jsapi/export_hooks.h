#pragma once

#include "nan.h"

namespace xprofiler {
void SetHooks(const Nan::FunctionCallbackInfo<v8::Value>& info);
}
