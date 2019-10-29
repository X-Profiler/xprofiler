#ifndef _SRC_CONFIGURE_H
#define _SRC_CONFIGURE_H

#include "common.h"
#include "nan.h"

namespace xprofiler {
using Nan::FunctionCallbackInfo;
using v8::Value;

// normal external
LOG_LEVEL GetLogLevel();
bool GetFormatAsAlinode();
std::string GetLogDir();

// javascript accessible
void Configure(const FunctionCallbackInfo<Value> &info);
void GetConfig(const FunctionCallbackInfo<Value> &info);
} // namespace xprofiler

#endif