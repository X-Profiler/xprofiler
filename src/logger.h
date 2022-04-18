#ifndef XPROFILER_SRC_LOGGER_H
#define XPROFILER_SRC_LOGGER_H

#include "library/common.h"
#include "nan.h"

namespace xprofiler {
// xprofiler logger
enum LOG_LEVEL { LOG_INFO, LOG_ERROR, LOG_DEBUG };
enum LOG_TYPE { LOG_TO_FILE, LOG_TO_TTY };

void InitOnceLogger();

// normal external
void Info(const char* component, const char* format, ...);
void Error(const char* component, const char* format, ...);
void Debug(const char* component, const char* format, ...);

void InfoT(const char* component, ThreadId thread_id, const char* format, ...);
void ErrorT(const char* component, ThreadId thread_id, const char* format, ...);
void DebugT(const char* component, ThreadId thread_id, const char* format, ...);

// javascript accessible
void JsInfo(const Nan::FunctionCallbackInfo<v8::Value>& info);
void JsError(const Nan::FunctionCallbackInfo<v8::Value>& info);
void JsDebug(const Nan::FunctionCallbackInfo<v8::Value>& info);
}  // namespace xprofiler

#endif /* XPROFILER_SRC_LOGGER_H */
