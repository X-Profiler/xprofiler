#ifndef XPROFILER_SRC_LOGBYPASS_LOG_H
#define XPROFILER_SRC_LOGBYPASS_LOG_H

#include "nan.h"

namespace xprofiler {
// javascript-accessible
void RunLogBypass(const Nan::FunctionCallbackInfo<v8::Value>& info);
void StopLogBypass();
}  // namespace xprofiler

#endif /* XPROFILER_SRC_LOGBYPASS_LOG_H */
