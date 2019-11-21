#ifndef _SRC_COMMANDS_LISTENER_H
#define _SRC_COMMANDS_LISTENER_H

#include "nan.h"

namespace xprofiler {
using Nan::FunctionCallbackInfo;
using v8::Value;

// javascript-accessible
void RunCommandsListener(const FunctionCallbackInfo<Value> &info);
}  // namespace xprofiler

#endif