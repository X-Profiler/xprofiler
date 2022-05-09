#include "configure.h"

#include "process_data.h"
#include "util-inl.h"

namespace xprofiler {
using Nan::FunctionCallbackInfo;
using Nan::Get;
using Nan::New;
using Nan::Set;
using Nan::ThrowTypeError;
using Nan::To;
using Nan::Utf8String;
using std::string;
using v8::Boolean;
using v8::Isolate;
using v8::Local;
using v8::Number;
using v8::Object;
using v8::String;
using v8::Value;

#define LOCAL_VALUE(key)     \
  Local<Value> key##_value = \
      Get(config, OneByteString(isolate, #key)).ToLocalChecked();

#define COVERT_STRING(key)                                                 \
  LOCAL_VALUE(key)                                                         \
  if (key##_value->IsString()) {                                           \
    Local<String> key##_string = To<String>(key##_value).ToLocalChecked(); \
    Utf8String key##_utf8string(key##_string);                             \
    ProcessData::Get()->config_store()->key = *key##_utf8string;           \
  }

#define CONVERT_UINT32(key)                    \
  LOCAL_VALUE(key)                             \
  if (key##_value->IsUint32()) {               \
    ProcessData::Get()->config_store()->key =  \
        To<uint32_t>(key##_value).ToChecked(); \
  }

#define CONVERT_UINT32_WITH_TYPE(key, type)                       \
  LOCAL_VALUE(key)                                                \
  if (key##_value->IsUint32()) {                                  \
    ProcessData::Get()->config_store()->key =                     \
        static_cast<type>(To<uint32_t>(key##_value).ToChecked()); \
  }

#define CONVERT_BOOL(key)                     \
  LOCAL_VALUE(key)                            \
  if (key##_value->IsBoolean()) {             \
    ProcessData::Get()->config_store()->key = \
        To<bool>(key##_value).ToChecked();    \
  }

#define CONFIG_LOCAL_STRING(key, type)      \
  Set(config, OneByteString(isolate, #key), \
      New<type>(ProcessData::Get()->config_store()->key).ToLocalChecked());

#define CONFIG_NATIVE_NUMBER(key, type)     \
  Set(config, OneByteString(isolate, #key), \
      New<type>(ProcessData::Get()->config_store()->key));

void Configure(const FunctionCallbackInfo<Value>& info) {
  Isolate* isolate = info.GetIsolate();
  if (!info[0]->IsObject()) {
    ThrowTypeError(New<String>("config must be object!").ToLocalChecked());
    return;
  }
  Local<Object> config = To<Object>(info[0]).ToLocalChecked();

  COVERT_STRING(log_dir)
  CONVERT_UINT32(log_interval)
  CONVERT_UINT32_WITH_TYPE(log_level, LOG_LEVEL)
  CONVERT_UINT32_WITH_TYPE(log_type, LOG_TYPE)
  CONVERT_BOOL(enable_log_uv_handles)
  CONVERT_BOOL(log_format_alinode)
  CONVERT_BOOL(patch_http)
  CONVERT_UINT32(patch_http_timeout)
  CONVERT_BOOL(check_throw)
  CONVERT_BOOL(enable_fatal_error_hook)
  CONVERT_BOOL(enable_fatal_error_report)
  CONVERT_BOOL(enable_fatal_error_coredump)

  info.GetReturnValue().Set(New<Boolean>(true));
}

void GetConfig(const FunctionCallbackInfo<Value>& info) {
  Isolate* isolate = info.GetIsolate();
  Local<Object> config = New<Object>();

  CONFIG_LOCAL_STRING(log_dir, String)
  CONFIG_NATIVE_NUMBER(log_interval, Number)
  CONFIG_NATIVE_NUMBER(log_level, Number)
  CONFIG_NATIVE_NUMBER(log_type, Number)
  CONFIG_NATIVE_NUMBER(enable_log_uv_handles, Boolean)
  CONFIG_NATIVE_NUMBER(log_format_alinode, Boolean)
  CONFIG_NATIVE_NUMBER(patch_http, Boolean)
  CONFIG_NATIVE_NUMBER(patch_http_timeout, Number)
  CONFIG_NATIVE_NUMBER(check_throw, Boolean)
  CONFIG_NATIVE_NUMBER(enable_fatal_error_hook, Boolean)
  CONFIG_NATIVE_NUMBER(enable_fatal_error_report, Boolean)
  CONFIG_NATIVE_NUMBER(enable_fatal_error_coredump, Boolean)

  info.GetReturnValue().Set(config);
}

}  // namespace xprofiler
