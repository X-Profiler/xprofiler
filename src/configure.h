#ifndef _SRC_CONFIGURE_H
#define _SRC_CONFIGURE_H

#include "library/common.h"
#include "library/error.h"
#include "nan.h"

namespace xprofiler {
using Nan::FunctionCallbackInfo;
using std::string;
using v8::Value;

// normal external
#define V(ret, func) \
  ret Get##func();   \
  void Set##func(ret value);
V(string, LogDir)
V(uint32_t, LogInterval)
V(bool, FormatAsAlinode)
V(bool, EnableLogUvHandles)
V(LOG_LEVEL, LogLevel)
V(LOG_TYPE, LogType)
V(bool, EnableFatalErrorHook)
#undef V

// javascript accessible
void Configure(const FunctionCallbackInfo<Value> &info);
void GetConfig(const FunctionCallbackInfo<Value> &info);
}  // namespace xprofiler

#endif