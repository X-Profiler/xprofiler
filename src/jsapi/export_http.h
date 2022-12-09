#ifndef XPROFILER_SRC_JSAPI_HTTP_H
#define XPROFILER_SRC_JSAPI_HTTP_H

#include "nan.h"
#include "xpf_mutex-inl.h"

namespace xprofiler {
void SetHttpConfig(const Nan::FunctionCallbackInfo<v8::Value>& info);
void AddLiveRequest(const Nan::FunctionCallbackInfo<v8::Value>& info);
void AddCloseRequest(const Nan::FunctionCallbackInfo<v8::Value>& info);
void AddSentRequest(const Nan::FunctionCallbackInfo<v8::Value>& info);
void AddRequestTimeout(const Nan::FunctionCallbackInfo<v8::Value>& info);
void AddHttpStatusCode(const Nan::FunctionCallbackInfo<v8::Value>& info);
void AddHttpProfilingDetail(const Nan::FunctionCallbackInfo<v8::Value>& info);
}  // namespace xprofiler

#endif /* XPROFILER_SRC_LOGBYPASS_HTTP_H */
