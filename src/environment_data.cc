#include "environment_data.h"

#include <memory>

#include "logbypass/log.h"
#include "process_data.h"
#include "util.h"
#include "xpf_node.h"
#include "xpf_v8.h"

namespace xprofiler {
using v8::Isolate;

// static
EnvironmentData* EnvironmentData::GetCurrent() {
  EnvironmentRegistry* registry = ProcessData::Get()->environment_registry();
  EnvironmentRegistry::NoExitScope scope(registry);

  CHECK_NE(registry->begin(), registry->end());
  return *registry->begin();
}

// static
EnvironmentData* EnvironmentData::GetCurrent(v8::Isolate* isolate) {
  EnvironmentRegistry* registry = ProcessData::Get()->environment_registry();
  EnvironmentRegistry::NoExitScope scope(registry);
  return registry->Get(isolate);
}

// static
EnvironmentData* EnvironmentData::GetCurrent(
    const Nan::FunctionCallbackInfo<v8::Value>& info) {
  return EnvironmentData::GetCurrent(info.GetIsolate());
}

// static
void EnvironmentData::Create(v8::Isolate* isolate) {
  EnvironmentRegistry* registry = ProcessData::Get()->environment_registry();
  EnvironmentRegistry::NoExitScope no_exit(registry);

  // TODO(legendecas): context awareness support.
  CHECK_EQ(registry->begin(), registry->end());

  HandleScope scope(isolate);
  uv_loop_t* loop = node::GetCurrentEventLoop(isolate);
  CHECK_NOT_NULL(loop);

  registry->Register(isolate, std::unique_ptr<EnvironmentData>(
                                  new EnvironmentData(isolate, loop)));
  xprofiler::AtExit(isolate, AtExit, isolate);
}

EnvironmentData::EnvironmentData(v8::Isolate* isolate, uv_loop_t* loop)
    : isolate_(isolate), loop_(loop) {
  CHECK_EQ(0, uv_async_init(loop, &interrupt_async_, InterruptIdleCallback));
  uv_unref(reinterpret_cast<uv_handle_t*>(&interrupt_async_));
  CHECK_EQ(0, uv_async_init(loop, &statistics_async_, CollectStatistics));
  uv_unref(reinterpret_cast<uv_handle_t*>(&statistics_async_));
}

// static
void EnvironmentData::AtExit(void* arg) {
  Isolate* isolate = reinterpret_cast<Isolate*>(arg);
  EnvironmentRegistry* registry = ProcessData::Get()->environment_registry();
  EnvironmentRegistry::NoExitScope scope(registry);
  registry->Unregister(isolate);
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

// static
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

// static
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

// static
void EnvironmentData::CollectStatistics(uv_async_t* handle) {
  EnvironmentData* env_data =
      ContainerOf(&EnvironmentData::statistics_async_, handle);
  CollectMemoryStatistics(env_data);
  CollectLibuvHandleStatistics(env_data);
}

}  // namespace xprofiler
