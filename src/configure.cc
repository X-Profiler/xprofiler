#include "configure.h"

namespace xprofiler {
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

static string log_dir = "/tmp";
static uint32_t log_interval = 60;
static bool enable_log_uv_handles = true;
static bool log_format_alinode = false;
static LOG_LEVEL log_level = LOG_ERROR;

void Configure(const FunctionCallbackInfo<Value> &info) {
  if (!info[0]->IsObject()) {
    ThrowTypeError(New<String>("config must be object!").ToLocalChecked());
    return;
  }

  Local<Object> config = To<Object>(info[0]).ToLocalChecked();

  // set log dir
  Local<Value> log_dir_value =
      Get(config, New<String>("log_dir").ToLocalChecked()).ToLocalChecked();
  if (log_dir_value->IsString()) {
    Local<String> log_dir_string = To<String>(log_dir_value).ToLocalChecked();
    Utf8String log_dir_utf8string(log_dir_string);
    log_dir = *log_dir_utf8string;
  }

  // set log interval
  Local<Value> log_interval_value =
      Get(config, New<String>("log_interval").ToLocalChecked())
          .ToLocalChecked();
  if (log_interval_value->IsUint32()) {
    log_interval = To<uint32_t>(log_interval_value).ToChecked();
  }

  // enable collecting uv handles
  Local<Value> enable_log_uv_handles_value =
      Get(config, New<String>("enable_log_uv_handles").ToLocalChecked())
          .ToLocalChecked();
  if (enable_log_uv_handles_value->IsBoolean()) {
    enable_log_uv_handles = To<bool>(enable_log_uv_handles_value).ToChecked();
  }

  // log format: standard or alinode
  Local<Value> log_format_alinode_value =
      Get(config, New<String>("log_format_alinode").ToLocalChecked())
          .ToLocalChecked();
  if (log_format_alinode_value->IsBoolean()) {
    log_format_alinode = To<bool>(log_format_alinode_value).ToChecked();
  }

  // log level: 0 info, 1 error, 2 debug
  Local<Value> log_level_value =
      Get(config, New<String>("log_level").ToLocalChecked()).ToLocalChecked();
  if (log_level_value->IsUint32()) {
    log_level =
        static_cast<LOG_LEVEL>(To<uint32_t>(log_level_value).ToChecked());
  }

  info.GetReturnValue().Set(New<Boolean>(true));
}

void GetConfig(const FunctionCallbackInfo<Value> &info) {
  Local<Object> config = New<Object>();
  Set(config, New<String>("log_dir").ToLocalChecked(),
      New<String>(log_dir).ToLocalChecked());
  Set(config, New<String>("log_interval").ToLocalChecked(),
      New<Number>(log_interval));
  Set(config, New<String>("enable_log_uv_handles").ToLocalChecked(),
      New<Boolean>(enable_log_uv_handles));
  Set(config, New<String>("log_format_alinode").ToLocalChecked(),
      New<Boolean>(log_format_alinode));
  Set(config, New<String>("log_level").ToLocalChecked(),
      New<Number>(log_level));
  info.GetReturnValue().Set(config);
}

LOG_LEVEL GetLogLevel() { return log_level; }

bool GetFormatAsAlinode() { return log_format_alinode; }

std::string GetLogDir() { return log_dir; }
} // namespace xprofiler