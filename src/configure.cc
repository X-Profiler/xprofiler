#include "configure.h"

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
static LOG_LEVEL log_level = LOG_ERROR;
static LOG_TYPE log_type = LOG_TO_FILE;
static bool enable_log_uv_handles = true;
static bool log_format_alinode = false;
static bool enable_fatal_error_hook = true;
static bool enable_oom_hook = false;
static bool patch_http = true;
static uint32_t patch_http_timeout = 30;
static bool check_throw = true;

void Configure(const FunctionCallbackInfo<Value> &info) {
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
  CONVERT_BOOL(enable_fatal_error_hook)
  CONVERT_BOOL(enable_oom_hook)
  CONVERT_BOOL(patch_http)
  CONVERT_UINT32(patch_http_timeout)
  CONVERT_BOOL(check_throw)

  info.GetReturnValue().Set(New<Boolean>(true));
}

void GetConfig(const FunctionCallbackInfo<Value> &info) {
  Local<Object> config = New<Object>();

  CONFIG_LOCAL_STRING(log_dir, String)
  CONFIG_NATIVE_NUMBER(log_interval, Number)
  CONFIG_NATIVE_NUMBER(log_level, Number)
  CONFIG_NATIVE_NUMBER(log_type, Number)
  CONFIG_NATIVE_NUMBER(enable_log_uv_handles, Boolean)
  CONFIG_NATIVE_NUMBER(log_format_alinode, Boolean)
  CONFIG_NATIVE_NUMBER(enable_fatal_error_hook, Boolean)
  CONFIG_NATIVE_NUMBER(enable_oom_hook, Boolean)
  CONFIG_NATIVE_NUMBER(patch_http, Boolean)
  CONFIG_NATIVE_NUMBER(patch_http_timeout, Number)
  CONFIG_NATIVE_NUMBER(check_throw, Boolean)

  info.GetReturnValue().Set(config);
}

// define getter / setter
DEFINE_GET_SET_FUNCTION(LogDir, string, log_dir)
DEFINE_GET_SET_FUNCTION(LogInterval, uint32_t, log_interval)
DEFINE_GET_SET_FUNCTION(LogLevel, LOG_LEVEL, log_level)
DEFINE_GET_SET_FUNCTION(LogType, LOG_TYPE, log_type)
DEFINE_GET_SET_FUNCTION(FormatAsAlinode, bool, log_format_alinode)
DEFINE_GET_SET_FUNCTION(EnableLogUvHandles, bool, enable_log_uv_handles)
DEFINE_GET_SET_FUNCTION(EnableFatalErrorHook, bool, enable_fatal_error_hook)
DEFINE_GET_SET_FUNCTION(EnableOOMErrorHook, bool, enable_oom_hook)
DEFINE_GET_SET_FUNCTION(PatchHttp, bool, patch_http)
DEFINE_GET_SET_FUNCTION(PatchHttpTimeout, uint32_t, patch_http_timeout)
DEFINE_GET_SET_FUNCTION(CheckThrow, bool, check_throw)
}  // namespace xprofiler