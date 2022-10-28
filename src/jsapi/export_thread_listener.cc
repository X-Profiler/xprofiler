#include "export_thread_listener.h"

#include "commands/listener.h"
#include "environment_data.h"

namespace xprofiler {
using Nan::False;
using Nan::FunctionCallbackInfo;
using Nan::ThrowTypeError;
using Nan::True;
using v8::Value;

void RunCommandsListener(const FunctionCallbackInfo<Value>& info) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(info);
  int rc = StartCommandsListener(env_data);
  if (rc != 0) {
    ThrowTypeError("xprofiler: create uv commands listener thread failed!");
  }
  info.GetReturnValue().Set(rc == 0 ? True() : False());
}
}  // namespace xprofiler
