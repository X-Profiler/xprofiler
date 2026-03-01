#include "export_thread_logbypass.h"

#include "environment_data.h"
#include "logbypass/log.h"

namespace xprofiler {
using Nan::FunctionCallbackInfo;
using Nan::True;
using v8::Value;

void RunLogBypass(const FunctionCallbackInfo<Value>& info) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(info);
  StartLogThread(env_data);
  info.GetReturnValue().Set(True());
}

}  // namespace xprofiler
