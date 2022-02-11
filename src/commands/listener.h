#ifndef XPROFILER_SRC_COMMANDS_LISTENER_H
#define XPROFILER_SRC_COMMANDS_LISTENER_H

#include "nan.h"

namespace xprofiler {
using Nan::FunctionCallbackInfo;
using v8::Value;

// javascript-accessible
void RunCommandsListener(const FunctionCallbackInfo<Value>& info);
void StopCommandsListener();
}  // namespace xprofiler

#endif /* XPROFILER_SRC_COMMANDS_LISTENER_H */
