#ifndef XPROFILER_SRC_LOGGER_H
#define XPROFILER_SRC_LOGGER_H

#include "library/common.h"
#include "nan.h"

namespace xprofiler {
using Nan::FunctionCallbackInfo;
using v8::Value;

// xprofiler logger
enum LOG_LEVEL { LOG_INFO, LOG_ERROR, LOG_DEBUG };
enum LOG_TYPE { LOG_TO_FILE, LOG_TO_TTL };

void InitOnceLogger();

// normal external
void Info(const char *log_type, const char *format, ...);
void Error(const char *log_type, const char *format, ...);
void Debug(const char *log_type, const char *format, ...);

// javascript accessible
void JsInfo(const FunctionCallbackInfo<Value> &info);
void JsError(const FunctionCallbackInfo<Value> &info);
void JsDebug(const FunctionCallbackInfo<Value> &info);
}  // namespace xprofiler

#endif /* XPROFILER_SRC_LOGGER_H */
