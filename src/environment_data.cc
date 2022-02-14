#include "environment_data.h"

#include <memory>

#include "util.h"
#include "xpf_node.h"
#include "xpf_v8.h"

namespace xprofiler {
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

  HandleScope scope(isolate);
  uv_loop_t* loop = node::GetCurrentEventLoop(isolate);
  CHECK_NOT_NULL(loop);

  per_process::environment_data =
      std::unique_ptr<EnvironmentData>(new EnvironmentData(isolate, loop));
  xprofiler::AtExit(isolate, AtExit, nullptr);

  return per_process::environment_data.get();
}

EnvironmentData::EnvironmentData(v8::Isolate* isolate, uv_loop_t* loop)
    : isolate_(isolate), loop_(loop) {
  CHECK_EQ(0, uv_async_init(loop, &interrupt_async_, InterruptIdleCallback));
  uv_unref(reinterpret_cast<uv_handle_t*>(&interrupt_async_));
  CHECK_EQ(0, uv_async_init(loop, &statistics_async_, CollectStatistics));
  uv_unref(reinterpret_cast<uv_handle_t*>(&statistics_async_));
}

void EnvironmentData::AtExit(void* arg) {
  // TODO(legendecas): environment registry.
  // TODO(hyj1991): avoid 0xC0000005 on windows
  // per_process::environment_data.reset();
}

void EnvironmentData::SendCollectStatistics() {
  uv_async_send(&statistics_async_);
}

void EnvironmentData::RequestInterrupt(InterruptCallback interrupt) {
  {
    Mutex::ScopedLock lock(interrupt_mutex_);
    interrupt_requests_.push_back(interrupt);
  }
  isolate_->RequestInterrupt(InterruptBusyCallback, this);
  uv_async_send(&interrupt_async_);
}

void EnvironmentData::InterruptBusyCallback(v8::Isolate* isolate, void* data) {
  EnvironmentData* env_data = static_cast<EnvironmentData*>(data);
  std::list<InterruptCallback> requests;
  {
    Mutex::ScopedLock lock(env_data->interrupt_mutex_);
    requests.swap(env_data->interrupt_requests_);
  }

  for (auto it : requests) {
    it(env_data, InterruptKind::kBusy);
  }
}

void EnvironmentData::InterruptIdleCallback(uv_async_t* handle) {
  EnvironmentData* env_data =
      ContainerOf(&EnvironmentData::interrupt_async_, handle);
  std::list<InterruptCallback> requests;
  {
    Mutex::ScopedLock lock(env_data->interrupt_mutex_);
    requests.swap(env_data->interrupt_requests_);
  }

  for (auto it : requests) {
    it(env_data, InterruptKind::kIdle);
  }
}

void EnvironmentData::CollectStatistics(uv_async_t* handle) {
  EnvironmentData* env_data =
      ContainerOf(&EnvironmentData::statistics_async_, handle);
  CollectMemoryStatistics(env_data);
  CollectLibuvHandleStatistics(env_data);
}

}  // namespace xprofiler
