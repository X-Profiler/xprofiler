#include "environment_data.h"

#include <memory>

#include "logbypass/log.h"
#include "process_data.h"
#include "util.h"
#include "xpf_node.h"
#include "xpf_v8.h"

namespace xprofiler {
using v8::Isolate;

EnvironmentData* EnvironmentData::GetCurrent() {
  // TODO(legendecas): environment registry.
  CHECK_NE(per_process::process_data.environment_data, nullptr);
  return per_process::process_data.environment_data.get();
}

EnvironmentData* EnvironmentData::GetCurrent(v8::Isolate* isolate) {
  return EnvironmentData::GetCurrent();
}

EnvironmentData* EnvironmentData::GetCurrent(
    const Nan::FunctionCallbackInfo<v8::Value>& info) {
  return EnvironmentData::GetCurrent(info.GetIsolate());
}

EnvironmentData* EnvironmentData::Create(v8::Isolate* isolate) {
  // TODO(legendecas): environment registry.
  CHECK_EQ(per_process::process_data.environment_data, nullptr);

  HandleScope scope(isolate);
  uv_loop_t* loop = node::GetCurrentEventLoop(isolate);
  CHECK_NOT_NULL(loop);

  per_process::process_data.environment_data =
      std::unique_ptr<EnvironmentData>(new EnvironmentData(isolate, loop));

  return per_process::process_data.environment_data.get();
}

EnvironmentData::EnvironmentData(v8::Isolate* isolate, uv_loop_t* loop)
    : isolate_(isolate), loop_(loop) {
  CHECK_EQ(0, uv_async_init(loop, &interrupt_async_, InterruptIdleCallback));
  uv_unref(reinterpret_cast<uv_handle_t*>(&interrupt_async_));
  CHECK_EQ(0, uv_async_init(loop, &statistics_async_, CollectStatistics));
  uv_unref(reinterpret_cast<uv_handle_t*>(&statistics_async_));
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
