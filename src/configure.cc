#include "library/common.h"
#include "nan.h"

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
using v8::Local;
using v8::Number;
using v8::Object;
using v8::String;
using v8::Value;

static string log_dir = "/tmp";
static uint32_t log_interval = 60;
static bool enable_log_uv_handles = true;
static bool log_format_alinode = false;
static LOG_LEVEL log_level = LOG_ERROR;
static LOG_TYPE log_type = LOG_TO_FILE;

#define COVERT_STRING(key)                                                 \
  if (key##_value->IsString()) {                                           \
    Local<String> key##_string = To<String>(key##_value).ToLocalChecked(); \
    Utf8String key##_utf8string(key##_string);                             \
    key = *key##_utf8string;                                               \
  }

#define CONVERT_UINT32(key) \
  if (key##_value->IsUint32()) key = To<uint32_t>(key##_value).ToChecked();

#define CONVERT_UINT32_V2(key, type) \
  if (key##_value->IsUint32())       \
    key = static_cast<type>(To<uint32_t>(key##_value).ToChecked());

#define CONVERT_BOOL(key) \
  if (key##_value->IsBoolean()) key = To<bool>(key##_value).ToChecked();

void Configure(const FunctionCallbackInfo<Value> &info) {
  if (!info[0]->IsObject()) {
    ThrowTypeError(New<String>("config must be object!").ToLocalChecked());
    return;
  }
  Local<Object> config = To<Object>(info[0]).ToLocalChecked();
#define S(key)               \
  Local<Value> key##_value = \
      Get(config, New<String>(#key).ToLocalChecked()).ToLocalChecked();
#define V(key, cvrt) S(key) cvrt(key)
#define W(key, cvrt, type) S(key) cvrt(key, type)
  // set log dir
  V(log_dir, COVERT_STRING)

  // set log interval
  V(log_interval, CONVERT_UINT32)

  // enable collecting uv handles
  V(enable_log_uv_handles, CONVERT_BOOL)

  // log format: standard or alinode
  V(log_format_alinode, CONVERT_BOOL)

  // log level: 0 info, 1 error, 2 debug
  W(log_level, CONVERT_UINT32_V2, LOG_LEVEL)

  // log type: 0 file, 1 ttl
  W(log_type, CONVERT_UINT32_V2, LOG_TYPE)
#undef S
#undef V
#undef W
  info.GetReturnValue().Set(New<Boolean>(true));
}

void GetConfig(const FunctionCallbackInfo<Value> &info) {
  Local<Object> config = New<Object>();
#define V(key, type)                              \
  Set(config, New<String>(#key).ToLocalChecked(), \
      New<type>(key).ToLocalChecked());
#define W(key, type) \
  Set(config, New<String>(#key).ToLocalChecked(), New<type>(key));
  V(log_dir, String)
  W(log_interval, Number)
  W(enable_log_uv_handles, Boolean)
  W(log_format_alinode, Boolean)
  W(log_level, Number)
  W(log_type, Number)
#undef V
#undef W
  info.GetReturnValue().Set(config);
}

#define V(ret, func, vari)         \
  ret Get##func() { return vari; } \
  void Set##func(ret value) { vari = value; }
V(string, LogDir, log_dir)
V(uint32_t, LogInterval, log_interval)
V(bool, FormatAsAlinode, log_format_alinode)
V(bool, EnableLogUvHandles, enable_log_uv_handles)
V(LOG_LEVEL, LogLevel, log_level)
V(LOG_TYPE, LogType, log_type)
#undef V
}  // namespace xprofiler