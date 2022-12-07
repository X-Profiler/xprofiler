#include "export_configure.h"

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
using v8::Array;
using v8::Boolean;
using v8::Isolate;
using v8::Local;
using v8::Number;
using v8::Object;
using v8::String;
using v8::Value;

#define SET_LOCAL_VALUE(key, v8_type, native_type)                          \
  Set(config, OneByteString(isolate, #key),                                 \
      New<v8_type>(                                                         \
          ProcessData::Get()->config_store()->GetConfig<native_type>(#key)) \
          .ToLocalChecked());

#define SET_NATIVE_VALUE(key, v8_type, native_type) \
  Set(config, OneByteString(isolate, #key),         \
      New<v8_type>(                                 \
          ProcessData::Get()->config_store()->GetConfig<native_type>(#key)));

void Configure(const FunctionCallbackInfo<Value>& info) {
  Isolate* isolate = info.GetIsolate();
  if (!info[0]->IsArray()) {
    ThrowTypeError(New<String>("config must be array!").ToLocalChecked());
    return;
  }
  Local<Array> configuration = Local<Array>::Cast(info[0]);

  for (uint32_t i = 0; i < configuration->Length(); i++) {
    Local<Object> config =
        To<Object>(Get(configuration, i).ToLocalChecked()).ToLocalChecked();

    Local<String> format =
        To<String>(
            Get(config, OneByteString(isolate, "format")).ToLocalChecked())
            .ToLocalChecked();

    Local<String> name =
        To<String>(Get(config, OneByteString(isolate, "name")).ToLocalChecked())
            .ToLocalChecked();

    Utf8String name_s(name);

    // type string
    if (format->StrictEquals(OneByteString(isolate, "string"))) {
      Local<String> value =
          To<String>(
              Get(config, OneByteString(isolate, "value")).ToLocalChecked())
              .ToLocalChecked();
      Utf8String value_s(value);
      ProcessData::Get()->config_store()->SetConfig<std::string>(*name_s,
                                                                 *value_s);
    }

    // type uint32
    if (format->StrictEquals(OneByteString(isolate, "number"))) {
      uint32_t value =
          To<uint32_t>(
              Get(config, OneByteString(isolate, "value")).ToLocalChecked())
              .ToChecked();
      ProcessData::Get()->config_store()->SetConfig<uint32_t>(*name_s, value);
    }

    // type bool
    if (format->StrictEquals(OneByteString(isolate, "boolean"))) {
      bool value =
          To<bool>(
              Get(config, OneByteString(isolate, "value")).ToLocalChecked())
              .ToChecked();
      ProcessData::Get()->config_store()->SetConfig<bool>(*name_s, value);
    }
  }

  info.GetReturnValue().Set(New<Boolean>(true));
}

void GetConfig(const FunctionCallbackInfo<Value>& info) {
  Isolate* isolate = info.GetIsolate();
  Local<Object> config = New<Object>();

  SET_LOCAL_VALUE(log_dir, String, string)
  SET_NATIVE_VALUE(log_interval, Number, uint32_t)
  SET_NATIVE_VALUE(log_level, Number, LOG_LEVEL)
  SET_NATIVE_VALUE(log_type, Number, LOG_TYPE)
  SET_NATIVE_VALUE(log_format_alinode, Boolean, bool)
  SET_NATIVE_VALUE(patch_http, Boolean, bool)
  SET_NATIVE_VALUE(patch_http_timeout, Number, uint32_t)
  SET_NATIVE_VALUE(check_throw, Boolean, bool)
  SET_NATIVE_VALUE(enable_log_uv_handles, Boolean, bool)
  SET_NATIVE_VALUE(enable_fatal_error_hook, Boolean, bool)
  SET_NATIVE_VALUE(enable_fatal_error_report, Boolean, bool)
  SET_NATIVE_VALUE(enable_fatal_error_coredump, Boolean, bool)
  SET_NATIVE_VALUE(enable_http_profiling, Boolean, bool)

  info.GetReturnValue().Set(config);
}

}  // namespace xprofiler
