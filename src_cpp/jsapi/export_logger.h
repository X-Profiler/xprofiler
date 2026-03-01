#ifndef XPROFILER_SRC_JSAPI_LOGGER_H
#define XPROFILER_SRC_JSAPI_LOGGER_H

#include "logger.h"
#include "nan.h"

namespace xprofiler {
void JsInfo(const Nan::FunctionCallbackInfo<v8::Value>& info);
void JsError(const Nan::FunctionCallbackInfo<v8::Value>& info);
void JsDebug(const Nan::FunctionCallbackInfo<v8::Value>& info);
}  // namespace xprofiler

#endif /* XPROFILER_SRC_JSAPI_LOGGER_H */
