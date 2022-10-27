#include "include/export_logger.h"

#include "environment_data.h"

namespace xprofiler {
using Nan::FunctionCallbackInfo;
using Nan::New;
using Nan::ThrowTypeError;
using Nan::To;
using Nan::Utf8String;
using std::string;
using std::to_string;
using v8::Local;
using v8::String;
using v8::Value;

#define JS_LOG_WITH_LEVEL(level)                                               \
  if (!info[0]->IsString() || !info[1]->IsString()) {                          \
    ThrowTypeError(                                                            \
        New<String>("log type and content must be string!").ToLocalChecked()); \
    return;                                                                    \
  }                                                                            \
  EnvironmentData* env_data = EnvironmentData::GetCurrent(info);               \
                                                                               \
  Local<String> component_string = To<String>(info[0]).ToLocalChecked();       \
  Utf8String component(component_string);                                      \
  Local<String> log_content_string = To<String>(info[1]).ToLocalChecked();     \
  Utf8String log_content(log_content_string);                                  \
  Log(level, *component, env_data->thread_id(), *log_content);

void JsInfo(const FunctionCallbackInfo<Value>& info) {
  JS_LOG_WITH_LEVEL(LOG_INFO)
}

void JsError(const FunctionCallbackInfo<Value>& info) {
  JS_LOG_WITH_LEVEL(LOG_ERROR)
}

void JsDebug(const FunctionCallbackInfo<Value>& info) {
  JS_LOG_WITH_LEVEL(LOG_DEBUG)
}

};  // namespace xprofiler
