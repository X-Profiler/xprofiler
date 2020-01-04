#ifndef _SRC_CONFIGURE_H
#define _SRC_CONFIGURE_H

#include "library/common.h"
#include "library/error.h"
#include "nan.h"

namespace xprofiler {
using Nan::FunctionCallbackInfo;
using std::string;
using v8::Value;

#define DECLARE_GETTER_FUNCTION(func_name, type) type Get##func_name()

#define DECLARE_SETTER_FUNCTION(func_name, type) void Set##func_name(type value)

#define DECLARE_GET_SET_FUNCTION(func_name, type) \
  DECLARE_GETTER_FUNCTION(func_name, type);       \
  DECLARE_SETTER_FUNCTION(func_name, type);

#define DEFINE_GET_SET_FUNCTION(func_name, type, vari)      \
  DECLARE_GETTER_FUNCTION(func_name, type) { return vari; } \
  DECLARE_SETTER_FUNCTION(func_name, type) { vari = value; }

#define LOCAL_VALUE(key)     \
  Local<Value> key##_value = \
      Get(config, New<String>(#key).ToLocalChecked()).ToLocalChecked();

#define COVERT_STRING(key)                                                 \
  LOCAL_VALUE(key)                                                         \
  if (key##_value->IsString()) {                                           \
    Local<String> key##_string = To<String>(key##_value).ToLocalChecked(); \
    Utf8String key##_utf8string(key##_string);                             \
    key = *key##_utf8string;                                               \
  }

#define CONVERT_UINT32(key) \
  LOCAL_VALUE(key)          \
  if (key##_value->IsUint32()) key = To<uint32_t>(key##_value).ToChecked();

#define CONVERT_UINT32_WITH_TYPE(key, type) \
  LOCAL_VALUE(key)                          \
  if (key##_value->IsUint32())              \
    key = static_cast<type>(To<uint32_t>(key##_value).ToChecked());

#define CONVERT_BOOL(key) \
  LOCAL_VALUE(key)        \
  if (key##_value->IsBoolean()) key = To<bool>(key##_value).ToChecked();

#define CONFIG_LOCAL_STRING(key, type)            \
  Set(config, New<String>(#key).ToLocalChecked(), \
      New<type>(key).ToLocalChecked());

#define CONFIG_NATIVE_NUMBER(key, type) \
  Set(config, New<String>(#key).ToLocalChecked(), New<type>(key));

// declare getter / setter
DECLARE_GET_SET_FUNCTION(LogDir, string)
DECLARE_GET_SET_FUNCTION(LogInterval, uint32_t)
DECLARE_GET_SET_FUNCTION(LogLevel, LOG_LEVEL)
DECLARE_GET_SET_FUNCTION(LogType, LOG_TYPE)
DECLARE_GET_SET_FUNCTION(FormatAsAlinode, bool)
DECLARE_GET_SET_FUNCTION(EnableLogUvHandles, bool)
DECLARE_GET_SET_FUNCTION(EnableFatalErrorHook, bool)
DECLARE_GET_SET_FUNCTION(PatchHttp, bool)

// javascript accessible
void Configure(const FunctionCallbackInfo<Value> &info);
void GetConfig(const FunctionCallbackInfo<Value> &info);
}  // namespace xprofiler

#endif