#include "export_configure.h"

#include "process_data.h"
#include "util-inl.h"
#include "xpf_v8.h"

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

#define GET_LOCAL_VALUE(key, v8_type)                                         \
  Local<v8_type> key =                                                        \
      To<v8_type>(Get(config, OneByteString(isolate, #key)).ToLocalChecked()) \
          .ToLocalChecked();

#define GET_NATIVE_VALUE(key, native_type)                            \
  native_type key =                                                   \
      To<native_type>(                                                \
          Get(config, OneByteString(isolate, #key)).ToLocalChecked()) \
          .ToChecked();

#define SET_LOCAL_VALUE(key, v8_type, native_type)                         \
  Set(config, OneByteString(isolate, key.c_str()),                         \
      New<v8_type>(                                                        \
          ProcessData::Get()->config_store()->GetConfig<native_type>(key)) \
          .ToLocalChecked());

#define SET_NATIVE_VALUE(key, v8_type, native_type) \
  Set(config, OneByteString(isolate, key.c_str()),  \
      New<v8_type>(                                 \
          ProcessData::Get()->config_store()->GetConfig<native_type>(key)));

void Configure(const FunctionCallbackInfo<Value>& info) {
  Isolate* isolate = info.GetIsolate();
  HandleScope scope(isolate);

  if (!info[0]->IsArray()) {
    ThrowTypeError(New<String>("config must be array!").ToLocalChecked());
    return;
  }
  Local<Array> configuration = Local<Array>::Cast(info[0]);

  for (uint32_t i = 0; i < configuration->Length(); i++) {
    Local<Object> config =
        To<Object>(Get(configuration, i).ToLocalChecked()).ToLocalChecked();

    GET_LOCAL_VALUE(name, String)
    GET_LOCAL_VALUE(format, String)
    GET_LOCAL_VALUE(configurable, Boolean)

    Utf8String name_s(name);
    Utf8String format_s(format);
    ProcessData::Get()->config_store()->DescribeConfig(
        *name_s, *format_s, To<bool>(configurable).ToChecked());

    // type string
    if (format->StrictEquals(OneByteString(isolate, "string"))) {
      GET_LOCAL_VALUE(value, String)
      Utf8String value_s(value);
      ProcessData::Get()->config_store()->SetConfig<std::string>(*name_s,
                                                                 *value_s);
    }

    // type uint32
    if (format->StrictEquals(OneByteString(isolate, "number"))) {
      GET_NATIVE_VALUE(value, uint32_t)
      ProcessData::Get()->config_store()->SetConfig<uint32_t>(*name_s, value);
    }

    // type bool
    if (format->StrictEquals(OneByteString(isolate, "boolean"))) {
      GET_NATIVE_VALUE(value, bool)
      ProcessData::Get()->config_store()->SetConfig<bool>(*name_s, value);
    }
  }

  info.GetReturnValue().Set(New<Boolean>(true));
}

void GetConfig(const FunctionCallbackInfo<Value>& info) {
  Isolate* isolate = info.GetIsolate();
  Local<Object> config = New<Object>();

  ProcessData::Get()->config_store()->TraverseConfig(
      [isolate, &config](string& key, string& type, bool configurable) {
        if (type == "string") SET_LOCAL_VALUE(key, String, string)
        if (type == "number") SET_NATIVE_VALUE(key, Number, uint32_t)
        if (type == "boolean") SET_NATIVE_VALUE(key, Boolean, bool)
      });

  info.GetReturnValue().Set(config);
}

}  // namespace xprofiler
