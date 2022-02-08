#include "environment_data.h"

#include <memory>

#include "util.h"
#include "xpf_node.h"

namespace xprofiler {
using Nan::HandleScope;
using v8::Isolate;

namespace per_process {
// TODO(legendecas): environment registry.
std::unique_ptr<EnvironmentData> environment_data;
}  // namespace per_process

EnvironmentData* EnvironmentData::GetCurrent() {
  CHECK_NE(per_process::environment_data, nullptr);
  return per_process::environment_data.get();
}

EnvironmentData* EnvironmentData::GetCurrent(v8::Isolate* isolate) {
  // TODO(legendecas): environment registry.
  CHECK_NE(per_process::environment_data, nullptr);
  return per_process::environment_data.get();
}

EnvironmentData* EnvironmentData::GetCurrent(
    const Nan::FunctionCallbackInfo<v8::Value>& info) {
  // TODO(legendecas): environment registry.
  CHECK_NE(per_process::environment_data, nullptr);
  return per_process::environment_data.get();
}

EnvironmentData* EnvironmentData::Create(v8::Isolate* isolate) {
  // TODO(legendecas): environment registry.
  CHECK_EQ(per_process::environment_data, nullptr);

  CHECK_EQ(isolate, Isolate::GetCurrent());
  HandleScope scope;

  per_process::environment_data =
      std::unique_ptr<EnvironmentData>(new EnvironmentData(isolate));
  xprofiler::AtExit(isolate, AtExit, nullptr);

  return per_process::environment_data.get();
}

EnvironmentData::EnvironmentData(v8::Isolate* isolate) : isolate_(isolate) {
  uv_loop_t* loop = node::GetCurrentEventLoop(isolate);
  CHECK_EQ(0, uv_async_init(loop, &statistics_async_, CollectStatistics));
  uv_unref(reinterpret_cast<uv_handle_t*>(&statistics_async_));
}

void EnvironmentData::AtExit(void* arg) {
  // TODO(legendecas): environment registry.
  per_process::environment_data.reset();
}

void EnvironmentData::SendCollectStatistics() {
  uv_async_send(&statistics_async_);
}

void EnvironmentData::CollectStatistics(uv_async_t* handle) {
  EnvironmentData* env_data =
      ContainerOf(&EnvironmentData::statistics_async_, handle);
  CollectMemoryStatistics(env_data);
  CollectLibuvHandleStatistics(env_data);
}

}  // namespace xprofiler
