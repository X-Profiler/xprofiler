#include "include/export_environment.h"

#include "environment_data.h"
#include "util-inl.h"
#include "xpf_v8.h"

namespace xprofiler {
using v8::Boolean;
using v8::Context;
using v8::Isolate;
using v8::Local;
using v8::Number;
using v8::Object;

void JsSetupEnvironmentData(const Nan::FunctionCallbackInfo<v8::Value>& info) {
  Isolate* isolate = info.GetIsolate();
  HandleScope scope(isolate);
  Local<Context> context = isolate->GetCurrentContext();

  Local<Object> data = info[0].As<Object>();
  Local<Number> thread_id =
      data->Get(context, OneByteString(isolate, "threadId"))
          .ToLocalChecked()
          .As<Number>();
  Local<Boolean> is_main_thread =
      data->Get(context, OneByteString(isolate, "isMainThread"))
          .ToLocalChecked()
          .As<Boolean>();
  Local<v8::String> node_version =
      data->Get(context, OneByteString(isolate, "nodeVersion"))
          .ToLocalChecked()
          .As<v8::String>();

  Nan::Utf8String node_version_string(node_version);

  EnvironmentData::JsSetupEnvironmentData(isolate, is_main_thread->Value(),
                                          thread_id->Value(),
                                          (*node_version_string));
}
}  // namespace xprofiler