#ifndef _SRC_LOGBYPASS_HTTP_H
#define _SRC_LOGBYPASS_HTTP_H

#include "nan.h"

namespace xprofiler {
using Nan::FunctionCallbackInfo;
using v8::Value;

int InitHttpStatus();
void WriteHttpStatus(bool log_format_alinode);

// javascript-accessible
void AddLiveRequest(const FunctionCallbackInfo<Value> &info);
void AddCloseRequest(const FunctionCallbackInfo<Value> &info);
void AddSentRequest(const FunctionCallbackInfo<Value> &info);
}  // namespace xprofiler

#endif
